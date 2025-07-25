FROM stackable/image/opensearch/security-plugin AS opensearch-security-plugin
FROM stackable/image/opensearch/performance-analyzer AS opensearch-performance-analyzer
FROM stackable/image/java-devel AS opensearch-builder

ARG PRODUCT
ARG RELEASE
ARG OPENSEARCH_SECURITY_PLUGIN
ARG OPENSEARCH_PERFORMANCE_ANALYZER
ARG STACKABLE_USER_UID

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

NEW_VERSION="${PRODUCT}-stackable${RELEASE}"
# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-${NEW_VERSION}-src.tar.gz .
./gradlew clean localDistro -Dbuild.snapshot=false
./gradlew cyclonedxBom --warning-mode=summary -Dbuild.snapshot=false
cp -r build/distribution/local/opensearch-${PRODUCT} /stackable/opensearch
cp distribution/docker/src/docker/bin/docker-entrypoint.sh /stackable/opensearch/opensearch-docker-entrypoint.sh
# repository-s3 plugin
cd plugins/repository-s3
../../gradlew clean assemble -Dbuild.snapshot=false
mkdir /stackable/opensearch/plugins/repository-s3
cp build/distributions/repository-s3-${PRODUCT}.zip /stackable/opensearch/plugins/repository-s3/repository-s3-${PRODUCT}.zip
cp -r config/repository-s3 /stackable/opensearch/config/repository-s3
cd /stackable/opensearch/plugins/repository-s3
unzip repository-s3-${PRODUCT}.zip
rm repository-s3-${PRODUCT}.zip
# Fix an error where OpenSearch running in a container is denied access to swap space size
sed -i -e 's/OPENSEARCH_DISTRIBUTION_TYPE=tar/OPENSEARCH_DISTRIBUTION_TYPE=docker/' /stackable/opensearch/bin/opensearch-env
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
    /stackable/opensearch/config/opensearch-performance-analyzer

RUN <<EOF
unzip opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}.zip
rm opensearch-performance-analyzer-${OPENSEARCH_PERFORMANCE_ANALYZER}.zip
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
mv config /stackable/opensearch/config/opensearch-security
EOF

WORKDIR /stackable/opensearch

RUN <<EOF
# The JDK from jdk-base is used instead.
rm -r jdk

# Change the group permissions already in the builder image to reduce
# the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u "${HOME}"
chmod +x /stackable/opensearch/opensearch-docker-entrypoint.sh
EOF


# The OpenSearch Performance Analyzer needs a JDK, not just a JRE.
# With a JRE, the following exception is thrown:
# java.lang.ClassNotFoundException: com.sun.tools.attach.VirtualMachine
FROM stackable/image/jdk-base

ARG PRODUCT
ARG RELEASE
ARG OPENSEARCH_SECURITY_PLUGIN
ARG OPENSEARCH_PERFORMANCE_ANALYZER
ARG STACKABLE_USER_UID

ARG NAME="OpenSearch"
ARG DESCRIPTION="This image is deployed by a custom Helm Chart"
ARG HOME=/stackable
ARG OPENSEARCH_HOME=${HOME}/opensearch

LABEL name="${NAME}"
LABEL version="${PRODUCT}"
LABEL release="${RELEASE}"
LABEL summary="The Stackable image for OpenSearch"
LABEL description="${DESCRIPTION}"

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.version="${PRODUCT}"
LABEL org.opencontainers.image.revision="${RELEASE}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch \
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

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 ${HOME}
chmod -R g=u ${HOME}
ln -s /stackable/opensearch-${PRODUCT}-stackable${RELEASE} ${OPENSEARCH_HOME}
chown -h ${STACKABLE_USER_UID}:0 ${OPENSEARCH_HOME}
EOF

USER ${STACKABLE_USER_UID}
ENV HOME=${HOME}
ENV OPENSEARCH_HOME=${OPENSEARCH_HOME}
ENV PATH="${PATH}:/${OPENSEARCH_HOME}:${OPENSEARCH_HOME}/bin"
ENV OPENSEARCH_CONTAINER=true

WORKDIR ${OPENSEARCH_HOME}
CMD ["./opensearch-docker-entrypoint.sh"]
