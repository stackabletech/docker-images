# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# ==============================================================================
# START ZOOKEEPER BUILDER

FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG JMX_EXPORTER
ARG STACKABLE_USER_UID

# Copy patches into the builder
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/patches/patchable.toml /stackable/src/zookeeper/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/patches/${PRODUCT} /stackable/src/zookeeper/stackable/patches/${PRODUCT}
# Copy JMX config into the builder
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/jmx /stackable/jmx

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Download ZooKeeper sources from our own repo
RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout zookeeper ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/zookeeper-${PRODUCT}-src.tar.gz .

# Exclude the `zookeeper-client` submodule, this is not needed and has c parts
# that created all kinds of issues for the build container
mvn --batch-mode --no-transfer-progress -pl "!zookeeper-client/zookeeper-client-c" clean install checkstyle:check spotbugs:check -DskipTests -Pfull-build

# Unpack the archive which contains the build artifacts from above. Remove some
# unused files to shrink the final image size.
mv zookeeper-assembly/target/apache-zookeeper-${PRODUCT}-bin.tar.gz /stackable
tar -C /stackable -xvzf /stackable/apache-zookeeper-${PRODUCT}-bin.tar.gz
mv zookeeper-assembly/target/bom.json /stackable/apache-zookeeper-${PRODUCT}-bin/apache-zookeeper-${PRODUCT}.cdx.json
rm -rf /stackable/apache-zookeeper-${PRODUCT}-bin/docs
rm /stackable/apache-zookeeper-${PRODUCT}-bin/README_packaging.md

# Download the JMX exporter jar from our own repo
curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
  -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# set correct groups
chmod -R g=u /stackable
EOF

# END ZOOKEEPER BUILDER
# ==============================================================================

# ==============================================================================
# START FINAL ZOOKEEPER IMAGE

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL  \
  name="Apache ZooKeeper" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT}" \
  release="${RELEASE}" \
  summary="The Stackable image for Apache ZooKeeper." \
  description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

# Copy over the ZooKeeper binary folder
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/apache-zookeeper-${PRODUCT}-bin /stackable/apache-zookeeper-${PRODUCT}-bin/
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/zookeeper-${PRODUCT}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/jmx /stackable/jmx/
COPY zookeeper/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

# Add link pointing from /stackable/zookeeper to /stackable/apache-zookeeper-${PRODUCT}-bin/
# to preserve the folder name with the version.
ln -s /stackable/apache-zookeeper-${PRODUCT}-bin/ /stackable/zookeeper
chown -h ${STACKABLE_USER_UID}:0 /stackable/zookeeper

# fix missing permissions
chmod g=u /stackable/jmx
chmod g=u /stackable/apache-zookeeper-${PRODUCT}-bin/
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

ENV ZOOKEEPER_HOME=/stackable/zookeeper
ENV PATH="${PATH}":/stackable/zookeeper/bin

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/zookeeper
CMD ["bin/zkServer.sh", "start-foreground", "conf/zoo_sample.cfg"]

# END FINAL ZOOKEEPER IMAGE
# ==============================================================================
