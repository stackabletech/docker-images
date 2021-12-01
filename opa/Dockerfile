FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

WORKDIR /stackable/opa
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

ARG PRODUCT

USER stackable

RUN curl -L https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT}.tar.gz | tar -xzC . && \
    chmod +x opa

CMD ["./opa", "run", "-s"]
