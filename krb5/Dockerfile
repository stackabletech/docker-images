# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# RedHat UBI does not package krb5-server, so we cannot use it as a KDC
# FROM local-image/stackable-base
FROM rockylinux/rockylinux:9.8.20260525.0-ubi@sha256:01d66807e8ec11cf13002b6c0f9300cd09a28e7f1e135950bbac0492e940ea96

ARG PRODUCT_VERSION

RUN dnf update --assumeyes && \
    dnf install --assumeyes krb5-workstation-${PRODUCT_VERSION} krb5-server-${PRODUCT_VERSION} && \
    dnf clean all
