# We use the official Superset image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.

ARG PRODUCT
ARG _BASE_IMAGE_TAG
ARG RELEASE="1"

FROM apache/superset:$_BASE_IMAGE_TAG

LABEL name="Apache Superset" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Superset." \
      description="This image is deployed by the Stackable Operator for Apache Superset."

COPY superset/licenses /licenses

USER root
RUN apt update &&\
    apt install -y python3-dev libldap2-dev libsasl2-dev libssl-dev &&\
    rm -rf /var/lib/apt/lists/*
USER superset

RUN pip install statsd python-ldap
