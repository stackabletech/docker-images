# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# hadolint ignore=DL3006
FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG ASYNC_PROFILER
ARG JMX_EXPORTER
ARG PROTOBUF
ARG TOPOLOGY_PROVIDER
ARG TARGETARCH
ARG TARGETOS

WORKDIR /stackable

COPY hadoop/stackable /stackable


# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HDFS Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HDFS operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HDFS Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
RUN curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod -x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

RUN ARCH="${TARGETARCH/amd64/x64}" && \
    curl --fail -L "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC . && \
    ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

# This Protobuf version is the exact version as used in the Hadoop Dockerfile
# See https://github.com/apache/hadoop/blob/trunk/dev-support/docker/pkg-resolver/install-protobuf.sh
# (this was hardcoded in the Dockerfile in earlier versions of Hadoop, make sure to look at the exact version in Github)
WORKDIR /opt/protobuf-src
RUN curl --fail -L -s -S https://repo.stackable.tech/repository/packages/protobuf/protobuf-java-${PROTOBUF}.tar.gz -o /opt/protobuf.tar.gz && \
    tar xzf /opt/protobuf.tar.gz --strip-components 1 --no-same-owner && \
    ./configure --prefix=/opt/protobuf && \
    make "-j$(nproc)" && \
    make install && \
    rm -rf /opt/protobuf-src

ENV PROTOBUF_HOME /opt/protobuf
ENV PATH "${PATH}:/opt/protobuf/bin"

WORKDIR /stackable

# Hadoop Pipes requires libtirpc to build, whose headers are not packaged in RedHat UBI, so skip building this module
# Build from source to enable FUSE module, and to apply custom patches.
# Also skip building the yarn, mapreduce and minicluster modules.
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hadoop/hadoop-${PRODUCT}-src.tar.gz" | tar -xzC . && \
    patches/apply_patches.sh ${PRODUCT} && \
    cd hadoop-${PRODUCT}-src && \
    mvn clean package -Pdist,native -pl '!hadoop-tools/hadoop-pipes,!hadoop-yarn-project,!hadoop-mapreduce-project,!hadoop-minicluster' -Drequire.fuse=true -DskipTests -Dmaven.javadoc.skip=true && \
    cp -r hadoop-dist/target/hadoop-${PRODUCT} /stackable/hadoop-${PRODUCT} && \
    # HDFS fuse-dfs is not part of the regular dist output, so we need to copy it in ourselves
    cp hadoop-hdfs-project/hadoop-hdfs-native-client/target/main/native/fuse-dfs/fuse_dfs /stackable/hadoop-${PRODUCT}/bin && \
    rm -rf /stackable/hadoop-${PRODUCT}-src

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
RUN /bin/log4shell.sh "/stackable/hadoop-${PRODUCT}"

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s "/stackable/hadoop-${PRODUCT}"
# ===

# Final Image
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG TOPOLOGY_PROVIDER
ARG HDFS_UTILS

LABEL name="Apache Hadoop" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Hadoop." \
      description="This image is deployed by the Stackable Operator for Apache Hadoop / HDFS."

# fuse is required for fusermount (called by fuse_dfs)
# fuse-libs is required for fuse_dfs (not included in fuse)
# openssl -> not sure
RUN microdnf update && \
    microdnf install \
    fuse \
    fuse-libs \
    # tar is required for `kubectl cp` which can be used to copy the log files
    # or profiler flamegraph from the Pod
    tar && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY hadoop/licenses /licenses

# Without this fuse_dfs does not work
# It is so non-root users (as we are) can mount a FUSE device and let other users access it
RUN echo "user_allow_other" > /etc/fuse.conf

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable --from=builder /stackable/hadoop-${PRODUCT} /stackable/hadoop-${PRODUCT}/
COPY --chown=stackable:stackable --from=builder /stackable/jmx /stackable/jmx/
COPY --chown=stackable:stackable --from=builder /stackable/async-profiler /stackable/async-profiler/
RUN ln -s /stackable/hadoop-${PRODUCT} /stackable/hadoop

# The topology provider provides rack awareness functionality for HDFS by allowing users to specify Kubernetes
# labels to build a rackID from
# source code is at: https://github.com/stackabletech/hdfs-topology-provider
# N.B. the artifact name changed from 0.2.0 onwards i.e. from topology-provider-0.1.0.jar to hdfs-topology-provider-0.2.0.jar
RUN curl --fail -L -s -S https://repo.stackable.tech/repository/packages/hdfs-topology-provider/hdfs-topology-provider-${TOPOLOGY_PROVIDER}.jar -o /stackable/hadoop/share/hadoop/common/lib/hdfs-topology-provider-${TOPOLOGY_PROVIDER}.jar

# The Stackable HDFS utils contain an OPA authorizer and group mapper
RUN curl --fail -L -s -S https://repo.stackable.tech/repository/packages/hdfs-utils/hdfs-utils-${HDFS_UTILS}.jar -o /stackable/hadoop/share/hadoop/common/lib/hdfs-utils-${HDFS_UTILS}.jar

COPY hadoop/stackable/fuse_dfs_wrapper /stackable/hadoop/bin

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

# Remove unneeded binaries:
#  - source code
#  - mapreduce/yarn binaries that were built as cross-project dependencies
#  - minicluster (only used for testing) and test .jars
#  - json-io: this is a transitive dependency pulled in by cedarsoft/java-utils/json-io and is excluded in 3.4.0. See CVE-2023-34610.
RUN rm -rf /stackable/hadoop/share/hadoop/common/sources/ && \
  rm -rf /stackable/hadoop/share/hadoop/hdfs/sources/ && \
  rm -rf /stackable/hadoop/share/hadoop/tools/sources/ && \
  rm -rf /stackable/hadoop/share/hadoop/tools/lib/json-io-*.jar && \
  rm -rf /stackable/hadoop/share/hadoop/tools/lib/hadoop-mapreduce-client-*.jar && \
  rm -rf /stackable/hadoop/share/hadoop/tools/lib/hadoop-yarn-server*.jar && \
  find . -name hadoop-minicluster-*.jar -type f -delete && \
  find . -name hadoop-client-minicluster-*.jar -type f -delete && \
  find . -name hadoop-*tests.jar -type f -delete

WORKDIR /stackable/hadoop
CMD ["echo", "This image is not meant to be 'run' directly."]
