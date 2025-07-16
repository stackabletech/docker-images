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
tar -czf /stackable/opensearch-security-plugin-${PRODUCT}-src.tar.gz .
./gradlew clean assemble
EOF

RUN <<EOF
# Change the group permissions already in the builder image to reduce the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u "${HOME}"
EOF
