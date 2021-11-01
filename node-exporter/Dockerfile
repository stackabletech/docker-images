FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# Download tar and gzip
RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable

WORKDIR /stackable

RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT_VERSION

RUN curl -L https://repo.stackable.tech/repository/packages/node-exporter/node_exporter-${PRODUCT_VERSION}.linux-amd64.tar.gz | tar -xzC .

WORKDIR /stackable/node_exporter-${PRODUCT_VERSION}.linux-amd64
CMD ["./node_exporter"]

