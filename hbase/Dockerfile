# syntax=docker/dockerfile:1
FROM stackable/image/java-base

ARG PRODUCT
ARG PHOENIX
ARG RELEASE

LABEL name="Apache HBase" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache HBase." \
      description="This image is deployed by the Stackable Operator for Apache HBase."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip && \
    microdnf install python3 python3-pip && \
    microdnf clean all

RUN ln -s /usr/bin/python3 /usr/bin/python && \
    ln -s /usr/bin/pip3 /usr/bin/pip

ENV HOME=/stackable

COPY --chown=stackable:stackable hbase/stackable /stackable
COPY hbase/licenses /licenses

USER stackable
WORKDIR /stackable

RUN curl -L https://repo.stackable.tech/repository/packages/hbase/hbase-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/hbase-${PRODUCT} /stackable/hbase && \
    curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar && \
    curl -L https://repo.stackable.tech/repository/packages/phoenix/phoenix-hbase-${PHOENIX}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/phoenix-hbase-${PHOENIX}-bin /stackable/phoenix && \
    ln -s /stackable/phoenix/phoenix-server-hbase-${PHOENIX}.jar /stackable/hbase/lib/phoenix-server-hbase-${PHOENIX}.jar

ENV HBASE_CONF_DIR=/stackable/hbase/conf

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
RUN /bin/log4shell.sh /stackable/hbase-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/hbase-${PRODUCT}
# ===

WORKDIR /stackable/hbase
CMD ["./bin/hbase", "master", "start" ]
