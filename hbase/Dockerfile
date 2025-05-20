# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/hbase/phoenix AS phoenix

FROM stackable/image/hbase/hbase-operator-tools AS hbase-operator-tools

FROM stackable/image/hbase/hbase-opa-authorizer AS hbase-opa-authorizer

FROM stackable/image/java-devel AS hbase-builder

ARG PRODUCT
ARG HBASE_HBASE_OPERATOR_TOOLS
ARG ASYNC_PROFILER
ARG HBASE_PROFILE
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

COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/patches/patchable.toml /stackable/src/hbase/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/stackable/patches/${PRODUCT} /stackable/src/hbase/stackable/patches/${PRODUCT}

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

# Create snapshot of the source code including custom patches
tar -czf /stackable/hbase-${PRODUCT}-src.tar.gz .

# The release scripts of HBase also run the build twice (three times in fact, once again to build the site which we skip here).
# I chose to replicate that exact behavior for consistency so please don't merge the two mvn runs into one unless you really know what you're doing!
# Cannot skip building tests here because the assembly plugin needs a shell script from the test directory.
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhadoop.profile=3.0 \
  -Dhadoop-three.version=${HADOOP} \
  -DskipTests \
  clean install

mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhadoop.profile=3.0 \
  -Dhadoop-three.version=${HADOOP} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  -Prelease \
  install assembly:single

tar -xzf hbase-assembly/target/hbase-${PRODUCT}-bin.tar.gz -C /stackable/
mv hbase-assembly/target/bom.json /stackable/hbase-${PRODUCT}/hbase-${PRODUCT}.cdx.json

# Remove sources
(cd .. && rm -r ${PRODUCT})

ln -s "/stackable/hbase-${PRODUCT}" /stackable/hbase

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
  # So we try to copy both and if one of them doesn't exist buildx will just ignore it :)
  /stackable/hadoop/share/hadoop/tools/lib/bundle-*.jar \
  /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-*.jar \
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

# Final Image
FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG HADOOP
ARG HBASE_PROFILE
ARG HBASE_HBASE_OPERATOR_TOOLS
ARG HBASE_HBASE_OPA_AUTHORIZER
ARG HBASE_PHOENIX
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
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-builder /stackable/hbase-${PRODUCT}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-builder /stackable/async-profiler /stackable/async-profiler/

COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools /stackable/hbase-operator-tools-${HBASE_HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools-${HBASE_HBASE_OPERATOR_TOOLS}/
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools /stackable/hbase-operator-tools-${HBASE_HBASE_OPERATOR_TOOLS}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools /stackable/bin/hbck2 /stackable/bin/hbck2
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-operator-tools /stackable/bin/hbase-entrypoint.sh /stackable/hbase-${PRODUCT}/bin/hbase-entrypoint.sh

COPY --chown=${STACKABLE_USER_UID}:0 --from=phoenix /stackable/phoenix /stackable/phoenix/
COPY --chown=${STACKABLE_USER_UID}:0 --from=phoenix /stackable/phoenix-${HBASE_PHOENIX}-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-s3-builder /stackable/bin/export-snapshot-to-s3 /stackable/bin/export-snapshot-to-s3
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-s3-builder /stackable/hadoop/share/hadoop/tools/lib/ /stackable/hadoop/share/hadoop/tools/lib/

# Copy the dependencies from Hadoop which are required for the Azure Data Lake
# Storage (ADLS) to /stackable/hbase-${PRODUCT}/lib which is on the classpath.
# hadoop-azure-${HADOOP}.jar contains the AzureBlobFileSystem which is required
# by hadoop-common-${HADOOP}.jar if the scheme of a file system is "abfs://".
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder \
  /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar \
  /stackable/hbase-${PRODUCT}/lib/

COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-opa-authorizer /stackable/hbase-opa-authorizer-${HBASE_HBASE_OPA_AUTHORIZER}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hbase-opa-authorizer /stackable/hbase-opa-authorizer/target/hbase-opa-authorizer*.jar /stackable/hbase-${PRODUCT}/lib

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

ln --symbolic --logical --verbose "/stackable/hbase-operator-tools-${HBASE_HBASE_OPERATOR_TOOLS}" /stackable/hbase-operator-tools
chown --no-dereference ${STACKABLE_USER_UID}:0 /stackable/hbase-operator-tools
chmod g=u /stackable/hbase-operator-tools

ln --symbolic --logical --verbose "/stackable/phoenix/phoenix-server-hbase-${HBASE_PROFILE}.jar" "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"
chown --no-dereference ${STACKABLE_USER_UID}:0 "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"
chmod g=u "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"

# fix missing permissions
chmod g=u /stackable/async-profiler
chmod g=u /stackable/bin
chmod g=u /stackable/phoenix
chmod g=u /stackable/*-src.tar.gz
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
