FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5

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

ENV FLASK_ENV="production" \
    HOME="/stackable" \
    SUPERSET_PORT="8088"
ENV PATH="${HOME}/app/bin:${PATH}" \
    PYTHONPATH="${HOME}/app/pythonpath"

COPY superset/licenses /licenses
COPY superset/constraints-${PRODUCT}.txt /tmp/constraints.txt

RUN microdnf update \
        && microdnf install \
            gcc \
            gcc-c++ \
            libffi-devel \
            python${PYTHON}-devel \
            python${PYTHON}-pip \
            python${PYTHON}-wheel \
            openssl-devel \
            cyrus-sasl-devel \
            openldap-devel \
        && microdnf clean all \
        && python3 -m venv --system-site-packages ${HOME}/app \
        && source ${HOME}/app/bin/activate \
        && pip install --upgrade setuptools pip \
        && pip install \
            --upgrade \
            --constraint /tmp/constraints.txt \
            apache-superset==${PRODUCT} \
            gevent \
            psycopg2-binary \
            statsd \
            python-ldap \
        && rm /tmp/constraints.txt \
        && groupadd --gid 1000 stackable \
        && useradd --uid 1000 --gid stackable --home-dir ${HOME} stackable \
        && chown --recursive stackable:stackable ${HOME}

USER stackable
WORKDIR ${HOME}

HEALTHCHECK CMD curl -f http://localhost:${SUPERSET_PORT}/health

CMD gunicorn \
    --bind 0.0.0.0:${SUPERSET_PORT} \
    --worker-class gthread \
    --threads 20 \
    --timeout 60 \
    --limit-request-line 0 \
    --limit-request-field_size 0 \
    "superset.app:create_app()"
