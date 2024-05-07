# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL4006

# ==============================================================================
# START ZOOKEEPER BUILDER

FROM stackable/image/java-base AS builder

ARG PRODUCT
ARG JMX_EXPORTER

RUN microdnf update && \
    microdnf install \
    # git for the apply_patches.sh script
    git \
    gzip \
    maven \
    tar \
    zip \
    # hostname is needed by the maven ant run plugin for the "set-hostname-property" step
    hostname && \
    microdnf clean all

# Copy patches and JMX config into the builder
COPY --chown=stackable:stackable zookeeper/stackable /stackable

USER stackable
WORKDIR /stackable

# Download ZooKeeper sources from our own repo
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/zookeeper/apache-zookeeper-${PRODUCT}.tar.gz" | tar -xzC . && \
    # Apply any required patches
    patches/apply_patches.sh ${PRODUCT} && \
    cd /stackable/apache-zookeeper-${PRODUCT}/ && \
    # Exclude the `zookeeper-client` submodule, this is not needed and has c parts
    # that created all kinds of issues for the build container
    mvn -pl "!zookeeper-client/zookeeper-client-c" clean install checkstyle:check spotbugs:check -DskipTests -Pfull-build && \
    mv zookeeper-assembly/target/apache-zookeeper-${PRODUCT}-bin.tar.gz /stackable && \
    cd /stackable && \
    # Unpack the archive which contains the build artifacts from above. Remove some
    # unused files to shrink the final image size.
    tar xvzf /stackable/apache-zookeeper-${PRODUCT}-bin.tar.gz && \
    rm -rf /stackable/apache-zookeeper-${PRODUCT}-bin/docs && \
    rm /stackable/apache-zookeeper-${PRODUCT}-bin/README_packaging.md && \
    # Download the JMX exporter jar from our own repo
    curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
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

USER stackable
WORKDIR /stackable

# Copy over the ZooKeeper binary folder
COPY --link --chown=stackable:stackable --from=builder /stackable/apache-zookeeper-${PRODUCT}-bin /stackable/apache-zookeeper-${PRODUCT}-bin/
COPY --link --chown=stackable:stackable --from=builder /stackable/jmx /stackable/jmx/
COPY zookeeper/licenses /licenses

# Add link pointing from /stackable/zookeeper to /stackable/apache-zookeeper-${PRODUCT}-bin/
# to preserve the folder name with the version.
RUN ln -s /stackable/apache-zookeeper-${PRODUCT}-bin/ /stackable/zookeeper

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

ENV ZOOKEEPER_HOME=/stackable/zookeeper
ENV PATH="${PATH}":/stackable/zookeeper/bin

WORKDIR /stackable/zookeeper
CMD ["bin/zkServer.sh", "start-foreground", "conf/zoo_sample.cfg"]

# END FINAL ZOOKEEPER IMAGE
# ==============================================================================
