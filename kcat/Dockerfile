# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Normally we would use stackable/image/stackable-base here, *but* we fail to link kcat at the end with error messages
# shown in Snippet 1, which we were not able to solve.
FROM stackable/image/java-base AS builder

ARG PRODUCT

RUN microdnf update \
    && microdnf install \
    cmake \
    cyrus-sasl-devel \
    gcc-c++ \
    krb5-devel \
    libcurl-devel \
    make \
    openssl-devel \
    tar \
    wget \
    which \
    zlib \
    zlib-devel && \
    microdnf clean all && \
    rm -rf /var/cache/yum

WORKDIR /stackable

RUN curl --fail -L -O https://repo.stackable.tech/repository/packages/kcat/kcat-${PRODUCT}.tar.gz \
    && tar xvfz kcat-${PRODUCT}.tar.gz \
    && cd kcat-${PRODUCT} \
    && ./bootstrap.sh

COPY --chown=stackable:stackable kcat/licenses /licenses
