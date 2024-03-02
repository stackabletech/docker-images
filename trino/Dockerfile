# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base

ARG PRODUCT
# External OPA authorizer only required for 414 and 428, included in 438
# The `OPA_AUTHORIZER` is not set in the conf.py for version 438 to remove the ARG as soon as 414 and 428 are removed
ARG OPA_AUTHORIZER
ARG JMX_EXPORTER
ARG RELEASE
ARG STORAGE_CONNECTOR

LABEL name="Trino" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Trino." \
      description="This image is deployed by the Stackable Operator for Trino."

RUN microdnf update && \
    microdnf install tar gzip zip python3 httpd-tools && \
    microdnf clean all && \
    alternatives --set python /usr/bin/python3

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable trino/stackable /stackable
COPY --chown=stackable:stackable trino/licenses /licenses

RUN curl --fail -L https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}.tar.gz | tar -xzC . && \
    ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server

RUN curl --fail https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

# TODO: remove the following RUN statement once Trino versions 414 and 428 are removed
RUN if [[ -n ${OPA_AUTHORIZER} ]] ; then \
        curl --fail -L https://repo.stackable.tech/repository/packages/trino-opa-authorizer/trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}.tar.gz | tar -xzC /stackable/trino-server/plugin ; \
    fi

RUN curl --fail https://repo.stackable.tech/repository/packages/trino-storage/trino-storage-${STORAGE_CONNECTOR}.zip -o /tmp/trino-storage-${STORAGE_CONNECTOR}.zip && \
    unzip /tmp/trino-storage-${STORAGE_CONNECTOR}.zip -d /stackable/trino-server/plugin && \
    rm -f /tmp/trino-storage-${STORAGE_CONNECTOR}.zip

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
RUN /bin/log4shell.sh /stackable/trino-server-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/trino-server-${PRODUCT}
# ===

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
