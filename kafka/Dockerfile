# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/kafka/kcat AS kcat

FROM stackable/image/java-devel AS kafka-builder

ARG PRODUCT
ARG SCALA
ARG OPA_AUTHORIZER
ARG JMX_EXPORTER
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/jmx/ /stackable/jmx/
COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/patches/patchable.toml /stackable/src/kafka/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/patches/${PRODUCT} /stackable/src/kafka/stackable/patches/${PRODUCT}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout kafka ${PRODUCT})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/kafka-${PRODUCT}-src.tar.gz .

# TODO: Try to install gradle via package manager (if possible) instead of fetching it from the internet
# We don't specify "-x test" to skip the tests, as we might bump some Kafka internal dependencies in the future and
# it's a good idea to run the tests in this case.
./gradlew clean releaseTarGz
./gradlew cyclonedxBom
tar -xf core/build/distributions/kafka_${SCALA}-${PRODUCT}.tgz -C /stackable
cp build/reports/bom.json /stackable/kafka_${SCALA}-${PRODUCT}.cdx.json
rm -rf /stackable/kafka_${SCALA}-${PRODUCT}/site-docs/
(cd .. && rm -rf ${PRODUCT})

# TODO (@NickLarsenNZ): Compile from source: https://github.com/StyraInc/opa-kafka-plugin
curl https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER}-all.jar \
  -o /stackable/kafka_${SCALA}-${PRODUCT}/libs/opa-authorizer-${OPA_AUTHORIZER}-all.jar

# JMX exporter
curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
  -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
chmod +x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

# change groups
chmod -R g=u /stackable
EOF

FROM stackable/image/java-base AS final

ARG RELEASE
ARG PRODUCT
ARG SCALA
ARG KAFKA_KCAT
ARG STACKABLE_USER_UID

LABEL \
  name="Apache Kafka" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT}" \
  release="${RELEASE}" \
  summary="The Stackable image for Apache Kafka." \
  description="This image is deployed by the Stackable Operator for Apache Kafka."

COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka_${SCALA}-${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT}.cdx.json /stackable/kafka_${SCALA}-${PRODUCT}/kafka_${SCALA}-${PRODUCT}.cdx.json
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka-${PRODUCT}-src.tar.gz  /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/jmx/ /stackable/jmx/
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat /stackable/bin/kcat-${KAFKA_KCAT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat-${KAFKA_KCAT}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 kafka/licenses /licenses

WORKDIR /stackable

RUN <<EOF
microdnf update
# cyrus-sasl-gssapi: needed by kcat for kerberos
microdnf install \
  cyrus-sasl-gssapi

microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

ln -s /stackable/bin/kcat-${KAFKA_KCAT} /stackable/bin/kcat
chown -h ${STACKABLE_USER_UID}:0 /stackable/bin/kcat
# kcat was located in /stackable/kcat - legacy
ln -s /stackable/bin/kcat /stackable/kcat
chown -h ${STACKABLE_USER_UID}:0 /stackable/kcat
ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka
chown -h ${STACKABLE_USER_UID}:0 /stackable/kafka

# fix missing permissions
chmod g=u /stackable/bin
chmod g=u /stackable/jmx
chmod g=u /stackable/kafka_${SCALA}-${PRODUCT}
chmod g=u /stackable/*-src.tar.gz
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

USER ${STACKABLE_USER_UID}

ENV PATH="${PATH}:/stackable/bin:/stackable/kafka/bin"

WORKDIR /stackable/kafka
CMD ["kafka-server-start.sh", "/stackable/kafka/config/server.properties"]
