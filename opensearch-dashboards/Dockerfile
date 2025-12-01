# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# hadolint global ignore=DL3059
# (Multiple consecutive RUN instructions. Consider consolidation.)
# check=error=true

FROM local-image/opensearch-dashboards/alerting-dashboards-plugin AS alerting-dashboards-plugin
FROM local-image/opensearch-dashboards/anomaly-detection-dashboards-plugin AS anomaly-detection-dashboards-plugin
FROM local-image/opensearch-dashboards/dashboards-assistant AS dashboards-assistant
FROM local-image/opensearch-dashboards/dashboards-flow-framework AS dashboards-flow-framework
FROM local-image/opensearch-dashboards/dashboards-maps AS dashboards-maps
FROM local-image/opensearch-dashboards/dashboards-notifications AS dashboards-notifications
FROM local-image/opensearch-dashboards/dashboards-observability AS dashboards-observability
FROM local-image/opensearch-dashboards/dashboards-reporting AS dashboards-reporting
FROM local-image/opensearch-dashboards/dashboards-search-relevance AS dashboards-search-relevance
FROM local-image/opensearch-dashboards/index-management-dashboards-plugin AS index-management-dashboards-plugin
FROM local-image/opensearch-dashboards/ml-commons-dashboards AS ml-commons-dashboards
FROM local-image/opensearch-dashboards/query-insights-dashboards AS query-insights-dashboards
FROM local-image/opensearch-dashboards/dashboards-query-workbench AS dashboards-query-workbench
FROM local-image/opensearch-dashboards/security-analytics-dashboards-plugin AS security-analytics-dashboards-plugin
FROM local-image/opensearch-dashboards/security-dashboards-plugin AS security-dashboards-plugin

FROM local-image/opensearch-dashboards/opensearch-build AS opensearch-build
FROM local-image/stackable-devel AS opensearch-dashboards-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG OPENSEARCH_DASHBOARDS_ALERTING_DASHBOARDS_PLUGIN_VERSION
ARG OPENSEARCH_DASHBOARDS_ANOMALY_DETECTION_DASHBOARDS_PLUGIN_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_ASSISTANT_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_FLOW_FRAMEWORK_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_MAPS_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_NOTIFICATIONS_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_OBSERVABILITY_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_QUERY_WORKBENCH_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_REPORTING_VERSION
ARG OPENSEARCH_DASHBOARDS_DASHBOARDS_SEARCH_RELEVANCE_VERSION
ARG OPENSEARCH_DASHBOARDS_INDEX_MANAGEMENT_DASHBOARDS_PLUGIN_VERSION
ARG OPENSEARCH_DASHBOARDS_ML_COMMONS_DASHBOARDS_VERSION
ARG OPENSEARCH_DASHBOARDS_QUERY_INSIGHTS_DASHBOARDS_VERSION
ARG OPENSEARCH_DASHBOARDS_SECURITY_ANALYTICS_DASHBOARDS_PLUGIN_VERSION
ARG OPENSEARCH_DASHBOARDS_SECURITY_DASHBOARDS_PLUGIN_VERSION
ARG NODEJS_VERSION
ARG YARN_VERSION
ARG CDXGEN_VERSION
ARG STACKABLE_USER_UID
ARG TARGETARCH

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/stackable/patches/patchable.toml /stackable/src/opensearch-dashboards/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch-dashboards/stackable/patches/${PRODUCT_VERSION}
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=alerting-dashboards-plugin \
    /stackable/src/opensearch-dashboards/alerting-dashboards-plugin/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_ALERTING_DASHBOARDS_PLUGIN_VERSION} \
    /stackable/plugins/alerting-dashboards-plugin
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=anomaly-detection-dashboards-plugin \
    /stackable/src/opensearch-dashboards/anomaly-detection-dashboards-plugin/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_ANOMALY_DETECTION_DASHBOARDS_PLUGIN_VERSION} \
    /stackable/plugins/anomaly-detection-dashboards-plugin
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-assistant \
    /stackable/src/opensearch-dashboards/dashboards-assistant/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_ASSISTANT_VERSION} \
    /stackable/plugins/dashboards-assistant
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-flow-framework \
    /stackable/src/opensearch-dashboards/dashboards-flow-framework/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_FLOW_FRAMEWORK_VERSION} \
    /stackable/plugins/dashboards-flow-framework
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-maps \
    /stackable/src/opensearch-dashboards/dashboards-maps/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_MAPS_VERSION} \
    /stackable/plugins/dashboards-maps
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-notifications \
    /stackable/src/opensearch-dashboards/dashboards-notifications/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_NOTIFICATIONS_VERSION} \
    /stackable/plugins/dashboards-notifications
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-observability \
    /stackable/src/opensearch-dashboards/dashboards-observability/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_OBSERVABILITY_VERSION} \
    /stackable/plugins/dashboards-observability
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-reporting \
    /stackable/src/opensearch-dashboards/dashboards-reporting/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_REPORTING_VERSION} \
    /stackable/plugins/dashboards-reporting
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-search-relevance \
    /stackable/src/opensearch-dashboards/dashboards-search-relevance/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_SEARCH_RELEVANCE_VERSION} \
    /stackable/plugins/dashboards-search-relevance
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=index-management-dashboards-plugin \
    /stackable/src/opensearch-dashboards/index-management-dashboards-plugin/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_INDEX_MANAGEMENT_DASHBOARDS_PLUGIN_VERSION} \
    /stackable/plugins/index-management-dashboards-plugin
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=ml-commons-dashboards \
    /stackable/src/opensearch-dashboards/ml-commons-dashboards/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_ML_COMMONS_DASHBOARDS_VERSION} \
    /stackable/plugins/ml-commons-dashboards
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=query-insights-dashboards \
    /stackable/src/opensearch-dashboards/query-insights-dashboards/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_QUERY_INSIGHTS_DASHBOARDS_VERSION} \
    /stackable/plugins/query-insights-dashboards
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=dashboards-query-workbench \
    /stackable/src/opensearch-dashboards/dashboards-query-workbench/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_DASHBOARDS_QUERY_WORKBENCH_VERSION} \
    /stackable/plugins/dashboards-query-workbench
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=security-analytics-dashboards-plugin \
    /stackable/src/opensearch-dashboards/security-analytics-dashboards-plugin/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_SECURITY_ANALYTICS_DASHBOARDS_PLUGIN_VERSION} \
    /stackable/plugins/security-analytics-dashboards-plugin
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=security-dashboards-plugin \
    /stackable/src/opensearch-dashboards/security-dashboards-plugin/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_SECURITY_DASHBOARDS_PLUGIN_VERSION} \
    /stackable/plugins/security-dashboards-plugin

# tar - extract OpenSearch Dashboards from archive & create source code archive
# unzip - extract plugin from archive
# yarn - build Opensearch Dashboards
# cdxgen - generate CycloneDX SBOM
RUN <<EOF
ARCH="${TARGETARCH/amd64/x64}"
microdnf update
microdnf install tar unzip
microdnf clean all
rm -rf /var/cache/yum
curl "https://repo.stackable.tech/repository/packages/node/node-v${NODEJS_VERSION}-linux-${ARCH}.tar.gz" | tar -xzC /usr/local --strip-components=1
npm install -g yarn@${YARN_VERSION} @cyclonedx/cdxgen@${CDXGEN_VERSION}
EOF

USER ${STACKABLE_USER_UID}

RUN /stackable/patchable --images-repo-root=src checkout opensearch-dashboards ${PRODUCT_VERSION}
WORKDIR /stackable/src/opensearch-dashboards/patchable-work/worktree/${PRODUCT_VERSION}

RUN <<EOF
ARCH="${TARGETARCH/amd64/x64}"
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"
# Create snapshot of the source code including custom patches
# Exclude cypress directory to prevent bloat in the final image
tar -cz --exclude ./cypress -f /stackable/opensearch-dashboards-${NEW_VERSION}-src.tar.gz .
# Bootstrap for the main OpenSearch Dashboards project
yarn osd bootstrap
# Running yarn build-platform without a platform flag will build the local architecture
yarn build-platform --release
mkdir /stackable/opensearch-dashboards
tar -xzf "target/opensearch-dashboards-${PRODUCT_VERSION}-linux-${ARCH}.tar.gz" -C /stackable/opensearch-dashboards --strip-components=1
EOF

RUN <<EOF
cd /stackable/src/opensearch-dashboards/patchable-work/worktree/${PRODUCT_VERSION}
cp -r /stackable/plugins .
# Remove dependencies from previous bootstrapping
yarn osd clean
# Bootstrap for the platform plugins.
# --single-version=loose allows diverging versions for dependencies between different plugins
yarn osd bootstrap --single-version=loose
EOF

RUN <<EOF
cd /stackable/src/opensearch-dashboards/patchable-work/worktree/${PRODUCT_VERSION}
# Build all plugins
cd plugins
for plugin in $(ls); do
    (
        cd "$plugin"
        yarn build
        find ~+/build -name "*.zip" -exec \
            /stackable/opensearch-dashboards/bin/opensearch-dashboards-plugin install "file://{}" \;
    )
done
EOF

RUN cdxgen --target yarn --json-pretty

RUN <<EOF
# Change the group permissions already in the builder image to reduce
# the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u /stackable
EOF

FROM local-image/vector AS final

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

ARG NAME="OpenSearch Dashboards"
ARG DESCRIPTION="This image is currently not deployed by a Stackable Operator."
ARG HOME=/stackable
ARG OPENSEARCH_DASHBOARDS_HOME=${HOME}/opensearch-dashboards

LABEL \
  name="${NAME}" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT_VERSION}" \
  release="${RELEASE_VERSION}" \
  summary="The Stackable image for OpenSearch Dashboards." \
  description="${DESCRIPTION}"

# # https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.version="${PRODUCT_VERSION}"
LABEL org.opencontainers.image.revision="${RELEASE_VERSION}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"
# https://docs.openshift.com/container-platform/4.16/openshift_images/create-images.html#defining-image-metadata
# https://github.com/projectatomic/ContainerApplicationGenericLabels/blob/master/vendor/redhat/labels.md
LABEL io.openshift.tags="ubi10,stackable,opensearch-dashboards,opensearch"
LABEL io.k8s.description="${DESCRIPTION}"
LABEL io.k8s.display-name="${NAME}"

COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-dashboards-builder \
    /stackable/opensearch-dashboards \
    /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-dashboards-builder \
    /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz \
    /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-build \
    /stackable/opensearch-dashboards-docker-entrypoint.sh \
    /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/opensearch-dashboards-docker-entrypoint.sh
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-build \
    /stackable/opensearch_dashboards.yml \
    /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/config/opensearch_dashboards.yml
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-dashboards-builder \
    /stackable/src/opensearch-dashboards/patchable-work/worktree/${PRODUCT_VERSION}/bom.json \
    /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-dashboards-builder \
    /usr/local/bin/node \
    /usr/local/bin/node
COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum
EOF

RUN <<EOF
# # All files and folders owned by root group to support running as arbitrary users.
# # This is best practice as all container users will belong to the root group (0).
chown ${STACKABLE_USER_UID}:0 ${HOME}
chmod g=u /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
chmod g=u /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/opensearch-dashboards-docker-entrypoint.sh
chmod g=u /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/config/opensearch_dashboards.yml
ln -s /stackable/opensearch-dashboards-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} ${OPENSEARCH_DASHBOARDS_HOME}
chown -h ${STACKABLE_USER_UID}:0 ${OPENSEARCH_DASHBOARDS_HOME}

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
ENV OPENSEARCH_DASHBOARDS_HOME=${OPENSEARCH_DASHBOARDS_HOME}
ENV PATH="${PATH}:/${OPENSEARCH_DASHBOARDS_HOME}:${OPENSEARCH_DASHBOARDS_HOME}/bin"

WORKDIR ${OPENSEARCH_DASHBOARDS_HOME}
CMD ["opensearch-dashboards-docker-entrypoint.sh"]
