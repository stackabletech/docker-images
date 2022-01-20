FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip shadow-utils curl && \
    microdnf clean all

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    mkdir /stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

