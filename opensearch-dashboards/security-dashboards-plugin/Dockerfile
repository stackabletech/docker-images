# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/stackable-devel AS opensearch-securitydashboards-builder

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/security-dashboards-plugin/stackable/patches/patchable.toml /stackable/src/opensearch-dashboards/security-dashboards-plugin/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch-dashboards/security-dashboards-plugin/stackable/patches/${PRODUCT} /stackable/src/opensearch-dashboards/security-dashboards-plugin/stackable/patches/${PRODUCT}

# tar - archive source code
RUN <<EOF
microdnf update
microdnf install tar
microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch-dashboards/security-dashboards-plugin ${PRODUCT})"
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"
# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-dashboards-security-dashboards-plugin${NEW_VERSION}-src.tar.gz .
EOF
