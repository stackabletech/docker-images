# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/statsd_exporter AS statsd_exporter-builder

FROM stackable/image/vector AS builder

ARG PRODUCT
ARG PYTHON
ARG AUTHLIB
ARG TARGETARCH
ARG TARGETOS

COPY superset/constraints-${PRODUCT}.txt /tmp/constraints.txt

RUN microdnf update \
    && microdnf install \
        cyrus-sasl-devel \
        # Needed to find all patch files, used in `apply_patches.sh`
        findutils \
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

RUN python3 -m venv /stackable/app \
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
    && if [ ! -z "$AUTHLIB" ]; then pip install Authlib==${AUTHLIB}; fi \
    # Only needed for superset 4.0.2 right now to fix logout issues with OIDC
    && if [ "$PRODUCT" == "4.0.2" ]; then \
        pip install \
        --no-cache-dir \
        --upgrade \
        Flask_OIDC \
        Flask-OpenID; fi

COPY superset/stackable/patches /patches
RUN /patches/apply_patches.sh ${PRODUCT}

WORKDIR /stackable
COPY --from=statsd_exporter-builder /statsd_exporter/statsd_exporter /stackable/statsd_exporter

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
        cyrus-sasl \
        openldap \
        openldap-clients \
        openssl-libs \
        openssl-pkcs11 \
        python${PYTHON} \
    && microdnf clean all && \
    rm -rf /var/cache/yum

COPY superset/licenses /licenses

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
