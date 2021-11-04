FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

ARG PRODUCT_VERSION

WORKDIR /stackable/opa
RUN chown -R stackable:stackable /stackable

USER stackable

RUN curl -L https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT_VERSION}.tar.gz | tar -xzC .; \
    chmod +x opa

CMD ["./opa", "run", "-s"]
