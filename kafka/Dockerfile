# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL4006

FROM stackable/image/java-base AS builder

ARG JMX_EXPORTER
ARG KCAT
ARG OPA_AUTHORIZER
ARG PRODUCT
ARG SCALA

RUN microdnf install -y \
    cmake \
    cyrus-sasl-devel \
    gcc-c++ \
    libcurl-devel \
    make \
    openssl-devel \
    tar \
    wget \
    which \
    # For the Log4shell scanner
    zip \
    zlib \
    zlib-devel && \
    microdnf clean all

WORKDIR /stackable
RUN curl --fail -L -O https://repo.stackable.tech/repository/packages/kcat/kcat-${KCAT}.tar.gz \
    && tar xvfz kcat-${KCAT}.tar.gz \
    && cd kcat-${KCAT} \
    && ./bootstrap.sh

RUN curl --fail -L https://repo.stackable.tech/repository/packages/kafka/kafka_${SCALA}-${PRODUCT}.tgz | tar -xzC . && \
    ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka

COPY --chown=stackable:stackable kafka/stackable /stackable
RUN curl --fail -L https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

RUN curl --fail -L https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER}-all.jar \
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


FROM stackable/image/java-base

ARG KCAT
ARG PRODUCT
ARG RELEASE
ARG SCALA

LABEL name="Apache Kafka" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Kafka." \
      description="This image is deployed by the Stackable Operator for Apache Kafka."

# This is needed for kubectl
COPY kafka/kubernetes.repo /etc/yum.repos.d/kubernetes.repo
RUN microdnf update && \
    microdnf install \
    # Can be removed once listener-operator integration is used
    kubectl && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable


COPY --chown=stackable:stackable kafka/licenses /licenses

COPY --link --chown=stackable:stackable --from=builder /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka_${SCALA}-${PRODUCT}
COPY --link --chown=stackable:stackable --from=builder /stackable/jmx /stackable/jmx/
COPY --link --chown=stackable:stackable --from=builder /stackable/kcat-${KCAT}/kcat /stackable/kcat-${KCAT}

RUN ln -s /stackable/kcat-${KCAT} /stackable/kcat && \
    ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka

ENV PATH="${PATH}":/stackable/bin:/stackable/kafka/bin

WORKDIR /stackable/kafka
CMD ["bin/kafka-server-start.sh", "/stackable/kafka/config/server.properties"]
