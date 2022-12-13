FROM docker.stackable.tech/stackable/java-base:11-stackable0.2.2@sha256:7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e

ARG PRODUCT
ARG RELEASE="1"

LABEL name="Apache NiFi" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache NiFi." \
      description="This image is deployed by the Stackable Operator for Apache NiFi."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip openssl libxslt-devel libxml2-devel gcc shadow-utils && \
    microdnf install python3-devel python3-pip python3-setuptools && \
    microdnf clean all

RUN pip3 install --no-cache-dir nipyapi==0.19.1
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable

USER stackable
WORKDIR /stackable

# The bcrypt tool is needed by NiFi to locally encrypt the admin password that is mounted as a secret in cleartext
COPY --chown=stackable:stackable nifi/bin/stackable-bcrypt-1.0-SNAPSHOT-jar-with-dependencies.jar /bin/stackable-bcrypt.jar
COPY --chown=stackable:stackable nifi/stackable /stackable
COPY --chown=stackable:stackable nifi/licenses /licenses
COPY --chown=stackable:stackable nifi/python /stackable/python

RUN curl -L https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/nifi-${PRODUCT} /stackable/nifi

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
RUN /bin/log4shell.sh /stackable/nifi-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/nifi-${PRODUCT}
# ===

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
