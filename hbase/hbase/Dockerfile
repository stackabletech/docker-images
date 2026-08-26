FROM local-image/hadoop/hadoop AS hadoop-builder

FROM local-image/java-devel AS hbase-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG ASYNC_PROFILER_VERSION
ARG HADOOP_HADOOP_VERSION
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP_VERSION}
ARG TARGETARCH
ARG TARGETOS
ARG STACKABLE_USER_UID
ARG CDXGEN_NODEJS_VERSION
ARG CDXGEN_SPEC_VERSION
ARG CDXGEN_VERSION

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY hbase/licenses /licenses

RUN <<EOF
# Install Node, cdxgen and Python to create the SBOM of the web UI (npm) dependencies.
# The CycloneDX Maven plugin only covers the Java dependencies.
# Python runs hbase_webapps_deps.py.
# cdxgen requires Node >= 24, so it gets its own Node installation in /opt/node-cdxgen and
# is invoked with that prepended to PATH.
# -fsSL is not needed: the shared /root/.curlrc sets location, fail, silent and show-error.
ARCH="${TARGETARCH/amd64/x64}"
mkdir -p /opt/node-cdxgen
curl "https://repo.stackable.tech/repository/packages/node/node-v${CDXGEN_NODEJS_VERSION}-linux-${ARCH}.tar.xz" | \
    tar --extract --xz --directory=/opt/node-cdxgen --strip-components=1
PATH="/opt/node-cdxgen/bin:$PATH" npm install --global "@cdxgen/cdxgen@${CDXGEN_VERSION}"

microdnf update
microdnf install python3
microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/patches/patchable.toml /stackable/src/hbase/hbase/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/patches/${PRODUCT_VERSION} /stackable/src/hbase/hbase/stackable/patches/${PRODUCT_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/hbase_webapps_deps.py /stackable/hbase_webapps_deps.py

COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-libs /stackable/patched-libs
# Cache mounts are owned by root by default
# We need to explicitly give the uid to use
# And every cache needs its own id, we can't share them between stages because we might delete the caches
# at the end of a run while other stages are still using it.
# While this might work in theory it didn't in practice (FileNotFound exceptions etc.)

# The cache id has to include the product version that we are building because otherwise
# docker encounters race conditions when building multiple versions in parallel, as all
# builder containers will share the same cache and the `rm -rf` commands will fail
# with a "directory not empty" error on the first builder to finish, as other builders
# are still working in the cache directory.
RUN --mount=type=cache,id=maven-hbase-${PRODUCT_VERSION},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository \
  --mount=type=cache,id=npm-hbase-${PRODUCT_VERSION},uid=${STACKABLE_USER_UID},target=/stackable/.npm \
  <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hbase/hbase ${PRODUCT_VERSION})"

# Make Maven aware of custom Stackable libraries
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-${NEW_VERSION}-src.tar.gz .

# The release scripts of HBase also run the build twice (three times in fact, once again to build the site which we skip here).
# I chose to replicate that exact behavior for consistency so please don't merge the two mvn runs into one unless you really know what you're doing!
# Cannot skip building tests here because the assembly plugin needs a shell script from the test directory.
mvn \
  -Dhadoop.profile=3.0 \
  -Dhadoop-three.version=${HADOOP_VERSION}-stackable${RELEASE_VERSION} \
  -DskipTests \
  clean install

mvn \
  -Dhadoop.profile=3.0 \
  -Dhadoop-three.version=${HADOOP_VERSION}-stackable${RELEASE_VERSION} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  -Prelease \
  install assembly:single

tar -xzf hbase-assembly/target/hbase-${NEW_VERSION}-bin.tar.gz -C /stackable/
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" hbase-assembly/target/bom.json
mv hbase-assembly/target/bom.json /stackable/hbase-${NEW_VERSION}/hbase-${NEW_VERSION}.cdx.json

# Create the SBOM for the web UI JavaScript dependencies. These are unpacked from webjars
# by the maven-dependency-plugin instead of being declared as project dependencies, so the
# CycloneDX Maven plugin does not cover them, see hbase_webapps_deps.py.
(
  export PATH="/opt/node-cdxgen/bin:$PATH"
  WEBAPPS_SBOM_DIR="$(mktemp --directory)"
  python3 /stackable/hbase_webapps_deps.py . "${ORIGINAL_VERSION}" "${WEBAPPS_SBOM_DIR}/package.json"
  cd "${WEBAPPS_SBOM_DIR}"
  # cdxgen reads the components from a lockfile, so one is generated for the package.json above.
  # Usually a lockfile also resolves the npm dependencies of the listed packages, while a
  # webjar only ever ships the files of the library itself. Such a dependency would therefore
  # show up in the SBOM without being shipped. In this case it is not a problem, because the libraries
  # HBase unpacks (jquery, moment and bootstrap) have no npm dependencies.
  npm install --package-lock-only --no-audit --no-fund
  cdxgen \
    --type js \
    --spec-version "${CDXGEN_SPEC_VERSION}" \
    `# cdxgen would otherwise name the root component after the scratch directory` \
    --project-name hbase-webapps \
    --output "/stackable/hbase-${NEW_VERSION}/hbase-webapps-${NEW_VERSION}.cdx.json"
)

# Remove sources
(cd .. && rm -r ${PRODUCT_VERSION})

ln -s "/stackable/hbase-${NEW_VERSION}" /stackable/hbase

mkdir -p /stackable/patched-libs/maven/org/apache
cp -r /stackable/.m2/repository/org/apache/hbase /stackable/patched-libs/maven/org/apache

###
### Async Profiler
###
cd /stackable
export ARCH="${TARGETARCH/amd64/x64}"
curl --fail "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER_VERSION}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC .
ln -s "/stackable/async-profiler-${ASYNC_PROFILER_VERSION}-${TARGETOS}-${ARCH}" /stackable/async-profiler

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
  rm -rf /stackable/.npm/*
  rm -rf /stackable/.cache/*
fi

# set correct groups
chmod --recursive g=u /stackable
EOF
