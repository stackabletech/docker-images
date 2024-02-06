# syntax=docker/dockerfile:1
FROM stackable/image/java-base AS builder

ARG PRODUCT

RUN microdnf update && \
    microdnf install \
    # Required to build Omid \
    maven \
    # Required to unpack Omid tarball
    tar \
    git \
    # Required by log4shell mitigation script
    zip && \
    microdnf clean all

USER stackable
COPY omid/stackable /stackable
WORKDIR /stackable


SHELL ["bash", "-euo", "pipefail", "-c"]

RUN curl --fail -L https://github.com/apache/phoenix-omid/archive/refs/tags/${PRODUCT}.tar.gz | tar -xzC .

# The entire point of this patchset is to add exclusions for the htrace dependency to the omid pom.
# It also adds a top level dependency on the htrace noop implementation created in
# https://issues.apache.org/jira/browse/HBASE-24802
# This doesn't remove all occurrences of htrace, as it is also contained in a shaded hbase fat jar.
# This is removed by rebuilding parts of HBase further down and replacing the jar.
#
# We also rename the maven version to 1.1.1RC1 in the pom, because otherwise our build chain breaks, when the created
# artifacts don't match the version ( 1.1.1 vs 1.1.1RC1 ).
# This part of the patch needs to be removed when moving on to a 1.1.1 release.
RUN patches/omid/apply_patches.sh ${PRODUCT}

WORKDIR /stackable/phoenix-omid-${PRODUCT}

RUN mvn package -Phbase-2 -DskipTests && \
    tar -xf tso-server/target/omid-tso-server-${PRODUCT}-bin.tar.gz -C /stackable

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
RUN /bin/log4shell.sh /stackable/omid-tso-server-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/omid-tso-server-${PRODUCT}
# ===


# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base AS hbasebuilder

ARG PRODUCT
ARG HBASECLIENT
ARG HADOOP_M2

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# zip required for the log4shell stuff
RUN microdnf update && \
    microdnf install \
    # The GNU gettext utilities contain the envsubst program which
    # substitutes the values of environment variables.
    gettext \
    git \
    gzip \
    maven \
    tar \
    zip && \
    microdnf clean all


COPY hbase/licenses /licenses
COPY --chown=stackable:stackable omid/stackable /stackable

USER stackable
WORKDIR /stackable

# We do not need to build the entirety of HBase, ideally we'd like to not build anything at all here, but the jar for
# the hbase-protocol-shaded maven project also contains the htrace dependencies that we'd like to remove.
# So we patch the hbase-protocol-shaded pom to state that htrace is "<provided>" to exclude it from the jar and then
# build this library and replace it in the final docker image
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/hbase/hbase-${HBASECLIENT}-src.tar.gz" | tar -xzC .
RUN mv hbase-${HBASECLIENT} hbase-${HBASECLIENT}-src
RUN chmod +x patches/hbase/apply_patches.sh
RUN patches/hbase/apply_patches.sh ${HBASECLIENT}
RUN cd /stackable/hbase-${HBASECLIENT}-src/hbase-protocol-shaded && \
    mvn -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP_M2} clean install -DskipTests

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG HBASECLIENT
ARG JMX_EXPORTER

LABEL name="Apache Phoenix Omid" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Phoenix Omid." \
      description="This image is deployed by the Stackable Operator for Apache HBase."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && microdnf clean all

ENV HOME=/stackable

COPY --chown=stackable:stackable omid/stackable /stackable
COPY omid/licenses /licenses

USER stackable
WORKDIR /stackable

RUN ln -s /stackable/omid-tso-server-${PRODUCT} /stackable/omid-tso-server && \
    curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar


#COPY omid/stackable/hbase-noop-htrace-4.1.5.jar /stackable/omid-tso-server-${PRODUCT}/lib/
COPY --from=builder /stackable/omid-tso-server-${PRODUCT} /stackable/omid-tso-server-${PRODUCT}
RUN rm /stackable/omid-tso-server-${PRODUCT}/lib/hbase-protocol-shaded-2.4.17.jar
COPY --from=hbasebuilder /stackable/hbase-${HBASECLIENT}-src/hbase-protocol-shaded/target/hbase-protocol-shaded-2.4.17.jar /stackable/omid-tso-server-${PRODUCT}/lib/

ENV HBASECLIENT_CONF_DIR=/stackable/hbase/conf
# omid.sh places this file at the front of the classpath: remove it to allow the config map entry to take precedence
RUN rm /stackable/omid-tso-server/conf/hbase-site.xml
WORKDIR /stackable/omid-tso-server
CMD ["./bin/omid.sh", "tso" ]
