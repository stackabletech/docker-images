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
    fuse-devel \
    gcc \
    gcc-c++ \
    git \
    java-"${PRODUCT}"-openjdk-devel \
    krb5-devel \
    libcurl-devel \
    make \
    maven \
    openssl-devel \
    tar \
    wget \
    which \
    xz \
    zlib-devel \
    unzip zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}

