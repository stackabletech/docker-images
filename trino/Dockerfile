# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/java-devel AS storage-connector-builder

ARG STORAGE_CONNECTOR

WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/trino-storage/trino-storage-${STORAGE_CONNECTOR}-src.tar.gz" | tar -xzC .
RUN --mount=type=cache,target=/root/.m2/repository \
    cd trino-storage-${STORAGE_CONNECTOR}-src && \
    # Upstream builds are marked as -SNAPSHOT, even for release builds
    mvn versions:set -DnewVersion=${STORAGE_CONNECTOR} && \
    # We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
    ./mvnw package -DskipTests -Dmaven.gitcommitid.skip=true

FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG STORAGE_CONNECTOR

WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}-src.tar.gz" | tar -xzC .

RUN --mount=type=cache,target=/root/.m2/repository \
    cd "trino-server-${PRODUCT}-src" && \
    # We need to use ./mvnw instead of mvn to get a recent maven version (which is required to build Trino)
    ./mvnw package -DskipTests -Dmaven.gitcommitid.skip=true --projects="!docs,!core/trino-server-rpm" && \
    # Delete the worst intermediate build products to free some space
    rm -r /stackable/trino-server-${PRODUCT}-src/plugin/*/target /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/trino-server-${PRODUCT} && \
    # Extract from tarball to save space; the tarball deduplicates jars (replacing them with symlinks),
    # while the raw output folder does not
    tar -xzf /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/trino-server-${PRODUCT}.tar.gz -C /stackable && \
    chown --recursive stackable /stackable/trino-server-${PRODUCT} && \
    # Delete all intermediate build products to free some more space
    rm -r /stackable/trino-server-${PRODUCT}-src

COPY --from=storage-connector-builder /stackable/trino-storage-${STORAGE_CONNECTOR}-src/target/trino-storage-${STORAGE_CONNECTOR} /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${STORAGE_CONNECTOR}

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

FROM stackable/image/java-devel AS jmx-exporter-builder

ARG JMX_EXPORTER

WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus-${JMX_EXPORTER}-src.tar.gz" | tar -xzC .
RUN --mount=type=cache,target=/root/.m2/repository \
    cd jmx_prometheus-${JMX_EXPORTER}-src && \
    mvn package

FROM stackable/image/java-base

ARG PRODUCT
ARG JMX_EXPORTER
ARG RELEASE

LABEL name="Trino" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Trino." \
      description="This image is deployed by the Stackable Operator for Trino."

RUN microdnf update && \
    microdnf install \
    gzip \
    httpd-tools \
    python \
    tar \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable trino/stackable /stackable
COPY --chown=stackable:stackable trino/licenses /licenses

COPY --from=builder /stackable/trino-server-${PRODUCT} /stackable/trino-server-${PRODUCT}
COPY --from=jmx-exporter-builder /stackable/jmx_prometheus-${JMX_EXPORTER}-src/jmx_prometheus_javaagent/target/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
RUN ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
