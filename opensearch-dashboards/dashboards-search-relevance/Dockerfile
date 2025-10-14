# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/stackable-devel

ARG PRODUCT_VERSION
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/dashboards-search-relevance/stackable/patches/patchable.toml /stackable/src/opensearch-dashboards/dashboards-search-relevance/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/dashboards-search-relevance/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch-dashboards/dashboards-search-relevance/stackable/patches/${PRODUCT_VERSION}

USER ${STACKABLE_USER_UID}

RUN /stackable/patchable --images-repo-root=src checkout opensearch-dashboards/dashboards-search-relevance ${PRODUCT_VERSION}
