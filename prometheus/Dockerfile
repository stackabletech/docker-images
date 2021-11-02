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

ARG PRODUCT_VERSION

RUN curl -L https://repo.stackable.tech/repository/packages/prometheus/prometheus-${PRODUCT_VERSION}.linux-amd64.tar.gz | tar -xzC . && \
    chown -R stackable:stackable /stackable

USER stackable

WORKDIR /stackable/prometheus-${PRODUCT_VERSION}.linux-amd64
ADD stackable/script/prometheus-wrapper.sh .

CMD ["./prometheus", "--config.file=./prometheus.yml"]

