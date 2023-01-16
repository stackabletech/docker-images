FROM registry.access.redhat.com/ubi8/ubi-minimal:8.7@sha256:c7036d048252451451bfc70493c2a849737e91c5517559ec508a0dbe7dfb6d15 AS builder

ARG PRODUCT
ARG RELEASE="1"

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

# The bcrypt tool is needed by NiFi to locally encrypt the admin password that is mounted as a secret in cleartext
COPY tools/bin/stackable-bcrypt-1.0-SNAPSHOT-jar-with-dependencies.jar /bin/stackable-bcrypt.jar
# add all python scripts
COPY tools/python /stackable/python
COPY tools/licenses /licenses

RUN pip3 install --no-cache-dir -r /stackable/python/requirements.txt && \
    groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

