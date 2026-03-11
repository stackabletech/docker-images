# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/java-devel

ARG PRODUCT_VERSION
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 kafka/kafka-opa-plugin/stackable/patches/patchable.toml /stackable/src/kafka/kafka-opa-plugin/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 kafka/kafka-opa-plugin/stackable/patches/${PRODUCT_VERSION} /stackable/src/kafka/kafka-opa-plugin/stackable/patches/${PRODUCT_VERSION}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout kafka/kafka-opa-plugin ${PRODUCT_VERSION})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/kafka-opa-plugin-${PRODUCT_VERSION}-src.tar.gz .

./gradlew clean shadowJar
EOF
