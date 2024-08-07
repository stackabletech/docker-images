# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/java-devel AS druid-builder

ARG PRODUCT
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

RUN <<EOF
microdnf update

# python-pyyaml:
#   This note was last checked for version 30.0.0
#   Required for the compilation of Druid.
#   This requirement is documented in docs/development/build.md and version 5.1 or later is required.
#   UBI 9 ships with 5.4.x so that should be fine
#
# patch: Required for the apply-patches.sh script
microdnf install \
python-pyyaml \
patch

microdnf clean all
rm -rf /var/cache/yum
EOF

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable druid/stackable/patches/apply_patches.sh /stackable/apache-druid-${PRODUCT}-src/patches/apply_patches.sh
COPY --chown=stackable:stackable druid/stackable/patches/${PRODUCT} /stackable/apache-druid-${PRODUCT}-src/patches/${PRODUCT}

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use which is hardcoded to "1000" in stackable-base
# The cache id has to include the product version that we are building because otherwise
# docker encounters race conditions when building multiple versions in parallel, as all
# builder containers will share the same cache and the `rm -rf` commands will fail
# with a "directory not empty" error on the first builder to finish, as other builders
# are still working in the cache directory.

RUN --mount=type=cache,id=maven-${PRODUCT},uid=1000,target=/stackable/.m2/repository \
    --mount=type=cache,id=npm-${PRODUCT},uid=1000,target=/stackable/.npm \
    --mount=type=cache,id=cache-${PRODUCT},uid=1000,target=/stackable/.cache \
    <<EOF
curl --fail -L "https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-src.tar.gz" | tar -xzC .
cd apache-druid-${PRODUCT}-src
./patches/apply_patches.sh ${PRODUCT}

mvn --batch-mode --no-transfer-progress clean install -Pdist,stackable-bundle-contrib-exts -DskipTests -Dmaven.javadoc.skip=true
mv distribution/target/apache-druid-${PRODUCT}-bin/apache-druid-${PRODUCT} /stackable/
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
curl --fail -L "https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz" | tar -xzC /stackable/apache-druid-${PRODUCT}/extensions
EOF

FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE

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

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
rm -rf /var/cache/yum
EOF

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable --from=druid-builder /stackable/apache-druid-${PRODUCT} /stackable/apache-druid-${PRODUCT}
COPY --chown=stackable:stackable druid/stackable/bin /stackable/bin
COPY --chown=stackable:stackable druid/licenses /licenses

RUN <<EOF
ln -s /stackable/apache-druid-${PRODUCT} /stackable/druid

# Force to overwrite the existing 'run-druid'
ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid
EOF

ENV PATH="${PATH}":/stackable/druid/bin

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
