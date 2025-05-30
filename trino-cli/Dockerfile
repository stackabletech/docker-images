# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Trino CLI" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
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
curl -O "https://repo.stackable.tech/repository/packages/trino-cli/trino-cli-${PRODUCT}-executable.jar"
ln -s "trino-cli-${PRODUCT}-executable.jar" trino-cli-executable.jar

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
ENTRYPOINT ["java", "-jar", "/stackable/trino-cli/trino-cli-executable.jar"]
