# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG RELEASE
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
COPY --chown=${STACKABLE_USER_UID}:0 omid/stackable/patches/${PRODUCT} /stackable/src/omid/stackable/patches/${PRODUCT}

RUN --mount=type=cache,id=maven-omid-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
  set -x
  cd "$(/stackable/patchable --images-repo-root=src checkout omid ${PRODUCT})"

  ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
  NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

  mvn versions:set -DnewVersion=$NEW_VERSION

  # Create snapshot of the source code including custom patches
  tar -czf /stackable/omid-${NEW_VERSION}-src.tar.gz .
  mvn --batch-mode --no-transfer-progress package -Phbase-2 -DskipTests
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

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG JMX_EXPORTER
ARG STACKABLE_USER_UID

LABEL name="Apache Phoenix Omid" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Phoenix Omid." \
      description="This image is deployed by the Stackable Operator for Apache HBase."

COPY omid/licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 omid/stackable /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-tso-server-${PRODUCT}-stackable${RELEASE} /stackable/omid-tso-server-${PRODUCT}-stackable${RELEASE}
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-examples-${PRODUCT}-stackable${RELEASE} /stackable/omid-examples-${PRODUCT}-stackable${RELEASE}
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-${PRODUCT}-stackable${RELEASE}-src.tar.gz /stackable

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

ln -s /stackable/omid-tso-server-${PRODUCT}-stackable${RELEASE} /stackable/omid-tso-server
ln -s /stackable/omid-examples-${PRODUCT}-stackable${RELEASE} /stackable/omid-examples
curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
-o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
chmod -x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
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
