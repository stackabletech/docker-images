FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5
LABEL maintainer="Stackable GmbH"

ARG PRODUCT
ARG PYTHON

RUN microdnf update \
        && microdnf install \
            gcc \
            gcc-c++ \
            libffi-devel \
            #python3-devel \
            python3 \
            python3-pip \
            #python3-wheel \
            openssl-devel \
            cyrus-sasl-devel \
            openldap-devel \
        && microdnf clean all \
        && python3 -m venv --system-site-packages app \
        && source app/bin/activate \
        && pip install --upgrade pip \
        && pip install apache-airflow==2.1.4
            #--constraint "https://raw.githubusercontent.com/apache/airflow/constraints-2.1.4/constraints-3.7.txt"
