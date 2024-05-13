#
# Base imaege for builder stages
#

# hadolint ignore=DL3006
FROM stackable/image/stackable-base

ARG PRODUCT

# hadolint ignore=DL3041
RUN microdnf update && \
    microdnf install -y \
    cmake \
    cyrus-sasl-devel \
    # diff is required by maven during the build of hbase \
    # Cannot run program "diff" (in directory "/stackable/hbase-2.4.12-src/hbase-shaded/hbase-shaded-check-invariants"
    diffutils \
    fuse-devel \
    gcc \
    gcc-c++ \
    # The GNU gettext utilities contain the envsubst program which
    # substitutes the values of environment variables.
    gettext \
    # For the apply_patches.sh script
    git \
    # needed by the maven ant run plugin for the "set-hostname-property" step in zookeeper
    hostname \
    java-"${PRODUCT}"-openjdk-devel \
    krb5-devel \
    libcurl-devel \
    make \
    maven \
    openssl-devel \
    # Required to unpack Omid tarball
    tar \
    wget \
    which \
    xz \
    zlib-devel \
    # Required for log4shell.sh
    unzip zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}

