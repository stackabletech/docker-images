# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/stackable-base

ARG PRODUCT
ARG KUBECTL_VERSION
ARG RELEASE
ARG JQ_VERSION
ARG YQ_VERSION
ARG TARGETARCH
ARG STACKABLE_USER_UID

LABEL name="Stackable Tools" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="Stackable Tools for Kubernetes." \
    description="Stackable Tools for Kubernetes."

RUN microdnf update && \
    microdnf install \
    gettext \
    gzip \
    iputils \
    openssl \
    tar \
    # util-linux-core contains a basic set of Linux utilities, including
    # the command logger which allows to enter messages into the system
    # log.
    # cpe:2.3:a:kernel:util-linux:*:*:*:*:*:*:*:*
    # https://nvd.nist.gov/vuln/search/results?isCpeNameSearch=true&query=cpe%3A2.3%3Aa%3Akernel%3Autil-linux%3A*%3A*%3A*%3A*%3A*%3A*%3A*%3A
    util-linux-core \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY tools/licenses /licenses

WORKDIR /stackable/bin
ENV PATH=/stackable/bin:$PATH

# Get latest stable version from curl -L -s https://dl.k8s.io/release/stable.txt
RUN <<EOF
curl "https://dl.k8s.io/release/v${KUBECTL_VERSION}/bin/linux/${TARGETARCH}/kubectl" \
    -o /stackable/bin/kubectl
chmod +x /stackable/bin/kubectl

curl "https://github.com/stedolan/jq/releases/download/jq-${JQ_VERSION}/jq-linux64" \
    -o /stackable/bin/jq
chmod +x /stackable/bin/jq

# Needed to to patch manifests in OLM environments with tolerations
# and environment variables.
curl "https://github.com/mikefarah/yq/releases/download/v${YQ_VERSION}/yq_linux_${TARGETARCH}" \
    -o /stackable/bin/yq
chmod +x /stackable/bin/yq

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

# ----------------------------------------
# Attention: We are changing the group of all files in /stackable directly above
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}
