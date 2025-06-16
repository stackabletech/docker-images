# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

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
WORKDIR /stackable

CMD ["java", "-jar", "hello-world.jar"]
