# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS hbase-builder

ARG PRODUCT
ARG HBASE_OPERATOR_TOOLS
ARG ASYNC_PROFILER
ARG PHOENIX
ARG HBASE_PROFILE
ARG JMX_EXPORTER
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

COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/patches/${PRODUCT} /stackable/src/hbase/stackable/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/jmx/config${JMX_EXPORTER} /stackable/jmx

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
###
### HBase
###
cd "$(/stackable/patchable --images-repo-root=src checkout hbase ${PRODUCT})"

# The release scripts of HBase also run the build twice (three times in fact, once again to build the site which we skip here).
# I chose to replicate that exact behavior for consistency so please don't merge the two mvn runs into one unless you really know what you're doing!
mvn --batch-mode --no-transfer-progress -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP} clean install -DskipTests
mvn --batch-mode --no-transfer-progress -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP} install assembly:single -DskipTests -Dcheckstyle.skip=true -Prelease
tar -xzf hbase-assembly/target/hbase-${PRODUCT}-bin.tar.gz -C /stackable/
mv hbase-assembly/target/bom.json /stackable/hbase-${PRODUCT}/hbase-${PRODUCT}.cdx.json

# Remove sources
(cd .. && rm -r ${PRODUCT})

ln -s "/stackable/hbase-${PRODUCT}" /stackable/hbase

###
### JMX Prometheus Exporter/Agent
###
if [[ -n "${JMX_EXPORTER}" ]] ; then
  curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
  chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
  ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar
fi

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


FROM stackable/image/java-devel AS opa-authorizer-builder

ARG OPA_AUTHORIZER
ARG DELETE_CACHES
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN --mount=type=cache,id=maven-opa,uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
###
### OPA Authorizer (only for 2.6 upwards)
###
if [[ -n "$OPA_AUTHORIZER" ]]; then
    git clone --depth 1 --branch "$OPA_AUTHORIZER" https://github.com/stackabletech/hbase-opa-authorizer.git
    mvn \
      --batch-mode \
      --no-transfer-progress \
      package \
      -DskipTests \
      -fhbase-opa-authorizer
else
  # Create a dummy jar to avoid errors when copying it the final image
  mkdir -p hbase-opa-authorizer/target
  touch hbase-opa-authorizer/target/hbase-opa-authorizer.jar
fi

if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
fi

# set correct groups
chmod --recursive g=u /stackable
EOF


FROM stackable/image/java-devel AS hbase-operator-tools-builder

ARG HBASE_OPERATOR_TOOLS
ARG HBASE_THIRDPARTY
ARG PRODUCT
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

# Resolve paths in bin/hbck2
# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/bin/hbck2.env /stackable/bin/
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-operator-tools/stackable/patches/${HBASE_OPERATOR_TOOLS} /stackable/src/hbase-operator-tools/stackable/patches/${HBASE_OPERATOR_TOOLS}
COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/bin/hbase-entrypoint.sh /stackable/bin/

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use
RUN --mount=type=cache,id=maven-hbase-operator-tools-${HBASE_OPERATOR_TOOLS},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hbase-operator-tools ${HBASE_OPERATOR_TOOLS})"

# Remove the Git repo because `git-commit-id-maven-plugin` tries to get the latest commit if a Git repo is present,
# which fails due to a problem with worktrees, see https://github.com/git-commit-id/git-commit-id-maven-plugin/issues/215
rm .git

mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${PRODUCT} \
  -Dhbase-thirdparty.version=${HBASE_THIRDPARTY} \
  -DskipTests \
  package assembly:single

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}
tar -xz \
  -f hbase-operator-tools-assembly/target/hbase-operator-tools-*-bin.tar.gz \
  -C /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/ \
  --strip-components=1
mv hbase-operator-tools-assembly/target/bom.json /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}.cdx.json
(cd .. && rm -r ${HBASE_OPERATOR_TOOLS})

envsubst '${PRODUCT}:${HBASE_OPERATOR_TOOLS}' < /stackable/bin/hbck2.env > /stackable/bin/hbck2
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


# Splitting this out into its own builder so that Hadoop & HBase can be built in parallel
# envsubst is only available in java-devel which is why we don't just do this in the final image
FROM stackable/image/java-devel AS hadoop-s3-builder

ARG PRODUCT
ARG HADOOP
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 \
  # The artifact name of the AWS bundle has changed between Haddop 3.3.6 and 3.4.1
  # from aws-java-sdk-bundle-*.jar to bundle-*.jar.
  # See: https://hadoop.apache.org/docs/stable/hadoop-aws/tools/hadoop-aws/aws_sdk_upgrade.html
  /stackable/hadoop/share/hadoop/tools/lib/bundle-*.jar \
  /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar \
  /stackable/hadoop/share/hadoop/tools/lib/

COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/bin/export-snapshot-to-s3.env /stackable/bin/

RUN <<EOF
# Resolve paths in bin/export-snapshot-to-s3
export LIBS=$(find /stackable/hadoop/share/hadoop -name '*.jar' -printf '%p:' | sed 's/:$//')

# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
envsubst '${PRODUCT}:${LIBS}' < /stackable/bin/export-snapshot-to-s3.env > /stackable/bin/export-snapshot-to-s3
chmod +x /stackable/bin/export-snapshot-to-s3
rm /stackable/bin/export-snapshot-to-s3.env

# set correct groups
chmod --recursive g=u /stackable
EOF


FROM stackable/image/java-devel AS phoenix-builder

ARG PRODUCT
ARG ASYNC_PROFILER
ARG PHOENIX
ARG HBASE_PROFILE
ARG HADOOP
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY --chown=${STACKABLE_USER_UID}:0 hbase/phoenix/stackable/patches/${PHOENIX} /stackable/src/phoenix/stackable/patches/${PHOENIX}
USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN --mount=type=cache,id=maven-phoenix-${PHOENIX},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout phoenix ${PHOENIX})"

# The Maven command can be found inside of the scripts in the create-release folder (release-util.sh as of Phoenix 5.2.0)
# https://github.com/apache/phoenix/tree/5.2.0/dev/create-release
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${PRODUCT} \
  -Dhbase.profile=${HBASE_PROFILE} \
  -Dhadoop.version=${HADOOP} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  clean \
  package

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin
tar -xz -f phoenix-assembly/target/phoenix-hbase-*-bin.tar.gz -C /stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin/ --strip-components=1
mv phoenix-assembly/target/bom.json /stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin/phoenix-${HBASE_PROFILE}-${PHOENIX}.cdx.json

# Remove sources
(cd .. && rm -r ${PHOENIX})
ln -s "/stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin" /stackable/phoenix

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


# Final Image
FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG HADOOP
ARG PHOENIX
ARG HBASE_PROFILE
ARG HBASE_OPERATOR_TOOLS
ARG STACKABLE_USER_UID

ARG NAME="Apache HBase"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for Apache HBase"

LABEL name="${NAME}"
LABEL version="${PRODUCT}"
LABEL release="${RELEASE}"
LABEL summary="The Stackable image for Apache HBase"
LABEL description="${DESCRIPTION}"

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.documentation="https://docs.stackable.tech/home/stable/hbase/"
LABEL org.opencontainers.image.version="${PRODUCT}"
LABEL org.opencontainers.image.revision="${RELEASE}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

# https://docs.openshift.com/container-platform/4.16/openshift_images/create-images.html#defining-image-metadata
# https://github.com/projectatomic/ContainerApplicationGenericLabels/blob/master/vendor/redhat/labels.md
LABEL io.openshift.tags="ubi9,stackable,hbase,sdp,nosql"
LABEL io.k8s.description="${DESCRIPTION}"
LABEL io.k8s.display-name="${NAME}"

COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-builder /stackable/hbase-${PRODUCT} /stackable/hbase-${PRODUCT}/
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-builder /stackable/async-profiler /stackable/async-profiler/
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-builder /stackable/jmx /stackable/jmx/

COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools-builder /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools-builder /stackable/bin/hbck2 /stackable/bin/hbck2
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools-builder /stackable/bin/hbase-entrypoint.sh /stackable/hbase-${PRODUCT}/bin/hbase-entrypoint.sh

COPY --chown=${STACKABLE_USER_UID}:0 --from=phoenix-builder /stackable/phoenix /stackable/phoenix/

COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-s3-builder /stackable/bin/export-snapshot-to-s3 /stackable/bin/export-snapshot-to-s3
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-s3-builder /stackable/hadoop/share/hadoop/tools/lib/ /stackable/hadoop/share/hadoop/tools/lib/

# Copy the dependencies from Hadoop which are required for the Azure Data Lake
# Storage (ADLS) to /stackable/hbase-${PRODUCT}/lib which is on the classpath.
# hadoop-azure-${HADOOP}.jar contains the AzureBlobFileSystem which is required
# by hadoop-common-${HADOOP}.jar if the scheme of a file system is "abfs://".
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder \
  /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar \
  /stackable/hbase-${PRODUCT}/lib/

COPY --chown=${STACKABLE_USER_UID}:0 --from=opa-authorizer-builder /stackable/hbase-opa-authorizer/target/hbase-opa-authorizer*.jar /stackable/hbase-${PRODUCT}/lib

RUN <<EOF
microdnf update

# The tar and python packages are required by the Phoenix command line.
# We add zip and gzip because tar without compression is seldom useful.
microdnf install \
  gzip \
  python \
  python-pip \
  tar \
  zip

microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

ln --symbolic --logical --verbose "/stackable/hbase-${PRODUCT}" /stackable/hbase
chown --no-dereference ${STACKABLE_USER_UID}:0 /stackable/hbase
chmod g=u /stackable/hbase

ln --symbolic --logical --verbose "/stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}" /stackable/hbase-operator-tools
chown --no-dereference ${STACKABLE_USER_UID}:0 /stackable/hbase-operator-tools
chmod g=u /stackable/hbase-operator-tools

ln --symbolic --logical --verbose "/stackable/phoenix/phoenix-server-hbase-${HBASE_PROFILE}.jar" "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"
chown --no-dereference ${STACKABLE_USER_UID}:0 "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"
chmod g=u "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"

# fix missing permissions
chmod g=u /stackable/async-profiler
chmod g=u /stackable/bin
chmod g=u /stackable/jmx
chmod g=u /stackable/phoenix
# the whole directory tree /stackable/hadoop/share/hadoop/tools/lib/ must be adapted
find /stackable/hadoop -type d -exec chmod g=u {} +
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.

USER ${STACKABLE_USER_UID}
ENV HBASE_CONF_DIR=/stackable/hbase/conf
ENV HOME=/stackable
ENV PATH="${PATH}:/stackable/bin:/stackable/hbase/bin"
ENV ASYNC_PROFILER_HOME=/stackable/async-profiler

WORKDIR /stackable/hbase
CMD ["./bin/hbase-entrypoint", "master", "localhost", "16010" ]
