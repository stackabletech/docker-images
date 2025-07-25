# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
# https://hub.docker.com/_/python/tags
# In Docker Hub, open up the tag and look for Index Digest. Otherwise do:
# docker pull python:3.12-slim-bullseye and see the digest that appears in the output.
FROM python:3.12-slim-bullseye@sha256:f6d639b794b394cbeb7a9327d5af9976f0e8d61353bcf41916984775c9bbed1a

ARG PRODUCT
ARG RELEASE
ARG KEYCLOAK_VERSION
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG STACKABLE_USER_NAME

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


COPY testing-tools/python /stackable/python
COPY testing-tools/licenses /licenses


RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  curl \
  gzip \
  jq \
  `# krb5-user/libkrb5-dev are needed for Kerberos support. ` \
  krb5-user \
  libkrb5-dev \
  kubernetes-client \
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
  `# Java 11 seems like the best middle-ground for all tools` \
  openjdk-11-jdk-headless

apt-get clean
rm -rf /var/lib/apt/lists/*

curl --fail -L https://repo.stackable.tech/repository/packages/keycloak/keycloak-${KEYCLOAK_VERSION}.tar.gz | tar -xzC /stackable
ln -s /stackable/keycloak-${KEYCLOAK_VERSION} /stackable/keycloak

pip install --no-cache-dir --upgrade pip
pip install --no-cache-dir -r /stackable/python/requirements.txt

groupadd --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}
useradd \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --create-home \
  --home-dir /stackable \
   ${STACKABLE_USER_NAME}

chown -R ${STACKABLE_USER_UID}:0 /stackable
EOF

ENV PATH=/stackable/keycloak/bin:$PATH

USER ${STACKABLE_USER_UID}

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable
