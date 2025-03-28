# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/trino-storage-connector AS trino-storage-connector-image

FROM stackable/image/java-devel AS trino-builder

ARG PRODUCT
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/${PRODUCT} /stackable/src/trino/stackable/patches/${PRODUCT}

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
rm -rf .git
git init
git config user.email "fake.commiter@stackable.tech"
git config user.name "Fake commiter"
git commit --allow-empty --message "Fake commit, so that we can create a tag"
git tag ${PRODUCT}

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw --batch-mode --no-transfer-progress package -DskipTests --projects="!docs,!core/trino-server-rpm"

# Delete the worst intermediate build products to free some space
rm -r plugin/*/target core/trino-server/target/trino-server-${PRODUCT}

# Extract from tarball to save space; the tarball deduplicates jars (replacing them with symlinks),
# while the raw output folder does not
tar -xzf core/trino-server/target/trino-server-${PRODUCT}.tar.gz -C /stackable
mv core/trino-server/target/bom.json /stackable/trino-server-${PRODUCT}/trino-server-${PRODUCT}.cdx.json
chown --recursive ${STACKABLE_USER_UID}:0 /stackable/trino-server-${PRODUCT}

# Delete all intermediate build products to free some more space
cd ..
rm -r ./${PRODUCT}
EOF

COPY --from=trino-storage-connector-image /stackable/src/trino-storage-connector/patchable-work/worktree/${PRODUCT}/target/trino-storage-${PRODUCT} /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${PRODUCT}
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/trino-server-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/trino-server-${PRODUCT}
# ===

FROM stackable/image/java-base AS final

ARG PRODUCT
ARG JMX_EXPORTER
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Trino" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Trino." \
      description="This image is deployed by the Stackable Operator for Trino."

RUN microdnf update && \
    microdnf install \
    gzip \
    httpd-tools \
    python \
    tar \
    # Required by snappy and duckdb, see https://github.com/trinodb/trino/pull/25143
    libstdc++ \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable /stackable
COPY --chown=${STACKABLE_USER_UID}:0 trino/licenses /licenses

COPY --from=trino-builder /stackable/trino-server-${PRODUCT} /stackable/trino-server-${PRODUCT}

RUN <<EOF
ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server

curl --fail https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

# ----------------------------------------
# Attention: We are changing the group of all files in /stackable directly above
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
