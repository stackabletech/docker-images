FROM docker.stackable.tech/sandbox/logging/java-base:11-stackable0.3.0@sha256:4e47b9dd91d5ce5c7b8be9561b3efcf108106fe71ec9b97976a647d7025c14fd

ARG PRODUCT
ARG RELEASE="1"

LABEL name="Apache ZooKeeper" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache ZooKeeper." \
      description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip && \
    microdnf install openssl && \
    microdnf clean all

COPY --chown=stackable:stackable zookeeper/stackable /stackable
COPY zookeeper/licenses /licenses

USER stackable
WORKDIR /stackable

RUN curl -L https://repo.stackable.tech/repository/packages/zookeeper/apache-zookeeper-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-zookeeper-${PRODUCT}-bin /stackable/zookeeper

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

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
