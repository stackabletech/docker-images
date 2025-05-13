# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true;skip=InvalidDefaultArgInFrom

ARG GIT_SYNC

# For updated versions check https://github.com/kubernetes/git-sync/releases
# which should contain a image location (e.g. registry.k8s.io/git-sync/git-sync:v3.6.8)
FROM oci.stackable.tech/sdp/git-sync/git-sync:${GIT_SYNC} AS gitsync-image

FROM stackable/image/statsd_exporter AS statsd_exporter-builder

FROM python:3.12-bookworm AS opa-auth-manager-builder

COPY airflow/opa-auth-manager/ /tmp/opa-auth-manager

WORKDIR /tmp/opa-auth-manager

RUN <<EOF
pip install --no-cache-dir poetry
poetry build
poetry install
poetry run pytest
EOF

FROM stackable/image/vector AS airflow-build-image

ARG PRODUCT
ARG STATSD_EXPORTER
ARG PYTHON
ARG TARGETARCH
ARG STACKABLE_USER_UID
ARG S3FS
ARG CYCLONEDX_BOM
ARG UV

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
        python${PYTHON} \
        python${PYTHON}-devel \
        python${PYTHON}-pip \
        python${PYTHON}-wheel \
        # The airflow odbc provider can compile without the development files (headers and libraries) (see https://github.com/stackabletech/docker-images/pull/683)
        unixODBC \
        # Needed to modify the SBOM
        jq && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY airflow/stackable/constraints/${PRODUCT}/constraints-python${PYTHON}.txt /tmp/constraints.txt
COPY airflow/stackable/constraints/${PRODUCT}/build-constraints-python${PYTHON}.txt /tmp/build-constraints.txt
COPY --from=opa-auth-manager-builder /tmp/opa-auth-manager/dist/opa_auth_manager-0.1.0-py3-none-any.whl /tmp/

RUN <<EOF
python${PYTHON} -m venv --system-site-packages /stackable/app

source /stackable/app/bin/activate

# Upgrade pip to the latest version
# Also install uv to get support for build constraints
pip install --no-cache-dir --upgrade pip
pip install --no-cache-dir uv==${UV}

uv pip install --no-cache-dir apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT} --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt
# Needed for pandas S3 integration to e.g. write and read csv and parquet files to/from S3
uv pip install --no-cache-dir s3fs==${S3FS} cyclonedx-bom==${CYCLONEDX_BOM}
# Needed for OIDC
uv pip install --no-cache-dir Flask_OIDC==2.2.0 Flask-OpenID==1.3.1

uv pip install --no-cache-dir /tmp/opa_auth_manager-0.1.0-py3-none-any.whl

# Create the SBOM for Airflow
# Important: All `pip install` commands must be above this line, otherwise the SBOM will be incomplete
cyclonedx-py environment --schema-version 1.5 --outfile /tmp/sbom.json

# Break circular dependencies by removing the apache-airflow dependency from the providers
jq '.dependencies |= map(if .ref | test("^apache-airflow-providers-") then
    .dependsOn |= map(select(. != "apache-airflow=='${PRODUCT}'"))
else
    .
end)' /tmp/sbom.json > /stackable/app/airflow-${PRODUCT}.cdx.json
EOF

COPY --from=statsd_exporter-builder /statsd_exporter/statsd_exporter /stackable/statsd_exporter
COPY --from=statsd_exporter-builder /statsd_exporter/statsd_exporter-${STATSD_EXPORTER}.cdx.json /stackable/statsd_exporter-${STATSD_EXPORTER}.cdx.json
COPY --from=gitsync-image --chown=${STACKABLE_USER_UID}:0 /git-sync /stackable/git-sync

RUN <<EOF
mkdir -pv /stackable/airflow
mkdir -pv /stackable/airflow/dags
mkdir -pv /stackable/airflow/logs
chmod --recursive g=u /stackable
EOF


FROM stackable/image/vector AS airflow-main-image

ARG PRODUCT
ARG PYTHON
ARG RELEASE
ARG TINI
ARG TARGETARCH
ARG STACKABLE_USER_UID

LABEL name="Apache Airflow" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache Airflow." \
    description="This image is deployed by the Stackable Operator for Apache Airflow."

ENV HOME=/stackable
ENV AIRFLOW_USER_HOME_DIR=/stackable
ENV PATH=$PATH:/bin:$HOME/app/bin
ENV AIRFLOW_HOME=$HOME/airflow

COPY --from=airflow-build-image --chown=${STACKABLE_USER_UID}:0 /stackable/ ${HOME}/
COPY --from=airflow-build-image --chown=${STACKABLE_USER_UID}:0 /stackable/git-sync ${HOME}/git-sync

COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/utils/entrypoint.sh /entrypoint.sh
COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/utils/run-airflow.sh /run-airflow.sh

COPY airflow/licenses /licenses

# Update image and install needed packages
RUN <<EOF
microdnf update

# git: Needed for the gitsync functionality
# openldap: Needed for authentication of clients against LDAP servers
# openssh-clients: We need the openssh libs for the gitsync functionality (the clone target could be e.g. git@github.com:org/repo.git)
# python: Airflow needs Python
microdnf install \
  ca-certificates \
  cyrus-sasl \
  git \
  libpq \
  openldap \
  openldap-clients \
  openssh-clients \
  openssl-libs \
  openssl-pkcs11 \
  python${PYTHON} \
  socat \
  unixODBC
microdnf clean all
rm -rf /var/cache/yum

# Get the correct `tini` binary for our architecture.
# It is used as an init alternative in the entrypoint
curl -o /usr/bin/tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI}-${TARGETARCH}"

# fix missing permissions
chmod a+x /entrypoint.sh
chmod a+x /run-airflow.sh
chmod +x /usr/bin/tini
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in ${HOME} are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh ${HOME} ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

USER ${STACKABLE_USER_UID}
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
