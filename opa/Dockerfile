FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# OPA PRODUCT_VERSION
ARG PRODUCT_VERSION

RUN microdnf update && \
    microdnf install shadow-utils && \
    microdnf clean all

# Download OPA
WORKDIR /stackable/opa
RUN curl -L -o opa https://openpolicyagent.org/downloads/v$PRODUCT_VERSION/opa_linux_amd64_static && \
    chmod 755 ./opa

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable && \
    chown -R stackable /stackable

USER stackable

WORKDIR /stackable
CMD ["/stackable/opa/opa", "run", "-s"]
