# Build stage

FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:c5ffdf5938d73283cec018f2adf59f0ed9f8c376d93e415a27b16c3c6aad6f45 AS builder

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
        setuptools \
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
        trino[sqlalchemy]

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

# Final image
FROM prom/statsd-exporter:0.3.0@sha256:a9c27602d6f6b86527657922b6a87c12789f7f9b39a90f1513e8c665c941f26a as statsd-exporter
FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:e58664de16551db29fb0eaaeb3c4a44eaf95ad89a5b2399a1107041c4f2d6d34

ARG PRODUCT
ARG PYTHON
ARG RELEASE=1

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
        shadow-utils \
    && microdnf clean all \
    && groupadd --gid 1000 stackable \
    && useradd --uid 1000 --gid stackable --home-dir ${HOME} stackable

COPY superset/licenses /licenses
COPY --from=builder /stackable ${HOME}
COPY --from=statsd-exporter /bin/statsd_exporter /stackable/statsd_exporter

RUN chown --recursive stackable:stackable ${HOME}

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
