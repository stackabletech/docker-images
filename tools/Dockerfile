# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL4006

FROM stackable/image/stackable-base

ARG PRODUCT
ARG KUBECTL_VERSION
ARG RELEASE
ARG JQ_VERSION

LABEL name="Stackable Tools" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="Stackable Tools for Kubernetes." \
    description="Stackable Tools for Kubernetes."

RUN microdnf update && \
    microdnf install \
    gettext \
    gzip \
    iputils \
    openssl \
    tar \
    zip && \
    microdnf clean all

COPY tools/licenses /licenses

USER stackable
WORKDIR /stackable/bin
ENV PATH=/stackable/bin:$PATH

# Get latest stable version from curl -L -s https://dl.k8s.io/release/stable.txt
RUN	curl -L https://dl.k8s.io/release/v${KUBECTL_VERSION}/bin/linux/amd64/kubectl \
    -o /stackable/bin/kubectl && chmod +x /stackable/bin/kubectl

RUN	curl -L https://github.com/stedolan/jq/releases/download/jq-${JQ_VERSION}/jq-linux64 \
    -o /stackable/bin/jq && \
    chmod +x /stackable/bin/jq


USER stackable
