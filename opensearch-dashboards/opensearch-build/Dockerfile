# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/stackable-devel AS opensearch-build

ARG PRODUCT_VERSION
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/opensearch-build/stackable/patches/patchable.toml /stackable/src/opensearch-dashboards/opensearch-build/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/opensearch-build/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch-dashboards/opensearch-build/stackable/patches/${PRODUCT_VERSION}

USER ${STACKABLE_USER_UID}

RUN <<EOF
    cd "$(/stackable/patchable --images-repo-root=src checkout opensearch-dashboards/opensearch-build ${PRODUCT_VERSION})"
    cp docker/release/config/opensearch-dashboards/opensearch-dashboards-docker-entrypoint-3.x.sh /stackable/opensearch-dashboards-docker-entrypoint.sh
    cp config/opensearch_dashboards-3.x.yml /stackable/opensearch_dashboards.yml
EOF
