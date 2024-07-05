# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# Ignoring SC2164 globally because we inherit SHELL from the base image which contains "-e" for bash
# hadolint global ignore=DL3038,DL4006,SC2164

# hadolint ignore=DL3006
FROM stackable/image/java-devel AS druid-builder

ARG PRODUCT
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER

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

RUN <<EOF
curl --fail -L "https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-src.tar.gz" | tar -xzC .
cd apache-druid-${PRODUCT}-src
./patches/apply_patches.sh ${PRODUCT}

mvn --batch-mode --no-transfer-progress clean install -Pdist,stackable-bundle-contrib-exts -DskipTests -Dmaven.javadoc.skip=true
mv distribution/target/apache-druid-${PRODUCT}-bin/apache-druid-${PRODUCT} /stackable/

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
rm -rf /stackable/apache-druid-${PRODUCT}-src
rm -rf /stackable/.m2
rm -rf /stackable/.npm
rm -rf /stackable/.cache

# Do not remove the /stackable/apache-druid-${PRODUCT}/quickstart folder, it is needed for loading the Wikipedia
# testdata in kuttl tests and the getting started guide.

# Install OPA authorizer extension.
curl --fail -L "https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz" | tar -xzC /stackable/apache-druid-${PRODUCT}/extensions
EOF

# hadolint ignore=DL3006
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
