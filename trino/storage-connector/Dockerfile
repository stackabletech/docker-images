# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/trino/trino AS trino-builder

FROM stackable/image/java-devel AS storage-connector-builder

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/storage-connector/stackable/patches/patchable.toml /stackable/src/trino-storage-connector/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 trino/storage-connector/stackable/patches/${PRODUCT} /stackable/src/trino-storage-connector/stackable/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=trino-builder /stackable/patched-libs /stackable/patched-libs
USER ${STACKABLE_USER_UID}

# adding a hadolint ignore for SC2215, due to https://github.com/hadolint/hadolint/issues/980
# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-${PRODUCT},target=/stackable/.m2/repository,uid=${STACKABLE_USER_UID} <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout trino-storage-connector ${PRODUCT})"

# Make Maven aware of custom Stackable libraries
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

ORIGINAL_VERSION="${PRODUCT}"
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

# Create snapshot of the source code including custom patches
tar -czf /stackable/trino-storage-connector-${NEW_VERSION}-src.tar.gz .

mvn versions:set -DnewVersion=${NEW_VERSION}

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw \
  --batch-mode \
  --no-transfer-progress \
  package \
  -DskipTests `# Skip test execution` \
  -Ddep.trino.version=${NEW_VERSION} `# Use custom Stackable Trino version in tests` \
  -Dcheckstyle.skip `# Skip checkstyle checks. We dont care if the code is properly formatted, it just wastes time` \
  -Dmaven.javadoc.skip=true `# Dont generate javadoc` \
  -Dmaven.gitcommitid.skip=true `# The gitcommitid plugin cannot work with git workspaces (ie: patchable)`
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" target/bom.json

# Set correct permissions
chmod --recursive g=u target /stackable/trino-storage-connector-${NEW_VERSION}-src.tar.gz
EOF
