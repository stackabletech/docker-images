# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS storage-connector-builder

ARG STORAGE_CONNECTOR

WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/trino-storage/trino-storage-${STORAGE_CONNECTOR}-src.tar.gz" | tar -xzC .
RUN --mount=type=cache,target=/root/.m2/repository cd trino-storage-${STORAGE_CONNECTOR}-src && \
    # Upstream builds are marked as -SNAPSHOT, even for release builds
    mvn versions:set -DnewVersion=${STORAGE_CONNECTOR} && \
    mvn package -DskipTests -Dmaven.gitcommitid.skip=true

# # TODO: remove the OPA authorizer builder once Trino versions 414 and 428 are removed
# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS opa-authorizer-builder

ARG PRODUCT
ARG OPA_AUTHORIZER

WORKDIR /stackable

RUN if [ -n "${OPA_AUTHORIZER}" ]; then \
        curl --fail -L "https://repo.stackable.tech/repository/packages/trino-opa-authorizer/trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}-src.tar.gz" | tar -xzC .; \
    fi
RUN --mount=type=cache,target=/root/.m2/repository if [ -n "${OPA_AUTHORIZER}" ]; then \
        cd trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}-src && \
        mvn package -DskipTests; \
    fi

# Create an empty dummy plugin folder for newer versions that don't need the plugin anymore
RUN mkdir -p trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}-src/target/out

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS builder

ARG PRODUCT
# External OPA authorizer only required for 414 and 428, included in 438
# The `OPA_AUTHORIZER` is not set in the conf.py for version 438 to remove the ARG as soon as 414 and 428 are removed
ARG OPA_AUTHORIZER
ARG STORAGE_CONNECTOR

WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}-src.tar.gz" | tar -xzC .
RUN --mount=type=cache,target=/root/.m2/repository cd "trino-server-${PRODUCT}-src" && \
    mvn package -DskipTests -Dmaven.gitcommitid.skip=true --projects="!docs,!core/trino-server-rpm" && \
    # Delete intermediate build products to free some space
    rm -r /stackable/trino-server-${PRODUCT}-src/plugin/*/target /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/trino-server-${PRODUCT}.tar.gz && \
    mv /stackable/trino-server-${PRODUCT}-src/core/trino-server/target/trino-server-${PRODUCT} /stackable/trino-server-${PRODUCT} && \
    chown --recursive stackable /stackable/trino-server-${PRODUCT}

COPY --from=storage-connector-builder /stackable/trino-storage-${STORAGE_CONNECTOR}-src/target/trino-storage-${STORAGE_CONNECTOR} /stackable/trino-server-${PRODUCT}/plugin/trino-storage-${STORAGE_CONNECTOR}
# # TODO: remove the following COPY statement once Trino versions 414 and 428 are removed
COPY --from=opa-authorizer-builder /stackable/trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}-src/target/out /stackable/trino-server-${PRODUCT}/plugin/trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}
# We have no way to copy a folder conditionally, so delete the opa authorizer folder if no version is selected
# Otherwise Trino will crash since empty plugin folders are not allowed
RUN [ -n "${OPA_AUTHORIZER}" ] || rmdir /stackable/trino-server-${PRODUCT}/plugin/trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}

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

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS jmx-exporter-builder

ARG JMX_EXPORTER

WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus-${JMX_EXPORTER}-src.tar.gz" | tar -xzC .
RUN --mount=type=cache,target=/root/.m2/repository cd jmx_prometheus-${JMX_EXPORTER}-src && \
    mvn package

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
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

# ===
# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
