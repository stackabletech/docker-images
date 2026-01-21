# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

FROM local-image/java-devel AS builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

COPY --chown=${STACKABLE_USER_UID}:0 shared/reload4j/stackable/patches/patchable.toml /stackable/src/shared/reload4j/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/reload4j/stackable/patches/${PRODUCT_VERSION} /stackable/src/shared/reload4j/stackable/patches/${PRODUCT_VERSION}

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout shared/reload4j ${PRODUCT_VERSION})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/reload4j-${PRODUCT_VERSION}-src.tar.gz .

mvn --batch-mode --no-transfer-progress clean install -DskipTests -Dmaven.test.skip=true -Dmaven.javadoc.skip=true
EOF
