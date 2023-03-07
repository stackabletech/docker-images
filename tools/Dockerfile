# syntax=docker/dockerfile:1
FROM stackable/image/stackable-base

ARG PRODUCT
ARG KUBECTL_VERSION
ARG RELEASE
ARG JQ_VERSION

LABEL name="Stackable Tools" \
    maintainer="info@stackable.de" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="Stackable Tools for Kubernetes." \
    description="Stackable Tools for Kubernetes."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install iputils tar gzip zip openssl && \
    microdnf clean all

COPY tools/licenses /licenses

USER stackable
WORKDIR /stackable/bin

# Get latest stable version from curl -L -s https://dl.k8s.io/release/stable.txt
RUN	curl -L https://dl.k8s.io/release/v${KUBECTL_VERSION}/bin/linux/amd64/kubectl \
    -o /stackable/bin/kubectl && chmod +x /stackable/bin/kubectl

RUN	curl -L https://github.com/stedolan/jq/releases/download/jq-${JQ_VERSION}/jq-linux64 \
    -o /stackable/bin/jq && \
    chmod +x /stackable/bin/jq


USER stackable
