# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

FROM local-image/java-devel AS opensearch-prometheus-exporter-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch/opensearch-prometheus-exporter/stackable/patches/patchable.toml /stackable/src/opensearch/opensearch-prometheus-exporter/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch/opensearch-prometheus-exporter/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch/opensearch-prometheus-exporter/stackable/patches/${PRODUCT_VERSION}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch/opensearch-prometheus-exporter ${PRODUCT_VERSION})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-prometheus-exporter-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz .
./gradlew clean assemble -Dbuild.snapshot=false
./gradlew cyclonedxBom --warning-mode=summary -Dbuild.snapshot=false
EOF

RUN <<EOF
# Change the group permissions already in the builder image to reduce the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u /stackable
EOF
