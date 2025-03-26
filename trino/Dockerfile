# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/trino-storage-connector AS trino-storage-connector-image

FROM stackable/image/java-devel AS trino-builder

ARG PRODUCT
ARG STACKABLE_USER_UID
ARG JMX_EXPORTER

RUN <<EOF
microdnf update

# patch: Required for the apply-patches.sh script
microdnf install \
patch

microdnf clean all
rm -rf /var/cache/yum
EOF

WORKDIR /stackable

RUN curl "https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}-src.tar.gz" | tar -xzC .

COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/apply_patches.sh /stackable/trino-server-${PRODUCT}-src/patches/apply_patches.sh
COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/patches/${PRODUCT} /stackable/trino-server-${PRODUCT}-src/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=trino-storage-connector-image /stackable/trino-storage-${PRODUCT}-src/target/trino-storage-${PRODUCT} /trino-storage-${PRODUCT}
# do not copy patches -> .dockerignore?
COPY --chown=${STACKABLE_USER_UID}:0 trino/stackable/jmx /stackable/jmx

# adding a hadolint ignore for SC2215, due to https://github.com/hadolint/hadolint/issues/980
# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-${PRODUCT},target=/root/.m2/repository <<EOF
cd "trino-server-${PRODUCT}-src"
./patches/apply_patches.sh ${PRODUCT}

# Trino is using something (git-commit-id-plugin in the past, maybe something else now) that is
# reading the Git history and searches for a tag to pull the version from. It sounds weird to me
# why someone would do that over just picking the version from the pom.xml, but they propably
# have their reasons. See e.g. https://github.com/trinodb/trino/discussions/18963.
# So we fake it till we make it and create a Git repo and the correct tag. The trino-operator
# smoke test checks that "select version()" is working.
git init
git config user.email "fake.commiter@stackable.tech"
git config user.name "Fake commiter"
git commit --allow-empty --message "Fake commit, so that we can create a tag"
git tag ${PRODUCT}

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw --batch-mode --no-transfer-progress package -DskipTests --projects="!docs,!core/trino-server-rpm"

# Delete intermediate build products to free some space and keep runners happy
rm -r /stackable/trino-server-${PRODUCT}-src/plugin/*/target /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/trino-server-${PRODUCT}

# Extract from tarball to save space; the tarball deduplicates jars (replacing them with symlinks),
# while the raw output folder does not
tar -xzf /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/trino-server-${PRODUCT}.tar.gz -C /stackable
mv /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/bom.json /stackable/trino-server-${PRODUCT}/trino-server-${PRODUCT}.cdx.json

# Delete intermediate build products to free some space and keep runners happy
rm -r /stackable/trino-server-${PRODUCT}-src
rm -r /stackable/.m2

# JMX Exporter
curl --fail https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
# Storage connector
mv /trino-storage-${PRODUCT}/ /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${PRODUCT}/
# Softlinks
ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar
ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server
# We need to change groups here and not in the final image (file changes bloat images)
chmod -R g=u /stackable
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

# If /stackable has any build artifacts / leftovers make sure its removed properly
# or only copy what is actually required in the final image like:
# COPY --from=trino-builder --chown=${STACKABLE_USER_UID}:0 /stackable/foo /stackable/foo 
COPY --from=trino-builder --chown=${STACKABLE_USER_UID}:0 /stackable /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/licenses /licenses

# ----------------------------------------
# Attention:
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
