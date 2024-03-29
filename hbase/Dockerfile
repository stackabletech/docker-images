# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL4006

FROM stackable/image/java-base AS builder

ARG PRODUCT
ARG HBASE_THIRDPARTY
ARG HBASE_OPERATOR_TOOLS
ARG ASYNC_PROFILER
ARG PHOENIX
ARG JMX_EXPORTER
ARG HADOOP_M2
ARG TARGETARCH
ARG TARGETOS

# zip required for the log4shell stuff
RUN microdnf update && \
    microdnf install \
    # The GNU gettext utilities contain the envsubst program which
    # substitutes the values of environment variables.
    gettext \
    git \
    gzip \
    # diff is required by maven during the build of hbase \
    # Cannot run program "diff" (in directory "/stackable/hbase-2.4.12-src/hbase-shaded/hbase-shaded-check-invariants"
    diffutils \
    maven \
    tar \
    zip && \
    microdnf clean all

COPY --chown=stackable:stackable hbase/stackable /stackable
COPY hbase/licenses /licenses

USER stackable
WORKDIR /stackable

# The release scripts of HBase also run the build twice (three times in fact, once again to build the site which we skip here).
# I chose to replicate that exact behavior for consistency so please don't merge the two mvn runs into one unless you really know what you're doing!
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hbase/hbase-${PRODUCT}-src.tar.gz" | tar -xzC .
RUN mv hbase-${PRODUCT} hbase-${PRODUCT}-src
RUN chmod +x patches/apply_patches.sh
RUN patches/apply_patches.sh ${PRODUCT}
RUN cd /stackable/hbase-${PRODUCT}-src/ && \
    mvn -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP_M2} clean install -DskipTests && \
    mvn -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP_M2} install assembly:single -DskipTests -Dcheckstyle.skip=true -Prelease && \
    tar -xzf hbase-assembly/target/hbase-${PRODUCT}-bin.tar.gz -C /stackable/ && \
    rm -rf /stackable/hbase-${PRODUCT}-src

# Build the hbase-operator-tools
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hbase-operator-tools/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src.tar.gz" | tar -xzC . && \
    mv hbase-operator-tools-${HBASE_OPERATOR_TOOLS} hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src && \
    mvn \
        -Dhbase.version=${PRODUCT} \
        -Dhbase-thirdparty.version=${HBASE_THIRDPARTY} \
        -DskipTests \
        -fhbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src \
        package assembly:single && \
    tar -xz \
        -f hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src/hbase-operator-tools-assembly/target/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-bin.tar.gz \
        -C /stackable/ && \
    rm -rf /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src

# Resolve paths in bin/hbck2
RUN mv bin/hbck2 bin/hbck2.env && \
    # The variable names are intentionally passed to envsubst in single-quotes,
    # so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
    # does not work, so please ignore the according warning (SC2016).
    envsubst '${PRODUCT}:${HBASE_OPERATOR_TOOLS}' < bin/hbck2.env > bin/hbck2 && \
    chmod +x bin/hbck2 && \
    rm bin/hbck2.env

# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HBase Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HBase operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HBase Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
RUN ln -s "/stackable/hbase-${PRODUCT}" /stackable/hbase && \
    curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
    -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar && \
    ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    curl --fail -L "https://repo.stackable.tech/repository/packages/phoenix/phoenix-hbase-${PHOENIX}-bin.tar.gz" | tar -xzC . && \
    ln -s "/stackable/phoenix-hbase-${PHOENIX}-bin" /stackable/phoenix && \
    ln -s "/stackable/phoenix/phoenix-server-hbase-${PHOENIX}.jar" "/stackable/hbase/lib/phoenix-server-hbase-${PHOENIX}.jar"

RUN ARCH="${TARGETARCH/amd64/x64}" && \
    curl --fail -L "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC . && \
    ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

ENV HBASE_CONF_DIR=/stackable/hbase/conf

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
RUN /bin/log4shell.sh /stackable/hbase-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/hbase-${PRODUCT}
# ===


# Final Image
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG HBASE_OPERATOR_TOOLS

LABEL name="Apache HBase" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache HBase." \
    description="This image is deployed by the Stackable Operator for Apache HBase."

# The tar and python packages are required by the Phoenix command line.
# We add zip and gzip because tar without compression is seldom useful.
RUN microdnf update && \
    microdnf install \
    tar \
    gzip \
    zip \
    python3 \
    python3-pip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

RUN ln -s /usr/bin/python3 /usr/bin/python && \
    ln -s /usr/bin/pip3 /usr/bin/pip

COPY --chown=stackable:stackable --from=builder /stackable/hbase-${PRODUCT} /stackable/hbase-${PRODUCT}/
COPY --chown=stackable:stackable --from=builder /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/
COPY --chown=stackable:stackable --from=builder /stackable/jmx /stackable/jmx/
COPY --chown=stackable:stackable --from=builder /stackable/phoenix /stackable/phoenix/
COPY --chown=stackable:stackable --from=builder /stackable/bin /stackable/bin/
COPY --chown=stackable:stackable --from=builder /stackable/async-profiler /stackable/async-profiler/
RUN ln -s /stackable/hbase-${PRODUCT} /stackable/hbase && \
    ln -s /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools

USER stackable
ENV HBASE_CONF_DIR=/stackable/hbase/conf
ENV HOME=/stackable
ENV PATH="${PATH}":/stackable/bin:/stackable/hbase/bin
ENV ASYNC_PROFILER_HOME=/stackable/async-profiler

WORKDIR /stackable/hbase
CMD ["./bin/hbase", "master", "start" ]
