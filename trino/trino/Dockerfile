# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true
FROM stackable/image/java-devel AS trino-builder

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 trino/trino/stackable/patches/patchable.toml /stackable/src/trino/trino/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 trino/trino/stackable/patches/${PRODUCT} /stackable/src/trino/trino/stackable/patches/${PRODUCT}

# adding a hadolint ignore for SC2215, due to https://github.com/hadolint/hadolint/issues/980
# hadolint ignore=SC2215
RUN --mount=type=cache,id=maven-${PRODUCT},target=/root/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout trino/trino ${PRODUCT})"

NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/trino-${NEW_VERSION}-src.tar.gz .

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
git tag ${NEW_VERSION}

SKIP_PROJECTS="!docs"
if [ "$PRODUCT" = "470" ] || [ "$PRODUCT" = "451" ]; then
  SKIP_PROJECTS="$SKIP_PROJECTS,!core/trino-server-rpm"
fi

# We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
./mvnw \
  --batch-mode \
  --no-transfer-progress \
  install \
  `# -Dmaven.test.skip # Unable to skip test compilation without an unused dependency error for software.amazon.awssdk:identity-spi` \
  -DskipTests `# Skip test execution` \
  -Dcheckstyle.skip `# Skip checkstyle checks. We dont care if the code is properly formatted, it just wastes time` \
  -Dmaven.javadoc.skip=true `# Dont generate javadoc` \
  -Ddep.presto-jdbc-under-test=${NEW_VERSION} \
  --projects="$SKIP_PROJECTS"

mkdir -p /stackable/patched-libs/maven/io
cp -r /root/.m2/repository/io/trino /stackable/patched-libs/maven/io

# Delete intermediate build products to free some space and keep runners happy
rm -r plugin/*/target core/trino-server/target/trino-server-${NEW_VERSION}

# Extract from tarball to save space; the tarball deduplicates jars (replacing them with symlinks),
# while the raw output folder does not
tar -xzf core/trino-server/target/trino-server-${NEW_VERSION}.tar.gz -C /stackable
mv core/trino-server/target/bom.json /stackable/trino-server-${NEW_VERSION}/trino-server-${NEW_VERSION}.cdx.json

# Delete intermediate build products to free some space and keep runners happy
(cd .. && rm -r ${PRODUCT})
rm -r /stackable/.m2

ln -s /stackable/trino-server-${NEW_VERSION} /stackable/trino-server

# Set correct permissions
chmod --recursive g=u /stackable
EOF
