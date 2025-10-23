# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM local-image/java-devel AS builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

COPY --chown=${STACKABLE_USER_UID}:0 shared/logback/stackable/patches/patchable.toml /stackable/src/shared/logback/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/logback/stackable/patches/${PRODUCT_VERSION} /stackable/src/shared/logback/stackable/patches/${PRODUCT_VERSION}

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout shared/logback ${PRODUCT_VERSION})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/logback-${PRODUCT_VERSION}-src.tar.gz .

mvn --batch-mode --no-transfer-progress clean install -DskipTests -Dmaven.test.skip=true -Dmaven.javadoc.skip=true
EOF
