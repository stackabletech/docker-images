# We use the official Superset image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.

ARG PRODUCT
ARG _BASE_IMAGE_TAG

FROM apache/superset:$_BASE_IMAGE_TAG
LABEL maintainer="Stackable GmbH"

COPY superset/stackable/superset_config.py /app/pythonpath/
