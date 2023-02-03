FROM registry.access.redhat.com/ubi8/ubi-minimal:8.7@sha256:c7036d048252451451bfc70493c2a849737e91c5517559ec508a0dbe7dfb6d15 AS builder

ARG PRODUCT
ARG HADOOP_SHORT_VERSION
ARG HADOOP_LONG_VERSION
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
ARG RELEASE

LABEL name="Apache Spark" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Spark on Kubernetes." \
      description="This image is deployed by the Stackable Operator for Apache Spark on Kubernetes."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip zip && \
    microdnf install shadow-utils && \
    # required for spark startup scripts
    microdnf install procps && \
    microdnf install hostname && \
    microdnf clean all

ENV HOME=/stackable
ENV JAVA_HOME=/usr/lib/jvm/jre-11
ENV SPARK_HOME=/stackable/spark
ENV PATH=$SPARK_HOME:$PATH:/bin:$JAVA_HOME/bin:$JAVA_HOME/jre/bin:$HOME/.local/bin

# according to arch, copy binary to the name "tini"
RUN curl -o /usr/bin/tini https://repo.stackable.tech/repository/packages/tini/tini-$(arch) 

COPY spark-k8s/stackable /stackable
COPY spark-k8s/licenses /licenses
  
RUN  chmod +x /usr/bin/tini

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 -d /stackable stackable && \
    chown -R stackable:stackable /stackable

USER stackable

WORKDIR /stackable
# Download Spark (including Hadoop)
RUN curl -L https://repo.stackable.tech/repository/packages/spark/spark-${PRODUCT}-bin-hadoop${HADOOP_SHORT_VERSION}.tgz | tar -xzC . && \
    ln -s /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP_SHORT_VERSION} /stackable/spark

# Download aws module for Hadoop (support for s3a://)
RUN curl -L https://repo.stackable.tech/repository/packages/aws/hadoop-aws-${HADOOP_LONG_VERSION}.jar \
    -o /stackable/spark/jars/hadoop-aws-${HADOOP_LONG_VERSION}.jar && \
    chmod -x /stackable/spark/jars/hadoop-aws-${HADOOP_LONG_VERSION}.jar

# Download aws sdk bundle containing all the needed S3 Classes for hadoop-aws. Must match version hadoop-aws was compiled against
RUN curl -L https://repo.stackable.tech/repository/packages/aws/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    -o /stackable/spark/jars/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar && \
    chmod -x /stackable/spark/jars/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar

# Download azure module for Hadoop (support for abfs://)
RUN curl -L https://repo.stackable.tech/repository/packages/aws/hadoop-azure-${HADOOP_LONG_VERSION}.jar \
    -o /stackable/spark/jars/hadoop-azure-${HADOOP_LONG_VERSION}.jar && \
    chmod -x /stackable/spark/jars/hadoop-azure-${HADOOP_LONG_VERSION}.jar

# Download azure libs containing all the needed ABFS Classes for hadoop-azure. Must match version hadoop-azure was compiled against
RUN curl -L https://repo.stackable.tech/repository/packages/azure/azure-storage-${AZURE_STORAGE}.jar \
    -o /stackable/spark/jars/azure-storage-${AZURE_STORAGE}.jar && \
    chmod -x /stackable/spark/jars/azure-storage-${AZURE_STORAGE}.jar
RUN curl -L https://repo.stackable.tech/repository/packages/azure/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar \
    -o /stackable/spark/jars/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar && \
    chmod -x /stackable/spark/jars/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar

# Download jmx exporter (needed for metrics)
RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

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
RUN /bin/log4shell.sh /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP_SHORT_VERSION}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP_SHORT_VERSION}
# ===

WORKDIR /stackable/spark
ENTRYPOINT [ "/stackable/spark/kubernetes/dockerfiles/spark/entrypoint.sh" ]
