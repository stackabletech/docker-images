FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip zip shadow-utils curl openssl && \
    microdnf clean all

# The bcrypt tool is needed by NiFi to locally encrypt the admin password that is mounted as a secret in cleartext
RUN curl -L https://github.com/bitnami/bcrypt-cli/releases/download/v1.0.2/bcrypt-linux-amd64.tar.gz | tar -xzC . && \
    mv bcrypt-linux-amd64 /bin/bcrypt && \
    chmod +x /bin/bcrypt

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    mkdir /stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

