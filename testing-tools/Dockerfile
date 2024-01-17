FROM python:3.11-slim-bullseye@sha256:286f2f1d6f2f730a44108656afb04b131504b610a6cb2f3413918e98dabba67e

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
