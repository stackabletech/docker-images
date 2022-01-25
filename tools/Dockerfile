FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip zip shadow-utils curl openssl && \
    microdnf clean all

# NiFi init containers need to be able to locally encrypt the admin password that is mounted as a secret in cleartext
# bcrypt tool is currently included as binary, source can be found in the stackable utils repo:
# https://github.com/stackabletech/stackable-utils
# Once the tool has seen a release this will be replaced by a download of the actual released version
COPY tools/bin/stackable-bcrypt /bin
RUN chmod +x /bin/stackable-bcrypt

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    mkdir /stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

