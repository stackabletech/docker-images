# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

FROM local-image/java-base

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

LABEL name="Trino CLI" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT_VERSION}" \
      release="${RELEASE_VERSION}" \
      summary="The Stackable image for the Trino CLI." \
      description="This image can be used to interact with Stackable Trino deployments."

RUN microdnf update && \
    microdnf install \
    gzip \
    tar \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum


COPY --chown=${STACKABLE_USER_UID}:0 trino-cli/licenses /licenses

WORKDIR /stackable/trino-cli

RUN <<EOF
curl -O "https://repo.stackable.tech/repository/packages/trino-cli/trino-cli-${PRODUCT_VERSION}"
chmod +x "trino-cli-${PRODUCT_VERSION}"
ln -s "trino-cli-${PRODUCT_VERSION}" trino-cli

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
ENTRYPOINT ["/stackable/trino-cli/trino-cli"]
