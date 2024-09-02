# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/java-devel AS hadoop-builder

ARG PRODUCT
ARG ASYNC_PROFILER
ARG JMX_EXPORTER
ARG PROTOBUF
ARG TARGETARCH
ARG TARGETOS

WORKDIR /stackable

COPY hadoop/stackable/jmx /stackable/jmx
COPY hadoop/stackable/fuse_dfs_wrapper /stackable/fuse_dfs_wrapper

# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HDFS Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HDFS operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HDFS Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
RUN <<EOF
curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
chmod -x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar
ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar
#TODO: Can the symlink go?

ARCH="${TARGETARCH/amd64/x64}"
curl --fail -L "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC .
ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

# This Protobuf version is the exact version as used in the Hadoop Dockerfile
# See https://github.com/apache/hadoop/blob/trunk/dev-support/docker/pkg-resolver/install-protobuf.sh
# (this was hardcoded in the Dockerfile in earlier versions of Hadoop, make sure to look at the exact version in Github)
# At the time of writing we could save around ~350MB if we included this in the later RUN statement and deleted it afterwards
mkdir /opt/protobuf-src
cd /opt/protobuf-src
curl --fail -L -s -S https://repo.stackable.tech/repository/packages/protobuf/protobuf-java-${PROTOBUF}.tar.gz -o /opt/protobuf.tar.gz
tar xzf /opt/protobuf.tar.gz --strip-components 1 --no-same-owner
./configure --prefix=/opt/protobuf
make "-j$(nproc)"
make install
rm -rf /opt/protobuf-src
rm -f /opt/protobuf.tar.gz
EOF

ENV PROTOBUF_HOME=/opt/protobuf
ENV PATH="${PATH}:/opt/protobuf/bin"

RUN rpm --install --replacepkgs https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm
RUN microdnf update && \
    microdnf install \
    # boost is a build dependency starting in Hadoop 3.4.0 if compiling native code
    boost1.78-devel && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY hadoop/stackable/patches /stackable/patches

# Hadoop Pipes requires libtirpc to build, whose headers are not packaged in RedHat UBI, so skip building this module
# Build from source to enable FUSE module, and to apply custom patches.
# Also skip building the yarn, mapreduce and minicluster modules: this will result in the modules being excluded but not all
# jar files will be stripped if they are needed elsewhere e.g. share/hadoop/yarn will not be part of the build, but yarn jars
# will still exist in share/hadoop/tools as they would be needed by the resource estimator tool. Such jars are removed in a later step.
RUN <<EOF
curl --fail -L "https://repo.stackable.tech/repository/packages/hadoop/hadoop-${PRODUCT}-src.tar.gz" | tar -xzC .
EOF
