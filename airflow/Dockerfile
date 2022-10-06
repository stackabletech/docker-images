FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:e58664de16551db29fb0eaaeb3c4a44eaf95ad89a5b2399a1107041c4f2d6d34 AS airflow-build-image

ARG PRODUCT
ARG PYTHON

COPY airflow/constraints-${PRODUCT}-python${PYTHON}.txt /tmp/constraints.txt

# the mysql provider is currently excluded.
# Requires implementation of https://github.com/apache/airflow/blob/2.2.5/scripts/docker/install_mysql.sh
ENV AIRFLOW_EXTRAS=async,amazon,celery,cncf.kubernetes,docker,dask,elasticsearch,ftp,grpc,hashicorp,http,ldap,google,google_auth,microsoft.azure,odbc,pandas,postgres,redis,sendgrid,sftp,slack,ssh,statsd,virtualenv

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
        && microdnf clean all \
        && python3 -m venv --system-site-packages /stackable/app \
        && source /stackable/app/bin/activate \
        && pip install --no-cache-dir --upgrade pip \
        && pip install --no-cache-dir apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT} --constraint /tmp/constraints.txt

FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:e58664de16551db29fb0eaaeb3c4a44eaf95ad89a5b2399a1107041c4f2d6d34 as airflow-main-image

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
    && yum -y update-minimal --security --sec-severity=Important --sec-severity=Critical \
    && yum clean all \
    && microdnf clean all

ENV HOME=/stackable
ENV AIRFLOW_USER_HOME_DIR=/stackable
ENV PATH=$PATH:/bin:$HOME/app/bin
ENV AIRFLOW_UID="50000"
ENV AIRFLOW_HOME=$HOME/airflow
ENV TINI_VERSION=v0.19.0

RUN mkdir -pv ${HOME} && \
    mkdir -pv ${AIRFLOW_HOME} && \
    mkdir -pv ${AIRFLOW_HOME}/dags && \
    mkdir -pv ${AIRFLOW_HOME}/logs

# Ugly to download both archs here, there isn't a decent way to export an environment variable or have an if-else switch 
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-arm64 spark-k8s/stackable/utils/tini-arm64
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-amd64 spark-k8s/stackable/utils/tini-amd64 

# according to arch, copy binary to the name "tini"
# throwing unused binaries away afterwards
RUN \
    if [ $(arch) = "aarch64" ]; then \
    cp spark-k8s/stackable/utils/tini-arm64 /usr/bin/tini; \
    rm -rf spark-k8s/*; \
    else \
    cp spark-k8s/stackable/utils/tini-amd64 /usr/bin/tini; \
    rm -rf sprk-k8s/*; \
    fi 

COPY airflow/stackable/utils/entrypoint.sh /entrypoint

RUN chmod a+x /entrypoint && \
    chmod +x /usr/bin/tini

RUN mkdir -pv ${HOME} && \
    mkdir -pv ${AIRFLOW_HOME} && \
    mkdir -pv ${AIRFLOW_HOME}/dags && \
    mkdir -pv ${AIRFLOW_HOME}/logs

COPY --from=airflow-build-image /stackable/ ${HOME}/

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=${AIRFLOW_UID} -d /stackable stackable && \
    chown -R stackable:stackable /stackable

USER stackable
WORKDIR /stackable

ENTRYPOINT ["/usr/bin/tini", "--", "/entrypoint"]
CMD []