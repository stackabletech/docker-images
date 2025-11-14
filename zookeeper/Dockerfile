# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

# ==============================================================================
# START ZOOKEEPER BUILDER

FROM local-image/shared/logback AS patched-logback

FROM local-image/java-devel AS builder

ARG PRODUCT_VERSION
ARG JMX_EXPORTER_VERSION
ARG STACKABLE_USER_UID
ARG RELEASE_VERSION

# Copy patches into the builder
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/patches/patchable.toml /stackable/src/zookeeper/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/patches/${PRODUCT_VERSION} /stackable/src/zookeeper/stackable/patches/${PRODUCT_VERSION}
# Copy JMX config into the builder
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/jmx /stackable/jmx
COPY --chown=${STACKABLE_USER_UID}:0 --from=patched-logback /stackable/.m2/repository /stackable/patched-logback-libs

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Download ZooKeeper sources from our own repo
RUN <<EOF
mkdir -p /stackable/.m2/repository
cp -r /stackable/patched-logback-libs/* /stackable/.m2/repository

cd "$(/stackable/patchable --images-repo-root=src checkout zookeeper ${PRODUCT_VERSION})"

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/zookeeper-${NEW_VERSION}-src.tar.gz .

# Exclude the `zookeeper-client` submodule, this is not needed and has c parts
# that created all kinds of issues for the build container
mvn \
  -pl "!zookeeper-client/zookeeper-client-c" \
  clean install \
  -DskipTests \
  -Pfull-build

# Unpack the archive which contains the build artifacts from above. Remove some
# unused files to shrink the final image size.
mv zookeeper-assembly/target/apache-zookeeper-${NEW_VERSION}-bin.tar.gz /stackable
tar -C /stackable -xvzf /stackable/apache-zookeeper-${NEW_VERSION}-bin.tar.gz
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" zookeeper-assembly/target/bom.json
mv zookeeper-assembly/target/bom.json /stackable/apache-zookeeper-${NEW_VERSION}-bin/apache-zookeeper-${NEW_VERSION}.cdx.json
rm -rf /stackable/apache-zookeeper-${NEW_VERSION}-bin/docs
rm /stackable/apache-zookeeper-${NEW_VERSION}-bin/README_packaging.md

# Download the JMX exporter jar from our own repo
curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar" \
  -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar"
chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# set correct groups
chmod -R g=u /stackable
EOF

# END ZOOKEEPER BUILDER
# ==============================================================================

# ==============================================================================
# START FINAL ZOOKEEPER IMAGE

FROM local-image/java-base

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

LABEL  \
  name="Apache ZooKeeper" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT_VERSION}" \
  release="${RELEASE_VERSION}" \
  summary="The Stackable image for Apache ZooKeeper." \
  description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

# Copy over the ZooKeeper binary folder
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/jmx /stackable/jmx/
# Copy over the patched logback files
COPY --chown=${STACKABLE_USER_UID}:0 --from=patched-logback /stackable/*-src.tar.gz /stackable
COPY zookeeper/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE_VERSION}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

# Add link pointing from /stackable/zookeeper to /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/
# to preserve the folder name with the version.
ln -s /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/ /stackable/zookeeper
chown -h ${STACKABLE_USER_UID}:0 /stackable/zookeeper

# fix missing permissions
chmod g=u /stackable/jmx
chmod g=u /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/
chmod g=u /stackable/*-src.tar.gz
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
