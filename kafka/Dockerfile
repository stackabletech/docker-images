# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/kcat AS kcat

FROM stackable/image/java-devel AS kafka-builder

ARG PRODUCT
ARG SCALA
ARG OPA_AUTHORIZER
ARG JMX_EXPORTER
ARG STACKABLE_USER_UID

RUN <<EOF
microdnf update

# patch: Required for the apply-patches.sh script
microdnf install \
patch

microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/patches/apply_patches.sh /stackable/kafka-${PRODUCT}-src/patches/apply_patches.sh
COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/patches/${PRODUCT} /stackable/kafka-${PRODUCT}-src/patches/${PRODUCT}

RUN curl "https://repo.stackable.tech/repository/packages/kafka/kafka-${PRODUCT}-src.tgz" | tar -xzC . && \
    cd kafka-${PRODUCT}-src && \
    ./patches/apply_patches.sh ${PRODUCT} && \
    # TODO: Try to install gradle via package manager (if possible) instead of fetching it from the internet
    # We don't specify "-x test" to skip the tests, as we might bump some Kafka internal dependencies in the future and
    # it's a good idea to run the tests in this case.
    ./gradlew clean releaseTarGz && \
    ./gradlew cyclonedxBom && \
    tar -xf core/build/distributions/kafka_${SCALA}-${PRODUCT}.tgz -C /stackable && \
    cp build/reports/bom.json /stackable/kafka_${SCALA}-${PRODUCT}.cdx.json && \
    rm -rf /stackable/kafka_${SCALA}-${PRODUCT}/site-docs/ && \
    rm -rf /stackable/kafka-${PRODUCT}-src

# TODO (@NickLarsenNZ): Compile from source: https://github.com/StyraInc/opa-kafka-plugin
RUN curl https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER}-all.jar \
    -o /stackable/kafka_${SCALA}-${PRODUCT}/libs/opa-authorizer-${OPA_AUTHORIZER}-all.jar

COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/jmx/ /stackable/jmx/
RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar


FROM stackable/image/java-base AS final

ARG RELEASE
ARG PRODUCT
ARG SCALA
ARG KCAT
ARG STACKABLE_USER_UID

LABEL name="Apache Kafka" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Kafka." \
      description="This image is deployed by the Stackable Operator for Apache Kafka."

COPY --chown=${STACKABLE_USER_UID}:0 kafka/licenses /licenses
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka_${SCALA}-${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT}.cdx.json /stackable/kafka_${SCALA}-${PRODUCT}/kafka_${SCALA}-${PRODUCT}.cdx.json
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/jmx/ /stackable/jmx/
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat-${KCAT}/kcat /stackable/bin/kcat-${KCAT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /licenses /licenses

WORKDIR /stackable

RUN <<EOF
microdnf update
# cyrus-sasl-gssapi: needed by kcat for kerberos
microdnf install \
  cyrus-sasl-gssapi

microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
rm -rf /var/cache/yum

ln -s /stackable/bin/kcat-${KCAT} /stackable/bin/kcat
# kcat was located in /stackable/kcat - legacy
ln -s /stackable/bin/kcat /stackable/kcat
ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka

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

USER ${STACKABLE_USER_UID}

ENV PATH="${PATH}:/stackable/bin:/stackable/kafka/bin"

WORKDIR /stackable/kafka
CMD ["kafka-server-start.sh", "/stackable/kafka/config/server.properties"]
