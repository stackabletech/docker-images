# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# The purpose of this stage is to gather jars and environment variables needed in the final stage.
# These are collected in the /stackable/spark directory.
FROM local-image/java-devel AS spark-source-builder

ARG RELEASE_VERSION
ARG SPARK_VERSION
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/stackable/patches/patchable.toml /stackable/src/spark-k8s/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/stackable/patches/${SPARK_VERSION} /stackable/src/spark-k8s/stackable/patches/${SPARK_VERSION}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout spark-k8s ${SPARK_VERSION})"

# NEW_VERSION="${SPARK_VERSION}-stackable${RELEASE_VERSION}"

# mvn versions:set -DnewVersion=$NEW_VERSION

mkdir -p /stackable/spark/jars

# Download log4j-slf4j-impl-x.x.x.jar containing the StaticLoggerBinder
# which is required by the connector.
# Spark contains only log4j-slf4j2-impl-x.x.x.jar but not
# log4j-slf4j-impl-x.x.x.jar. It is okay to have both JARs in the
# classpath as long as they have the same version.
mvn --non-recursive --file pom.xml \
    dependency:copy \
    -Dartifact=org.apache.logging.log4j:log4j-slf4j-impl:'${log4j.version}' \
    -DoutputDirectory=/stackable/spark/jars


# Create an environment files with properties needed to build the connector
# Get the Scala version used by Spark
SCALA_VERSION=$(grep "scala.version" pom.xml | head -n1 | awk -F '[<>]' '{print $3}')

# Get the Scala binary version used by Spark
SCALA_BINARY_VERSION=$(grep "scala.binary.version" pom.xml | head -n1 | awk -F '[<>]' '{print $3}')

echo "SCALA_VERSION=${SCALA_VERSION}" > /stackable/spark/env
echo "SCALA_BINARY_VERSION=${SCALA_BINARY_VERSION}" >> /stackable/spark/env
echo "SPARK_VERSION=${SPARK_VERSION}" >> /stackable/spark/env
EOF

# hbase-connectors-builder: Build the Spark HBase connector and copy
# required JARs into /stackable/spark/jars
FROM local-image/java-devel AS final

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG HADOOP_VERSION
ARG HBASE_VERSION
ARG HBASE_THIRDPARTY_VERSION
ARG THIRDPARTY_PROTOBUF_VERSION
ARG STACKABLE_USER_UID

# Patch the hbase-connectors source code
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/hbase-connectors/stackable/patches/patchable.toml /stackable/src/spark-k8s/hbase-connectors/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 spark-k8s/hbase-connectors/stackable/patches/${PRODUCT_VERSION} /stackable/src/spark-k8s/hbase-connectors/stackable/patches/${PRODUCT_VERSION}

# Copy jars and env from spark-source-builder
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-source-builder \
    /stackable/spark/jars \
    spark/jars
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-source-builder \
    /stackable/spark/env \
    spark/env

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout spark-k8s/hbase-connectors ${PRODUCT_VERSION})"

NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-connector-${NEW_VERSION}-src.tar.gz .

. /stackable/spark/env

# Build the Spark 4 HBase connector.
# The `spark4` modules are not part of the default reactor, hence the profile.
# The Scala binary version is deliberately not overridden: it must stay 2.13 for the spark4
# modules and 2.12 for the spark ones, otherwise both reactors produce the same
# hbase-spark-pushdown artifact and Maven rejects the build as a duplicate.
# `revision` is the project version (CI friendly version).
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
    --activate-profiles spark4 \
    --projects spark4/hbase-spark4 \
    --also-make \
    --define revision="${NEW_VERSION}" \
    --define spark4.version="${SPARK_VERSION}" \
    --define scala.version="${SCALA_VERSION}" \
    --define hadoop-three.version="${HADOOP_VERSION}" \
    --define hbase.version="${HBASE_VERSION}" \
    --define hbase-thirdparty.version="${HBASE_THIRDPARTY_VERSION}" \
    --define thirdparty.protobuf.version="${THIRDPARTY_PROTOBUF_VERSION}" \
    --define skipTests \
    --define maven.test.skip=true \
    clean package

cp "$(pwd)/spark4/hbase-spark4/target/hbase-spark4-${NEW_VERSION}.jar" /stackable/spark/jars/
# hbase-spark4 is a plain module JAR and does not bundle its dependencies, so ship the
# modules it needs at runtime: the shared SQL pushdown filters and the generated protobuf classes.
cp "$(pwd)/spark4/hbase-spark-pushdown/target/hbase-spark-pushdown_${SCALA_BINARY_VERSION}-${NEW_VERSION}.jar" /stackable/spark/jars/
cp "$(pwd)/spark/hbase-spark-protocol-shaded/target/hbase-spark-protocol-shaded-${NEW_VERSION}.jar" /stackable/spark/jars/

chmod g=u /stackable/hbase-connector-${NEW_VERSION}-src.tar.gz .
EOF
