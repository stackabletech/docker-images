FROM stackable/image/java-devel

ARG PRODUCT
ARG ASYNC_PROFILER
ARG HBASE_VERSION
ARG HBASE_PROFILE
ARG HADOOP_VERSION
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/patchable.toml /stackable/src/phoenix/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/${PRODUCT} /stackable/src/phoenix/stackable/patches/${PRODUCT}
USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN --mount=type=cache,id=maven-phoenix-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout phoenix ${PRODUCT})"

# The Maven command can be found inside of the scripts in the create-release folder (release-util.sh as of Phoenix 5.2.0)
# https://github.com/apache/phoenix/tree/5.2.0/dev/create-release
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${HBASE_VERSION} \
  -Dhbase.profile=${HBASE_PROFILE} \
  -Dhadoop.version=${HADOOP_VERSION} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  clean \
  package

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-bin
tar -xz -f phoenix-assembly/target/phoenix-hbase-*-bin.tar.gz -C /stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-bin/ --strip-components=1
mv phoenix-assembly/target/bom.json /stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-bin/phoenix-${HBASE_PROFILE}-${PRODUCT}.cdx.json

# Remove sources
(cd .. && rm -r ${PRODUCT})
ln -s "/stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-bin" /stackable/phoenix

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
