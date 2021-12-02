FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

WORKDIR /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

RUN curl -L https://repo.stackable.tech/repository/packages/node-exporter/node_exporter-${PRODUCT}.linux-amd64.tar.gz | tar -xzC . && \
    ln -s /stackable/node_exporter-${PRODUCT}.linux-amd64 /stackable/node_exporter

WORKDIR /stackable/node_exporter
CMD ["./node_exporter"]

