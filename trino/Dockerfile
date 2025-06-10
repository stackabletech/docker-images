# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/trino/storage-connector AS trino-storage-connector-image

FROM stackable/image/java-devel AS trino-builder

ARG PRODUCT
ARG STACKABLE_USER_UID
ARG JMX_EXPORTER

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/patchable.toml /stackable/src/trino/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/${PRODUCT} /stackable/src/trino/stackable/patches/${PRODUCT}

COPY --from=trino-storage-connector-image /stackable/src/trino-storage-connector/patchable-work/worktree/${PRODUCT}/target/trino-storage-${PRODUCT} /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${PRODUCT}
COPY --from=trino-storage-connector-image /stackable/src/trino-storage-connector/patchable-work/worktree/${PRODUCT}/target/bom.json /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${PRODUCT}/trino-storage-${PRODUCT}.cdx.json
COPY --from=trino-storage-connector-image /stackable/trino-storage-connector-${PRODUCT}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/jmx /stackable/jmx

# adding a hadolint ignore for SC2215, due to https://github.com/hadolint/hadolint/issues/980
# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-${PRODUCT},target=/root/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout trino ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/trino-${PRODUCT}-src.tar.gz .

SKIP_PROJECTS="!docs"
if [ "$PRODUCT" = "470" ] || [ "$PRODUCT" = "451" ]; then
  SKIP_PROJECTS="$SKIP_PROJECTS,!core/trino-server-rpm"
fi

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw \
  --batch-mode \
  --no-transfer-progress \
  package \
  `# -Dmaven.test.skip # Unable to skip test compilation without an unused dependency error for software.amazon.awssdk:identity-spi` \
  -DskipTests `# Skip test execution` \
  -Dcheckstyle.skip `# Skip checkstyle checks. We dont care if the code is properly formatted, it just wastes time` \
  -Dmaven.javadoc.skip=true `# Dont generate javadoc` \
  -Dmaven.gitcommitid.skip=true `# The gitcommitid plugin cannot work with git workspaces (ie: patchable)` \
  --projects="$SKIP_PROJECTS"

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
