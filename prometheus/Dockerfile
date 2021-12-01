FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Download tar and gzip
RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

WORKDIR /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

ARG PRODUCT

RUN curl -L https://repo.stackable.tech/repository/packages/prometheus/prometheus-${PRODUCT}.linux-amd64.tar.gz | tar -xzC . && \
    chown -R stackable:stackable /stackable && \
    ln -s /stackable/prometheus-${PRODUCT}.linux-amd64 /stackable/prometheus

USER stackable

WORKDIR /stackable/prometheus
COPY stackable/script/prometheus-wrapper.sh .

CMD ["./prometheus", "--config.file=./prometheus.yml"]

