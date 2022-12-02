FROM docker.stackable.tech/stackable/javabase:11-stackable0.2.2@sha256:7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e AS builder

RUN microdnf install -y zlib-devel openssl-devel cyrus-sasl-devel libcurl-devel && \
    microdnf install -y tar which wget zlib gcc-c++ make cmake && \
    microdnf clean all

WORKDIR /stackable
RUN curl -L -O https://github.com/edenhill/kcat/archive/refs/tags/1.7.0.tar.gz \
    && tar xvfz 1.7.0.tar.gz \
    && cd kcat-1.7.0 \
    && ./bootstrap.sh

FROM docker.stackable.tech/stackable/java-base:11-stackable0.2.2@sha256:7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e

ARG PRODUCT
ARG SCALA
ARG OPA_AUTHORIZER
ARG RELEASE="1"

LABEL name="Apache Kafka" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Kafka." \
      description="This image is deployed by the Stackable Operator for Apache Kafka."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip && \
    microdnf install shadow-utils openssl && \
    microdnf clean all

COPY kafka/stackable /stackable
COPY kafka/licenses /licenses

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable
WORKDIR /stackable

RUN curl -L https://repo.stackable.tech/repository/packages/kafka/kafka_${SCALA}-${PRODUCT}.tgz | tar -xzC . && \
    ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka

RUN curl -L https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

RUN curl -L https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER}-all.jar \
    -o /stackable/kafka/libs/opa-authorizer-${OPA_AUTHORIZER}-all.jar

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
RUN /bin/log4shell.sh /stackable/kafka_${SCALA}-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/kafka_${SCALA}-${PRODUCT}
# ===

COPY --from=builder /stackable/kcat-1.7.0/kcat /stackable/kcat

WORKDIR /stackable/kafka
CMD ["bin/kafka-server-start.sh", "/stackable/kafka/config/server.properties"]