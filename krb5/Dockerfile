# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# RedHat UBI does not package krb5-server, so we cannot use it as a KDC
# FROM stackable/image/stackable-base
FROM rockylinux:9.3@sha256:d7be1c094cc5845ee815d4632fe377514ee6ebcf8efaed6892889657e5ddaaa6

ARG PRODUCT

RUN dnf update --assumeyes && \
    dnf install --assumeyes krb5-workstation-${PRODUCT} krb5-server-${PRODUCT} && \
    dnf clean all
