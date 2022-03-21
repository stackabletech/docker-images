#
# We use the official Airflow image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.
#
# 2022.01.20: Some components (e.g. airflow triggerer) require python 3.7+
#
ARG PRODUCT=2.2.4
ARG PYTHON=3.9

FROM apache/airflow:$PRODUCT-python$PYTHON
LABEL maintainer="Stackable GmbH"


