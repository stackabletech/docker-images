#
# We use the official Airflow image here instead of building our own from scratch
# to avoid having to deal with python dependencies and Nexus integration. We might
# change this in the future.
#
# 2022.01.20: Some components (e.g. airflow triggerer) require python 3.7+
#
ARG PRODUCT
ARG PYTHON
ARG RELEASE="1"

FROM apache/airflow:$PRODUCT-python$PYTHON

LABEL name="Apache Airflow" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Airflow." \
      description="This image is deployed by the Stackable Operator for Apache Airflow."

COPY airflow/licenses /licenses
