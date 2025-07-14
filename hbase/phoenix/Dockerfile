FROM stackable/image/hadoop/hadoop AS hadoop-builder

FROM stackable/image/hbase/hbase AS hbase-builder

FROM stackable/image/java-devel

ARG PRODUCT
ARG RELEASE
ARG ASYNC_PROFILER
ARG HBASE_HBASE
# Reassign the arg to `HBASE_VERSION` for better readability.
ENV HBASE_VERSION=${HBASE_HBASE}
ARG HBASE_PROFILE
ARG HADOOP_HADOOP
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP}
ARG STACKABLE_USER_UID
ARG PHOENIX_VERSION

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/patchable.toml /stackable/src/phoenix/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/${PHOENIX_VERSION} /stackable/src/phoenix/stackable/patches/${PHOENIX_VERSION}
COPY --from=hbase-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-libs/maven/org/apache/hbase /stackable/patched-libs/maven/org/apache/hbase
COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-libs/maven/org/apache/hadoop /stackable/patched-libs/maven/org/apache/hadoop

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN --mount=type=cache,id=maven-phoenix-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF

# Make Maven aware of custom Stackable libraries
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

cd "$(/stackable/patchable --images-repo-root=src checkout phoenix ${PHOENIX_VERSION})"

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PHOENIX_VERSION}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/phoenix-${PRODUCT}-stackable${RELEASE}-src.tar.gz .

# The Maven command can be found inside of the scripts in the create-release folder (release-util.sh as of Phoenix 5.2.0)
# https://github.com/apache/phoenix/tree/5.2.0/dev/create-release
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${HBASE_VERSION}-stackable${RELEASE} \
  -Dhbase.profile=${HBASE_PROFILE} \
  -Dhadoop.version=${HADOOP_VERSION}-stackable${RELEASE} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  clean \
  package

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-stackable${RELEASE}-bin
tar -xz -f phoenix-assembly/target/phoenix-hbase-*-bin.tar.gz -C /stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-stackable${RELEASE}-bin/ --strip-components=1
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" phoenix-assembly/target/bom.json
mv phoenix-assembly/target/bom.json /stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-stackable${RELEASE}-bin/phoenix-${HBASE_PROFILE}-${PRODUCT}-stackable${RELEASE}.cdx.json

# Remove sources
(cd .. && rm -r ${PHOENIX_VERSION})
ln -s "/stackable/phoenix-${HBASE_PROFILE}-${PRODUCT}-stackable${RELEASE}-bin" /stackable/phoenix

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
