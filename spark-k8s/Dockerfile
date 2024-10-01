# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd
# check=error=true

# hadoop-builder: Provides Hadoop libraries
FROM stackable/image/hadoop AS hadoop-builder

# hbase-builder: Provides HBase libraries
FROM stackable/image/hbase AS hbase-builder

# spark-source-builder: Download the Spark source code into
# /stackable/spark and apply the patches
FROM stackable/image/java-devel AS spark-source-builder

ARG PRODUCT

RUN <<EOF
microdnf update

# patch: Required for the apply-patches.sh script
microdnf install \
patch

microdnf clean all
rm -rf /var/cache/yum
EOF

WORKDIR /stackable

RUN <<EOF
curl https://repo.stackable.tech/repository/packages/spark/spark-${PRODUCT}.tgz \
    | tar xz
ln -s spark-${PRODUCT} spark
EOF

WORKDIR /stackable/spark

COPY --chown=stackable:stackable \
    spark-k8s/stackable/patches/apply_patches.sh \
    patches/apply_patches.sh
COPY --chown=stackable:stackable \
    spark-k8s/stackable/patches/${PRODUCT} \
    patches/${PRODUCT}

RUN patches/apply_patches.sh ${PRODUCT}


# hbase-connectors-builder: Build the Spark HBase connector and copy
# required JARs into /stackable/spark/jars
FROM stackable/image/java-devel AS hbase-connectors-builder

ARG PRODUCT
ARG HADOOP
ARG HBASE
ARG HBASE_CONNECTOR

RUN <<EOF
microdnf update

# patch: Required for the apply-patches.sh script
microdnf install \
patch

microdnf clean all
rm -rf /var/cache/yum
EOF

WORKDIR /stackable

# Copy the pom.xml file from the patched Spark source code to read the
# versions used by Spark. The pom.xml defines child modules which are
# not required and not copied, therefore mvn must be called with the
# parameter --non-recursive.
COPY --chown=stackable:stackable --from=spark-source-builder \
    /stackable/spark/pom.xml \
    spark/

# Download the hbase-connectors source code
RUN <<EOF
curl https://repo.stackable.tech/repository/packages/hbase-connectors/hbase-connectors_${HBASE_CONNECTOR}.tar.gz \
    | tar xz
ln -s hbase-connectors-rel-${HBASE_CONNECTOR} hbase-connectors
EOF

# Patch the hbase-connectors source code
WORKDIR /stackable/hbase-connectors
COPY --chown=stackable:stackable \
    spark-k8s/stackable/hbase-connectors-patches/apply_patches.sh \
    patches/apply_patches.sh
COPY --chown=stackable:stackable \
    spark-k8s/stackable/hbase-connectors-patches/${HBASE_CONNECTOR} \
    patches/${HBASE_CONNECTOR}
RUN patches/apply_patches.sh ${HBASE_CONNECTOR}

WORKDIR /stackable/hbase-connectors/spark

RUN <<EOF
# Building the hbase-connectors with JDK 17 is not yet supported, see
# https://github.com/apache/hbase-connectors/pull/132.
# As there are no JDK profiles, access to the non-public elements must
# be enabled with --add-opens, see https://openjdk.org/jeps/403 and
# https://openjdk.org/jeps/261#Breaking-encapsulation.
export JDK_JAVA_OPTIONS="\
    --add-opens java.base/java.lang=ALL-UNNAMED \
    --add-opens java.base/java.util=ALL-UNNAMED"

# Get the Scala version used by Spark
SCALA_VERSION=$( \
    mvn --quiet --non-recursive --file /stackable/spark/pom.xml \
    org.apache.maven.plugins:maven-help-plugin:3.5.0:evaluate \
    -DforceStdout \
    -Dexpression='project.properties(scala.version)')

# Get the Scala binary version used by Spark
SCALA_BINARY_VERSION=$( \
    mvn --quiet --non-recursive --file /stackable/spark/pom.xml \
    org.apache.maven.plugins:maven-help-plugin:3.5.0:evaluate \
    -DforceStdout \
    -Dexpression='project.properties(scala.binary.version)')

# Build the Spark HBase connector
# Skip the tests because the MiniHBaseCluster does not get ready for
# whatever reason:
#   Caused by: java.lang.RuntimeException: Master not active after 30000ms
#     at org.apache.hadoop.hbase.util.JVMClusterUtil.waitForEvent(JVMClusterUtil.java:221)
#     at org.apache.hadoop.hbase.util.JVMClusterUtil.startup(JVMClusterUtil.java:177)
#     at org.apache.hadoop.hbase.LocalHBaseCluster.startup(LocalHBaseCluster.java:407)
#     at org.apache.hadoop.hbase.MiniHBaseCluster.init(MiniHBaseCluster.java:250)
mvn \
    --batch-mode \
    --no-transfer-progress \
    --define spark.version="${PRODUCT}" \
    --define scala.version="${SCALA_VERSION}" \
    --define scala.binary.version="${SCALA_BINARY_VERSION}" \
    --define hadoop-three.version="${HADOOP}" \
    --define hbase.version="${HBASE}" \
    --define skipTests \
    clean package
EOF

WORKDIR /stackable/spark/jars

RUN <<EOF
ln -s /stackable/hbase-connectors/spark/hbase-spark/target/hbase-spark-${HBASE_CONNECTOR}.jar

# Download log4j-slf4j-impl-x.x.x.jar containing the StaticLoggerBinder
# which is required by the connector.
# Spark contains only log4j-slf4j2-impl-x.x.x.jar but not
# log4j-slf4j-impl-x.x.x.jar. It is okay to have both JARs in the
# classpath as long as they have the same version.
mvn --quiet --non-recursive --file /stackable/spark/pom.xml \
    dependency:copy \
    -Dartifact=org.apache.logging.log4j:log4j-slf4j-impl:'${log4j.version}' \
    -DoutputDirectory=.
EOF


# spark-builder: Build Spark into /stackable/spark-${PRODUCT}/dist,
# download additional JARs and perform checks, like log4shell check.
FROM stackable/image/java-devel as spark-builder

ARG PRODUCT
ARG HADOOP
ARG HBASE
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG JMX_EXPORTER
ARG TARGETARCH
ARG TINI

WORKDIR /stackable/spark-${PRODUCT}

COPY --chown=stackable:stackable --from=spark-source-builder \
    /stackable/spark/ \
    ./

# >>> Build spark
# Compiling the tests takes a lot of time, so we skip them
# -Dmaven.test.skip=true skips both the compilation and execution of tests
# -DskipTests skips only the execution
#
# This will download it's own version of maven because the UBI version is too old:
# 134.0 [ERROR] Detected Maven Version: 3.6.3 is not in the allowed range [3.8.8,)
RUN export MAVEN_OPTS="-Xss64m -Xmx2g -XX:ReservedCodeCacheSize=1g" \
    && ./dev/make-distribution.sh \
      -Dhadoop.version="$HADOOP" \
      -Dmaven.test.skip=true \
      -DskipTests \
      -P'hadoop-3' -Pkubernetes -Phive -Phive-thriftserver
# <<< Build spark

# Get the correct `tini` binary for our architecture.
RUN curl -o /usr/bin/tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI}-${TARGETARCH}" \
    && chmod +x /usr/bin/tini

# We download these under dist so that log4shell checks them
WORKDIR /stackable/spark-${PRODUCT}/dist/jars

# Copy modules required for s3a://
COPY --from=hadoop-builder --chown=stackable:stackable \
    /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    ./

# Copy modules required for abfs://
COPY --from=hadoop-builder --chown=stackable:stackable \
    /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar \
    ./

# Copy the HBase connector including required modules
COPY --from=hbase-connectors-builder --chown=stackable:stackable \
    /stackable/spark/jars/* \
    ./

# Copy modules required to access HBase
COPY --from=hbase-builder --chown=stackable:stackable \
    /stackable/hbase/lib/shaded-clients/hbase-shaded-client-byo-hadoop-${HBASE}.jar \
    /stackable/hbase/lib/shaded-clients/hbase-shaded-mapreduce-${HBASE}.jar \
    ./
# Copy modules required to access HBase if $HBASE == 2.4.x
COPY --from=hbase-builder --chown=stackable:stackable \
    /stackable/hbase/lib/client-facing-thirdparty/htrace-core4-*-incubating.jar \
    /stackable/hbase/lib/client-facing-thirdparty/slf4j-reload4j-*.jar \
    ./
# Copy modules required to access HBase if $HBASE == 2.6.x
COPY --from=hbase-builder --chown=stackable:stackable \
    /stackable/hbase/lib/client-facing-thirdparty/opentelemetry-api-*.jar \
    /stackable/hbase/lib/client-facing-thirdparty/opentelemetry-context-*.jar \
    /stackable/hbase/lib/client-facing-thirdparty/opentelemetry-semconv-*-alpha.jar \
    ./

WORKDIR /stackable/spark-${PRODUCT}/dist/extra-jars

# Download jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging.
RUN curl -O https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
    && curl -O https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar \
    && curl -O https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar

WORKDIR /stackable/jmx

RUN curl -O "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"

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

FROM stackable/image/java-base AS final

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
    "python${PYTHON}" \
    "python${PYTHON}-pip" \
    zip \
    # This is needed by the Spark UI to display process information using jps and jmap
    # Copying the binaries from the builder stage failed.
    "java-${JAVA_VERSION}-openjdk-devel" \
    && microdnf clean all \
    && rm -rf /var/cache/yum

RUN ln -s /usr/bin/python${PYTHON} /usr/bin/python \
    && ln -s /usr/bin/pip-${PYTHON} /usr/bin/pip


ENV HOME=/stackable
ENV SPARK_HOME=/stackable/spark
ENV PATH=$SPARK_HOME:$PATH:/bin:$JAVA_HOME/bin:$JAVA_HOME/jre/bin:$HOME/.local/bin
ENV PYSPARK_PYTHON=/usr/bin/python
ENV PYTHONPATH=$SPARK_HOME/python

COPY --chown=stackable:stackable --from=spark-builder /stackable/spark-${PRODUCT}/dist /stackable/spark
COPY --chown=stackable:stackable --from=spark-builder /stackable/spark-${PRODUCT}/assembly/target/bom.json /stackable/spark/spark-${PRODUCT}.cdx.json
COPY --chown=stackable:stackable --from=spark-builder /stackable/jmx /stackable/jmx
COPY --from=spark-builder /usr/bin/tini /usr/bin/tini

RUN ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar \
    # Symlink example jar, so that we can easily use it in tests
    && ln -s /stackable/spark/examples/jars/spark-examples_*.jar /stackable/spark/examples/jars/spark-examples.jar

USER stackable
WORKDIR /stackable

COPY spark-k8s/stackable /stackable
COPY spark-k8s/licenses /licenses


WORKDIR /stackable/spark
ENTRYPOINT [ "/stackable/run-spark.sh" ]
