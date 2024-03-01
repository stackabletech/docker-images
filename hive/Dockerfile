# syntax=docker/dockerfile:1

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-base AS builder

ARG PRODUCT
ARG HADOOP
ARG JMX_EXPORTER
ARG JACKSON_DATAFORMAT_XML
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE


RUN microdnf update && \
    microdnf install \
    # For the apply_patches.sh script
    git \
    gzip \
    maven \
    tar \
    zip && \
    microdnf clean all

COPY --chown=stackable:stackable hive/stackable /stackable
COPY hive/licenses /licenses

USER stackable
WORKDIR /stackable

SHELL ["bash", "-euo", "pipefail", "-c"]

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT}-src.tar.gz" | tar -xzC .
RUN chmod +x patches/apply_patches.sh
RUN patches/apply_patches.sh ${PRODUCT}
RUN cd /stackable/apache-hive-${PRODUCT}-src/ && \
    mvn clean package -DskipTests -Pdist
RUN cd /stackable/apache-hive-${PRODUCT}-src/ && \
    tar -xzf packaging/target/apache-hive-${PRODUCT}-bin.tar.gz -C /stackable && \
    mv /stackable/apache-hive-${PRODUCT}-bin /stackable/apache-hive-${PRODUCT} && \
    ln -s /stackable/apache-hive-${PRODUCT}/ /stackable/hive && \
    cp /stackable/bin/start-metastore /stackable/hive/bin


# TODO: Remove hardcoded _new_ version
# Replace the old (postgresql-9.4.1208.jre7.jar) postgresql JDBC driver with a newer one, as the old one does only support MD5 based authentication.
# Because of this, the contained driver version does not work against more recent PostgresQL versions.
# See https://github.com/stackabletech/hive-operator/issues/170 for details.
# Note: We hardcode the versions here to make sure this replacement will be removed once Hive ships with a more recent driver
# version as the "rm" statement will fail.
RUN rm /stackable/apache-hive-${PRODUCT}/lib/postgresql-9.4.1208.jre7.jar && \
    curl --fail -L https://repo.stackable.tech/repository/packages/pgjdbc/postgresql-42.7.2.jar -o /stackable/hive/lib/postgresql-42.7.2.jar


COPY --link --from=hadoop-builder /stackable/hadoop /stackable/hadoop

# The next two sections for S3 and Azure use hardcoded version numbers on purpose instead of wildcards
# This way the build will fail should one of the files not be available anymore in a later Hadoop version!

# Add S3 Support for Hive (support for s3a://)
RUN cp /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar /stackable/hive/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar /stackable/hive/lib/

# Add Azure ABFS support (support for abfs://)
RUN cp /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar /stackable/hive/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE}.jar /stackable/hive/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar /stackable/hive/lib/

# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HDFS Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HDFS operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HDFS Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
RUN curl --fail -L -O "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

# Logging
RUN rm /stackable/hive/lib/log4j-slf4j-impl* && \
    curl --fail -L https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar -o /stackable/hive/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar

# ===
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/apache-hive-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/apache-hive-${PRODUCT}
# ===

# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base

ARG PRODUCT
ARG HADOOP
ARG RELEASE

LABEL name="Apache Hive" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Hive." \
      description="This image is deployed by the Stackable Operator for Apache Hive."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf clean all && \
    rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --link --from=builder /stackable/apache-hive-${PRODUCT} /stackable/apache-hive-${PRODUCT}
RUN ln -s /stackable/apache-hive-${PRODUCT}/ /stackable/hive

# It is useful to see which version of Hadoop is used at a glance
# Therefore the use of the full name here
COPY --link --from=builder /stackable/hadoop /stackable/hadoop-${HADOOP}
RUN ln -s /stackable/hadoop-${HADOOP}/ /stackable/hadoop

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

ENV HADOOP_HOME=/stackable/hadoop
ENV HIVE_HOME=/stackable/hive

WORKDIR /stackable/hive
CMD ["./bin/start-metastore", "--config", "conf", "--hive-bin-dir", "bin", "--db-type", "derby"]
