# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/java-base

ARG PRODUCT
ARG JMX_EXPORTER
ARG RELEASE

LABEL name="Apache ZooKeeper" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache ZooKeeper." \
      description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

RUN microdnf update && \
    microdnf install \
    gzip \
    tar \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY --chown=stackable:stackable zookeeper/stackable /stackable
COPY zookeeper/licenses /licenses

USER stackable
WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/zookeeper/apache-zookeeper-${PRODUCT}-bin.tar.gz" | tar -xzC . && \
    ln -s "/stackable/apache-zookeeper-${PRODUCT}-bin" /stackable/zookeeper

RUN curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
    -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# ===
# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/apache-zookeeper-${PRODUCT}-bin

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/apache-zookeeper-${PRODUCT}-bin
# ===

WORKDIR /stackable/zookeeper
CMD ["bin/zkServer.sh", "start-foreground", "conf/zoo_sample.cfg"]
