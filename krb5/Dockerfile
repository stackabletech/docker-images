# syntax=docker/dockerfile:1
# RedHat UBI does not package krb5-server, so we cannot use it as a KDC
# FROM stackable/image/stackable-base
FROM rockylinux:8.7@sha256:68bef3459bbb8c33841575a7f71c4de94718b7bbd103fd0417a537395d40c592

ARG PRODUCT

RUN dnf update --assumeyes && \
    dnf install --assumeyes krb5-workstation-${PRODUCT} krb5-server-${PRODUCT} && \
    dnf clean all
