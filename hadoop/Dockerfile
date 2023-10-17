# syntax=docker/dockerfile:1
FROM stackable/image/java-base AS builder

ARG PRODUCT
ARG JMX_EXPORTER

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# unzip & zip are required for log4shell.sh
# All others are required for the FUSE build
RUN microdnf update && \
    microdnf install \
    cmake \
    cyrus-sasl-devel \
    fuse-devel \
    gcc \
    gcc-c++ \
    java-11-openjdk-devel \
    maven \
    openssl-devel \
    tar \
    unzip \
    zip && \
    microdnf clean all

WORKDIR /stackable

# This is needed here because it creates the JMX directory, we could create it any other way but this works
COPY hadoop/stackable /stackable

# The source is needed to build FUSE. The rest of the src package will not make it into the final image.
# Both the src and binary variants extract into different root folders
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hadoop/hadoop-${PRODUCT}-src.tar.gz" | tar -xzC . && \
    curl --fail -L "https://repo.stackable.tech/repository/packages/hadoop/hadoop-${PRODUCT}.tar.gz" | tar -xzC . && \
    ln -s "/stackable/hadoop-${PRODUCT}" /stackable/hadoop && \
    rm -rf /stackable/hadoop/lib/native/examples && \
    rm -rf /stackable/hadoop/share/doc

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


# This Protobuf version is the exact version as used in the Hadoop Dockerfile
# See https://github.com/apache/hadoop/blob/trunk/dev-support/docker/pkg-resolver/install-protobuf.sh
# (this was hardcoded in the Dockerfile in earlier versions of Hadoop, make sure to look at the exact version in Github)
# For now all versions of Hadoop we support use Protobuf 3.7.1 so we can hardcode it here.
# Should it ever differ between versions we'll need to make this a variable as well.
RUN mkdir -p /opt/protobuf-src && \
    curl --fail -L -s -S https://repo.stackable.tech/repository/packages/protobuf/protobuf-java-3.7.1.tar.gz -o /opt/protobuf.tar.gz && \
    tar xzf /opt/protobuf.tar.gz --strip-components 1 -C /opt/protobuf-src --no-same-owner && \
    cd /opt/protobuf-src && \
    ./configure --prefix=/opt/protobuf && \
    make "-j$(nproc)" && \
    make install && \
    cd /root && \
    rm -rf /opt/protobuf-src

ENV PROTOBUF_HOME /opt/protobuf
ENV PATH "${PATH}:/opt/protobuf/bin"

WORKDIR /stackable/hadoop-${PRODUCT}-src/hadoop-hdfs-project/hadoop-hdfs-native-client

# This command comes from hadoop-hdfs-project/hadoop-hdfs-native-client/src/main/native/fuse-dfs/doc/README
RUN mvn clean package -Pnative -Drequire.fuse=true -DskipTests -Dmaven.javadoc.skip=true && \
    cp target/main/native/fuse-dfs/fuse_dfs /stackable/hadoop/bin && \
    rm -rf /stackable/hadoop-${PRODUCT}-src


# Final Image
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Apache Hadoop" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Hadoop." \
      description="This image is deployed by the Stackable Operator for Apache Hadoop / HDFS."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# fuse is required for fusermount (called by fuse_dfs)
# fuse-libs is required for fuse_dfs (not included in fuse)
# openssl -> not sure
RUN microdnf update && \
    microdnf install \
    fuse \
    fuse-libs \
    krb5-workstation && \
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
RUN ln -s /stackable/hadoop-${PRODUCT} /stackable/hadoop
COPY hadoop/stackable/fuse_dfs_wrapper /stackable/hadoop/bin

ENV HOME=/stackable
ENV LD_LIBRARY_PATH=/stackable/hadoop/lib/native:/usr/lib/jvm/jre/lib/server
ENV PATH="${PATH}":/stackable/hadoop/bin
ENV HADOOP_HOME=/stackable/hadoop
ENV HADOOP_CONF_DIR=/stackable/config

WORKDIR /stackable/hadoop
CMD ["echo", "This image is not meant to be 'run' directly."]
