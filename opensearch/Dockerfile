# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/opensearch/security-plugin AS opensearch-security-plugin
FROM local-image/opensearch/opensearch-prometheus-exporter AS opensearch-prometheus-exporter
FROM local-image/java-devel AS opensearch-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG TARGETARCH

# Local image versions
ARG OPENSEARCH_SECURITY_PLUGIN_VERSION
ARG OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION

# External plugin versions
ARG OPENSEARCH_ALERTING_PLUGIN_VERSION
ARG OPENSEARCH_JOB_SCHEDULER_PLUGIN_VERSION
ARG OPENSEARCH_ANOMALY_DETECTION_PLUGIN_VERSION
ARG OPENSEARCH_ASYNCHRONOUS_SEARCH_PLUGIN_VERSION
ARG OPENSEARCH_CROSS_CLUSTER_REPLICATION_PLUGIN_VERSION
ARG OPENSEARCH_CUSTOM_CODECS_PLUGIN_VERSION
ARG OPENSEARCH_FLOW_FRAMEWORK_PLUGIN_VERSION
ARG OPENSEARCH_GEOSPATIAL_PLUGIN_VERSION
ARG OPENSEARCH_INDEX_MANAGEMENT_PLUGIN_VERSION
ARG OPENSEARCH_KNN_PLUGIN_VERSION
ARG OPENSEARCH_ML_PLUGIN_VERSION
ARG OPENSEARCH_NEURAL_SEARCH_PLUGIN_VERSION
ARG OPENSEARCH_NOTIFICATIONS_CORE_PLUGIN_VERSION
ARG OPENSEARCH_NOTIFICATIONS_PLUGIN_VERSION
ARG OPENSEARCH_OBSERVABILITY_PLUGIN_VERSION
ARG OPENSEARCH_REPORTS_SCHEDULER_PLUGIN_VERSION
ARG OPENSEARCH_SEARCH_RELEVANCE_PLUGIN_VERSION
ARG OPENSEARCH_SECURITY_ANALYTICS_PLUGIN_VERSION
ARG OPENSEARCH_SKILLS_PLUGIN_VERSION
ARG OPENSEARCH_SQL_PLUGIN_VERSION
ARG OPENSEARCH_SYSTEM_TEMPLATES_PLUGIN_VERSION
ARG OPENSEARCH_UBI_PLUGIN_VERSION
ARG QUERY_INSIGHTS_PLUGIN_VERSION


WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch/stackable/patches/patchable.toml /stackable/src/opensearch/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch/stackable/patches/${PRODUCT_VERSION}

RUN <<EOF
microdnf update
microdnf install unzip
microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch ${PRODUCT_VERSION})"

ARCH="${TARGETARCH/amd64/x64}"
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"
# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-${NEW_VERSION}-src.tar.gz .
./scripts/build.sh -v "${PRODUCT_VERSION}" -s false -a "${ARCH}"
tar -xzf "artifacts/dist/opensearch-min-${PRODUCT_VERSION}-linux-${ARCH}.tar.gz" -C /stackable
unzip artifacts/core-plugins/repository-s3-${PRODUCT_VERSION}.zip -d /stackable/opensearch-${PRODUCT_VERSION}/plugins/repository-s3/
mv /stackable/opensearch-${PRODUCT_VERSION}/plugins/repository-s3/config /stackable/opensearch-${PRODUCT_VERSION}/config/repository-s3
unzip artifacts/core-plugins/telemetry-otel-${PRODUCT_VERSION}.zip -d /stackable/opensearch-${PRODUCT_VERSION}/plugins/telemetry-otel/
mv /stackable/opensearch-${PRODUCT_VERSION}/plugins/telemetry-otel/config /stackable/opensearch-${PRODUCT_VERSION}/config/telemetry-otel
./gradlew cyclonedxBom --warning-mode=summary -Dbuild.snapshot=false
cp distribution/docker/src/docker/bin/docker-entrypoint.sh /stackable/opensearch-${PRODUCT_VERSION}/opensearch-docker-entrypoint.sh
# Tell OpenSearch it is running in a container
sed -i -e 's/OPENSEARCH_DISTRIBUTION_TYPE=tar/OPENSEARCH_DISTRIBUTION_TYPE=docker/' /stackable/opensearch-${PRODUCT_VERSION}/bin/opensearch-env
EOF

WORKDIR /stackable/opensearch-security
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/src/opensearch/security-plugin/patchable-work/worktree/${OPENSEARCH_SECURITY_PLUGIN_VERSION}/build/distributions/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN_VERSION}.zip \
    opensearch-security-${OPENSEARCH_SECURITY_PLUGIN_VERSION}.zip

RUN <<EOF
unzip opensearch-security-${OPENSEARCH_SECURITY_PLUGIN_VERSION}.zip
rm opensearch-security-${OPENSEARCH_SECURITY_PLUGIN_VERSION}.zip
mv config /stackable/opensearch-${PRODUCT_VERSION}/config/opensearch-security
chmod 755 tools/*.sh
EOF

WORKDIR /stackable/opensearch-prometheus-exporter
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-prometheus-exporter \
    /stackable/src/opensearch/opensearch-prometheus-exporter/patchable-work/worktree/${OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION}/build/distributions/prometheus-exporter-${OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION}.zip \
    opensearch-prometheus-exporter-${OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION}.zip

RUN <<EOF
unzip opensearch-prometheus-exporter-${OPENSEARCH_SECURITY_PLUGIN_VERSION}.zip
rm opensearch-prometheus-exporter-${OPENSEARCH_SECURITY_PLUGIN_VERSION}.zip
EOF

WORKDIR /stackable/opensearch-${PRODUCT_VERSION}

RUN <<EOF
# The JDK from jdk-base is used instead.
rm -r jdk

# Install common plugins that are also used in the official image. The order is influenced by dependencies between plugins.
#
# TODO: The LTR plugin is currently missing because an official release for OpenSearch 3.x versions is missing.
# There are tags for 3.x versions in the git repo, so it can be built from source:
# See https://github.com/opensearch-project/opensearch-learning-to-rank-base
./bin/opensearch-plugin \
    install \
    --batch \
    "org.opensearch.plugin:alerting:${OPENSEARCH_ALERTING_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-job-scheduler:${OPENSEARCH_JOB_SCHEDULER_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-anomaly-detection:${OPENSEARCH_ANOMALY_DETECTION_PLUGIN_VERSION}" \
    "org.opensearch.plugin:asynchronous-search:${OPENSEARCH_ASYNCHRONOUS_SEARCH_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-cross-cluster-replication:${OPENSEARCH_CROSS_CLUSTER_REPLICATION_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-custom-codecs:${OPENSEARCH_CUSTOM_CODECS_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-flow-framework:${OPENSEARCH_FLOW_FRAMEWORK_PLUGIN_VERSION}" \
    "org.opensearch.plugin:geospatial:${OPENSEARCH_GEOSPATIAL_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-index-management:${OPENSEARCH_INDEX_MANAGEMENT_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-knn:${OPENSEARCH_KNN_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-ml-plugin:${OPENSEARCH_ML_PLUGIN_VERSION}" \
    "org.opensearch.plugin:neural-search:${OPENSEARCH_NEURAL_SEARCH_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-notifications-core:${OPENSEARCH_NOTIFICATIONS_CORE_PLUGIN_VERSION}" \
    "org.opensearch.plugin:notifications:${OPENSEARCH_NOTIFICATIONS_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-observability:${OPENSEARCH_OBSERVABILITY_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-reports-scheduler:${OPENSEARCH_REPORTS_SCHEDULER_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-search-relevance:${OPENSEARCH_SEARCH_RELEVANCE_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-security-analytics:${OPENSEARCH_SECURITY_ANALYTICS_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-skills:${OPENSEARCH_SKILLS_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-sql-plugin:${OPENSEARCH_SQL_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-system-templates:${OPENSEARCH_SYSTEM_TEMPLATES_PLUGIN_VERSION}" \
    "org.opensearch.plugin:opensearch-ubi:${OPENSEARCH_UBI_PLUGIN_VERSION}" \
    "org.opensearch.plugin:query-insights:${QUERY_INSIGHTS_PLUGIN_VERSION}"

# Change the group permissions already in the builder image to reduce
# the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod +x /stackable/opensearch-${PRODUCT_VERSION}/opensearch-docker-entrypoint.sh
chmod -R g=u /stackable

# The OpenSearch Security Plugin expects specific permissions in the config directory, otherwise
# warnings are shown at startup.
# see https://github.com/opensearch-project/security/blob/3.1.0.0/src/main/java/org/opensearch/security/OpenSearchSecurityPlugin.java#L547-L590
find /stackable/opensearch-${PRODUCT_VERSION}/config -type d -exec chmod 770 {} \;
find /stackable/opensearch-${PRODUCT_VERSION}/config -type f -exec chmod 660 {} \;

EOF

# The upstream image is built with a bundled JDK and not just a JRE, so we also ship OpenSearch with a JDK.
FROM local-image/jdk-base AS final

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG OPENSEARCH_SECURITY_PLUGIN_VERSION
ARG OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION
ARG STACKABLE_USER_UID

ARG NAME="OpenSearch"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for OpenSearch."
ARG HOME=/stackable
ARG OPENSEARCH_HOME=${HOME}/opensearch

LABEL \
  name="OpenSearch" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT_VERSION}" \
  release="${RELEASE_VERSION}" \
  summary="The Stackable image for OpenSearch." \
  description="This image is deployed by the Stackable Operator for OpenSearch."

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.version="${PRODUCT_VERSION}"
LABEL org.opencontainers.image.revision="${RELEASE_VERSION}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-${PRODUCT_VERSION} \
    /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-security \
    /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/plugins/opensearch-security
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-prometheus-exporter \
    /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/plugins/prometheus-exporter
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz \
    /stackable
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz \
    /stackable
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-prometheus-exporter \
    /stackable/opensearch-prometheus-exporter-${OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz \
    /stackable
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/src/opensearch/patchable-work/worktree/${PRODUCT_VERSION}/build/reports/cyclonedx/bom.json \
    /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/src/opensearch/security-plugin/patchable-work/worktree/${OPENSEARCH_SECURITY_PLUGIN_VERSION}/build/reports/bom.json \
    /stackable/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN_VERSION}-stackable${RELEASE_VERSION}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-prometheus-exporter \
    /stackable/src/opensearch/opensearch-prometheus-exporter/patchable-work/worktree/${OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION}/build/reports/bom.json \
    /stackable/opensearch-prometheus-exporter-${OPENSEARCH_OPENSEARCH_PROMETHEUS_EXPORTER_VERSION}-stackable${RELEASE_VERSION}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    opensearch/licenses \
    /licenses

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown ${STACKABLE_USER_UID}:0 ${HOME}
chmod g=u /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
chmod g=u /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/plugins/opensearch-security
chmod g=u /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/plugins/prometheus-exporter
chmod g=u /stackable/*-src.tar.gz
chmod g=u /stackable/*.cdx.json
ln -s /stackable/opensearch-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} ${OPENSEARCH_HOME}
chown -h ${STACKABLE_USER_UID}:0 ${OPENSEARCH_HOME}

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.

/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

USER ${STACKABLE_USER_UID}

ENV HOME=${HOME}
ENV OPENSEARCH_HOME=${OPENSEARCH_HOME}
ENV PATH="${PATH}:/${OPENSEARCH_HOME}:${OPENSEARCH_HOME}/bin"

WORKDIR ${OPENSEARCH_HOME}
CMD ["./opensearch-docker-entrypoint.sh"]
