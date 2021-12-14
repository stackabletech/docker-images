#
# We use the official Superset image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.
#
# 2021.11.02: The image tag below has the same digest as the 'latest' tag at the
#             time of writing this Dockerfile.
#
FROM apache/superset:9515ba68dc560307758774d1618c885e379d2011
LABEL maintainer="Stackable GmbH"

ARG PRODUCT

COPY superset/stackable/superset_config.py /app/pythonpath/
