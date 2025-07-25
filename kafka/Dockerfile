# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/kafka/kcat AS kcat
FROM stackable/image/kafka/kafka-opa-plugin AS kafka-opa-plugin

FROM stackable/image/java-devel AS kafka-builder

ARG PRODUCT
ARG RELEASE
ARG SCALA
ARG JMX_EXPORTER
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/jmx/ /stackable/jmx/
COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/patches/patchable.toml /stackable/src/kafka/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 kafka/stackable/patches/${PRODUCT} /stackable/src/kafka/stackable/patches/${PRODUCT}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout kafka ${PRODUCT})"

NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

# The comment in gradle.properties mentions to update the version in all of these files
find . -type f -print0 | xargs -0 sed -i "s/\-stackable0\.0\.0\-dev/-stackable${RELEASE}/g"

# Create snapshot of the source code including custom patches
tar -czf /stackable/kafka-${NEW_VERSION}-src.tar.gz .

# TODO: Try to install gradle via package manager (if possible) instead of fetching it from the internet
# We patch Kafka to use our Nexus build repo instead
# We don't specify "-x test" to skip the tests, as we might bump some Kafka internal dependencies in the future and
# it's a good idea to run the tests in this case.
./gradlew clean releaseTarGz
./gradlew cyclonedxBom
tar -xf core/build/distributions/kafka_${SCALA}-${NEW_VERSION}.tgz -C /stackable
cp build/reports/bom.json /stackable/kafka_${SCALA}-${NEW_VERSION}.cdx.json
rm -rf /stackable/kafka_${SCALA}-${NEW_VERSION}/site-docs/
(cd .. && rm -rf ${PRODUCT})

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
ARG KAFKA_KAFKA_OPA_PLUGIN
ARG STACKABLE_USER_UID

LABEL \
  name="Apache Kafka" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT}" \
  release="${RELEASE}" \
  summary="The Stackable image for Apache Kafka." \
  description="This image is deployed by the Stackable Operator for Apache Kafka."

COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE} /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}.cdx.json /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}.cdx.json
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/kafka-${PRODUCT}-stackable${RELEASE}-src.tar.gz  /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-builder /stackable/jmx/ /stackable/jmx/
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat /stackable/bin/kcat-${KAFKA_KCAT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat-${KAFKA_KCAT}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /licenses /licenses
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-opa-plugin /stackable/src/kafka/kafka-opa-plugin/patchable-work/worktree/${KAFKA_KAFKA_OPA_PLUGIN}/build/libs/opa-authorizer-${KAFKA_KAFKA_OPA_PLUGIN}-all.jar /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}/libs/opa-authorizer-${KAFKA_KAFKA_OPA_PLUGIN}-all.jar
COPY --chown=${STACKABLE_USER_UID}:0 --from=kafka-opa-plugin /stackable/kafka-opa-plugin-${KAFKA_KAFKA_OPA_PLUGIN}-src.tar.gz /stackable

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
ln -s /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE} /stackable/kafka
chown -h ${STACKABLE_USER_UID}:0 /stackable/kafka

# fix missing permissions
chmod g=u /stackable/bin
chmod g=u /stackable/jmx
chmod g=u /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}
chmod g=u /stackable/kafka_${SCALA}-${PRODUCT}-stackable${RELEASE}/libs/opa-authorizer-${KAFKA_KAFKA_OPA_PLUGIN}-all.jar
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
