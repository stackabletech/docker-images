FROM python:3.10-slim AS builder

ARG PRODUCT
ARG RELEASE="1"

LABEL name="Stackable Testing Tools" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="Stackable tools for integration tests." \
      description="Used by Stackable integration tests."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential=12.9 \
        ca-certificates=20210119 \
        curl=7.74.0-1.3+deb11u2 \
        gzip=1.10-4+deb11u1 \
        jq=1.6-2.1 \
        libssl-dev=1.1.1n-0+deb11u3 \
        libxml2-dev=2.9.10+dfsg-6.7+deb11u2 \
        libxslt1-dev=1.1.34-4 \
        pkg-config=0.29.2-1 \
        python3-certifi=2020.6.20-1 \
        python3-idna=2.10-1 \
        python3-requests=2.25.1+dfsg-2 \
        python3-semver=2.10.2-2 \
        python3-thrift=0.13.0-6 \
        python3-toml=0.10.1-1 \
        python3-urllib3=1.26.5-1~exp1 \
        tar=1.34+dfsg-1 \
        zip=3.0-12

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

COPY testing-tools/python /stackable/python
COPY testing-tools/licenses /licenses

RUN pip install --upgrade pip && \
    pip install --no-cache-dir -r /stackable/python/requirements.txt && \
    groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

