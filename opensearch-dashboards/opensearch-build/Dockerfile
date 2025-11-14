# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
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
