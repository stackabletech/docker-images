# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

# ==============================================================================
# START ZOOKEEPER BUILDER

FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG JMX_EXPORTER
ARG STACKABLE_USER_UID

# Copy patches and JMX config into the builder
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable /stackable

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Download ZooKeeper sources from our own repo
RUN curl "https://repo.stackable.tech/repository/packages/zookeeper/apache-zookeeper-${PRODUCT}.tar.gz" | tar -xzC . && \
    # Apply any required patches
    patches/apply_patches.sh ${PRODUCT} && \
    cd /stackable/apache-zookeeper-${PRODUCT}/ && \
    # Exclude the `zookeeper-client` submodule, this is not needed and has c parts
    # that created all kinds of issues for the build container
    mvn --batch-mode --no-transfer-progress -pl "!zookeeper-client/zookeeper-client-c" clean install checkstyle:check spotbugs:check -DskipTests -Pfull-build && \
    mv zookeeper-assembly/target/apache-zookeeper-${PRODUCT}-bin.tar.gz /stackable && \
    cd /stackable && \
    # Unpack the archive which contains the build artifacts from above. Remove some
    # unused files to shrink the final image size.
    tar xvzf /stackable/apache-zookeeper-${PRODUCT}-bin.tar.gz && \
    mv /stackable/apache-zookeeper-${PRODUCT}/zookeeper-assembly/target/bom.json /stackable/apache-zookeeper-${PRODUCT}-bin/apache-zookeeper-${PRODUCT}.cdx.json && \
    rm -rf /stackable/apache-zookeeper-${PRODUCT}-bin/docs && \
    rm /stackable/apache-zookeeper-${PRODUCT}-bin/README_packaging.md && \
    # Download the JMX exporter jar from our own repo
    curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
    -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# ===
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/apache-zookeeper-${PRODUCT}-bin

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts
# on SocketNode.class, which we do not remove with our scripts. Further
# investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/apache-zookeeper-${PRODUCT}-bin
# ===

# END ZOOKEEPER BUILDER
# ==============================================================================

# ==============================================================================
# START FINAL ZOOKEEPER IMAGE

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Apache ZooKeeper" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache ZooKeeper." \
      description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

RUN microdnf update && \
    microdnf clean all && \
    rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt && \
    rm -rf /var/cache/yum

WORKDIR /stackable

# Copy over the ZooKeeper binary folder
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/apache-zookeeper-${PRODUCT}-bin /stackable/apache-zookeeper-${PRODUCT}-bin/
COPY --chown=${STACKABLE_USER_UID}:0 --from=builder /stackable/jmx /stackable/jmx/
COPY zookeeper/licenses /licenses

# Add link pointing from /stackable/zookeeper to /stackable/apache-zookeeper-${PRODUCT}-bin/
# to preserve the folder name with the version.
RUN <<EOF
ln -s /stackable/apache-zookeeper-${PRODUCT}-bin/ /stackable/zookeeper

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

ENV ZOOKEEPER_HOME=/stackable/zookeeper
ENV PATH="${PATH}":/stackable/zookeeper/bin

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/zookeeper
CMD ["bin/zkServer.sh", "start-foreground", "conf/zoo_sample.cfg"]

# END FINAL ZOOKEEPER IMAGE
# ==============================================================================
