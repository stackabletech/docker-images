# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# hadoop-builder: Provides Hadoop libraries
FROM stackable/image/hadoop/hadoop AS hadoop-builder

# hbase-builder: Provides HBase libraries
FROM stackable/image/hbase AS hbase-builder

# spark-source-builder: Clone the Spark source code and apply patches
FROM stackable/image/java-devel AS spark-source-builder

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/stackable/patches/patchable.toml /stackable/src/spark-k8s/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/stackable/patches/${PRODUCT} /stackable/src/spark-k8s/stackable/patches/${PRODUCT}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout spark-k8s ${PRODUCT})"

NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/spark-${PRODUCT}-stackable${RELEASE}-src.tar.gz .
chmod g=u /stackable/spark-${PRODUCT}-stackable${RELEASE}-src.tar.gz
EOF

# hbase-connectors-builder: Build the Spark HBase connector and copy
# required JARs into /stackable/spark/jars
FROM stackable/image/java-devel AS hbase-connectors-builder

ARG PRODUCT
ARG RELEASE
ARG HADOOP_HADOOP
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP}
ARG HBASE
ARG HBASE_CONNECTOR
ARG STACKABLE_USER_UID

WORKDIR /stackable

# Copy the pom.xml file from the patched Spark source code to read the
# versions used by Spark. The pom.xml defines child modules which are
# not required and not copied, therefore mvn must be called with the
# parameter --non-recursive.
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-source-builder \
    /stackable/src/spark-k8s/patchable-work/worktree/${PRODUCT}/pom.xml \
    spark/

# Patch the hbase-connectors source code
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/hbase-connectors/stackable/patches/patchable.toml /stackable/src/spark-k8s/hbase-connectors/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/hbase-connectors/stackable/patches/${HBASE_CONNECTOR} /stackable/src/spark-k8s/hbase-connectors/stackable/patches/${HBASE_CONNECTOR}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout spark-k8s/hbase-connectors ${HBASE_CONNECTOR})/spark"

NEW_VERSION="${HBASE_CONNECTOR}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-connector-${HBASE_CONNECTOR}-stackable${RELEASE}-src.tar.gz .

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
    mvn --quiet --non-recursive --no-transfer-progress --batch-mode --file /stackable/spark/pom.xml \
    org.apache.maven.plugins:maven-help-plugin:3.5.0:evaluate \
    -DforceStdout \
    -Dexpression='project.properties(scala.version)')

# Get the Scala binary version used by Spark
SCALA_BINARY_VERSION=$( \
    mvn --quiet --non-recursive  --no-transfer-progress --batch-mode --file /stackable/spark/pom.xml \
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
    --define hadoop-three.version="${HADOOP_VERSION}" \
    --define hbase.version="${HBASE}" \
    --define skipTests \
    clean package

mkdir -p /stackable/spark/jars
ln -s "$(pwd)/hbase-spark/target/hbase-spark-${HBASE_CONNECTOR}-stackable${RELEASE}.jar" /stackable/spark/jars/hbase-spark-${HBASE_CONNECTOR}-stackable${RELEASE}.jar

cd /stackable/spark/jars

# Download log4j-slf4j-impl-x.x.x.jar containing the StaticLoggerBinder
# which is required by the connector.
# Spark contains only log4j-slf4j2-impl-x.x.x.jar but not
# log4j-slf4j-impl-x.x.x.jar. It is okay to have both JARs in the
# classpath as long as they have the same version.
mvn --quiet --non-recursive --no-transfer-progress --batch-mode --file /stackable/spark/pom.xml \
    dependency:copy \
    -Dartifact=org.apache.logging.log4j:log4j-slf4j-impl:'${log4j.version}' \
    -DoutputDirectory=./jars
chmod g=u /stackable/hbase-connector-${HBASE_CONNECTOR}-stackable${RELEASE}-src.tar.gz .
EOF


# spark-builder: Build Spark into /stackable/spark-${PRODUCT}/dist,
# download additional JARs and perform checks
FROM stackable/image/java-devel AS spark-builder

ARG PRODUCT
ARG HADOOP_HADOOP
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP}
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
ARG RELEASE
ARG STACKABLE_USER_UID
# Find the latest version here: https://github.com/apache/maven
# renovate: datasource=github-tags packageName=apache/maven
ARG MAVEN_VERSION="3.9.10"

WORKDIR /stackable/spark-${PRODUCT}-stackable${RELEASE}

COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-source-builder \
    /stackable/src/spark-k8s/patchable-work/worktree/${PRODUCT} \
    ./
COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-libs /stackable/patched-libs

# >>> Build spark
# Compiling the tests takes a lot of time, so we skip them
# -Dmaven.test.skip=true skips both the compilation and execution of tests
# -DskipTests skips only the execution
RUN <<EOF
    # Make Maven aware of custom Stackable libraries
    mv /stackable/patched-libs/maven /root/.m2/repository

    # We download the Maven binary from our own repository because:
    #
    # 1. The UBI Maven version is too old:
    #   134.0 [ERROR] Detected Maven Version: 3.6.3 is not in the allowed range [3.8.8,)
    # 2. The Maven download from archive.apache.org is not working reliably:
    curl "https://repo.stackable.tech/repository/packages/maven/apache-maven-${MAVEN_VERSION}-bin.tar.gz" | tar -xzC /tmp

    ORIGINAL_VERSION="${PRODUCT}"
    NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

    export MAVEN_OPTS="-Xss64m -Xmx2g -XX:ReservedCodeCacheSize=1g"

    ./dev/make-distribution.sh \
    --mvn /tmp/apache-maven-${MAVEN_VERSION}/bin/mvn \
    -Dhadoop.version="${HADOOP_VERSION}-stackable${RELEASE}" \
    -DskipTests \
    -P'hadoop-3' -Pkubernetes -Phive -Phive-thriftserver \
    --no-transfer-progress \
    --batch-mode

    sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" assembly/target/bom.json
EOF


# As of version 3.5.5, spark-connect jars are not included in the dist folder.
# To avoid classpath conflicts with existing spark applications,
# we create a new dist/connect folder, and copy them here.
RUN <<EOF

    # Get the Scala binary version
    SCALA_BINARY_VERSION=$( \
        mvn --quiet --non-recursive --no-transfer-progress --batch-mode --file pom.xml \
        org.apache.maven.plugins:maven-help-plugin:3.5.0:evaluate \
        -DforceStdout \
        -Dexpression='project.properties(scala.binary.version)')

    mkdir -p dist/connect
    cd dist/connect

    cp "/stackable/spark-${PRODUCT}-stackable${RELEASE}/connector/connect/server/target/spark-connect_${SCALA_BINARY_VERSION}-${PRODUCT}-stackable${RELEASE}.jar" .
    cp "/stackable/spark-${PRODUCT}-stackable${RELEASE}/connector/connect/common/target/spark-connect-common_${SCALA_BINARY_VERSION}-${PRODUCT}-stackable${RELEASE}.jar" .
    cp "/stackable/spark-${PRODUCT}-stackable${RELEASE}/connector/connect/client/jvm/target/spark-connect-client-jvm_${SCALA_BINARY_VERSION}-${PRODUCT}-stackable${RELEASE}.jar" .

    # The Spark operator expects a file named spark-connect_${SCALA_BINARY_VERSION}-${PRODUCT}.jar without the -stackable${RELEASE} suffix.
    ln -s "spark-connect_${SCALA_BINARY_VERSION}-${PRODUCT}-stackable${RELEASE}.jar" "spark-connect_${SCALA_BINARY_VERSION}-${PRODUCT}.jar"
EOF

# <<< Build spark

WORKDIR /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist/jars

# Copy modules required for s3a://
COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 \
    /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP_VERSION}-stackable${RELEASE}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/bundle-${AWS_JAVA_SDK_BUNDLE}.jar \
    ./

# Copy modules required for abfs://
COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 \
    /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP_VERSION}-stackable${RELEASE}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar \
    ./

# Copy the HBase connector including required modules
COPY --from=hbase-connectors-builder --chown=${STACKABLE_USER_UID}:0 \
    /stackable/spark/jars/* \
    ./

# Copy modules required to access HBase
COPY --from=hbase-builder --chown=${STACKABLE_USER_UID}:0 \
    /stackable/hbase/lib/shaded-clients/hbase-shaded-client-byo-hadoop-${HBASE}-stackable${RELEASE}.jar \
    /stackable/hbase/lib/shaded-clients/hbase-shaded-mapreduce-${HBASE}-stackable${RELEASE}.jar \
    ./
# Copy modules required to access HBase if $HBASE == 2.4.x
COPY --from=hbase-builder --chown=${STACKABLE_USER_UID}:0 \
    /stackable/hbase/lib/client-facing-thirdparty/htrace-core4-*-incubating.jar \
    /stackable/hbase/lib/client-facing-thirdparty/slf4j-reload4j-*.jar \
    ./
# Copy modules required to access HBase if $HBASE == 2.6.x
COPY --from=hbase-builder --chown=${STACKABLE_USER_UID}:0 \
    /stackable/hbase/lib/client-facing-thirdparty/opentelemetry-api-*.jar \
    /stackable/hbase/lib/client-facing-thirdparty/opentelemetry-context-*.jar \
    /stackable/hbase/lib/client-facing-thirdparty/opentelemetry-semconv-*-alpha.jar \
    ./

COPY spark-k8s/stackable/jmx /stackable/jmx

WORKDIR /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist/extra-jars

RUN <<EOF
# Download jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging.
curl --fail https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
  -o /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist/extra-jars/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar
curl --fail https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar \
  -o /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist/extra-jars/stax2-api-${STAX2_API}.jar
curl --fail https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar \
  -o /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist/extra-jars/woodstox-core-${WOODSTOX_CORE}.jar

# Get the correct `tini` binary for our architecture.
curl --fail "https://repo.stackable.tech/repository/packages/tini/tini-${TINI}-${TARGETARCH}" \
  -o /usr/bin/tini
chmod +x /usr/bin/tini

# JMX Exporter
curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
  -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

chmod -R g=u /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist
chmod -R g=u /stackable/spark-${PRODUCT}-stackable${RELEASE}/assembly/target/bom.json
chmod -R g=u /stackable/jmx
EOF

# TODO: java-base installs the Adoptium dnf repo and the Termurin jre which is not needed here.
# To reduce the size of this image, the Adoptium repo could be moved to stackable-base instead.
FROM stackable/image/java-base AS final

ARG PRODUCT
ARG PYTHON
ARG RELEASE
ARG JMX_EXPORTER
ARG HBASE_CONNECTOR
ARG STACKABLE_USER_UID

LABEL name="Apache Spark" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache Spark with PySpark support." \
    description="This image is deployed by the Stackable Operator for Apache Spark on Kubernetes."


ENV HOME=/stackable
ENV SPARK_HOME=/stackable/spark
# Override the java-base version of JAVA_HOME to point to the jdk.
ENV JAVA_HOME="/usr/lib/jvm/temurin-${JAVA_VERSION}-jdk"
ENV PATH=$SPARK_HOME/bin:$JAVA_HOME/bin:$PATH
ENV PYSPARK_PYTHON=/usr/bin/python
ENV PYTHONPATH=$SPARK_HOME/python


COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-builder /stackable/spark-${PRODUCT}-stackable${RELEASE}/dist /stackable/spark
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-source-builder /stackable/spark-${PRODUCT}-stackable${RELEASE}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-connectors-builder /stackable/hbase-connector-${HBASE_CONNECTOR}-stackable${RELEASE}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-builder /stackable/spark-${PRODUCT}-stackable${RELEASE}/assembly/target/bom.json /stackable/spark/spark-${PRODUCT}-stackable${RELEASE}.cdx.json
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-builder /stackable/jmx /stackable/jmx
COPY --from=spark-builder /usr/bin/tini /usr/bin/tini

COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/stackable/run-spark.sh /stackable/run-spark.sh
COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/licenses /licenses

RUN <<EOF
microdnf update

# procps:
#    Required for spark startup scripts.
# temurin-{version}-jdk:
#    Needed by the Spark UI to display process information using "jps" and "jmap".
#    Spark-Connect needs "javac" to compile auto-generated classes on the fly.
microdnf install --nodocs \
  gzip \
  hostname \
  procps \
  "python${PYTHON}" \
  "python${PYTHON}-pip" \
  zip \
  "temurin-${JAVA_VERSION}-jdk"
microdnf clean all
rm -rf /var/cache/yum

ln -s /usr/bin/python${PYTHON} /usr/bin/python
ln -s /usr/bin/pip-${PYTHON} /usr/bin/pip

# Symlink example jar, so that we can easily use it in tests
ln -s /stackable/spark/examples/jars/spark-examples_*.jar /stackable/spark/examples/jars/spark-examples.jar
chown -h ${STACKABLE_USER_UID}:0 /stackable/spark/examples/jars/spark-examples.jar

# fix permissions
chmod g=u /stackable/spark
chmod g=u /stackable/jmx
chmod g=u /stackable/run-spark.sh
chmod g=u /stackable/*-src.tar.gz
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

USER ${STACKABLE_USER_UID}

WORKDIR ${SPARK_HOME}
ENTRYPOINT [ "/stackable/run-spark.sh" ]
