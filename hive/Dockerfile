# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-base AS builder

# Apache Hive up t0 4.x(!) officially requires Java 8 (there is no distincion between building and running). As of
# 2024-04-15 we for sure need Java 8 for building, but we used a Java 11 runtime for months now without any problems.
# As we got weird TLS errors (https://stackable-workspace.slack.com/archives/C031A5BEFS7/p1713185172557459) with a
# Java 8 runtime we bumped the Runtime to Java 11 again. As we can only select a single version from the java-base
# image, we pick 11 (which is used in the final image), and install Java 8 here.
RUN microdnf update && \
    microdnf remove java-11-openjdk-headless java-11-openjdk-devel java-11-openjdk && \
    microdnf install java-1.8.0-openjdk-headless java-1.8.0-openjdk-devel --nodocs && \
    microdnf clean all
ENV JAVA_HOME=/usr/lib/jvm/jre-1.8.0

ARG PRODUCT
ARG HADOOP
ARG JMX_EXPORTER
ARG JACKSON_DATAFORMAT_XML
ARG POSTGRES_DRIVER
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE


RUN microdnf update && \
    microdnf install \
    # For the apply_patches.sh script
    git \
    gzip \
    java-"${JAVA_VERSION}"-openjdk-devel \
    maven \
    tar \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY --chown=stackable:stackable hive/stackable /stackable

USER stackable
WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT}-src.tar.gz" | tar -xzC . && \
    patches/apply_patches.sh ${PRODUCT} && \
    cd /stackable/apache-hive-${PRODUCT}-src/ && \
    mvn clean package -DskipTests --projects standalone-metastore && \
    mv standalone-metastore/target/apache-hive-metastore-${PRODUCT}-bin/apache-hive-metastore-${PRODUCT}-bin /stackable && \
    ln -s /stackable/apache-hive-metastore-${PRODUCT}-bin/ /stackable/hive-metastore && \
    cp /stackable/hive-metastore/bin/start-metastore /stackable/hive-metastore/bin/start-metastore.bak && \
    cp /stackable/bin/start-metastore /stackable/hive-metastore/bin && \
    rm -rf /stackable/apache-hive-${PRODUCT}-src

COPY --chown=stackable:stackable --from=hadoop-builder /stackable/hadoop /stackable/hadoop

# Add a PostgreSQL driver, as this is the primary used persistence
RUN curl --fail -L https://repo.stackable.tech/repository/packages/pgjdbc/postgresql-${POSTGRES_DRIVER}.jar -o /stackable/hive-metastore/lib/postgresql-${POSTGRES_DRIVER}.jar

# The next two sections for S3 and Azure use hardcoded version numbers on purpose instead of wildcards
# This way the build will fail should one of the files not be available anymore in a later Hadoop version!

# Add S3 Support for Hive (support for s3a://)
RUN cp /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar /stackable/hive-metastore/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar /stackable/hive-metastore/lib/

# Add Azure ABFS support (support for abfs://)
RUN cp /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar /stackable/hive-metastore/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE}.jar /stackable/hive-metastore/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar /stackable/hive-metastore/lib/

# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HDFS Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HDFS operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HDFS Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

# Logging
RUN rm /stackable/hive-metastore/lib/log4j-slf4j-impl* && \
    curl --fail -L https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar -o /stackable/hive-metastore/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar

# ===
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/apache-hive-metastore-${PRODUCT}-bin/

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
# log4shell_scanner does not work on symlinks!
RUN /bin/log4shell_scanner s /stackable/apache-hive-metastore-${PRODUCT}-bin/
# ===

# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base

ARG PRODUCT
ARG HADOOP
ARG RELEASE

LABEL name="Apache Hive metastore" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Hive metastore." \
      description="This image is deployed by the Stackable Operator for Apache Hive."

RUN microdnf update && \
    microdnf clean all && \
    rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

# TODO: Try to use --link here, as it should be faster
COPY --chown=stackable:stackable --from=builder /stackable/apache-hive-metastore-${PRODUCT}-bin /stackable/apache-hive-metastore-${PRODUCT}-bin
RUN ln -s /stackable/apache-hive-metastore-${PRODUCT}-bin/ /stackable/hive-metastore

# It is useful to see which version of Hadoop is used at a glance
# Therefore the use of the full name here
COPY --chown=stackable:stackable --from=builder /stackable/hadoop /stackable/hadoop-${HADOOP}
RUN ln -s /stackable/hadoop-${HADOOP}/ /stackable/hadoop

COPY --chown=stackable:stackable --from=builder /stackable/jmx /stackable/jmx
COPY hive/licenses /licenses

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

ENV HADOOP_HOME=/stackable/hadoop
ENV HIVE_HOME=/stackable/hive-metastore
ENV PATH="${PATH}":/stackable/hadoop/bin:/stackable/hive-metastore/bin

WORKDIR /stackable/hive-metastore
# Start command is set by operator to something like "bin/start-metastore --config /stackable/config --db-type postgres --hive-bin-dir bin"
