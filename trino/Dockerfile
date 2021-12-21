FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip zip python3 && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

COPY trino/stackable /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

RUN alternatives --set python /usr/bin/python3

USER stackable

ARG PRODUCT

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}.tar.gz | tar -xzC . && \
    ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/trino-server/plugin
RUN curl -L https://repo.stackable.tech/repository/packages/trino-opa-authorizer/trino-opa-authorizer-0.1.0.tar.gz | tar -xzC .

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
COPY shared/log4shell_1.1.2-log4shell_Linux_x86_64 /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/trino-server-${PRODUCT}
# ===

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
