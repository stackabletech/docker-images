# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# We use a custom patched airlift version to fast track https://github.com/airlift/airlift/pull/1943
# This can be removed once the PR is merged and Trino makes use of the new Airlift version containing the fix

FROM local-image/java-devel AS airlift-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/airlift/stackable/patches/patchable.toml /stackable/src/trino/airlift/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 trino/airlift/stackable/patches/${PRODUCT_VERSION} /stackable/src/trino/airlift/stackable/patches/${PRODUCT_VERSION}

# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-airlift-${PRODUCT_VERSION},target=/root/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout trino/airlift ${PRODUCT_VERSION})"

NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

mvn versions:set -DnewVersion=$NEW_VERSION -DartifactId='*' -DgroupId='*' -DgenerateBackupPoms=false

mvn \
  install \
  -DskipTests \
  -Dair.check.skip-all \
  -Dcheckstyle.skip \
  -Dmaven.javadoc.skip=true \
  -Dmaven.gitcommitid.skip=true \
  --projects='!sample-server,!skeleton-server'

mkdir -p /stackable/patched-libs/maven/io
cp -r /root/.m2/repository/io/airlift /stackable/patched-libs/maven/io

chmod --recursive g=u /stackable/patched-libs
EOF
