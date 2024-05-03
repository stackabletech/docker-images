# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/java-base

ARG PRODUCT
ARG PYTHON
ARG SPARK
ARG HADOOP_SHORT_VERSION
ARG HADOOP_LONG_VERSION
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG JMX_EXPORTER
ARG RELEASE
ARG TARGETARCH
ARG TINI

LABEL name="Apache Spark" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache Spark with PySpark support." \
    description="This image is deployed by the Stackable Operator for Apache Spark on Kubernetes."

RUN microdnf update && \
    microdnf install \
    gzip \
    hostname \
    # required for spark startup scripts
    procps \
    python${PYTHON} \
    python${PYTHON}-pip && \
    tar \
    zip \
    microdnf clean all \
    && rm -rf /var/cache/yum

RUN ln -s /usr/bin/python3 /usr/bin/python && \
    ln -s /usr/bin/pip3 /usr/bin/pip

ENV HOME=/stackable
ENV SPARK_HOME=/stackable/spark
ENV PATH=$SPARK_HOME:$PATH:/bin:$JAVA_HOME/bin:$JAVA_HOME/jre/bin:$HOME/.local/bin
ENV PYSPARK_PYTHON=/usr/bin/python
ENV PYTHONPATH=$SPARK_HOME/python:$PYTHONPATH

# Get the correct `tini` binary for our architecture.
RUN curl --fail -o /usr/bin/tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI}-${TARGETARCH}"

COPY spark-k8s/stackable /stackable
COPY spark-k8s/licenses /licenses

RUN chmod +x /usr/bin/tini

RUN chown -R stackable:stackable /stackable

USER stackable

WORKDIR /stackable
# Download Spark (including Hadoop)
RUN curl -L --fail https://repo.stackable.tech/repository/packages/spark/spark-${SPARK}-bin-hadoop${HADOOP_SHORT_VERSION}.tgz | tar -xzC . && \
    ln -s /stackable/spark-${SPARK}-bin-hadoop${HADOOP_SHORT_VERSION} /stackable/spark

# Download aws module for Hadoop (support for s3a://)
RUN curl -L --fail https://repo.stackable.tech/repository/packages/aws/hadoop-aws-${HADOOP_LONG_VERSION}.jar \
    -o /stackable/spark/jars/hadoop-aws-${HADOOP_LONG_VERSION}.jar && \
    chmod -x /stackable/spark/jars/hadoop-aws-${HADOOP_LONG_VERSION}.jar

# Download aws sdk bundle containing all the needed S3 Classes for hadoop-aws. Must match version hadoop-aws was compiled against
RUN curl -L --fail https://repo.stackable.tech/repository/packages/aws/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    -o /stackable/spark/jars/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar && \
    chmod -x /stackable/spark/jars/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar

# Download azure module for Hadoop (support for abfs://)
RUN curl -L --fail https://repo.stackable.tech/repository/packages/azure/hadoop-azure-${HADOOP_LONG_VERSION}.jar \
    -o /stackable/spark/jars/hadoop-azure-${HADOOP_LONG_VERSION}.jar && \
    chmod -x /stackable/spark/jars/hadoop-azure-${HADOOP_LONG_VERSION}.jar

# Download azure libs containing all the needed ABFS Classes for hadoop-azure. Must match version hadoop-azure was compiled against
RUN curl -L --fail https://repo.stackable.tech/repository/packages/azure/azure-storage-${AZURE_STORAGE}.jar \
    -o /stackable/spark/jars/azure-storage-${AZURE_STORAGE}.jar && \
    chmod -x /stackable/spark/jars/azure-storage-${AZURE_STORAGE}.jar
RUN curl -L --fail https://repo.stackable.tech/repository/packages/azure/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar \
    -o /stackable/spark/jars/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar && \
    chmod -x /stackable/spark/jars/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar

# Download jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging.
RUN mkdir /stackable/spark/extra-jars && \
    curl -L --fail -o /stackable/spark/extra-jars/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
    https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar && \
    curl -L --fail -o /stackable/spark/extra-jars/stax2-api-${STAX2_API}.jar \
    https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar && \
    curl -L --fail -o /stackable/spark/extra-jars/woodstox-core-${WOODSTOX_CORE}.jar \
    https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar

RUN curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod -x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# Symlink example jar, so that we can easily use it in tests
RUN ln -s /stackable/spark/examples/jars/spark-examples_*.jar /stackable/spark/examples/jars/spark-examples.jar

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
RUN /bin/log4shell.sh /stackable/spark-${SPARK}-bin-hadoop${HADOOP_SHORT_VERSION}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/spark-${SPARK}-bin-hadoop${HADOOP_SHORT_VERSION}
# ===

WORKDIR /stackable/spark
ENTRYPOINT [ "/stackable/run-spark.sh" ]
