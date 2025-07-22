
FROM stackable/image/opensearch/security-plugin AS opensearch-security-plugin
FROM stackable/image/java-devel AS opensearch-builder

ARG PRODUCT
ARG RELEASE
ARG OPENSEARCH_SECURITY_PLUGIN
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
./gradlew localDistro
./gradlew cyclonedxBom --warning-mode=summary
cp -r ./distribution/archives/linux-tar/build/install/opensearch-${PRODUCT}-SNAPSHOT /stackable/opensearch
cp ./distribution/docker/src/docker/bin/docker-entrypoint.sh /stackable/opensearch/opensearch-docker-entrypoint.sh
cp build/reports/bom.json /stackable/opensearch/opensearch-${PRODUCT}-SNAPSHOT.cdx.json
EOF

WORKDIR /stackable/opensearch-security-plugin
COPY --chown=${STACKABLE_USER_UID}:0 --from=opensearch-security-plugin /stackable/src/opensearch/security-plugin/patchable-work/worktree/${OPENSEARCH_SECURITY_PLUGIN}/build/distributions/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-SNAPSHOT.zip /stackable/opensearch-security-plugin/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-SNAPSHOT.zip
COPY --chown=${STACKABLE_USER_UID}:0 --from=opensearch-security-plugin /stackable/src/opensearch/security-plugin/patchable-work/worktree/${OPENSEARCH_SECURITY_PLUGIN}/build/reports/bom.json /stackable/opensearch-security-plugin/opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-SNAPSHOT.cdx.json

RUN <<EOF
unzip opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-SNAPSHOT.zip
rm -r opensearch-security-${OPENSEARCH_SECURITY_PLUGIN}-SNAPSHOT.zip
mv config /stackable/opensearch/config/opensearch-security
EOF

WORKDIR /stackable/opensearch

RUN <<EOF
echo y | bin/opensearch-plugin install repository-s3

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
    ${OPENSEARCH_HOME}
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-builder \
    /stackable/opensearch-security-plugin \
    ${OPENSEARCH_HOME}/plugins/opensearch-security
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=opensearch-security-plugin \
    /stackable/opensearch-security-plugin-${OPENSEARCH_SECURITY_PLUGIN}-src.tar.gz \
    /stackable

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 ${HOME}
chmod -R g=u ${HOME}
EOF

USER ${STACKABLE_USER_UID}
ENV HOME=${HOME}
ENV OPENSEARCH_HOME=${OPENSEARCH_HOME}
ENV PATH="${PATH}:/${OPENSEARCH_HOME}:${OPENSEARCH_HOME}/bin"
ENV OPENSEARCH_CONTAINER=true

WORKDIR ${OPENSEARCH_HOME}
CMD ["./opensearch-docker-entrypoint.sh"]
