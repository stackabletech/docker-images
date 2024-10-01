# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd
# check=error=true

# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
# https://hub.docker.com/_/python/tags
FROM python:3.12-slim-bullseye@sha256:dee6c73dad9e135ebf727e234f1bb2414b4634e34e03d90bc66c9762dec7e671

ARG PRODUCT
ARG RELEASE
ARG KEYCLOAK_VERSION

LABEL name="Stackable Testing Tools" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="Stackable tools for integration tests." \
      description="Used by Stackable integration tests."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

# This is needed so that krb5-user installs without prompting for a realm.
ENV DEBIAN_FRONTEND=noninteractive

# krb5-user/libkrb5-dev are needed for Kerberos support.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        gzip \
        jq \
        krb5-user \
        kubernetes-client \
        libkrb5-dev \
        libssl-dev \
        libxml2-dev \
        libxslt1-dev \
        pkg-config \
        python3-certifi \
        python3-idna \
        python3-semver \
        python3-thrift \
        python3-toml \
        python3-urllib3 \
        tar \
        zip \
        unzip \
        # Java 11 seems like the best middle-ground for all tools
        openjdk-11-jdk-headless && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY testing-tools/python /stackable/python
COPY testing-tools/licenses /licenses

ENV PATH=/stackable/keycloak/bin:$PATH
RUN curl --fail -L https://repo.stackable.tech/repository/packages/keycloak/keycloak-${KEYCLOAK_VERSION}.tar.gz | tar -xzC /stackable && \
    ln -s /stackable/keycloak-${KEYCLOAK_VERSION} /stackable/keycloak

RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r /stackable/python/requirements.txt && \
    groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable
