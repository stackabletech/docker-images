FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

USER stackable

ARG PRODUCT_VERSION

RUN curl -L https://repo.stackable.tech/repository/packages/node-exporter/node_exporter-${PRODUCT_VERSION}.linux-amd64.tar.gz | tar -xzC .

WORKDIR /stackable/node_exporter-${PRODUCT_VERSION}.linux-amd64
CMD ["./node_exporter"]

