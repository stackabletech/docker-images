# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/testing-tools

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

LABEL name="Stackable Testing Tools - Hive" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT_VERSION}" \
      release="${RELEASE_VERSION}" \
      summary="Stackable tools for Hive integration tests." \
      description="Hive-specific testing tools image based on testing-tools base image."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

USER root

COPY testing-tools/hive/requirements.txt /stackable/hive/requirements.txt

RUN <<EOF
microdnf update

microdnf clean all
rm -rf /var/cache/yum

pip install --no-cache-dir -r /stackable/hive/requirements.txt
EOF

USER ${STACKABLE_USER_UID}

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT_VERSION}

WORKDIR /stackable
