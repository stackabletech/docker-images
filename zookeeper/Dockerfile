# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL4006

FROM stackable/image/java-base AS builder

RUN microdnf update && \
    microdnf clean all

ARG PRODUCT
ARG JMX_EXPORTER


RUN microdnf update && \
    microdnf install \
    # For the apply_patches.sh script
    git \
    gzip \
    maven \
    tar \
    zip \
    # hostname is needed by the maven ant run plugin for the "set-hostname-property" step
    hostname \
    # needed for c parts of zookeeper-client-c module
    gcc \
    make \
    autoconf && \
    microdnf clean all

COPY --chown=stackable:stackable zookeeper/stackable /stackable

USER stackable
WORKDIR /stackable

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/zookeeper/apache-zookeeper-${PRODUCT}.tar.gz" | tar -xzC . && \
    patches/apply_patches.sh ${PRODUCT} && \
    cd /stackable/apache-zookeeper-${PRODUCT}/ && \
    # Exclude the `zookeeper-client` submodule, this is not needed and has c parts that created all kinds of issues for the  \
    # build container
    mvn -X -pl "!zookeeper-client/zookeeper-client-c" clean install checkstyle:check spotbugs:check -DskipTests -Pfull-build && \
    mv zookeeper-assembly/target/apache-zookeeper-${PRODUCT}-bin.tar.gz /stackable && \
    rm -rf apache-zookeeper-${PRODUCT} && \
    tar xvzf /stackable/apache-zookeeper-${PRODUCT}-bin.tar.gz && \
    mv /stackable/apache-zookeeper-${PRODUCT}-bin /stackable/apache-zookeeper-${PRODUCT} && \
    ln -s /stackable/apache-zookeeper-${PRODUCT}/ /stackable/zookeeper

#COPY --chown=stackable:stackable --from=hadoop-builder /stackable/hadoop /stackable/hadoop

# The symlink from JMX Exporter 0.16.1 to the versionless link exists because old HDFS Operators (up until and including 23.7) used to hardcode
# the version of JMX Exporter like this: "-javaagent:/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar"
# This is a TEMPORARY fix which means that we can keep the hardcoded path in HDFS operator FOR NOW as it will still point to a newer version of JMX Exporter, despite the "0.16.1" in the name.
# At the same time a new HDFS Operator will still work with older images which do not have the symlink to the versionless jar.
# After one of our next releases (23.11 or 24.x) we should update the operator to point at the non-versioned symlink (jmx_prometheus_javaagent.jar)
# And then we can also remove the symlink to 0.16.1 from this Dockerfile.
#RUN curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
#    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar && \
#    ln -s /stackable/jmx/jmx_prometheus_javaagent.jar /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

# Logging
#RUN rm /stackable/hive-metastore/lib/log4j-slf4j-impl* && \
#    curl --fail -L https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar -o /stackable/hive-metastore/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar

# ===
# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
#COPY shared/log4shell.sh /bin
#RUN /bin/log4shell.sh /stackable/apache-hive-metastore-${PRODUCT}/

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
#COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
#COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
#COPY shared/log4shell_scanner /bin/log4shell_scanner
# log4shell_scanner does not work on symlinks!
#RUN /bin/log4shell_scanner s /stackable/apache-zookeeper-${PRODUCT}/
# ===


FROM stackable/image/java-base

ARG PRODUCT
ARG JMX_EXPORTER
ARG RELEASE

LABEL name="Apache ZooKeeper" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache ZooKeeper." \
      description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

RUN microdnf update && \
    microdnf install tar gzip zip && \
    microdnf clean all

COPY --chown=stackable:stackable zookeeper/stackable /stackable
COPY zookeeper/licenses /licenses

USER stackable
WORKDIR /stackable

#RUN curl --fail -L "https://repo.stackable.tech/repository/packages/zookeeper/apache-zookeeper-${PRODUCT}-bin.tar.gz" | tar -xzC . && \
#    ln -s "/stackable/apache-zookeeper-${PRODUCT}-bin" /stackable/zookeeper

# TODO: Try to use --link here, as it should be faster
COPY --chown=stackable:stackable --from=builder /stackable/apache-zookeeper-${PRODUCT} /stackable/
#RUN cd /stackable && \
#    tar xvzf /stackable/apache-zookeeper-${PRODUCT}-bin.tar.gz && \
#    mv /stackable/apache-zookeeper-${PRODUCT}-bin /stackable/apache-zookeeper-${PRODUCT} && \
#    ln -s /stackable/apache-zookeeper-${PRODUCT}/ /stackable/apache-zookeeper

RUN curl --fail "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" \
    -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" && \
    ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

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
RUN /bin/log4shell.sh /stackable/apache-zookeeper-${PRODUCT}-bin

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/apache-zookeeper-${PRODUCT}-bin
# ===

WORKDIR /stackable/zookeeper
CMD ["bin/zkServer.sh", "start-foreground", "conf/zoo_sample.cfg"]
