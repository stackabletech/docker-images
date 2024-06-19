# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# hadolint ignore=DL3006
FROM stackable/image/java-devel AS builder

ARG PRODUCT

USER stackable
WORKDIR /stackable

RUN curl --fail -L https://dist.apache.org/repos/dist/release/phoenix/phoenix-omid-${PRODUCT}/phoenix-omid-${PRODUCT}-src.tar.gz | tar -xzC .

WORKDIR /stackable/phoenix-omid-${PRODUCT}
RUN mvn package -Phbase-2 -DskipTests && \
    tar -xf tso-server/target/omid-tso-server-${PRODUCT}-bin.tar.gz -C /stackable && \
    tar -xf examples/target/omid-examples-${PRODUCT}-bin.tar.gz -C /stackable

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

LABEL name="Apache Phoenix Omid" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Phoenix Omid." \
      description="This image is deployed by the Stackable Operator for Apache HBase."

RUN microdnf update && \
    microdnf clean all && \
    rm -rf /var/cache/yum

ENV HOME=/stackable

COPY --chown=stackable:stackable omid/stackable /stackable
COPY omid/licenses /licenses

USER stackable
WORKDIR /stackable

COPY --from=builder /stackable/omid-tso-server-${PRODUCT} /stackable/omid-tso-server-${PRODUCT}
COPY --from=builder /stackable/omid-examples-${PRODUCT} /stackable/omid-examples-${PRODUCT}

RUN ln -s /stackable/omid-tso-server-${PRODUCT} /stackable/omid-tso-server && \
    ln -s /stackable/omid-examples-${PRODUCT} /stackable/omid-examples && \
    curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar

ENV HBASE_CONF_DIR=/stackable/hbase/conf
# omid.sh places this file at the front of the classpath: remove it to allow the config map entry to take precedence
RUN rm /stackable/omid-tso-server/conf/hbase-site.xml
WORKDIR /stackable/omid-tso-server
CMD ["./bin/omid.sh", "tso" ]
