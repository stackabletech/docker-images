# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Apache NiFi" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache NiFi." \
      description="This image is deployed by the Stackable Operator for Apache NiFi."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip unzip libxslt-devel libxml2-devel gcc && \
    microdnf install python3-devel python3-pip python3-setuptools && \
    microdnf clean all

RUN pip3 install --no-cache-dir nipyapi==0.19.1

USER stackable
WORKDIR /stackable

# The bcrypt tool is needed by NiFi to locally encrypt the admin password that is mounted as a secret in cleartext
COPY --chown=stackable:stackable nifi/bin/stackable-bcrypt-1.0-SNAPSHOT-jar-with-dependencies.jar /bin/stackable-bcrypt.jar
COPY --chown=stackable:stackable nifi/stackable /stackable
COPY --chown=stackable:stackable nifi/licenses /licenses
COPY --chown=stackable:stackable nifi/python /stackable/python

# zip is different than tar and cannot be just piped, therefore the intermediate save and remove step to unzip
RUN curl --fail -L https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-bin.zip -o /stackable/nifi-${PRODUCT}-bin.zip && \
    unzip /stackable/nifi-${PRODUCT}-bin.zip && \
    rm /stackable/nifi-${PRODUCT}-bin.zip && \
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
