# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/trino/storage-connector AS trino-storage-connector-image

FROM stackable/image/trino/trino AS trino-builder

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG JMX_EXPORTER

LABEL \
    name="Trino" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Trino." \
    description="This image is deployed by the Stackable Operator for Trino."

RUN <<EOF
microdnf update
# libstdc++ required by snappy and duckdb, see https://github.com/trinodb/trino/pull/25143
microdnf install \
    gzip \
    httpd-tools \
    python \
    tar \
    libstdc++ \
    zip
microdnf clean all
rm -rf /var/cache/yum
EOF

COPY --from=trino-builder --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} /stackable/trino-server /stackable/trino-server-${PRODUCT}-stackable${RELEASE}
COPY --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} trino/licenses /licenses

COPY --from=trino-storage-connector-image --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} /stackable/src/trino-storage-connector/patchable-work/worktree/${PRODUCT}/target/trino-storage-${PRODUCT}-stackable${RELEASE} /stackable/trino-server-${PRODUCT}-stackable${RELEASE}/plugin/trino-storage-${PRODUCT}-stackable${RELEASE}
COPY --from=trino-storage-connector-image --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} /stackable/src/trino-storage-connector/patchable-work/worktree/${PRODUCT}/target/bom.json /stackable/trino-server-${PRODUCT}-stackable${RELEASE}/plugin/trino-storage-${PRODUCT}-stackable${RELEASE}/trino-storage-${PRODUCT}-stackable${RELEASE}.cdx.json
COPY --from=trino-storage-connector-image --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} /stackable/trino-storage-connector-${PRODUCT}-stackable${RELEASE}-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} trino/stackable/jmx /stackable/jmx

RUN <<EOF
# JMX Exporter
curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar
ln -s /stackable/trino-server-${PRODUCT}-stackable${RELEASE} /stackable/trino-server

# Set correct permissions
chown -R ${STACKABLE_USER_UID}:${STACKABLE_USER_GID} /stackable/jmx /stackable/trino-server
chmod --recursive g=u /stackable
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
WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
