FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS hbase-builder

ARG PRODUCT
ARG RELEASE
ARG ASYNC_PROFILER
ARG HADOOP
ARG TARGETARCH
ARG TARGETOS
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY hbase/licenses /licenses

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/patches/patchable.toml /stackable/src/hbase/hbase/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/patches/${PRODUCT} /stackable/src/hbase/hbase/stackable/patches/${PRODUCT}

COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-hadoop-libs /stackable/patched-hadoop-libs
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
RUN --mount=type=cache,id=maven-hbase-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hbase/hbase ${PRODUCT})"

# Copy Stackable Hadoop libraries
mkdir -p /stackable/.m2/repository/org/apache/hadoop
cp -r /stackable/patched-hadoop-libs/* /stackable/.m2/repository/org/apache/hadoop

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-${NEW_VERSION}-src.tar.gz .

# The release scripts of HBase also run the build twice (three times in fact, once again to build the site which we skip here).
# I chose to replicate that exact behavior for consistency so please don't merge the two mvn runs into one unless you really know what you're doing!
# Cannot skip building tests here because the assembly plugin needs a shell script from the test directory.
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhadoop.profile=3.0 \
  -Dhadoop-three.version=${HADOOP}-stackable${RELEASE} \
  -DskipTests \
  clean install

mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhadoop.profile=3.0 \
  -Dhadoop-three.version=${HADOOP}-stackable${RELEASE} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  -Prelease \
  install assembly:single

tar -xzf hbase-assembly/target/hbase-${NEW_VERSION}-bin.tar.gz -C /stackable/
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" hbase-assembly/target/bom.json
mv hbase-assembly/target/bom.json /stackable/hbase-${NEW_VERSION}/hbase-${NEW_VERSION}.cdx.json

# Remove sources
(cd .. && rm -r ${PRODUCT})

ln -s "/stackable/hbase-${NEW_VERSION}" /stackable/hbase

cp -r /stackable/.m2/repository/org/apache/hbase /stackable/patched-hbase-libs

###
### Async Profiler
###
cd /stackable
export ARCH="${TARGETARCH/amd64/x64}"
curl --fail "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC .
ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

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
