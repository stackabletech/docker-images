FROM stackable/image/java-devel

ARG PRODUCT
ARG HBASE_THIRDPARTY
ARG HBASE_VERSION
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

# Resolve paths in bin/hbck2
# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/bin/hbck2.env /stackable/bin/
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-operator-tools/stackable/patches/patchable.toml /stackable/src/hbase-operator-tools/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-operator-tools/stackable/patches/${PRODUCT} /stackable/src/hbase-operator-tools/stackable/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/bin/hbase-entrypoint.sh /stackable/bin/

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use
RUN --mount=type=cache,id=maven-hbase-operator-tools-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hbase-operator-tools ${PRODUCT})"

mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${HBASE_VERSION} \
  -Dhbase-thirdparty.version=${HBASE_THIRDPARTY} \
  -DskipTests \
  -Dmaven.test.skip=true \
  package assembly:single

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/hbase-operator-tools-${PRODUCT}
tar -xz \
  -f hbase-operator-tools-assembly/target/hbase-operator-tools-*-bin.tar.gz \
  -C /stackable/hbase-operator-tools-${PRODUCT}/ \
  --strip-components=1
mv hbase-operator-tools-assembly/target/bom.json /stackable/hbase-operator-tools-${PRODUCT}/hbase-operator-tools-${PRODUCT}.cdx.json
(cd .. && rm -r ${PRODUCT})

envsubst '${PRODUCT}:${PRODUCT}' < /stackable/bin/hbck2.env > /stackable/bin/hbck2
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
