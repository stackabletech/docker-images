# We use the official Superset image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.

ARG PRODUCT
ARG _BASE_IMAGE_TAG

FROM apache/superset:$_BASE_IMAGE_TAG
LABEL maintainer="Stackable GmbH"

USER root
RUN apt update &&\
    apt install -y python3-dev libldap2-dev libsasl2-dev libssl-dev &&\
    rm -rf /var/lib/apt/lists/*
USER superset

RUN pip install statsd python-ldap
