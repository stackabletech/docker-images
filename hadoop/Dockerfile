# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM stackable/image/java-devel AS hadoop-builder

ARG PRODUCT
ARG ASYNC_PROFILER
ARG JMX_EXPORTER
ARG PROTOBUF
ARG TARGETARCH
ARG TARGETOS
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 shared/protobuf/stackable/patches/patchable.toml /stackable/src/shared/protobuf/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/protobuf/stackable/patches/${PROTOBUF} /stackable/src/shared/protobuf/stackable/patches/${PROTOBUF}

RUN <<EOF
rpm --install --replacepkgs https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm
microdnf update
# boost is a build dependency starting in Hadoop 3.4.0 if compiling native code
# automake and libtool are required to build protobuf
microdnf install boost1.78-devel automake libtool
microdnf clean all
rm -rf /var/cache/yum
EOF

# This Protobuf version is the exact version as used in the Hadoop Dockerfile
# See https://github.com/apache/hadoop/blob/trunk/dev-support/docker/pkg-resolver/install-protobuf.sh
# (this was hardcoded in the Dockerfile in earlier versions of Hadoop, make sure to look at the exact version in Github)
RUN <<EOF
    cd "$(/stackable/patchable --images-repo-root=src checkout shared/protobuf ${PROTOBUF})"

    # Create snapshot of the source code including custom patches
    tar -czf /stackable/protobuf-${PROTOBUF}-src.tar.gz .

    ./autogen.sh
    ./configure --prefix=/opt/protobuf
    make "-j$(nproc)"
    make install
    (cd .. && rm -r ${PROTOBUF})
EOF

ENV PROTOBUF_HOME=/opt/protobuf
ENV PATH="${PATH}:/opt/protobuf/bin"

RUN <<EOF
# async-profiler
ARCH="${TARGETARCH/amd64/x64}"
curl "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC .
ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

# JMX Exporter
mkdir /stackable/jmx
curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
chmod -x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar
EOF

WORKDIR /build
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/stackable/patches/patchable.toml /build/src/hadoop/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/stackable/patches/${PRODUCT} /build/src/hadoop/stackable/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/stackable/fuse_dfs_wrapper /build
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/stackable/jmx /stackable/jmx
# Hadoop Pipes requires libtirpc to build, whose headers are not packaged in RedHat UBI, so skip building this module
# Build from source to enable FUSE module, and to apply custom patches.
# Also skip building the yarn, mapreduce and minicluster modules: this will result in the modules being excluded but not all
# jar files will be stripped if they are needed elsewhere e.g. share/hadoop/yarn will not be part of the build, but yarn jars
# will still exist in share/hadoop/tools as they would be needed by the resource estimator tool. Such jars are removed in a later step.
RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hadoop ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/hadoop-${PRODUCT}-src.tar.gz .

mvn \
    --batch-mode \
    --no-transfer-progress \
    clean package \
    -Pdist,native \
    -pl '!hadoop-tools/hadoop-pipes,!hadoop-yarn-project,!hadoop-mapreduce-project,!hadoop-minicluster' \
    -Drequire.fuse=true \
    -DskipTests \
    -Dmaven.javadoc.skip=true

cp -r hadoop-dist/target/hadoop-${PRODUCT} /stackable/hadoop-${PRODUCT}
mv hadoop-dist/target/bom.json /stackable/hadoop-${PRODUCT}/hadoop-${PRODUCT}.cdx.json

# HDFS fuse-dfs is not part of the regular dist output, so we need to copy it in ourselves
cp hadoop-hdfs-project/hadoop-hdfs-native-client/target/main/native/fuse-dfs/fuse_dfs /stackable/hadoop-${PRODUCT}/bin
rm -rf /build/hadoop-${PRODUCT}-src

ln -s /stackable/hadoop-${PRODUCT} /stackable/hadoop

mv /build/fuse_dfs_wrapper /stackable/hadoop/bin

# Remove unneeded binaries:
#  - code sources
#  - mapreduce/yarn binaries that were built as cross-project dependencies
#  - minicluster (only used for testing) and test .jars
#  - json-io: this is a transitive dependency pulled in by cedarsoft/java-utils/json-io and is excluded in 3.4.0. See CVE-2023-34610.
rm -rf /stackable/hadoop/share/hadoop/common/sources/
rm -rf /stackable/hadoop/share/hadoop/hdfs/sources/
rm -rf /stackable/hadoop/share/hadoop/tools/sources/
rm -rf /stackable/hadoop/share/hadoop/tools/lib/json-io-*.jar
rm -rf /stackable/hadoop/share/hadoop/tools/lib/hadoop-mapreduce-client-*.jar
rm -rf /stackable/hadoop/share/hadoop/tools/lib/hadoop-yarn-server*.jar
find /stackable -name 'hadoop-minicluster-*.jar' -type f -delete
find /stackable -name 'hadoop-client-minicluster-*.jar' -type f -delete
find /stackable -name 'hadoop-*tests.jar' -type f -delete
rm -rf /stackable/.m2

# Set correct groups; make sure only required artifacts for the final image are located in /stackable
chmod -R g=u /stackable
EOF

FROM stackable/image/java-devel AS hdfs-utils-builder

ARG HDFS_UTILS
ARG PRODUCT
ARG STACKABLE_USER_UID

# Starting with hdfs-utils 0.4.0 we need to use Java 17 for compilation.
# We can not simply use java-devel with Java 17, as it is also used to compile Hadoop in this
# Dockerfile, which needs Java 11. So we need to also use the java-devel image in version 11 and
# install Java 17 ourselves.
# The adptiom yum repo is already added by the java-devel Dockerfile.
RUN <<EOF
microdnf update
microdnf install -y temurin-17-jdk
microdnf clean all
rm -rf /var/cache/yum
EOF

ENV JAVA_HOME="/usr/lib/jvm/temurin-17-jdk"

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hadoop/hdfs-utils/stackable/patches/patchable.toml /stackable/src/hadoop/hdfs-utils/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/hdfs-utils/stackable/patches/${HDFS_UTILS} /stackable/src/hadoop/hdfs-utils/stackable/patches/${HDFS_UTILS}

# The Stackable HDFS utils contain an OPA authorizer, group mapper & topology provider.
# The topology provider provides rack awareness functionality for HDFS by allowing users to specify Kubernetes
# labels to build a rackID from.
# Starting with hdfs-utils version 0.3.0 the topology provider is not a standalone jar anymore and included in hdfs-utils.
RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hadoop/hdfs-utils ${HDFS_UTILS})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/hdfs-utils-${HDFS_UTILS}-src.tar.gz .

mvn \
    --batch-mode \
    --no-transfer-progress\
    clean package \
    -P hadoop-${PRODUCT} \
    -DskipTests \
    -Dmaven.javadoc.skip=true

mkdir -p /stackable
cp target/hdfs-utils-$HDFS_UTILS.jar /stackable/hdfs-utils-${HDFS_UTILS}.jar
rm -rf hdfs-utils-main

# Set correct groups
chmod g=u /stackable/hdfs-utils-${HDFS_UTILS}.jar /stackable/hdfs-utils-${HDFS_UTILS}-src.tar.gz
EOF

FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG HDFS_UTILS
ARG STACKABLE_USER_UID

LABEL \
    name="Apache Hadoop" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache Hadoop." \
    description="This image is deployed by the Stackable Operator for Apache Hadoop / HDFS."

COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hdfs-utils-builder /stackable/hdfs-utils-${HDFS_UTILS}.jar /stackable/hadoop-${PRODUCT}/share/hadoop/common/lib/hdfs-utils-${HDFS_UTILS}.jar
COPY --chown=${STACKABLE_USER_UID}:0 --from=hdfs-utils-builder /stackable/hdfs-utils-${HDFS_UTILS}-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hadoop/licenses /licenses

# fuse is required for fusermount (called by fuse_dfs)
# fuse-libs is required for fuse_dfs (not included in fuse)
# openssl -> not sure
RUN <<EOF
microdnf update
# tar is required for `kubectl cp` which can be used to copy the log files
# or profiler flamegraph from the Pod
# It is already installed in the base image but leaving here for documentation purposes
microdnf install \
  fuse \
  fuse-libs \
  tar
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

# Without this fuse_dfs does not work
# It is so non-root users (as we are) can mount a FUSE device and let other users access it
echo "user_allow_other" > /etc/fuse.conf
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

USER ${STACKABLE_USER_UID}

ENV HOME=/stackable
ENV LD_LIBRARY_PATH=/stackable/hadoop/lib/native:/usr/lib/jvm/jre/lib/server
ENV PATH="${PATH}":/stackable/hadoop/bin
ENV HADOOP_HOME=/stackable/hadoop
ENV HADOOP_CONF_DIR=/stackable/config
ENV ASYNC_PROFILER_HOME=/stackable/async-profiler
# The following 2 env-vars are required for common scripts even if the respective libraries are never used.
# HADOOP_HOME is often used internally if HADOOP_YARN_HOME/HADOOP_MAPRED_HOME are not set, although
# a subdirectory is also required in (at least)
#   hadoop-common-project/hadoop-common/src/main/bin/hadoop-functions.sh
# if HADOOP_YARN_HOME does not exist at all, so we set it here to a sensible default.
ENV HADOOP_YARN_HOME=/stackable/hadoop
ENV HADOOP_MAPRED_HOME=/stackable/hadoop

WORKDIR /stackable/hadoop
CMD ["echo", "This image is not meant to be 'run' directly."]
