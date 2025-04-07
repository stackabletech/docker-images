# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/statsd_exporter AS statsd_exporter-builder

FROM stackable/image/stackable-base AS opa-authorizer-builder

ARG PYTHON

COPY superset/stackable/opa-authorizer /tmp/opa-authorizer

RUN <<EOF
microdnf update
microdnf install \
    gcc \
    gcc-c++ \
    python${PYTHON} \
    python${PYTHON}-devel \
    python${PYTHON}-pip
microdnf clean all
rm -rf /var/cache/yum

pip install \
    --no-cache-dir \
    --upgrade \
    poetry==2.1.1 \
    pytest==8.3.4

cd /tmp/opa-authorizer

poetry sync
poetry run pytest
poetry build
EOF

FROM stackable/image/vector AS builder

ARG PRODUCT
ARG PYTHON
ARG AUTHLIB
ARG TARGETARCH
ARG TARGETOS

COPY superset/constraints-${PRODUCT}.txt /tmp/constraints.txt
COPY --from=opa-authorizer-builder /tmp/opa-authorizer/dist/opa_authorizer-0.1.0-py3-none-any.whl /tmp/

RUN microdnf update \
    && microdnf install \
    cyrus-sasl-devel \
    # Needed by ./configure to work out SQLite compilation flags, see snippet [1] at the end of file
    diffutils \
    # According to https://stackoverflow.com/q/19530974 normally sqlite3 should be shipped with the Python
    # distribution. However, while addig ARM support we noticed that this does not seem to be the case for the
    # Python installation shipped in the ARM image variant. So I guess Make is used to find out the sqlite
    # compilation flags (and propably to not build sqlite from source(?)), see snippet [1] at the end of file
    make \
    gcc \
    gcc-c++ \
    libffi-devel \
    openldap-devel \
    openssl-devel \
    patch \
    python${PYTHON} \
    python${PYTHON}-devel \
    python${PYTHON}-pip \
    python${PYTHON}-wheel \
    libpq-devel \
    && microdnf clean all && \
    rm -rf /var/cache/yum

# DL3042 false-positive, --no-chache-dir is specified a few lines below.
#        See https://github.com/hadolint/hadolint/issues/1042.
# hadolint ignore=DL3042
RUN python3 -m venv /stackable/app \
    && source /stackable/app/bin/activate \
    && pip install \
    --no-cache-dir \
    --upgrade \
    setuptools==75.2.0 \
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
    'trino[sqlalchemy]' \
    # Add optional dependencies for use in custom Superset configurations.
    # Since https://github.com/stackabletech/superset-operator/pull/530
    # admins can add custom configuration to superset_conf.py.
    Flask_OIDC==2.2.0 \
    Flask-OpenID==1.3.1 \
    # Redhat has removed `tzdata` from the ubi-minimal images: see https://bugzilla.redhat.com/show_bug.cgi?id=2223028.
    # Superset relies on ZoneInfo (https://docs.python.org/3/library/zoneinfo.html#data-sources) to resolve time zones, and this is done
    # by searching first under `TZPATH` (which is empty due to the point above) or for the tzdata python package.
    # That package is therefore added here (airflow has tzdata in its list of dependencies, but superset does not).
    tzdata \
    # We bumped this from 21.2.0 to 22.0.0 to fix CVE-2024-1135
    # Superset 4.1.0 will contain at least 22.0.0, the bump was done in https://github.com/apache/superset/commit/4f693c6db0dc5c7286a36b8d23e90541943ff13f
    # We only want to bump this for the 4.0.x line, as the others already have updated and we don't want to accidentially downgrade the version
    && if [[ "$PRODUCT" =~ ^4\.0\..* ]]; \
    then echo "Superset 4.0.x detected, installing gunicorn 22.0.0 to fix CVE-2024-1135" \
    && pip install gunicorn==22.0.0; \
    fi \
    && pip install \
    --no-cache-dir \
    --upgrade \
    python-json-logger \
    cyclonedx-bom \
    && if [ -n "$AUTHLIB" ]; then pip install Authlib==${AUTHLIB}; fi && \
    pip install --no-cache-dir /tmp/opa_authorizer-0.1.0-py3-none-any.whl

COPY --from=statsd_exporter-builder /statsd_exporter/statsd_exporter /stackable/statsd_exporter
COPY superset/stackable/patches /patches

RUN <<EOF
/patches/apply_patches.sh ${PRODUCT}
cd /stackable
source /stackable/app/bin/activate && cyclonedx-py environment --schema-version 1.5 --outfile app/superset-${PRODUCT}.cdx.json
chmod --recursive g=u /stackable
EOF

# Final image
FROM stackable/image/vector

ARG PRODUCT
ARG PYTHON
ARG RELEASE
ARG STACKABLE_USER_UID

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

COPY superset/licenses /licenses

COPY --from=builder --chown=${STACKABLE_USER_UID}:0 /stackable/ ${HOME}/

RUN <<EOF
microdnf update
microdnf install \
  cyrus-sasl \
  openldap \
  openldap-clients \
  openssl-libs \
  openssl-pkcs11 \
  "python${PYTHON}"

microdnf clean all
rm -rf /var/cache/yum
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

USER ${STACKABLE_USER_UID}
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

# SNIPPET 1
# 60.38   × Getting requirements to build wheel did not run successfully.
# 60.38   │ exit code: 1
# 60.38   ╰─> [77 lines of output]
# 60.38       running egg_info
# 60.38       writing apsw.egg-info/PKG-INFO
# 60.38       writing dependency_links to apsw.egg-info/dependency_links.txt
# 60.38       writing top-level names to apsw.egg-info/top_level.txt
# 60.38         Getting the SQLite amalgamation
# 60.38           Fetching https://sqlite.org/2023/sqlite-autoconf-3420000.tar.gz
# 60.38           Length: 3148813  SHA1: 036575929b174c1b829769255491ba2b32bda9ee  MD5: 0c5a92bc51cf07cae45b4a1e94653dea
# 60.38           Checksums verified
# 60.38       /usr/lib64/python3.9/tarfile.py:2239: RuntimeWarning: The default behavior of tarfile extraction has been changed to disallow common exploits (including CVE-2007-4559). By default, absolute/parent paths are disallowed and some mode bits are cleared. See https://access.redhat.com/articles/7004769 for more details.
# 60.38         warnings.warn(
# 60.38           Running configure to work out SQLite compilation flags
# 60.38       ./configure: line 8084: cmp: command not found
# 60.38       ./configure: line 8084: cmp: command not found
# 60.38       ./configure: line 9847: diff: command not found
# 60.38       config.status: error: in `/tmp/pip-install-eu6p7tvi/apsw_df5f74a30ca84a5c90de5ea3a0691bec/sqlite3':
# 60.38       config.status: error: Something went wrong bootstrapping makefile fragments
# 60.38           for automatic dependency tracking.  If GNU make was not used, consider
# 60.38           re-running the configure script with MAKE="gmake" (or whatever is
# 60.38           necessary).  You can also try re-running configure with the
# 60.38           '--disable-dependency-tracking' option to at least be able to build
# 60.38           the package (albeit without support for automatic dependency tracking).
# 60.38       See `config.log' for more details
