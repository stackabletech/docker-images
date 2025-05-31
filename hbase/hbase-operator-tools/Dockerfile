FROM stackable/image/hbase/hbase AS hbase-builder

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-devel

ARG PRODUCT
ARG RELEASE
ARG HBASE_THIRDPARTY
ARG HBASE_HBASE
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

# Resolve paths in bin/hbck2
# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/bin/hbck2.env /stackable/bin/
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-operator-tools/stackable/patches/patchable.toml /stackable/src/hbase/hbase-operator-tools/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-operator-tools/stackable/patches/${PRODUCT} /stackable/src/hbase/hbase-operator-tools/stackable/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase/stackable/bin/hbase-entrypoint.sh /stackable/bin/

COPY --from=hbase-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-hbase-libs /stackable/patched-hbase-libs
COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-hadoop-libs /stackable/patched-hadoop-libs

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use
RUN --mount=type=cache,id=maven-hbase-operator-tools-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hbase/hbase-operator-tools ${PRODUCT})"

# Copy Stackable libraries
mkdir -p /stackable/.m2/repository/org/apache/{hadoop,hbase}
cp -r /stackable/patched-hadoop-libs/* /stackable/.m2/repository/org/apache/hadoop
cp -r /stackable/patched-hbase-libs/* /stackable/.m2/repository/org/apache/hbase

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-operator-tools-${NEW_VERSION}-src.tar.gz .

mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${HBASE_HBASE}-stackable${RELEASE} \
  -Dhbase-thirdparty.version=${HBASE_THIRDPARTY} \
  -DskipTests \
  -Dmaven.test.skip=true \
  package assembly:single

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/hbase-operator-tools-${NEW_VERSION}
tar -xz \
  -f hbase-operator-tools-assembly/target/hbase-operator-tools-*-bin.tar.gz \
  -C /stackable/hbase-operator-tools-${NEW_VERSION}/ \
  --strip-components=1
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" hbase-operator-tools-assembly/target/bom.json
mv hbase-operator-tools-assembly/target/bom.json /stackable/hbase-operator-tools-${NEW_VERSION}/hbase-operator-tools-${NEW_VERSION}.cdx.json
(cd .. && rm -r ${PRODUCT})

envsubst '${HBASE_HBASE}:${PRODUCT}' < /stackable/bin/hbck2.env > /stackable/bin/hbck2
chmod +x /stackable/bin/hbck2
rm /stackable/bin/hbck2.env

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
