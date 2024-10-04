# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

# RedHat UBI does not package krb5-server, so we cannot use it as a KDC
# FROM stackable/image/stackable-base
FROM rockylinux:9.3@sha256:d7be1c094cc5845ee815d4632fe377514ee6ebcf8efaed6892889657e5ddaaa6

ARG PRODUCT

RUN dnf update --assumeyes && \
    dnf install --assumeyes krb5-workstation-${PRODUCT} krb5-server-${PRODUCT} && \
    dnf clean all
