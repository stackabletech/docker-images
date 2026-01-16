# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
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
