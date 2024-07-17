# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Ignoring DL3006 because we don't want to pin java-* images
# hadolint ignore=DL3006
FROM stackable/image/java-devel as builder

ARG PRODUCT
ARG HADOOP_LONG_VERSION
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG JMX_EXPORTER
ARG TARGETARCH
ARG TINI

WORKDIR /stackable

# >>> Build spark
# Compiling the tests takes a lot of time, so we skip them
# -Dmaven.test.skip=true skips both the compilation and execution of tests
# -DskipTests skips only the execution
#
# This will download it's own version of maven because the UBI version is too old:
# 134.0 [ERROR] Detected Maven Version: 3.6.3 is not in the allowed range [3.8.8,)
# hadolint ignore=DL3003
RUN curl --fail -L https://repo.stackable.tech/repository/packages/spark/spark-${PRODUCT}.tgz | tar -xzf - \
    && cd spark-${PRODUCT} \
    && export MAVEN_OPTS="-Xss64m -Xmx2g -XX:ReservedCodeCacheSize=1g" \
    && ./dev/make-distribution.sh \
      -Dhadoop.version="$HADOOP_LONG_VERSION" \
      -Dmaven.test.skip=true \
      -DskipTests \
      -P'hadoop-3' -Pkubernetes -Phive -Phive-thriftserver
# <<< Build spark

# Get the correct `tini` binary for our architecture.
RUN curl --fail -L -o /usr/bin/tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI}-${TARGETARCH}" \
    && chmod +x /usr/bin/tini

# We download these under dist so that log4shell checks them
WORKDIR /stackable/spark-${PRODUCT}/dist/jars

# Download various modules for Hadoop (e.g. support for s3a:// and abfs://)
RUN curl -L --fail -O https://repo.stackable.tech/repository/packages/aws/hadoop-aws-${HADOOP_LONG_VERSION}.jar \
    && curl -L --fail -O https://repo.stackable.tech/repository/packages/aws/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    && curl -L --fail -O https://repo.stackable.tech/repository/packages/azure/hadoop-azure-${HADOOP_LONG_VERSION}.jar \
    && curl -L --fail -O https://repo.stackable.tech/repository/packages/azure/azure-storage-${AZURE_STORAGE}.jar \
    && curl -L --fail -O https://repo.stackable.tech/repository/packages/azure/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar

WORKDIR /stackable/spark-${PRODUCT}/dist/extra-jars

# Download jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging.
RUN curl -L --fail -O https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
    && curl -L --fail -O https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar \
    && curl -L --fail -O https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar

WORKDIR /stackable/jmx

RUN curl --fail -L -O "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"

# ===
# Mitigation for CVE-2021-44228 (Log4Shell)
#
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/spark-${PRODUCT}/dist

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/spark-${PRODUCT}/dist
# ===

# Ignoring DL3006 because we don't want to pin java-* images
# hadolint ignore=DL3006
FROM stackable/image/java-base as final

ARG PRODUCT
ARG PYTHON
ARG RELEASE
ARG JMX_EXPORTER


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
    python${PYTHON}-pip \
    zip \
    # This is needed by the Spark UI to display process information using jps and jmap
    # Copying the binaries from the builder stage failed.
    java-${JAVA_VERSION}-openjdk-devel \
    && microdnf clean all \
    && rm -rf /var/cache/yum

RUN ln -s /usr/bin/python${PYTHON} /usr/bin/python \
    && ln -s /usr/bin/pip-${PYTHON} /usr/bin/pip


ENV HOME=/stackable
ENV SPARK_HOME=/stackable/spark
ENV PATH=$SPARK_HOME:$PATH:/bin:$JAVA_HOME/bin:$JAVA_HOME/jre/bin:$HOME/.local/bin
ENV PYSPARK_PYTHON=/usr/bin/python
ENV PYTHONPATH=$SPARK_HOME/python:$PYTHONPATH

COPY --chown=stackable:stackable --from=builder /stackable/spark-${PRODUCT}/dist /stackable/spark
COPY --chown=stackable:stackable --from=builder /stackable/jmx /stackable/jmx
COPY --from=builder /usr/bin/tini /usr/bin/tini

RUN ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar \
    # Symlink example jar, so that we can easily use it in tests
    && ln -s /stackable/spark/examples/jars/spark-examples_*.jar /stackable/spark/examples/jars/spark-examples.jar

USER stackable
WORKDIR /stackable

COPY spark-k8s/stackable /stackable
COPY spark-k8s/licenses /licenses


WORKDIR /stackable/spark
ENTRYPOINT [ "/stackable/run-spark.sh" ]
