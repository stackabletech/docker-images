# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

FROM local-image/java-devel AS builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG DELETE_CACHES="true"
ARG STACKABLE_USER_UID

RUN <<EOF
microdnf update

# patch: Required for the apply-patches.sh script
microdnf install \
patch

microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 omid/stackable/patches/patchable.toml /stackable/src/omid/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 omid/stackable/patches/${PRODUCT_VERSION} /stackable/src/omid/stackable/patches/${PRODUCT_VERSION}

RUN --mount=type=cache,id=maven-omid-${PRODUCT_VERSION},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
  set -x
  cd "$(/stackable/patchable --images-repo-root=src checkout omid ${PRODUCT_VERSION})"

  ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
  NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

  mvn versions:set -DnewVersion=$NEW_VERSION

  # Create snapshot of the source code including custom patches
  tar -czf /stackable/omid-${NEW_VERSION}-src.tar.gz .
  mvn package -Phbase-2 -DskipTests
  tar -xf tso-server/target/omid-tso-server-${NEW_VERSION}-bin.tar.gz -C /stackable
  sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" tso-server/target/bom.json
  mv tso-server/target/bom.json /stackable/omid-tso-server-${NEW_VERSION}/omid-tso-server-${NEW_VERSION}.cdx.json
  tar -xf examples/target/omid-examples-${NEW_VERSION}-bin.tar.gz -C /stackable
  sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" examples/target/bom.json
  mv examples/target/bom.json /stackable/omid-examples-${NEW_VERSION}/omid-examples-${NEW_VERSION}.cdx.json

if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
fi
EOF

FROM local-image/java-base

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG JMX_EXPORTER_VERSION
ARG STACKABLE_USER_UID

LABEL name="Apache Phoenix Omid" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT_VERSION}" \
      release="${RELEASE_VERSION}" \
      summary="The Stackable image for Apache Phoenix Omid." \
      description="This image is deployed by the Stackable Operator for Apache HBase."

COPY omid/licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 omid/stackable /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-tso-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/omid-tso-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-examples-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/omid-examples-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /stackable

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

ln -s /stackable/omid-tso-server-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/omid-tso-server
ln -s /stackable/omid-examples-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/omid-examples
curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar \
-o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar
chmod -x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar
# omid.sh places this file at the front of the classpath: remove it to allow the config map entry to take precedence
rm /stackable/omid-tso-server/conf/hbase-site.xml

# To support arbitrary user ids on OpenShift, this folder must belong to the root group.
mkdir /stackable/logs

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
WORKDIR /stackable/omid-tso-server

ENV HOME=/stackable
ENV HBASE_CONF_DIR=/stackable/hbase/conf
ENV OMID_OPTS="-Domid.log.dir=/stackable/logs"

CMD ["./bin/omid.sh", "tso" ]
