# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/opensearch-dashboards/security-dashboards-plugin AS security-dashboards-plugin
FROM local-image/opensearch-dashboards/opensearch-build AS opensearch-build
FROM local-image/stackable-devel AS opensearch-dashboards-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG OPENSEARCH_DASHBOARDS_SECURITY_DASHBOARDS_PLUGIN_VERSION
ARG NODEJS_VERSION
ARG YARN_VERSION
ARG STACKABLE_USER_UID
ARG TARGETARCH

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/stackable/patches/patchable.toml /stackable/src/opensearch-dashboards/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch-dashboards/stackable/patches/${PRODUCT_VERSION}
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=security-dashboards-plugin \
    /stackable/src/opensearch-dashboards/security-dashboards-plugin/patchable-work/worktree/${OPENSEARCH_DASHBOARDS_SECURITY_DASHBOARDS_PLUGIN_VERSION} \
    /stackable/security-dashboards-plugin

# tar, xz - extract nodejs bundle
# unzip - extract plugin from archive
RUN <<EOF
ARCH="${TARGETARCH/amd64/x64}"
microdnf update
microdnf install tar xz unzip
microdnf clean all
rm -rf /var/cache/yum
curl "https://nodejs.org/dist/v${NODEJS_VERSION}/node-v${NODEJS_VERSION}-linux-${ARCH}.tar.xz" | tar -xJC /usr/local --strip-components=1
echo "installing yarn"
npm install -g yarn@${YARN_VERSION}
echo "installing cdxgen"
npm update
npm show es-define-property versions
npm install -g @cyclonedx/cdxgen@11.7.0
EOF

USER ${STACKABLE_USER_UID}

RUN <<EOF
ARCH="${TARGETARCH/amd64/x64}"
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch-dashboards ${PRODUCT_VERSION})"
cp -r /stackable/security-dashboards-plugin plugins/security-dashboards-plugin
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"
# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-dashboards-${NEW_VERSION}-src.tar.gz .
HUSKY=0 yarn osd bootstrap --single-version=loose
yarn build-platform --release
mkdir /stackable/opensearch-dashboards
tar -xzf target/opensearch-dashboards-${PRODUCT_VERSION}-linux-${ARCH}.tar.gz -C /stackable/opensearch-dashboards --strip-components=1
cdxgen -t yarn --json-pretty
cd plugins/security-dashboards-plugin
yarn build
cdxgen -t yarn --json-pretty
unzip build/security-dashboards-${OPENSEARCH_DASHBOARDS_SECURITY_DASHBOARDS_PLUGIN_VERSION}.zip -d /stackable/security-dashboards-plugin
mv /stackable/security-dashboards-plugin/opensearch-dashboards/securityDashboards /stackable/opensearch-dashboards/plugins/securityDashboards
EOF


RUN <<EOF
# Change the group permissions already in the builder image to reduce
# the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u /stackable
EOF

FROM local-image/vector AS final

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG NODEJS
ARG STACKABLE_USER_UID

ARG NAME="OpenSearch Dashboards"
ARG DESCRIPTION="This image is currently not deployed by a Stackable Operator."
ARG HOME=/stackable
ARG OPENSEARCH_DASHBOARDS_HOME=${HOME}/opensearch-dashboards

LABEL \
  name="OpenSearch Dashboards" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT_VERSION}" \
  release="${RELEASE_VERSION}" \
  summary="The Stackable image for OpenSearch Dashboards." \
  description="This image is currently not deployed by a Stackable Operator."

# # https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.version="${PRODUCT_VERSION}"
LABEL org.opencontainers.image.revision="${RELEASE_VERSION}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

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
    /stackable/src/opensearch-dashboards/patchable-work/worktree/${PRODUCT_VERSION}/plugins/security-dashboards-plugin/bom.json \
    /stackable/opensearch-dashboards-security-dashboards-plugin-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-dashboards-builder \
    /usr/local/bin/node \
    /usr/local/bin/node

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
chmod g=u /stackable/*-src.tar.gz
chmod g=u /stackable/*-src.cdx.json
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
