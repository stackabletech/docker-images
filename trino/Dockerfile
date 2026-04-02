# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/trino/storage-connector AS trino-storage-connector-image

FROM local-image/trino/trino AS trino-builder

FROM local-image/java-base

ARG PRODUCT_VERSION
ARG TRINO_STORAGE_CONNECTOR_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG JMX_EXPORTER_VERSION

LABEL \
    name="Trino" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT_VERSION}" \
    release="${RELEASE_VERSION}" \
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

COPY --from=trino-builder --chown=${STACKABLE_USER_UID}:0 /stackable/trino-server /stackable/trino-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 trino/licenses /licenses

COPY --from=trino-storage-connector-image --chown=${STACKABLE_USER_UID}:0 /stackable/src/trino-storage-connector/patchable-work/worktree/${TRINO_STORAGE_CONNECTOR_VERSION}/target/trino-storage-${TRINO_STORAGE_CONNECTOR_VERSION}-stackable${RELEASE_VERSION} /stackable/trino-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/plugin/trino-storage-${TRINO_STORAGE_CONNECTOR_VERSION}-stackable${RELEASE_VERSION}
COPY --from=trino-storage-connector-image --chown=${STACKABLE_USER_UID}:0 /stackable/src/trino-storage-connector/patchable-work/worktree/${TRINO_STORAGE_CONNECTOR_VERSION}/target/bom.json /stackable/trino-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/plugin/trino-storage-${TRINO_STORAGE_CONNECTOR_VERSION}-stackable${RELEASE_VERSION}/trino-storage-${TRINO_STORAGE_CONNECTOR_VERSION}-stackable${RELEASE_VERSION}.cdx.json
COPY --from=trino-storage-connector-image --chown=${STACKABLE_USER_UID}:0 /stackable/trino-storage-connector-${TRINO_STORAGE_CONNECTOR_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/jmx /stackable/jmx

RUN <<EOF
# JMX Exporter
curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar
chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar
ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar /stackable/jmx/jmx_prometheus_javaagent.jar
ln -s /stackable/trino-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/trino-server

# Set correct permissions
chown -R ${STACKABLE_USER_UID}:0 /stackable/jmx /stackable/trino-server
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
