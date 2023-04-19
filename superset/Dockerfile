# syntax=docker/dockerfile:1

# Build stage
FROM stackable/image/vector AS builder

ARG PRODUCT
ARG PYTHON

COPY superset/constraints-${PRODUCT}.txt /tmp/constraints.txt

RUN microdnf update \
    && microdnf install \
        --assumeyes \
        cyrus-sasl-devel \
        gcc \
        gcc-c++ \
        libffi-devel \
        openldap-devel \
        openssl-devel \
        patch \
        python${PYTHON//./}-devel \
        python${PYTHON//./}-pip \
        python${PYTHON//./}-wheel \
        libpq-devel \
    && microdnf clean all \
    && python3 -m venv /stackable/app \
    && source /stackable/app/bin/activate \
    && pip install \
        --no-cache-dir \
        --upgrade \
        setuptools==66.1.1 \
        pip \
    && pip install \
        --no-cache-dir \
        --upgrade \
        --constraint /tmp/constraints.txt \
        apache-superset==${PRODUCT} \
        gevent \
        psycopg2-binary \
        statsd \
        pydruid \
        python-ldap \
        trino[sqlalchemy] \
    && pip install \
        --no-cache-dir \
        --upgrade \
        python-json-logger

# Patch Python packages

# In Flask-AppBuilder v4.1.1, a bug in the LDAP integration was fixed (see
# https://github.com/dpgaspar/Flask-AppBuilder/pull/1846). Superset 2.0.0 and
# higher depend on a Flask-AppBuilder version containing this fix. Prior
# Superset versions must be patched.

COPY superset/patches-${PRODUCT}/* /tmp/patches/

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN cat /tmp/patches/* | patch \
    --directory=/stackable/app/lib/python${PYTHON}/site-packages \
    --strip=1

# Get statsd-exporter
FROM prom/statsd-exporter:v0.23.1@sha256:75440b4fda85e4c603baaf7fedaeeedff0dfbca943bc7bafb94ade0355a7d02d as statsd-exporter

# Final image
FROM stackable/image/vector

ARG PRODUCT
ARG PYTHON
ARG RELEASE

LABEL name="Apache Superset" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Superset." \
      description="This image is deployed by the Stackable Operator for Apache Superset."

ENV FLASK_APP="superset.app:create_app()" \
    FLASK_ENV="production" \
    HOME="/stackable" \
    SUPERSET_PORT="8088"
ENV PATH="${HOME}/app/bin:${PATH}" \
    PYTHONPATH="${HOME}/app/pythonpath"

RUN microdnf update \
    && microdnf install \
        --assumeyes \
        cyrus-sasl \
        openldap \
        openldap-clients \
        openssl-libs \
        openssl-pkcs11 \
        python${PYTHON//./} \
    && microdnf clean all

COPY superset/licenses /licenses
COPY --from=statsd-exporter /bin/statsd_exporter /stackable/statsd_exporter
COPY --from=builder --chown=stackable:stackable /stackable/ ${HOME}/

USER stackable
WORKDIR ${HOME}

CMD ["/bin/sh", "-c", \
    "gunicorn \
    --bind 0.0.0.0:${SUPERSET_PORT} \
    --worker-class gthread \
    --threads 20 \
    --timeout 60 \
    --limit-request-line 0 \
    --limit-request-field_size 0 \
    'superset.app:create_app()'"]
