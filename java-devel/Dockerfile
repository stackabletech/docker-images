# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

#
# Base imaege for builder stages
#

FROM stackable/image/stackable-base

ARG PRODUCT

# We need to use EPEL, as openjdk 22 is not shipped with UBI9
RUN rpm --install --replacepkgs https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm

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

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true
