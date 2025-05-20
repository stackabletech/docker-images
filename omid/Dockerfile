# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM stackable/image/java-devel AS builder

ARG PRODUCT
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
  mvn --batch-mode --no-transfer-progress package -Phbase-2 -DskipTests
  tar -xf tso-server/target/omid-tso-server-${PRODUCT}-bin.tar.gz -C /stackable
  mv tso-server/target/bom.json /stackable/omid-tso-server-${PRODUCT}/omid-tso-server-${PRODUCT}.cdx.json
  tar -xf examples/target/omid-examples-${PRODUCT}-bin.tar.gz -C /stackable
  mv examples/target/bom.json /stackable/omid-examples-${PRODUCT}/omid-examples-${PRODUCT}.cdx.json

if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
fi
EOF

# ===
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/omid-tso-server-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/omid-tso-server-${PRODUCT}
# ===

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
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-tso-server-${PRODUCT} /stackable/omid-tso-server-${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/omid-examples-${PRODUCT} /stackable/omid-examples-${PRODUCT}

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum

ln -s /stackable/omid-tso-server-${PRODUCT} /stackable/omid-tso-server
ln -s /stackable/omid-examples-${PRODUCT} /stackable/omid-examples
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
