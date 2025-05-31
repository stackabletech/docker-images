FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/hbase/hbase AS hbase-builder

FROM stackable/image/java-devel

ARG PRODUCT
ARG RELEASE
ARG ASYNC_PROFILER
ARG HBASE_HBASE
ARG HBASE_PROFILE
ARG HADOOP
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/patchable.toml /stackable/src/phoenix/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/${PRODUCT} /stackable/src/phoenix/stackable/patches/${PRODUCT}
COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-hadoop-libs /stackable/patched-hadoop-libs
COPY --from=hbase-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-hbase-libs /stackable/patched-hbase-libs

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN --mount=type=cache,id=maven-phoenix-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF

mkdir -p /stackable/.m2/repository/org/apache/{hadoop,hbase}
cp -r /stackable/patched-hadoop-libs/* /stackable/.m2/repository/org/apache/hadoop
cp -r /stackable/patched-hbase-libs/* /stackable/.m2/repository/org/apache/hbase

cd "$(/stackable/patchable --images-repo-root=src checkout phoenix ${PRODUCT})"

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/phoenix-${NEW_VERSION}-src.tar.gz .

# The Maven command can be found inside of the scripts in the create-release folder (release-util.sh as of Phoenix 5.2.0)
# https://github.com/apache/phoenix/tree/5.2.0/dev/create-release
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${HBASE_HBASE}-stackable${RELEASE} \
  -Dhbase.profile=${HBASE_PROFILE} \
  -Dhadoop.version=${HADOOP}-stackable${RELEASE} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  clean \
  package

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/phoenix-${HBASE_PROFILE}-${NEW_VERSION}-bin
tar -xz -f phoenix-assembly/target/phoenix-hbase-*-bin.tar.gz -C /stackable/phoenix-${HBASE_PROFILE}-${NEW_VERSION}-bin/ --strip-components=1
mv phoenix-assembly/target/bom.json /stackable/phoenix-${HBASE_PROFILE}-${NEW_VERSION}-bin/phoenix-${HBASE_PROFILE}-${NEW_VERSION}.cdx.json

# Remove sources
(cd .. && rm -r ${PRODUCT})
ln -s "/stackable/phoenix-${HBASE_PROFILE}-${NEW_VERSION}-bin" /stackable/phoenix

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
