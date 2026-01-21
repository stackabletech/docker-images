# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

# RedHat UBI does not package krb5-server, so we cannot use it as a KDC
# FROM local-image/stackable-base
FROM rockylinux:9.3@sha256:d7be1c094cc5845ee815d4632fe377514ee6ebcf8efaed6892889657e5ddaaa6

ARG PRODUCT_VERSION

RUN dnf update --assumeyes && \
    dnf install --assumeyes krb5-workstation-${PRODUCT_VERSION} krb5-server-${PRODUCT_VERSION} && \
    dnf clean all
