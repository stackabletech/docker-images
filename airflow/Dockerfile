# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# Disabled error checks:
# - SecretsUsedInArgOrEnv : OPA_AUTH_MANAGER is a false positive and breaks the build.
# check=error=true;skip=InvalidDefaultArgInFrom,SecretsUsedInArgOrEnv

ARG GIT_SYNC_VERSION

# For updated versions check https://github.com/kubernetes/git-sync/releases
# which should contain a image location (e.g. registry.k8s.io/git-sync/git-sync:v3.6.8)
FROM oci.stackable.tech/sdp/git-sync/git-sync:${GIT_SYNC_VERSION} AS gitsync-image

FROM local-image/shared/statsd-exporter AS statsd_exporter-builder

FROM local-image/vector AS opa-auth-manager-builder

ARG OPA_AUTH_MANAGER
ARG PYTHON_VERSION
ARG UV_VERSION

COPY airflow/opa-auth-manager/${OPA_AUTH_MANAGER} /tmp/opa-auth-manager

WORKDIR /tmp/opa-auth-manager

RUN <<EOF
microdnf update
microdnf install python${PYTHON_VERSION}-pip
microdnf clean all

pip${PYTHON_VERSION} install --no-cache-dir uv==${UV_VERSION}

# This folder is required by the tests to set up an sqlite database
mkdir /root/airflow

# Warnings are disabled because they come from various third party testing libraries
# that we have no control over.
uv run pytest --disable-warnings
uv build
EOF

FROM local-image/stackable-devel AS airflow-build-image

ARG PRODUCT_VERSION
ARG PYTHON_VERSION
ARG TARGETARCH
ARG STACKABLE_USER_UID
ARG NODEJS_VERSION
ARG S3FS_VERSION
ARG CYCLONEDX_BOM_VERSION
ARG UV_VERSION

# Airflow "extras" packages are listed here: https://airflow.apache.org/docs/apache-airflow/stable/extra-packages-ref.html
# They evolve over time and thus belong to the version-specific arguments.
# The mysql provider is currently excluded.
# Requires implementation of https://github.com/apache/airflow/blob/2.2.5/scripts/docker/install_mysql.sh
ARG AIRFLOW_EXTRAS

RUN microdnf module enable -y nodejs:${NODEJS_VERSION} && \
    microdnf update && \
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
        python${PYTHON_VERSION} \
        python${PYTHON_VERSION}-devel \
        python${PYTHON_VERSION}-pip \
        python${PYTHON_VERSION}-wheel \
        # The airflow odbc provider can compile without the development files (headers and libraries) (see https://github.com/stackabletech/docker-images/pull/683)
        unixODBC \
        # Needed for Airflow UI assets
        npm \
        nodejs \
        # Needed to modify the SBOM
        jq && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY airflow/stackable/constraints/${PRODUCT_VERSION}/constraints-python${PYTHON_VERSION}.txt /tmp/constraints.txt
COPY airflow/stackable/constraints/${PRODUCT_VERSION}/build-constraints-python${PYTHON_VERSION}.txt /tmp/build-constraints.txt
COPY --from=opa-auth-manager-builder /tmp/opa-auth-manager/dist/opa_auth_manager-0.1.0-py3-none-any.whl /tmp/

COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/patches/patchable.toml /stackable/src/airflow/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/patches/${PRODUCT_VERSION} /stackable/src/airflow/stackable/patches/${PRODUCT_VERSION}

WORKDIR /stackable

RUN <<EOF
python${PYTHON_VERSION} -m venv --system-site-packages /stackable/app

source /stackable/app/bin/activate

# Upgrade pip to the latest version
# Also install uv to get support for build constraints
pip install --no-cache-dir --upgrade pip
pip install --no-cache-dir uv==${UV_VERSION}
uv tool install hatch

cd "$(/stackable/patchable --images-repo-root=src checkout airflow ${PRODUCT_VERSION})"

if [ -d "./airflow-core" ]; then
  # Airflow 3.x
  cd airflow-core/src/airflow/ui

  # build front-end assets
  npm install -g pnpm
  pnpm install --frozen-lockfile
  pnpm run build

  # build airflow wheel from airflow root folder
  # this picks up the UI assets from the pnpm build, and the dependencies from the root folder
  cd ../../..
  /root/.local/bin/hatch build -t wheel
  # First install the full apache-airflow package to get all dependencies including database drivers
  uv pip install --no-cache-dir apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT_VERSION} --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt
  # Then install the locally built core wheel to override the core package
  uv pip install --no-cache-dir dist/apache_airflow_core-${PRODUCT_VERSION}-py3-none-any.whl[${AIRFLOW_EXTRAS}] --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt
else
  # Airflow 2.x
  # build front-end assets
  cd airflow/www
  npm install -g yarn
  yarn install --frozen-lockfile
  yarn run build

  # build airflow wheel from airflow root folder
  cd ../..
  /root/.local/bin/hatch build -t wheel
  # First install the full apache-airflow package to get all dependencies including database drivers
  uv pip install --no-cache-dir apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT_VERSION} --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt
  # Then install the locally built wheel to override with patched version
  uv pip install --no-cache-dir dist/apache_airflow-${PRODUCT_VERSION}-py3-none-any.whl[${AIRFLOW_EXTRAS}] --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt
fi

# Needed for pandas S3 integration to e.g. write and read csv and parquet files to/from S3
uv pip install --no-cache-dir s3fs==${S3FS_VERSION} cyclonedx-bom==${CYCLONEDX_BOM_VERSION}
# Needed for OIDC
uv pip install --no-cache-dir Flask_OIDC==2.2.0 Flask-OpenID==1.3.1

uv pip install --no-cache-dir /tmp/opa_auth_manager-0.1.0-py3-none-any.whl

# Create the SBOM for Airflow
# Important: All `pip install` commands must be above this line, otherwise the SBOM will be incomplete
cyclonedx-py environment --schema-version 1.5 --outfile /tmp/sbom.json

# Break circular dependencies by removing the apache-airflow dependency from the providers
jq '.dependencies |= map(if .ref | test("^apache-airflow-providers-") then
    .dependsOn |= map(select(. != "apache-airflow=='${PRODUCT_VERSION}'"))
else
    .
end)' /tmp/sbom.json > /stackable/app/airflow-${PRODUCT_VERSION}.cdx.json
EOF

RUN <<EOF
mkdir -pv /stackable/airflow
mkdir -pv /stackable/airflow/dags
mkdir -pv /stackable/airflow/logs
chmod --recursive g=u /stackable
EOF


FROM local-image/vector AS airflow-main-image

ARG PRODUCT_VERSION
ARG PYTHON_VERSION
ARG RELEASE_VERSION
ARG TINI_VERSION
ARG TARGETARCH
ARG SHARED_STATSD_EXPORTER_VERSION
ARG STACKABLE_USER_UID

LABEL name="Apache Airflow" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT_VERSION}" \
    release="${RELEASE_VERSION}" \
    summary="The Stackable image for Apache Airflow." \
    description="This image is deployed by the Stackable Operator for Apache Airflow."

ENV HOME=/stackable
ENV AIRFLOW_USER_HOME_DIR=/stackable
ENV PATH=$PATH:/bin:$HOME/app/bin
ENV AIRFLOW_HOME=$HOME/airflow

COPY --from=airflow-build-image --chown=${STACKABLE_USER_UID}:0 /stackable/ ${HOME}/

COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/utils/entrypoint.sh /entrypoint.sh
COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/utils/run-airflow.sh /run-airflow.sh

COPY --from=statsd_exporter-builder --chown=${STACKABLE_USER_UID}:0 /statsd_exporter/statsd_exporter /stackable/statsd_exporter
COPY --from=statsd_exporter-builder --chown=${STACKABLE_USER_UID}:0 /statsd_exporter/statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}.cdx.json /stackable/statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}.cdx.json
COPY --from=gitsync-image --chown=${STACKABLE_USER_UID}:0 /git-sync /stackable/git-sync

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
  python${PYTHON_VERSION} \
  socat \
  unixODBC
microdnf clean all
rm -rf /var/cache/yum

# Get the correct `tini` binary for our architecture.
# It is used as an init alternative in the entrypoint
curl -o /usr/bin/tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI_VERSION}-${TARGETARCH}"

# fix missing permissions
chmod a+x /entrypoint.sh
chmod a+x /run-airflow.sh
chmod +x /usr/bin/tini
chmod g=u /stackable/statsd_exporter /stackable/statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}.cdx.json /stackable/git-sync
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
