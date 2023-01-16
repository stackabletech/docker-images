FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:c5ffdf5938d73283cec018f2adf59f0ed9f8c376d93e415a27b16c3c6aad6f45 AS airflow-build-image

ARG PRODUCT
ARG PYTHON

COPY airflow/constraints-${PRODUCT}-python${PYTHON}.txt /tmp/constraints.txt

# the mysql provider is currently excluded.
# Requires implementation of https://github.com/apache/airflow/blob/2.2.5/scripts/docker/install_mysql.sh
ENV AIRFLOW_EXTRAS=async,amazon,celery,cncf.kubernetes,docker,dask,elasticsearch,ftp,grpc,hashicorp,http,ldap,google,google_auth,microsoft.azure,odbc,pandas,postgres,redis,sendgrid,sftp,slack,ssh,statsd,virtualenv,trino

RUN microdnf update \
        && microdnf install -y \
            gcc \
            gcc-c++ \
            python${PYTHON}-devel \
            python${PYTHON}-pip \
            python${PYTHON}-wheel \
            openssl-devel \
            cyrus-sasl-devel \
            openldap-devel \
            unixODBC-devel \
            libpq-devel \
        && microdnf clean all \
        && python3 -m venv --system-site-packages /stackable/app \
        && source /stackable/app/bin/activate \
        && pip install --no-cache-dir --upgrade pip \
        && pip install --no-cache-dir apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT} --constraint /tmp/constraints.txt

FROM prom/statsd-exporter:0.3.0@sha256:a9c27602d6f6b86527657922b6a87c12789f7f9b39a90f1513e8c665c941f26a as statsd-exporter
FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:33931dce809712888d1a8061bfa676963f517daca993984afed3251bc1fb5987 as airflow-main-image

ARG PRODUCT
ARG PYTHON
ARG RELEASE="1"

LABEL name="Apache Airflow" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Airflow." \
      description="This image is deployed by the Stackable Operator for Apache Airflow."

COPY airflow/licenses /licenses

# Update image and install python
RUN microdnf install -y yum python${PYTHON} \
    openssl-libs \
    openssl-pkcs11 \
    openldap \
    openldap-clients \
    cyrus-sasl \
    unixODBC \
    libpq \
    && yum -y update-minimal --security --sec-severity=Important --sec-severity=Critical \
    && yum clean all \
    && microdnf clean all

ENV HOME=/stackable
ENV AIRFLOW_USER_HOME_DIR=/stackable
ENV PATH=$PATH:/bin:$HOME/app/bin
ENV AIRFLOW_UID="50000"
ENV AIRFLOW_HOME=$HOME/airflow

RUN mkdir -pv ${HOME} && \
    mkdir -pv ${AIRFLOW_HOME} && \
    mkdir -pv ${AIRFLOW_HOME}/dags && \
    mkdir -pv ${AIRFLOW_HOME}/logs

# according to arch, copy binary to the name "tini"
RUN curl -o /usr/bin/tini https://repo.stackable.tech/repository/packages/tini/tini-$(arch) 

COPY airflow/stackable/utils/entrypoint.sh /entrypoint
COPY --from=statsd-exporter /bin/statsd_exporter /stackable/statsd_exporter

RUN chmod a+x /entrypoint && \
    chmod +x /usr/bin/tini

RUN mkdir -pv ${HOME} && \
    mkdir -pv ${AIRFLOW_HOME} && \
    mkdir -pv ${AIRFLOW_HOME}/dags && \
    mkdir -pv ${AIRFLOW_HOME}/logs

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=${AIRFLOW_UID} -d /stackable stackable && \
    chown -R stackable:stackable /stackable

COPY --from=airflow-build-image --chown=stackable:stackable /stackable/ ${HOME}/

USER stackable
WORKDIR /stackable

ENTRYPOINT ["/usr/bin/tini", "--", "/entrypoint"]
CMD []
