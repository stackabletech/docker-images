FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# OPA PRODUCT_VERSION
ARG PRODUCT_VERSION

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

# Download OPA
WORKDIR /stackable/opa

RUN curl -L https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT_VERSION}.tar.gz | tar -xzC .
RUN chmod 755 opa

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable && \
    chown -R stackable /stackable

USER stackable

WORKDIR /stackable
CMD ["/stackable/opa/opa", "run", "-s"]
