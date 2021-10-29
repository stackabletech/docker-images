FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# Download java, tar and gzip
RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    # spark scripts
    microdnf install procps && \
    microdnf install hostname && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable

WORKDIR /stackable
ADD stackable .

RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT_VERSION
ARG HADOOP_VERSION

RUN curl -L https://archive.apache.org/dist/spark/spark-${PRODUCT_VERSION}/spark-${PRODUCT_VERSION}-bin-hadoop${HADOOP_VERSION}.tgz | tar -xzC .

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/spark-${PRODUCT_VERSION}-bin-hadoop${HADOOP_VERSION}
CMD ["sbin/start-master.sh"]
