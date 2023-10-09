# syntax=docker/dockerfile:1
FROM stackable/image/java-base

ARG PRODUCT
ARG JMX_EXPORTER
ARG HADOOP_LIBS
ARG JACKSON_DATAFORMAT_XML
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
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
    microdnf install tar gzip zip which && \
    microdnf clean all

COPY --chown=stackable:stackable hive/stackable /stackable
COPY hive/licenses /licenses

USER stackable
WORKDIR /stackable

# Download hive and hadoop
RUN curl --fail -L https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-hive-${PRODUCT}-bin/apache-hive-${PRODUCT}-bin /stackable/hive && \
    ln -s /stackable/apache-hive-${PRODUCT}-bin/hadoop-${HADOOP_LIBS} /stackable/hadoop && \
    # Force to overwrite the existing 'start-metastore'
    ln -sf /stackable/bin/start-metastore /stackable/hive/bin/start-metastore

# Download aws module for Hadoop (support for s3a://)
RUN curl --fail -L https://repo.stackable.tech/repository/packages/aws/hadoop-aws-${HADOOP_LIBS}.jar \
    -o /stackable/hive/lib/hadoop-aws-${HADOOP_LIBS}.jar && \
    chmod -x /stackable/hive/lib/hadoop-aws-${HADOOP_LIBS}.jar

# Download aws sdk bundle containing all the needed S3 Classes for hadoop-aws. Must match version hadoop-aws was compiled against
RUN curl --fail -L https://repo.stackable.tech/repository/packages/aws/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    -o /stackable/hive/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar && \
    chmod -x /stackable/hive/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar

# Download azure module for Hadoop (support for abfs://)
RUN curl --fail -L https://repo.stackable.tech/repository/packages/aws/hadoop-azure-${HADOOP_LIBS}.jar \
    -o /stackable/hive/lib/hadoop-azure-${HADOOP_LIBS}.jar && \
    chmod -x /stackable/hive/lib/hadoop-azure-${HADOOP_LIBS}.jar

# Download azure libs containing all the needed ABFS Classes for hadoop-azure. Must match version hadoop-azure was compiled against
RUN curl --fail -L https://repo.stackable.tech/repository/packages/azure/azure-storage-${AZURE_STORAGE}.jar \
    -o /stackable/hive/lib/azure-storage-${AZURE_STORAGE}.jar && \
    chmod -x /stackable/hive/lib/azure-storage-${AZURE_STORAGE}.jar
RUN curl --fail -L https://repo.stackable.tech/repository/packages/azure/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar \
    -o /stackable/hive/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar && \
    chmod -x /stackable/hive/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar

# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HDFS Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HDFS operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HDFS Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
RUN curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod -x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

# Logging
RUN rm /stackable/hive/lib/log4j-slf4j-impl* && \
    curl --fail https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
    -o /stackable/hive/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar && \
    chmod -x /stackable/hive/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar

ENV HADOOP_HOME=/stackable/hadoop
ENV HIVE_HOME=/stackable/hive

# ===
# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/hive

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/hive
# ===

WORKDIR /stackable/hive
CMD ["./bin/start-metastore", "--config", "conf", "--hive-bin-dir", "bin", "--db-type", "derby"]
