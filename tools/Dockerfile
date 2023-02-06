# syntax=docker/dockerfile:1
FROM stackable/image/stackable-base

ARG PRODUCT
ARG RELEASE

LABEL name="Stackable Tools" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable Tools image." \
      description="This tools / helper image is deployed by several Stackable Operators."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

COPY tools/kubernetes.repo /etc/yum.repos.d/kubernetes.repo
RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip zip shadow-utils curl openssl kubectl \
                     python3-devel python3-pip python3-setuptools \
                     gcc libxslt-devel libxml2-devel && \
    microdnf clean all

COPY tools/licenses /licenses

RUN chown -R stackable:stackable /stackable

USER stackable

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

