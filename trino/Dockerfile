# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS storage-connector-builder

ARG STORAGE_CONNECTOR
ARG STORAGE_CONNECTOR_GIT_REV

WORKDIR /stackable

RUN git clone https://github.com/snowlift/trino-storage --branch=v${STORAGE_CONNECTOR} --depth=1 && \
    cd trino-storage && git checkout ${STORAGE_CONNECTOR_GIT_REV}
RUN --mount=type=cache,target=/root/.m2/repository cd trino-storage && \
    # Upstream builds are marked as -SNAPSHOT, even for release builds
    mvn versions:set -DnewVersion=${STORAGE_CONNECTOR} && \
    mvn package -DskipTests

# # TODO: remove the OPA authorizer builder once Trino versions 414 and 428 are removed
# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS opa-authorizer-builder

ARG OPA_AUTHORIZER
ARG OPA_AUTHORIZER_GIT_REV

WORKDIR /stackable

RUN if [[ -n "${OPA_AUTHORIZER}" ]]; then \
        # Need to use deep clone because trino-opa-authorizer doesn't publish tags
        git clone https://github.com/stackabletech/trino-opa-authorizer && \
        cd trino-opa-authorizer && git checkout ${OPA_AUTHORIZER_GIT_REV}; \
    fi
RUN --mount=type=cache,target=/root/.m2/repository if [[ -n "${OPA_AUTHORIZER}" ]]; then \
        cd trino-opa-authorizer && \
        mvn package -DskipTests; \
    fi

# Create an empty dummy plugin folder for newer versions that don't need the plugin anymore
RUN mkdir -p trino-opa-authorizer/target/out

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS builder

ARG PRODUCT
ARG PRODUCT_GIT_REV
# External OPA authorizer only required for 414 and 428, included in 438
# The `OPA_AUTHORIZER` is not set in the conf.py for version 438 to remove the ARG as soon as 414 and 428 are removed
ARG OPA_AUTHORIZER
ARG STORAGE_CONNECTOR

WORKDIR /stackable

RUN git clone https://github.com/trinodb/trino --branch=${PRODUCT} --depth=1 && \
    cd trino && git checkout ${PRODUCT_GIT_REV}
RUN --mount=type=cache,target=/root/.m2/repository cd trino && \
    mvn package -DskipTests --projects="!docs"
RUN tar -xzf /stackable/trino/core/trino-server/target/trino-server-${PRODUCT}.tar.gz -C /stackable && \
    chown --recursive stackable /stackable/trino-server-${PRODUCT}

COPY --from=storage-connector-builder /stackable/trino-storage/target/trino-storage-${STORAGE_CONNECTOR} /stackable/trino-server-${PRODUCT}/plugin
# # TODO: remove the following COPY statement once Trino versions 414 and 428 are removed
COPY --from=opa-authorizer-builder /stackable/trino-opa-authorizer/target/out /stackable/trino-server/plugin/trino-opa-authorizer-${PRODUCT}-${OPA_AUTHORIZER}

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

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS jmx-exporter-builder

ARG JMX_EXPORTER
ARG JMX_EXPORTER_GIT_REV

WORKDIR /stackable

RUN git clone https://github.com/prometheus/jmx_exporter --branch=${JMX_EXPORTER} --depth=1 && \
    cd jmx_exporter && git checkout ${JMX_EXPORTER_GIT_REV}
RUN --mount=type=cache,target=/root/.m2/repository cd jmx_exporter && \
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
RUN ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server

COPY --from=jmx-exporter-builder /stackable/jmx_exporter/jmx_prometheus_javaagent/target/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar
RUN ln -s /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar /stackable/jmx/jmx_prometheus_javaagent.jar

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
