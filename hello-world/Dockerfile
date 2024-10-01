# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Hello World" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the Stackable Hello World Operator" \
      description="This image is deployed by the Stackable Hello World Operator."

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

curl "https://repo.stackable.tech/repository/packages/hello-world/hello-world-${PRODUCT}.jar" -o /stackable/hello-world.jar

# All files and folders owned by root to support running as arbitrary users
# This is best practice as all container users will belong to the root group (0)
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

CMD ["java", "-jar", "hello-world.jar"]
