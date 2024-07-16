# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

ARG GIT_SYNC

# For updated versions check https://github.com/kubernetes/git-sync/releases
# which should contain a image location (e.g. registry.k8s.io/git-sync/git-sync:v3.6.8)
FROM oci.stackable.tech/sdp/git-sync:${GIT_SYNC} as gitsync-image

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/statsd_exporter AS statsd_exporter-builder

FROM stackable/image/vector AS airflow-build-image

ARG PRODUCT
ARG PYTHON
ARG TARGETARCH
ARG TARGETOS

COPY airflow/constraints-${PRODUCT}-python${PYTHON}.txt /tmp/constraints.txt

# The mysql provider is currently excluded.
# Requires implementation of https://github.com/apache/airflow/blob/2.2.5/scripts/docker/install_mysql.sh
ENV AIRFLOW_EXTRAS=async,amazon,celery,cncf.kubernetes,docker,dask,elasticsearch,ftp,grpc,hashicorp,http,ldap,google,google_auth,microsoft.azure,odbc,pandas,postgres,redis,sendgrid,sftp,slack,ssh,statsd,virtualenv,trino

RUN microdnf update && \
    microdnf install \
        cyrus-sasl-devel \
        # Needed by ./configure to build gevent, see snippet [1] at the end of file
        diffutils \
        # Needed to build gevent, see snippet [1] at the end of file
        make \
        gcc \
        gcc-c++ \
        libpq-devel \
        openldap-devel \
        openssl-devel \
        python${PYTHON}-devel \
        python${PYTHON}-pip \
        python${PYTHON}-wheel \
        # The airflow odbc provider can compile without the development files (headers and libraries) (see https://github.com/stackabletech/docker-images/pull/683)
        unixODBC && \
    microdnf clean all && \
    rm -rf /var/cache/yum

RUN python3 -m venv --system-site-packages /stackable/app && \
    source /stackable/app/bin/activate && \
    pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT} --constraint /tmp/constraints.txt && \
    # Needed for pandas S3 integration to e.g. write and read csv and parquet files to/from S3
    pip install --no-cache-dir s3fs

WORKDIR /stackable
COPY --from=statsd_exporter-builder /statsd_exporter/statsd_exporter /stackable/statsd_exporter

FROM stackable/image/vector AS airflow-main-image

ARG PRODUCT
ARG PYTHON
ARG RELEASE
ARG TINI
ARG TARGETARCH
ARG TARGETOS

LABEL name="Apache Airflow" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Airflow." \
      description="This image is deployed by the Stackable Operator for Apache Airflow."

COPY airflow/licenses /licenses

# Update image and install python
# g++ is required for libstdc++ that is preloaded in entrypoint.sh
RUN microdnf update && \
    microdnf install \
    ca-certificates \
    cyrus-sasl \
    git \
    g++ \
    libpq \
    openldap \
    openldap-clients \
    openssh-clients \
    openssl-libs \
    openssl-pkcs11 \
    python${PYTHON} \
    socat \
    unixODBC && \
    microdnf clean all && \
    rm -rf /var/cache/yum

ENV HOME=/stackable
ENV AIRFLOW_USER_HOME_DIR=/stackable
ENV PATH=$PATH:/bin:$HOME/app/bin
ENV AIRFLOW_HOME=$HOME/airflow


# Get the correct `tini` binary for our architecture.
# It is used as an init alternative in the entrypoint
RUN mkdir -pv ${AIRFLOW_HOME} && \
    mkdir -pv ${AIRFLOW_HOME}/dags && \
    mkdir -pv ${AIRFLOW_HOME}/logs && \
    chown --recursive stackable:stackable ${AIRFLOW_HOME} && \
    curl --fail -o /usr/bin/tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI}-${TARGETARCH}"

COPY airflow/stackable/utils/entrypoint.sh /entrypoint.sh
COPY airflow/stackable/utils/run-airflow.sh /run-airflow.sh
RUN chmod a+x /entrypoint.sh && \
    chmod a+x /run-airflow.sh && \
    chmod +x /usr/bin/tini

COPY --from=airflow-build-image --chown=stackable:stackable /stackable/ ${HOME}/
COPY --from=gitsync-image --chown=stackable:stackable /git-sync /stackable/git-sync

USER stackable
WORKDIR /stackable

ENTRYPOINT ["/usr/bin/tini", "--", "/run-airflow.sh"]
CMD []

# SNIPPET 1
# 137.0       Running '(cd  "/tmp/pip-install-cyuymnu6/gevent_0f8b4d282c464210b62acdf399e4a04c/deps/libev"  && sh ./configure -C > configure-output.txt )' in /tmp/pip-install-cyuymnu6/gevent_0f8b4d282c464210b62acdf399e4a04c
# 137.0       ./configure: line 6350: cmp: command not found
# 137.0       ./configure: line 6350: cmp: command not found
# 137.0       ./configure: line 8279: diff: command not found
# 137.0       config.status: error: in `/tmp/pip-install-cyuymnu6/gevent_0f8b4d282c464210b62acdf399e4a04c/deps/libev':
# 137.0       config.status: error: Something went wrong bootstrapping makefile fragments
# 137.0           for automatic dependency tracking.  Try re-running configure with the
# 137.0           '--disable-dependency-tracking' option to at least be able to build
# 137.0           the package (albeit without support for automatic dependency tracking).
