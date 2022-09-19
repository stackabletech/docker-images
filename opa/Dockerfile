FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6-941 AS builder

ARG PRODUCT
ARG RELEASE="1"

LABEL name="Open Policy Agent" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for OPA." \
      description="This image is deployed by the Stackable Operator for OPA."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

COPY opa/licenses /licenses

WORKDIR /stackable/opa
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

RUN curl -L https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT}.tar.gz | tar -xzC . && \
    chmod +x opa

CMD ["./opa", "run", "-s"]
