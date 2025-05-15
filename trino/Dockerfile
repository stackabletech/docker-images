# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/java-devel AS trino-builder

ARG PRODUCT
ARG STACKABLE_USER_UID
ARG JMX_EXPORTER

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/patchable.toml /stackable/src/trino/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/${PRODUCT} /stackable/src/trino/stackable/patches/${PRODUCT}

COPY --from=stackable/image/trino/storage-connector /stackable/src/trino-storage-connector/patchable-work/worktree/${PRODUCT}/target/trino-storage-${PRODUCT} /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/jmx /stackable/jmx

# adding a hadolint ignore for SC2215, due to https://github.com/hadolint/hadolint/issues/980
# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-${PRODUCT},target=/root/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout trino ${PRODUCT})"

# Trino is using something (git-commit-id-plugin in the past, maybe something else now) that is
# reading the Git history and searches for a tag to pull the version from. It sounds weird to me
# why someone would do that over just picking the version from the pom.xml, but they propably
# have their reasons. See e.g. https://github.com/trinodb/trino/discussions/18963.
# So we fake it till we make it and create a Git repo and the correct tag. The trino-operator
# smoke test checks that "select version()" is working.
# Also, we need to initialize a new Git repo because `git-commit-id-maven-plugin` has a problem with worktrees, see https://github.com/git-commit-id/git-commit-id-maven-plugin/issues/215
rm .git
git init
git config user.email "fake.commiter@stackable.tech"
git config user.name "Fake commiter"
git commit --allow-empty --message "Fake commit, so that we can create a tag"
git tag ${PRODUCT}

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw --batch-mode --no-transfer-progress package -DskipTests --projects="!docs,!core/trino-server-rpm"

# Delete intermediate build products to free some space and keep runners happy
rm -r plugin/*/target core/trino-server/target/trino-server-${PRODUCT}

# Extract from tarball to save space; the tarball deduplicates jars (replacing them with symlinks),
# while the raw output folder does not
tar -xzf core/trino-server/target/trino-server-${PRODUCT}.tar.gz -C /stackable
mv core/trino-server/target/bom.json /stackable/trino-server-${PRODUCT}/trino-server-${PRODUCT}.cdx.json

# Delete intermediate build products to free some space and keep runners happy
(cd .. && rm -r ${PRODUCT})
rm -r /stackable/.m2

# JMX Exporter
curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar

# Softlinks
ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar
ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server

# Set correct permissions
chmod --recursive g=u /stackable
EOF

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL \
    name="Trino" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Trino." \
    description="This image is deployed by the Stackable Operator for Trino."

RUN <<EOF
microdnf update
# libstdc++ required by snappy and duckdb, see https://github.com/trinodb/trino/pull/25143
microdnf install \
    gzip \
    httpd-tools \
    python \
    tar \
    libstdc++ \
    zip
microdnf clean all
rm -rf /var/cache/yum
EOF

COPY --from=trino-builder --chown=${STACKABLE_USER_UID}:0 /stackable /stackable
COPY --chown=${STACKABLE_USER_UID}:0 trino/licenses /licenses

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
