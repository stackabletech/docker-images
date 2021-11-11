FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

RUN microdnf update \
    && microdnf install --nodocs java-11-openjdk-headless tar gzip which shadow-utils \
    && microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

ADD stackable /stackable
RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT_VERSION
ARG HADOOP_VERSION=2.10.1

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT_VERSION}-bin.tar.gz | tar -xzC .

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

RUN curl -L https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-s3/1.12.106/aws-java-sdk-s3-1.12.106.jar \
    -o /stackable/hive/lib/aws-java-sdk-s3-1.12.106.jar && \
    chmod -x lib/aws-java-sdk-s3-1.12.106.jar

RUN curl -L https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-core/1.12.106/aws-java-sdk-core-1.12.106.jar \
    -o /stackable/hive/lib/aws-java-sdk-core-1.12.106.jar && \
    chmod -x lib/aws-java-sdk-core-1.12.106.jar

RUN mkdir /stackable/hive && \
    mv /stackable/apache-hive-${PRODUCT_VERSION}-bin/apache-hive-${PRODUCT_VERSION}-bin/* /stackable/hive && \
    mv /stackable/apache-hive-${PRODUCT_VERSION}-bin/hadoop-${HADOOP_VERSION}/* /stackable/hadoop && \
    rm -rf /stackable/apache-hive-${PRODUCT_VERSION}-bin/ && }

COPY /stackable/bin/start-metastore /stackable/hive/bin

ENV HADOOP_HOME=/stackable/hadoop

WORKDIR /stackable/hive
CMD ["./bin/start-metastore", "--config", "conf", "--hive-bin-dir", "bin", "--db-type", "derby"]
