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

cd "$(/stackable/patchable --images-repo-root=src checkout spark-k8s/hbase-connectors ${PRODUCT_VERSION})"/spark

NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-connector-${NEW_VERSION}-src.tar.gz .

# Building the hbase-connectors with JDK 17 is not yet supported, see
# https://github.com/apache/hbase-connectors/pull/132.
# As there are no JDK profiles, access to the non-public elements must
# be enabled with --add-opens, see https://openjdk.org/jeps/403 and
# https://openjdk.org/jeps/261#Breaking-encapsulation.
export JDK_JAVA_OPTIONS="\
    --add-opens java.base/java.lang=ALL-UNNAMED \
    --add-opens java.base/java.util=ALL-UNNAMED"

source /stackable/spark/env

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
    --define spark.version="${SPARK_VERSION}" \
    --define scala.version="${SCALA_VERSION}" \
    --define scala.binary.version="${SCALA_BINARY_VERSION}" \
    --define hadoop-three.version="${HADOOP_VERSION}" \
    --define hbase.version="${HBASE_VERSION}" \
    --define skipTests \
    --define maven.test.skip=true \
    clean package


cp "$(pwd)/hbase-spark/target/hbase-spark-${NEW_VERSION}.jar" /stackable/spark/jars/hbase-spark-${NEW_VERSION}.jar

chmod g=u /stackable/hbase-connector-${NEW_VERSION}-src.tar.gz .
EOF
