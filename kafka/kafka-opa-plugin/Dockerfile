# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/java-devel

ARG PRODUCT
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} kafka/kafka-opa-plugin/stackable/patches/patchable.toml /stackable/src/kafka/kafka-opa-plugin/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:${STACKABLE_USER_GID} kafka/kafka-opa-plugin/stackable/patches/${PRODUCT} /stackable/src/kafka/kafka-opa-plugin/stackable/patches/${PRODUCT}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout kafka/kafka-opa-plugin ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/kafka-opa-plugin-${PRODUCT}-src.tar.gz .

./gradlew clean shadowJar
EOF
