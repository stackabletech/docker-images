#
# Base imaege for builder stages
#
# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi9/ubi-minimal@sha256:2636170dc55a0931d013014a72ae26c0c2521d4b61a28354b3e2e5369fa335a3

ARG PRODUCT

RUN microdnf update && \
    microdnf install \
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

