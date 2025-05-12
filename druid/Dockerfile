# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/java-devel AS druid-builder

ARG PRODUCT
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER
ARG STACKABLE_USER_UID

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

COPY --chown=${STACKABLE_USER_UID}:0 druid/stackable/patches/${PRODUCT} /stackable/src/druid/stackable/patches/${PRODUCT}

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use which is hardcoded to "1000" in stackable-base
# The cache id has to include the product version that we are building because otherwise
# docker encounters race conditions when building multiple versions in parallel, as all
# builder containers will share the same cache and the `rm -rf` commands will fail
# with a "directory not empty" error on the first builder to finish, as other builders
# are still working in the cache directory.

RUN --mount=type=cache,id=maven-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository \
  --mount=type=cache,id=npm-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.npm \
  --mount=type=cache,id=cache-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.cache \
  <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout druid ${PRODUCT})"

mvn \
  --batch-mode \
  --no-transfer-progress \
  clean install \
  -Pdist,stackable-bundle-contrib-exts \
  -DskipTests \
  -Dmaven.javadoc.skip=true \
  -Dmaven.gitcommitid.skip=true

mv distribution/target/apache-druid-${PRODUCT}-bin/apache-druid-${PRODUCT} /stackable/
mv distribution/target/bom.json /stackable/apache-druid-${PRODUCT}/apache-druid-${PRODUCT}.cdx.json
rm -rf /stackable/apache-druid-${PRODUCT}-src

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
if [ "${DELETE_CACHES}" = "true" ] ; then
rm -rf /stackable/.m2/repository/*
rm -rf /stackable/.npm/*
rm -rf /stackable/.cache/*
fi

# Do not remove the /stackable/apache-druid-${PRODUCT}/quickstart folder, it is needed for loading the Wikipedia
# testdata in kuttl tests and the getting started guide.

# Install OPA authorizer extension.
curl "https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz" | tar -xzC /stackable/apache-druid-${PRODUCT}/extensions

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


COPY --chown=${STACKABLE_USER_UID}:0 --from=druid-builder /stackable/apache-druid-${PRODUCT} /stackable/apache-druid-${PRODUCT}

COPY --chown=${STACKABLE_USER_UID}:0 druid/stackable/bin /stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 druid/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

ln -sf /stackable/apache-druid-${PRODUCT} /stackable/druid
chown -h ${STACKABLE_USER_UID}:0 stackable/druid

# Force to overwrite the existing 'run-druid'
ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid
chown -h ${STACKABLE_USER_UID}:0 /stackable/druid/bin/run-druid

# fix missing permissions
chmod -R g=u /stackable/bin
chmod g=u /stackable/apache-druid-${PRODUCT}
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
