# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# RedHat UBI does not package krb5-server, so we cannot use it as a KDC
# FROM stackable/image/stackable-base
FROM rockylinux:8.9@sha256:c464612ef7e3d54d658c3eaa4778b5cdc990ec7a4d9ab63b0f00c9994c6ce980

ARG PRODUCT

RUN dnf update --assumeyes && \
    dnf install --assumeyes krb5-workstation-${PRODUCT} krb5-server-${PRODUCT} && \
    dnf clean all
