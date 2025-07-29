# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/java-devel AS security-plugin-builder

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch/security-plugin/stackable/patches/patchable.toml /stackable/src/opensearch/security-plugin/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch/security-plugin/stackable/patches/${PRODUCT} /stackable/src/opensearch/security-plugin/stackable/patches/${PRODUCT}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch/security-plugin ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-security-${PRODUCT}-stackable${RELEASE}-src.tar.gz .
./gradlew clean assemble -Dbuild.snapshot=false
./gradlew cyclonedxBom --warning-mode=summary -Dbuild.snapshot=false
EOF

RUN <<EOF
# Change the group permissions already in the builder image to reduce the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u "${HOME}"
EOF
