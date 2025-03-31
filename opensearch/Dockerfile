ARG PRODUCT


FROM opensearchproject/opensearch:${PRODUCT} AS source


FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG STACKABLE_USER_UID

RUN <<EOF
microdnf update

# patch: Required for the apply-patches.sh script
microdnf install patch

microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}:0
WORKDIR /stackable

COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    --from=source \
    /usr/share/opensearch \
    opensearch
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    opensearch/stackable/patches/apply_patches.sh \
    opensearch/patches/apply_patches.sh
COPY \
    --chown=${STACKABLE_USER_UID}:0 \
    opensearch/stackable/patches/${PRODUCT} \
    opensearch/patches/${PRODUCT}

WORKDIR /stackable/opensearch

RUN <<EOF
echo y | bin/opensearch-plugin install repository-s3
EOF

RUN <<EOF
patches/apply_patches.sh ${PRODUCT}
rm -r patches

# The JDK from jdk-base is used instead.
rm -r jdk

# Change the group permissions already in the builder image to reduce
# the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u ${HOME}
EOF


# The OpenSearch Performance Analyzer needs a JDK, not just a JRE.
# With a JRE, the following exception is thrown:
# java.lang.ClassNotFoundException: com.sun.tools.attach.VirtualMachine
FROM stackable/image/jdk-base

ARG PRODUCT
ARG RELEASE
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
    --from=builder \
    /stackable/opensearch \
    ${OPENSEARCH_HOME}

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

WORKDIR ${OPENSEARCH_HOME}
CMD ["./opensearch-docker-entrypoint.sh"]
