# Build stage

FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5 as builder

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
        python${PYTHON}-devel \
        python${PYTHON}-pip \
        python${PYTHON}-wheel \
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
        python-ldap

# Final image

FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5

ARG PYTHON
ARG RELEASE=1

LABEL name="Apache Superset" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Superset." \
      description="This image is deployed by the Stackable Operator for Apache Superset."

ENV FLASK_ENV="production" \
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
        python${PYTHON} \
        shadow-utils \
    && microdnf clean all \
    && groupadd --gid 1000 stackable \
    && useradd --uid 1000 --gid stackable --home-dir ${HOME} stackable

COPY superset/licenses /licenses
COPY --from=builder /stackable ${HOME}

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
