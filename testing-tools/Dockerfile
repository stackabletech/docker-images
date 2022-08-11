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
    apt-get install -y python3-thrift python3-certifi python3-idna python3-semver python3-toml python3-urllib3 python3-requests \
        tar gzip zip curl libssl-dev jq pkg-config \
        build-essential ca-certificates libxslt1-dev libxml2-dev 

COPY testing-tools/python /stackable/python
COPY testing-tools/licenses /licenses

RUN pip3 install --no-cache-dir -r /stackable/python/requirements.txt && \
    groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT}

WORKDIR /stackable

