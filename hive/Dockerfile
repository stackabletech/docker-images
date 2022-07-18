FROM docker.stackable.tech/stackable/java-base:11-stackable0

ARG PRODUCT
ARG HADOOP
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
ARG RELEASE="1"


LABEL name="apache hive" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Hive." \
      description="This image is deployed by the Stackable Operator for Apache Hive."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip which shadow-utils && \
    microdnf clean all

COPY hive/stackable /stackable
COPY hive/licenses /licenses

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable
WORKDIR /stackable

# Download hive and hadoop
RUN curl -L https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-hive-${PRODUCT}-bin/apache-hive-${PRODUCT}-bin /stackable/hive && \
    ln -s /stackable/apache-hive-${PRODUCT}-bin/hadoop-${HADOOP} /stackable/hadoop && \
    # Force to overwrite the existing 'start-metastore'
    ln -sf /stackable/bin/start-metastore /stackable/hive/bin/start-metastore

# Download aws module for Hadoop (support for s3a://)
RUN curl -L https://repo.stackable.tech/repository/packages/aws/hadoop-aws-${HADOOP}.jar \
    -o /stackable/hive/lib/hadoop-aws-${HADOOP}.jar && \
    chmod -x /stackable/hive/lib/hadoop-aws-${HADOOP}.jar

# Download aws sdk bundle containing all the needed S3 Classes for hadoop-aws. Must match version hadoop-aws was compiled against
RUN curl -L https://repo.stackable.tech/repository/packages/aws/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    -o /stackable/hive/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar && \
    chmod -x /stackable/hive/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar

# Download azure module for Hadoop (support for abfs://)
RUN curl -L https://repo.stackable.tech/repository/packages/aws/hadoop-azure-${HADOOP}.jar \
    -o /stackable/hive/lib/hadoop-azure-${HADOOP}.jar && \
    chmod -x /stackable/hive/lib/hadoop-azure-${HADOOP}.jar

# Download azure libs containing all the needed ABFS Classes for hadoop-azure. Must match version hadoop-azure was compiled against
RUN curl -L https://repo.stackable.tech/repository/packages/azure/azure-storage-"${AZURE_STORAGE}".jar \
    -o /stackable/hive/lib/azure-storage-${AZURE-STORAGE}.jar && \
    chmod -x /stackable/hive/lib/azure-storage-${AZURE-STORAGE}.jar
RUN curl -L https://repo.stackable.tech/repository/packages/azure/azure-keyvault-core-"${AZURE_KEYVAULT_CORE}".jar \
    -o /stackable/hive/lib/azure-keyvault-core-${AZURE-KEYVAULT}.jar && \
    chmod -x /stackable/hive/lib/azure-keyvault-core-${AZURE-KEYVAULT}.jar

# Download jmx exporter (needed for metrics)
RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

ENV HADOOP_HOME=/stackable/hadoop

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

WORKDIR /stackable/hive
CMD ["./bin/start-metastore", "--config", "conf", "--hive-bin-dir", "bin", "--db-type", "derby"]
