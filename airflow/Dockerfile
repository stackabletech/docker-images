FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5
LABEL maintainer="Stackable GmbH"

ARG PRODUCT
ARG PYTHON

COPY airflow/constraints-${PRODUCT}-python${PYTHON}.txt /tmp/constraints.txt
COPY airflow/stackable/utils/tini-v0.19.0 /usr/bin/tini
COPY airflow/stackable/utils/entrypoint.sh /entrypoint

RUN chmod a+x /entrypoint
RUN chmod +x /usr/bin/tini

RUN microdnf update \
        && microdnf install \
            gcc \
            gcc-c++ \
            python${PYTHON}-devel \
            python${PYTHON}-pip \
            python${PYTHON}-wheel \
            openssl-devel \
            cyrus-sasl-devel \
            openldap-devel \
        && microdnf clean all \
        && python3 -m venv --system-site-packages /stackable/app \
        && source /stackable/app/bin/activate \
        && pip install --upgrade pip \
        && pip install apache-airflow[async,amazon,celery,cncf.kubernetes,docker,dask,elasticsearch,ftp,grpc,hashicorp,http,ldap,google,microsoft.azure,postgres,redis,sendgrid,sftp,slack,ssh,statsd,virtualenv]==${PRODUCT} \
            --constraint /tmp/constraints.txt

ENV HOME=/stackable
ENV AIRFLOW_USER_HOME_DIR=/stackable
ENV GID=0

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable
WORKDIR /stackable


ENTRYPOINT ["/usr/bin/tini", "--", "/entrypoint"]
CMD []


# TODOs
# -----
# removed mysql
# add odbc, pandas, google_auth?