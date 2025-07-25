# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/hadoop/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS druid-builder

ARG PRODUCT
ARG RELEASE
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER
ARG STACKABLE_USER_UID
ARG HADOOP_HADOOP
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP}

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

RUN <<EOF
microdnf update

# python-pyyaml:
#   This note was last checked for version 31.0.1
#   Required for the compilation of Druid.
#   This requirement is documented in docs/development/build.md and version 5.1 or later is required.
#   UBI 9 ships with 5.4.x so that should be fine
#
microdnf install python-pyyaml

microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 druid/stackable/patches/patchable.toml /stackable/src/druid/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 druid/stackable/patches/${PRODUCT} /stackable/src/druid/stackable/patches/${PRODUCT}

COPY --from=hadoop-builder --chown=${STACKABLE_USER_UID}:0 /stackable/patched-libs /stackable/patched-libs
# Cache mounts are owned by root by default
# We need to explicitly give the uid to use.
# The cache id has to include the product version that we are building because otherwise
# docker encounters race conditions when building multiple versions in parallel, as all
# builder containers will share the same cache and the `rm -rf` commands will fail
# with a "directory not empty" error on the first builder to finish, as other builders
# are still working in the cache directory.

# Let's have patchable as a dedicated step, as it fetches the druid sourcecode over the network,
# thus taking a bit (which is annoying while development)
RUN /stackable/patchable --images-repo-root=src checkout druid ${PRODUCT} > /tmp/DRUID_SOURCE_DIR

RUN --mount=type=cache,id=maven-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository \
  --mount=type=cache,id=npm-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.npm \
  --mount=type=cache,id=cache-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.cache \
  <<EOF
cd "$(cat /tmp/DRUID_SOURCE_DIR)" || exit 1
rm /tmp/DRUID_SOURCE_DIR

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Make Maven aware of custom Stackable libraries
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

# Create snapshot of the source code including custom patches
tar -czf /stackable/druid-${NEW_VERSION}-src.tar.gz .

mvn \
  --batch-mode \
  --no-transfer-progress \
  clean install \
  -Pdist,stackable-bundle-contrib-exts \
  -Dhadoop.compile.version=${HADOOP_VERSION}-stackable${RELEASE} \
  -DskipTests `# Skip test execution` \
  -Dcheckstyle.skip `# Skip checkstyle checks. We dont care if the code is properly formatted, it just wastes time` \
  -Dmaven.javadoc.skip=true `# Dont generate javadoc` \
  -Dmaven.gitcommitid.skip=true `# The gitcommitid plugin cannot work with git workspaces (ie: patchable)` \
  $(if [[ ${PRODUCT} != 30.* ]]; then echo --projects '!quidem-ut'; fi) `# This is just a maven module for tests. https://github.com/apache/druid/pull/16867 added https://raw.githubusercontent.com/kgyrtkirk/datasets/repo/ as a Maven repository, which fails to pull for us (Failed to execute goal on project druid-quidem-ut: Could not resolve dependencies for project org.apache.druid:druid-quidem-ut:jar:33.0.0: com.github.kgyrtkirk.datasets:kttm-nested:jar:0.1 was not found in https://build-repo.stackable.tech/repository/maven-public/). By disabling the maven module we dont pull in this weird dependency...`

mv distribution/target/apache-druid-${NEW_VERSION}-bin/apache-druid-${NEW_VERSION} /stackable/
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" distribution/target/bom.json
mv distribution/target/bom.json /stackable/apache-druid-${NEW_VERSION}/apache-druid-${NEW_VERSION}.cdx.json
rm -rf /stackable/apache-druid-${NEW_VERSION}-src

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
  rm -rf /stackable/.npm/*
  rm -rf /stackable/.cache/*
fi

# Do not remove the /stackable/apache-druid-${NEW_VERSION}/quickstart folder, it is needed for loading the Wikipedia
# testdata in kuttl tests and the getting started guide.

# Install OPA authorizer extension.
curl "https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz" | tar -xzC /stackable/apache-druid-${NEW_VERSION}/extensions

# change groups
chmod -R g=u /stackable
EOF

FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

ARG NAME="Apache Druid"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for Apache Druid"

LABEL name="${NAME}"
LABEL version="${PRODUCT}"
LABEL release="${RELEASE}"
LABEL summary="The Stackable image for Apache Druid"
LABEL description="${DESCRIPTION}"

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.documentation="https://docs.stackable.tech/home/stable/druid/"
LABEL org.opencontainers.image.version="${PRODUCT}"
LABEL org.opencontainers.image.revision="${RELEASE}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

# https://docs.openshift.com/container-platform/4.16/openshift_images/create-images.html#defining-image-metadata
# https://github.com/projectatomic/ContainerApplicationGenericLabels/blob/master/vendor/redhat/labels.md
LABEL io.openshift.tags="ubi9,stackable,druid,sdp"
LABEL io.k8s.description="${DESCRIPTION}"
LABEL io.k8s.display-name="${NAME}"


COPY --chown=${STACKABLE_USER_UID}:0 --from=druid-builder /stackable/apache-druid-${PRODUCT}-stackable${RELEASE} /stackable/apache-druid-${PRODUCT}-stackable${RELEASE}
COPY --chown=${STACKABLE_USER_UID}:0 --from=druid-builder /stackable/druid-${PRODUCT}-stackable${RELEASE}-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 druid/stackable/bin /stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 druid/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

ln -sf /stackable/apache-druid-${PRODUCT}-stackable${RELEASE} /stackable/druid
chown -h ${STACKABLE_USER_UID}:0 stackable/druid

# Force to overwrite the existing 'run-druid'
ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid
chown -h ${STACKABLE_USER_UID}:0 /stackable/druid/bin/run-druid

# fix missing permissions
chmod -R g=u /stackable/bin
chmod g=u /stackable/apache-druid-${PRODUCT}-stackable${RELEASE} /stackable/druid-${PRODUCT}-stackable${RELEASE}-src.tar.gz
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
ENV PATH="${PATH}":/stackable/druid/bin

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
