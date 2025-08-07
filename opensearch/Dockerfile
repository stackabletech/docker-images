# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/opensearch/security-plugin AS opensearch-security-plugin
FROM stackable/image/opensearch/performance-analyzer AS opensearch-performance-analyzer
FROM stackable/image/java-devel AS opensearch-builder

ARG PRODUCT
ARG RELEASE
ARG OPENSEARCH_SECURITY_PLUGIN
ARG OPENSEARCH_PERFORMANCE_ANALYZER
ARG STACKABLE_USER_UID
ARG TARGETARCH

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch/stackable/patches/patchable.toml /stackable/src/opensearch/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch/stackable/patches/${PRODUCT} /stackable/src/opensearch/stackable/patches/${PRODUCT}

RUN <<EOF
microdnf update
microdnf install unzip
microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch ${PRODUCT})"

ARCH="${TARGETARCH/amd64/x64}"
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"
# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-${NEW_VERSION}-src.tar.gz .
./scripts/build.sh -v "${PRODUCT}" -s false -a "${ARCH}"
tar -xzf "artifacts/dist/opensearch-min-${PRODUCT}-linux-${ARCH}.tar.gz" -C /stackable
unzip artifacts/core-plugins/repository-s3-${PRODUCT}.zip -d /stackable/opensearch-${PRODUCT}/plugins/repository-s3/
mv /stackable/opensearch-${PRODUCT}/plugins/repository-s3/config /stackable/opensearch-${PRODUCT}/config/repository-s3
unzip artifacts/core-plugins/telemetry-otel-${PRODUCT}.zip -d /stackable/opensearch-${PRODUCT}/plugins/telemetry-otel/
mv /stackable/opensearch-${PRODUCT}/plugins/telemetry-otel/config /stackable/opensearch-${PRODUCT}/config/telemetry-otel
./gradlew cyclonedxBom --warning-mode=summary -Dbuild.snapshot=false
cp distribution/docker/src/docker/bin/docker-entrypoint.sh /stackable/opensearch-${PRODUCT}/opensearch-docker-entrypoint.sh
# Tell OpenSearch it is running in a container
sed -i -e 's/OPENSEARCH_DISTRIBUTION_TYPE=tar/OPENSEARCH_DISTRIBUTION_TYPE=docker/' /stackable/opensearch-${PRODUCT}/bin/opensearch-env
EOF

WORKDIR /stackable/opensearch-security
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/src/opensearch/security-plugin/patchable-work/worktree/${OPENSEARCH_SECURITY_PLUGIN}/build/distributions/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}.zip \
    opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}.zip

RUN <<EOF
unzip opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}.zip
rm opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}.zip
mv config /stackable/opensearch-${PRODUCT}/config/opensearch-security
EOF

WORKDIR /stackable/opensearch-performance-analyzer
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-performance-analyzer \
    /stackable/src/opensearch/performance-analyzer/patchable-work/worktree/${OPENSEARCH_PERFORMANCE_ANALYZER}/build/distributions/opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}.zip \
    opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}.zip
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-performance-analyzer \
    /stackable/src/opensearch/performance-analyzer/patchable-work/worktree/${OPENSEARCH_PERFORMANCE_ANALYZER}/config \
    /stackable/opensearch-${PRODUCT}/config/opensearch-performance-analyzer

RUN <<EOF
unzip opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}.zip
rm opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}.zip
EOF

WORKDIR /stackable/opensearch-${PRODUCT}

RUN <<EOF
# The JDK from jdk-base is used instead.
rm -r jdk

# Change the group permissions already in the builder image to reduce
# the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod +x /stackable/opensearch-${PRODUCT}/opensearch-docker-entrypoint.sh
chmod -R g=u /stackable
EOF

# The OpenSearch Performance Analyzer needs a JDK, not just a JRE.
# With a JRE, the following exception is thrown:
# java.lang.ClassNotFoundException: com.sun.tools.attach.VirtualMachine
FROM stackable/image/jdk-base AS final

ARG PRODUCT
ARG RELEASE
ARG OPENSEARCH_SECURITY_PLUGIN
ARG OPENSEARCH_PERFORMANCE_ANALYZER
ARG STACKABLE_USER_UID

ARG NAME="OpenSearch"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for OpenSearch."
ARG HOME=/stackable
ARG OPENSEARCH_HOME=${HOME}/opensearch

LABEL \
  name="OpenSearch" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT}" \
  release="${RELEASE}" \
  summary="The Stackable image for OpenSearch." \
  description="This image is deployed by the Stackable Operator for OpenSearch."

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.version="${PRODUCT}"
LABEL org.opencontainers.image.revision="${RELEASE}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-${PRODUCT} \
    /stackable/opensearch-${PRODUCT}-stackable${RELEASE}
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-security \
    /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/plugins/opensearch-security
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-performance-analyzer \
    /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/plugins/opensearch-performance-analyzer
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-${PRODUCT}-stackable${RELEASE}-src.tar.gz \
    /stackable
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-stackable${RELEASE}-src.tar.gz \
    /stackable
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-performance-analyzer \
    /stackable/opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}-stackable${RELEASE}-src.tar.gz \
    /stackable
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/src/opensearch/patchable-work/worktree/${PRODUCT}/build/reports/bom.json \
    /stackable/opensearch-${PRODUCT}-stackable${RELEASE}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/src/opensearch/security-plugin/patchable-work/worktree/${OPENSEARCH_SECURITY_PLUGIN}/build/reports/bom.json \
    /stackable/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-stackable${RELEASE}.cdx.json
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-performance-analyzer \
    /stackable/src/opensearch/performance-analyzer/patchable-work/worktree/${OPENSEARCH_PERFORMANCE_ANALYZER}/build/reports/bom.json \
    /stackable/opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}-stackable${RELEASE}.cdx.json

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

echo '## OpenDistro Performance Analyzer' >> /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/config/jvm.options
echo "-Dclk.tck=100" >> /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/config/jvm.options
echo "-Djdk.attach.allowAttachSelf=true" >> /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/config/jvm.options
echo "-Djava.security.policy=file:///stackable/opensearch/config/opensearch-performance-analyzer/opensearch_security.policy" >> /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/config/jvm.options
echo "--add-opens=jdk.attach/sun.tools.attach=ALL-UNNAMED" >> /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/config/jvm.options

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown ${STACKABLE_USER_UID}:0 ${HOME}
chmod g=u /stackable/opensearch-${PRODUCT}-stackable${RELEASE}
chmod g=u /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/plugins/opensearch-security
chmod g=u /stackable/opensearch-${PRODUCT}-stackable${RELEASE}/plugins/opensearch-performance-analyzer
chmod g=u /stackable/*-src.tar.gz
chmod g=u /stackable/*.cdx.json
ln -s /stackable/opensearch-${PRODUCT}-stackable${RELEASE} ${OPENSEARCH_HOME}
chown -h ${STACKABLE_USER_UID}:0 ${OPENSEARCH_HOME}

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.

/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

USER ${STACKABLE_USER_UID}

ENV HOME=${HOME}
ENV OPENSEARCH_HOME=${OPENSEARCH_HOME}
ENV PATH="${PATH}:/${OPENSEARCH_HOME}:${OPENSEARCH_HOME}/bin"

WORKDIR ${OPENSEARCH_HOME}
CMD ["./opensearch-docker-entrypoint.sh"]
