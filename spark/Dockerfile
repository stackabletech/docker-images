FROM docker.stackable.tech/stackable/java-base:11-stackable0.2.2@sha256:7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e

ARG PRODUCT
ARG HADOOP
ARG RELEASE="1"

LABEL name="Apache Spark" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Spark standalone." \
      description="This image is deployed by the standalone Stackable Operator for Apache Spark."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip && \
    microdnf install shadow-utils && \
    # required for spark startup scripts
    microdnf install procps && \
    microdnf install hostname && \
    microdnf install python3 python3-pip && \
    microdnf clean all

RUN ln -s /usr/bin/python3 /usr/bin/python
RUN ln -s /usr/bin/pip3 /usr/bin/pip

ENV HOME=/stackable
ENV SPARK_HOME=/stackable/spark
ENV PATH=$SPARK_HOME:$PATH:/bin:$JAVA_HOME/bin:$JAVA_HOME/jre/bin:$HOME/.local/bin
ENV PYSPARK_PYTHON=/usr/bin/python
ENV PYTHONPATH=$SPARK_HOME/python:$PYTHONPATH

COPY spark/stackable /stackable
COPY spark/requirements-${PRODUCT}.txt /tmp/requirements.txt
COPY spark/licenses /licenses

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/spark/spark-${PRODUCT}-bin-hadoop${HADOOP}.tgz | tar -xzC . && \
    ln -s /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP} /stackable/spark

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar
    
# pyspark dependencies
RUN python -m pip install --user --upgrade pip
RUN python -m pip install --user --no-cache-dir -r /tmp/requirements.txt

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
RUN /bin/log4shell.sh /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP}
# ===

WORKDIR /stackable/spark
CMD ["sbin/start-master.sh"]
