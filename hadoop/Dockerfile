# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/hadoop/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS hdfs-utils-builder

ARG HDFS_UTILS
ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID
ARG HADOOP_HADOOP
# Reassign the arg to `HADOOP_VERSION` for better readability.
# It is passed as `HADOOP_HADOOP`, because versions.py has to contain `hadoop/hadoop` to establish a dependency on the Hadoop builder.
# The value of `hadoop/hadoop` is transformed by `bake` and automatically passed as `HADOOP_HADOOP` arg.
ENV HADOOP_VERSION=${HADOOP_HADOOP}

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

COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-libs /stackable/patched-libs

# The Stackable HDFS utils contain an OPA authorizer, group mapper & topology provider.
# The topology provider provides rack awareness functionality for HDFS by allowing users to specify Kubernetes
# labels to build a rackID from.
# Starting with hdfs-utils version 0.3.0 the topology provider is not a standalone jar anymore and included in hdfs-utils.
RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hadoop/hdfs-utils ${HDFS_UTILS})"

# Make Maven aware of custom Stackable libraries
mkdir -p /stackable/.m2/repository
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

# Create snapshot of the source code including custom patches
tar -czf /stackable/hdfs-utils-${HDFS_UTILS}-src.tar.gz .

mvn \
    --batch-mode \
    --no-transfer-progress\
    clean package \
    -P hadoop-${HADOOP_VERSION} \
    -Dhadoop.version=${HADOOP_VERSION}-stackable${RELEASE} \
    -DskipTests \
    -Dmaven.javadoc.skip=true

cp target/hdfs-utils-$HDFS_UTILS.jar /stackable/hdfs-utils-${HDFS_UTILS}.jar
rm -rf hdfs-utils-main

# Set correct groups
chmod g=u /stackable/hdfs-utils-${HDFS_UTILS}.jar /stackable/hdfs-utils-${HDFS_UTILS}-src.tar.gz
EOF

FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG HADOOP_HADOOP
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP}
ARG HDFS_UTILS
ARG STACKABLE_USER_UID
ARG ASYNC_PROFILER
ARG JMX_EXPORTER
ARG TARGETARCH
ARG TARGETOS

LABEL \
    name="Apache Hadoop" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache Hadoop." \
    description="This image is deployed by the Stackable Operator for Apache Hadoop / HDFS."

COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE} /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE}
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/*-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 --from=hdfs-utils-builder /stackable/hdfs-utils-${HDFS_UTILS}.jar /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE}/share/hadoop/common/lib/hdfs-utils-${HDFS_UTILS}.jar
COPY --chown=${STACKABLE_USER_UID}:0 --from=hdfs-utils-builder /stackable/hdfs-utils-${HDFS_UTILS}-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hadoop/stackable/jmx /stackable/jmx
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

ln -s "/stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE}" /stackable/hadoop

# async-profiler
ARCH="${TARGETARCH/amd64/x64}"
curl "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC /stackable
ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

# JMX Exporter
curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
chmod -x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# Set correct permissions and ownerships
chown --recursive ${STACKABLE_USER_UID}:0 /stackable/hadoop /stackable/jmx /stackable/async-profiler "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}"
chmod --recursive g=u /stackable/jmx /stackable/async-profiler "/stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE}"

# Workaround for https://issues.apache.org/jira/browse/HADOOP-12845
# The problem is that our stackable-devel image does contain the openssl-devel package
# That package creates a symlink from /usr/lib/libcrypto.so to the real libcrypto
# The non -devel package, which is used in this image, does NOT create this symlink.
# That's why the Hadoop build works even with the 'require.openssl' flag but in the production
# image the 'hadoop checknative' tool still fails because it can't find the 'libcrypto.so' symlink.
# Therefore we create this symlink here.
ln -s /usr/lib64/libcrypto.so.3 /usr/lib64/libcrypto.so

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
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
