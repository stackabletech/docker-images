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

WORKDIR /stackable

RUN curl -L --fail https://github.com/apache/hbase-connectors/archive/307607cf7287084b3ce49cdd96d094e2ede9363a.tar.gz | tar xz

WORKDIR /stackable/hbase-connectors-307607cf7287084b3ce49cdd96d094e2ede9363a/spark

RUN export JDK_JAVA_OPTIONS="\
        --add-opens java.base/java.lang=ALL-UNNAMED \
        --add-opens java.base/java.util=ALL-UNNAMED" \
    && mvn \
        --define spark.version=${PRODUCT} \
        --define scala.version=2.12.18 \
        --define hadoop-three.version=${HADOOP_LONG_VERSION} \
        --define scala.binary.version=2.12 \
        --define hbase.version=2.6.0 \
        --define skipTests \
        clean package

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

COPY --chown=stackable:stackable \
    --from=builder \
    /stackable/hbase-connectors-307607cf7287084b3ce49cdd96d094e2ede9363a/spark/hbase-spark/target/hbase-spark-1.1.0-SNAPSHOT.jar \
    /stackable/spark/jars
COPY --chown=stackable:stackable \
    --from=docker.stackable.tech/apoc/stackable/hbase:2.6.0-stackable0.0.0-dev \
    /stackable/hbase-2.6.0/lib/HikariCP-java7-2.4.12.jar \
    /stackable/hbase-2.6.0/lib/agrona-1.12.0.jar \
    /stackable/hbase-2.6.0/lib/aircompressor-0.24.jar \
    /stackable/hbase-2.6.0/lib/animal-sniffer-annotations-1.17.jar \
    /stackable/hbase-2.6.0/lib/aopalliance-1.0.jar \
    /stackable/hbase-2.6.0/lib/asm-commons-9.4.jar \
    /stackable/hbase-2.6.0/lib/asm-tree-9.4.jar \
    /stackable/hbase-2.6.0/lib/avro-1.11.3.jar \
    /stackable/hbase-2.6.0/lib/azure-data-lake-store-sdk-2.3.9.jar \
    /stackable/hbase-2.6.0/lib/azure-keyvault-core-1.0.0.jar \
    /stackable/hbase-2.6.0/lib/azure-storage-7.0.1.jar \
    /stackable/hbase-2.6.0/lib/brotli4j-1.11.0.jar \
    /stackable/hbase-2.6.0/lib/byte-buddy-1.12.19.jar \
    /stackable/hbase-2.6.0/lib/byte-buddy-1.14.9.jar \
    /stackable/hbase-2.6.0/lib/byte-buddy-agent-1.12.19.jar \
    /stackable/hbase-2.6.0/lib/caffeine-2.8.8.jar \
    /stackable/hbase-2.6.0/lib/checker-qual-2.5.2.jar \
    /stackable/hbase-2.6.0/lib/checker-qual-3.8.0.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/audience-annotations-0.13.0.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/commons-logging-1.2.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/jcl-over-slf4j-1.7.36.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/jul-to-slf4j-1.7.36.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/log4j-1.2-api-2.17.2.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/log4j-api-2.17.2.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/log4j-core-2.17.2.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/log4j-slf4j-impl-2.17.2.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/opentelemetry-api-1.15.0.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/opentelemetry-context-1.15.0.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/opentelemetry-semconv-1.15.0-alpha.jar \
    # /stackable/hbase-2.6.0/lib/client-facing-thirdparty/slf4j-api-1.7.36.jar \
    /stackable/hbase-2.6.0/lib/commons-beanutils-1.9.4.jar \
    /stackable/hbase-2.6.0/lib/commons-cli-1.5.0.jar \
    /stackable/hbase-2.6.0/lib/commons-codec-1.15.jar \
    /stackable/hbase-2.6.0/lib/commons-collections-3.2.2.jar \
    /stackable/hbase-2.6.0/lib/commons-compress-1.21.jar \
    /stackable/hbase-2.6.0/lib/commons-configuration2-2.8.0.jar \
    /stackable/hbase-2.6.0/lib/commons-crypto-1.1.0.jar \
    /stackable/hbase-2.6.0/lib/commons-csv-1.9.0.jar \
    /stackable/hbase-2.6.0/lib/commons-daemon-1.0.13.jar \
    /stackable/hbase-2.6.0/lib/commons-io-2.11.0.jar \
    /stackable/hbase-2.6.0/lib/commons-lang3-3.9.jar \
    /stackable/hbase-2.6.0/lib/commons-math3-3.6.1.jar \
    /stackable/hbase-2.6.0/lib/commons-net-3.9.0.jar \
    /stackable/hbase-2.6.0/lib/commons-text-1.10.0.jar \
    /stackable/hbase-2.6.0/lib/curator-client-4.2.0.jar \
    /stackable/hbase-2.6.0/lib/curator-framework-4.2.0.jar \
    /stackable/hbase-2.6.0/lib/curator-recipes-4.2.0.jar \
    /stackable/hbase-2.6.0/lib/disruptor-3.4.4.jar \
    /stackable/hbase-2.6.0/lib/dnsjava-2.1.7.jar \
    /stackable/hbase-2.6.0/lib/ehcache-3.3.1.jar \
    /stackable/hbase-2.6.0/lib/error_prone_annotations-2.26.1.jar \
    /stackable/hbase-2.6.0/lib/failureaccess-1.0.jar \
    /stackable/hbase-2.6.0/lib/fst-2.50.jar \
    /stackable/hbase-2.6.0/lib/geronimo-jcache_1.0_spec-1.0-alpha-1.jar \
    /stackable/hbase-2.6.0/lib/gson-2.9.0.jar \
    /stackable/hbase-2.6.0/lib/guava-27.0-jre.jar \
    /stackable/hbase-2.6.0/lib/guice-4.0.jar \
    /stackable/hbase-2.6.0/lib/guice-servlet-4.0.jar \
    /stackable/hbase-2.6.0/lib/hadoop-annotations-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-auth-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-azure-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-azure-datalake-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-common-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-distcp-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-hdfs-3.3.6-tests.jar \
    /stackable/hbase-2.6.0/lib/hadoop-hdfs-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-hdfs-client-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-app-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-common-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-core-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-hs-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-jobclient-3.3.6-tests.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-jobclient-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-mapreduce-client-shuffle-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-minicluster-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-minikdc-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-registry-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-shaded-guava-1.1.1.jar \
    /stackable/hbase-2.6.0/lib/hadoop-shaded-protobuf_3_7-1.1.1.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-api-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-client-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-common-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-applicationhistoryservice-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-common-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-nodemanager-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-resourcemanager-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-tests-3.3.6-tests.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-timelineservice-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hadoop-yarn-server-web-proxy-3.3.6.jar \
    /stackable/hbase-2.6.0/lib/hbase-annotations-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-annotations-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-asyncfs-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-asyncfs-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-backup-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-client-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-common-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-common-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-compression-aircompressor-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-compression-brotli-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-compression-lz4-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-compression-snappy-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-compression-zstd-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-endpoint-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-external-blockcache-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-hadoop-compat-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-hadoop-compat-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-hadoop2-compat-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-hadoop2-compat-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-hbtop-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-http-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-it-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-it-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-logging-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-mapreduce-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-mapreduce-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-metrics-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-metrics-api-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-opa-authorizer-0.1.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-openssl-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-procedure-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-procedure-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-protocol-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-protocol-shaded-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-replication-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-resource-bundle-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-rest-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-rsgroup-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-rsgroup-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-server-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-server-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-gson-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-jackson-jaxrs-json-provider-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-jersey-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-jetty-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-miscellaneous-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-netty-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-netty-tcnative-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shaded-protobuf-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-shell-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-testing-util-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-thrift-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/hbase-unsafe-4.1.7.jar \
    /stackable/hbase-2.6.0/lib/hbase-zookeeper-2.6.0-tests.jar \
    /stackable/hbase-2.6.0/lib/hbase-zookeeper-2.6.0.jar \
    /stackable/hbase-2.6.0/lib/httpclient-4.5.14.jar \
    /stackable/hbase-2.6.0/lib/httpcore-4.4.16.jar \
    /stackable/hbase-2.6.0/lib/j2objc-annotations-1.1.jar \
    /stackable/hbase-2.6.0/lib/jackson-annotations-2.17.0.jar \
    /stackable/hbase-2.6.0/lib/jackson-core-2.17.0.jar \
    /stackable/hbase-2.6.0/lib/jackson-databind-2.17.0.jar \
    /stackable/hbase-2.6.0/lib/jackson-dataformat-xml-2.17.0.jar \
    /stackable/hbase-2.6.0/lib/jackson-jaxrs-base-2.12.7.jar \
    /stackable/hbase-2.6.0/lib/jackson-jaxrs-json-provider-2.12.7.jar \
    /stackable/hbase-2.6.0/lib/jackson-module-jaxb-annotations-2.12.7.jar \
    /stackable/hbase-2.6.0/lib/jackson-module-jaxb-annotations-2.17.0.jar \
    /stackable/hbase-2.6.0/lib/jakarta.inject-2.6.1.jar \
    /stackable/hbase-2.6.0/lib/jakarta.validation-api-2.0.2.jar \
    /stackable/hbase-2.6.0/lib/jamon-runtime-2.4.1.jar \
    /stackable/hbase-2.6.0/lib/java-util-1.9.0.jar \
    /stackable/hbase-2.6.0/lib/javassist-3.30.2-GA.jar \
    /stackable/hbase-2.6.0/lib/javax-websocket-client-impl-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/javax-websocket-server-impl-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/javax.activation-api-1.2.0.jar \
    /stackable/hbase-2.6.0/lib/javax.annotation-api-1.2.jar \
    /stackable/hbase-2.6.0/lib/javax.el-3.0.1-b12.jar \
    /stackable/hbase-2.6.0/lib/javax.inject-1.jar \
    /stackable/hbase-2.6.0/lib/javax.servlet-api-3.1.0.jar \
    /stackable/hbase-2.6.0/lib/javax.servlet.jsp-2.3.4.jar \
    /stackable/hbase-2.6.0/lib/javax.servlet.jsp-api-2.3.1.jar \
    /stackable/hbase-2.6.0/lib/javax.websocket-api-1.0.jar \
    /stackable/hbase-2.6.0/lib/javax.websocket-client-api-1.0.jar \
    /stackable/hbase-2.6.0/lib/jaxb-api-2.3.1.jar \
    /stackable/hbase-2.6.0/lib/jaxb-impl-2.2.3-1.jar \
    /stackable/hbase-2.6.0/lib/jcip-annotations-1.0-1.jar \
    /stackable/hbase-2.6.0/lib/jcodings-1.0.58.jar \
    # /stackable/hbase-2.6.0/lib/jdk11 \
    /stackable/hbase-2.6.0/lib/jersey-json-1.20.jar \
    /stackable/hbase-2.6.0/lib/jettison-1.5.4.jar \
    /stackable/hbase-2.6.0/lib/jetty-annotations-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-client-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-http-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-io-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-jndi-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-plus-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-security-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-server-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-servlet-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-util-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-util-ajax-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-webapp-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jetty-xml-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/jline-3.9.0.jar \
    /stackable/hbase-2.6.0/lib/jna-5.2.0.jar \
    /stackable/hbase-2.6.0/lib/joni-2.2.1.jar \
    /stackable/hbase-2.6.0/lib/jsch-0.1.55.jar \
    /stackable/hbase-2.6.0/lib/json-io-2.5.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-admin-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-client-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-common-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-core-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-crypto-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-identity-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-server-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-simplekdc-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerb-util-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerby-asn1-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerby-config-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerby-pkix-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerby-util-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kerby-xdr-1.0.1.jar \
    /stackable/hbase-2.6.0/lib/kotlin-stdlib-1.4.10.jar \
    /stackable/hbase-2.6.0/lib/kotlin-stdlib-common-1.4.10.jar \
    /stackable/hbase-2.6.0/lib/leveldbjni-all-1.8.jar \
    /stackable/hbase-2.6.0/lib/libthrift-0.14.2.jar \
    /stackable/hbase-2.6.0/lib/listenablefuture-9999.0-empty-to-avoid-conflict-with-guava.jar \
    /stackable/hbase-2.6.0/lib/lz4-java-1.8.0.jar \
    /stackable/hbase-2.6.0/lib/metrics-core-3.2.6.jar \
    /stackable/hbase-2.6.0/lib/mssql-jdbc-6.2.1.jre7.jar \
    /stackable/hbase-2.6.0/lib/native-linux-x86_64-1.11.0.jar \
    /stackable/hbase-2.6.0/lib/netty-3.10.6.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-all-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-buffer-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-dns-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-haproxy-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-http-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-http2-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-memcache-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-mqtt-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-redis-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-smtp-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-socks-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-stomp-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-codec-xml-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-common-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-handler-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-handler-proxy-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-handler-ssl-ocsp-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-resolver-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-resolver-dns-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-resolver-dns-classes-macos-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-resolver-dns-native-macos-4.1.111.Final-osx-aarch_64.jar \
    /stackable/hbase-2.6.0/lib/netty-resolver-dns-native-macos-4.1.111.Final-osx-x86_64.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-classes-epoll-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-classes-kqueue-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-epoll-4.1.111.Final-linux-aarch_64.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-epoll-4.1.111.Final-linux-riscv64.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-epoll-4.1.111.Final-linux-x86_64.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-epoll-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-kqueue-4.1.111.Final-osx-aarch_64.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-kqueue-4.1.111.Final-osx-x86_64.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-native-unix-common-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-rxtx-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-sctp-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/netty-transport-udt-4.1.111.Final.jar \
    /stackable/hbase-2.6.0/lib/nimbus-jose-jwt-9.8.1.jar \
    /stackable/hbase-2.6.0/lib/objenesis-3.3.jar \
    /stackable/hbase-2.6.0/lib/okhttp-4.9.3.jar \
    /stackable/hbase-2.6.0/lib/okio-2.8.0.jar \
    /stackable/hbase-2.6.0/lib/opentelemetry-api-1.15.0.jar \
    /stackable/hbase-2.6.0/lib/opentelemetry-context-1.15.0.jar \
    /stackable/hbase-2.6.0/lib/opentelemetry-semconv-1.15.0-alpha.jar \
    # /stackable/hbaes-2.6.0/phoenix-server-hbase-2.6-5.3.0-4afe457.jar \
    /stackable/hbase-2.6.0/lib/protobuf-java-2.5.0.jar \
    /stackable/hbase-2.6.0/lib/re2j-1.1.jar \
    # /stackable/hbase-2.6.0/lib/ruby \
    /stackable/hbase-2.6.0/lib/service-1.11.0.jar \
    # /stackable/hbase-2.6.0/lib/shaded-clients \
    /stackable/hbase-2.6.0/lib/snappy-java-1.1.10.5.jar \
    /stackable/hbase-2.6.0/lib/spymemcached-2.12.3.jar \
    /stackable/hbase-2.6.0/lib/stax2-api-4.2.1.jar \
    /stackable/hbase-2.6.0/lib/stax2-api-4.2.2.jar \
    # /stackable/hbase-2.6.0/lib/test \
    /stackable/hbase-2.6.0/lib/token-provider-1.0.1.jar \
    # /stackable/hbase-2.6.0/lib/trace \
    /stackable/hbase-2.6.0/lib/websocket-api-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/websocket-client-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/websocket-common-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/websocket-server-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/websocket-servlet-9.4.51.v20230217.jar \
    /stackable/hbase-2.6.0/lib/wildfly-openssl-1.1.3.Final.jar \
    /stackable/hbase-2.6.0/lib/woodstox-core-5.4.0.jar \
    /stackable/hbase-2.6.0/lib/woodstox-core-6.6.1.jar \
    # /stackable/hbase-2.6.0/lib/zkcli \
    /stackable/hbase-2.6.0/lib/zookeeper-3.8.4.jar \
    /stackable/hbase-2.6.0/lib/zookeeper-jute-3.8.4.jar \
    /stackable/hbase-2.6.0/lib/zstd-jni-1.5.6-3.jar \
    /stackable/spark/jars

WORKDIR /stackable/spark/jars

RUN curl -L --fail -O https://repo1.maven.org/maven2/org/apache/logging/log4j/log4j-slf4j-impl/2.20.0/log4j-slf4j-impl-2.20.0.jar

WORKDIR /stackable/spark
ENTRYPOINT [ "/stackable/run-spark.sh" ]
