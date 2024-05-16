# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

####### ToDo's ########
# Build-step 
# - git clone
# - maven clean install (java-devel image I guess)
# - copy bin
# - test stuff

FROM stackable/image/java-devel as druid-builder

ARG PRODUCT
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER

USER stackable
WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-src.tar.gz" | tar -xzC . && \
    cd apache-druid-${PRODUCT}-src && \
    mvn clean package -DskipTests -Dmaven.javadoc.skip=true && \
    cp -r apache-druid-${PRODUCT}-src/target /stackable/druid && \
    rm -rf /stackable/apache-druid-${PRODUCT}-src

    #Removed flags -Drequire.fuse=true -Pdist,native

# Install the Prometheus emitter extension. This bundle contains the emitter and all jar dependencies. 
RUN curl --fail "https://repo.stackable.tech/repository/packages/druid/druid-prometheus-emitter-${PRODUCT}.tar.gz" | tar -xzC /stackable/druid/extensions && \
    # Install OPA authorizer extension.
    curl --fail "https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz" | tar -xzC /stackable/druid/extensions && \
    # Install jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging, and remove stax-ex.
    rm /stackable/druid/lib/stax-ex-*.jar && \
    curl --fail -L -o /stackable/druid/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
        "https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar" && \
    curl --fail -L -o /stackable/druid/lib/stax2-api-${STAX2_API}.jar \
        "https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar" && \
    curl --fail -L -o /stackable/druid/lib/woodstox-core-${WOODSTOX_CORE}.jar \
        "https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar"

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
RUN /bin/log4shell.sh "/stackable/hadoop-${PRODUCT}"

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s "/stackable/hadoop-${PRODUCT}"
# ===

## Final Image
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Apache Druid" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Druid." \
      description="This image is deployed by the Stackable Operator for Apache Druid."

RUN microdnf update && \
    # microdnf install \
    # findutils \
    # gzip \
    # tar \
    # zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --from=builder /stackable/druid /stackable/druid
COPY --chown=stackable:stackable druid/stackable /stackable
COPY --chown=stackable:stackable druid/licenses /licenses

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
