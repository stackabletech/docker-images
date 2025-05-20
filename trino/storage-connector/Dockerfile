# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/java-devel AS storage-connector-builder

ARG PRODUCT
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/storage-connector/stackable/patches/patchable.toml /stackable/src/trino-storage-connector/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 trino/storage-connector/stackable/patches/${PRODUCT} /stackable/src/trino-storage-connector/stackable/patches/${PRODUCT}

# adding a hadolint ignore for SC2215, due to https://github.com/hadolint/hadolint/issues/980
# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-${PRODUCT},target=/root/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout trino-storage-connector ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/trino-storage-connector-${PRODUCT}-src.tar.gz .

# Upstream builds are marked as -SNAPSHOT, even for release builds
mvn versions:set -DnewVersion=${PRODUCT}

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw --batch-mode --no-transfer-progress package -DskipTests -Dmaven.gitcommitid.skip=true
EOF
