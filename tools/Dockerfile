FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip zip shadow-utils curl openssl && \
    # python3-setuptools
    microdnf install python3-devel python3-pip && \
    microdnf clean all

# The bcrypt tool is needed by NiFi to locally encrypt the admin password that is mounted as a secret in cleartext
COPY tools/bin/stackable-bcrypt-1.0-SNAPSHOT-jar-with-dependencies.jar /bin/stackable-bcrypt.jar
# add all python scripts
COPY tools/python /python

RUN pip3 install nipyapi

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    mkdir /stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

