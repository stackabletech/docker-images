#
# We use the official Airflow image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.
#
# 2022.01.20: Some components (e.g. airflow triggerer) require python 3.7+
#
FROM apache/airflow:2.2.3-python3.8
LABEL maintainer="Stackable GmbH"

ARG PRODUCT

COPY airflow/stackable/airflow_config.py /app/pythonpath/
