# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
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
curl -O https://repo.stackable.tech/repository/packages/trino-cli/trino-cli-${PRODUCT}-executable.jar
ln -s trino-cli-${PRODUCT}-executable.jar trino-cli-executable.jar

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

USER ${STACKABLE_USER_UID}
ENTRYPOINT ["java", "-jar", "/stackable/trino-cli/trino-cli-executable.jar"]
