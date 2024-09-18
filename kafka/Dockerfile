# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd
# check=error=true

FROM stackable/image/kcat AS kcat

FROM stackable/image/java-devel as kafka-builder

ARG PRODUCT
ARG SCALA
ARG OPA_AUTHORIZER
ARG JMX_EXPORTER

USER stackable
WORKDIR /stackable

RUN curl "https://repo.stackable.tech/repository/packages/kafka/kafka-${PRODUCT}-src.tgz" | tar -xzC . && \
    cd kafka-${PRODUCT}-src && \
    # TODO: Try to install gradle via package manager (if possible) instead of fetching it from the internet
    # We don't specify "-x test" to skip the tests, as we might bump some Kafka internal dependencies in the future and
    # it's a good idea to run the tests in this case.
    ./gradlew clean releaseTarGz && \
    tar -xf core/build/distributions/kafka_${SCALA}-${PRODUCT}.tgz -C /stackable && \
    rm -rf /stackable/kafka_${SCALA}-${PRODUCT}/site-docs/ && \
    rm -rf /stackable/kafka-${PRODUCT}-src

# TODO (@NickLarsenNZ): Compile from source: https://github.com/StyraInc/opa-kafka-plugin
RUN curl https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER}-all.jar \
    -o /stackable/kafka_${SCALA}-${PRODUCT}/libs/opa-authorizer-${OPA_AUTHORIZER}-all.jar

COPY --chown=stackable:stackable kafka/stackable/jmx/ /stackable/jmx/
RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

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

FROM stackable/image/java-base AS final

ARG RELEASE
ARG PRODUCT
ARG SCALA
ARG KCAT

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
    rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable kafka/licenses /licenses

# We copy opa-authorizer.jar and jmx-exporter through the builder image to have an absolutely minimal final image
# (e.g. we don't even need curl in it).
COPY --chown=stackable:stackable --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka_${SCALA}-${PRODUCT}
COPY --chown=stackable:stackable --from=kafka-builder /stackable/jmx/ /stackable/jmx/
COPY --chown=stackable:stackable --from=kcat /stackable/kcat-${KCAT}/kcat /stackable/bin/kcat-${KCAT}
COPY --chown=stackable:stackable --from=kcat /licenses /licenses

RUN ln -s /stackable/bin/kcat-${KCAT} /stackable/bin/kcat && \
    # kcat was located in /stackable/kcat - legacy
    ln -s /stackable/bin/kcat /stackable/kcat && \
    ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka

ENV PATH="${PATH}:/stackable/bin:/stackable/kafka/bin"

WORKDIR /stackable/kafka
CMD ["kafka-server-start.sh", "/stackable/kafka/config/server.properties"]
