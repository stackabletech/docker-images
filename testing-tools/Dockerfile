# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM python:3.11-slim-bullseye@sha256:5120c734a5ba810c47b97a80cdc6e03004bd4ff7ad98e492b9ccdaf70ff2b8d6

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
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

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
RUN curl --fail https://repo.stackable.tech/repository/packages/keycloak/keycloak-${KEYCLOAK_VERSION}.tar.gz | tar -xzC /stackable && \
    ln -s /stackable/keycloak-${KEYCLOAK_VERSION} /stackable/keycloak

RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r /stackable/python/requirements.txt && \
    groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable
