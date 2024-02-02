# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Get statsd-exporter
ARG STATSD_EXPORTER
FROM oci.stackable.tech/sdp/statsd-exporter:${STATSD_EXPORTER} as statsd-exporter

# Build stage
FROM stackable/image/vector AS builder

ARG PRODUCT
ARG PYTHON
ARG AUTHLIB

COPY superset/constraints-${PRODUCT}.txt /tmp/constraints.txt

RUN microdnf update \
    && microdnf install \
        --assumeyes \
        cyrus-sasl-devel \
        # Needed to find all patch files
        findutils \
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
        # Redhat has removed `tzdata` from the ubi-minimal images: see https://bugzilla.redhat.com/show_bug.cgi?id=2223028.
        # Superset relies on ZoneInfo (https://docs.python.org/3/library/zoneinfo.html#data-sources) to resolve time zones, and this is done
        # by searching first under `TZPATH` (which is empty due to the point above) or for the tzdata python package.
        # That package is therefore added here (airflow has tzdata in its list of dependencies, but superset does not).
        tzdata \
    && pip install \
        --no-cache-dir \
        --upgrade \
        python-json-logger \
    && if [ ! -z "$AUTHLIB" ]; then pip install Authlib==${AUTHLIB}; fi

COPY superset/stackable/patches /patches
RUN /patches/apply_patches.sh ${PRODUCT}

# Final image
FROM stackable/image/vector

ARG PRODUCT
ARG PYTHON
ARG RELEASE

LABEL name="Apache Superset" \
      maintainer="info@stackable.tech" \
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
