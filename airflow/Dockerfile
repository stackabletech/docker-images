FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5
LABEL maintainer="Stackable GmbH"

ARG PRODUCT
ARG PYTHON

RUN microdnf update \
        && microdnf install \
            python39 \
            python39-pip \
            python39-wheel \
            #openssl-devel \
            #cyrus-sasl-devel \
            #openldap-devel \
        && microdnf clean all \
        && python3 -m venv --system-site-packages app \
        && source app/bin/activate \
        && pip install --upgrade pip \
        && pip install apache-airflow[async,amazon,celery,cncf.kubernetes,docker,dask,elasticsearch,ftp,grpc,hashicorp,http,google,microsoft.azure,postgres,redis,sendgrid,sftp,slack,ssh,statsd,virtualenv]==${PRODUCT} \
            --constraint "https://raw.githubusercontent.com/apache/airflow/constraints-2.2.5-fix/constraints-3.9.txt"


CMD ["/app/bin/airflow", "version"]

# 2.2.3/3.8 (**success**)
#   removed: mysql,ldap
#   constraints-2-2-3-fixed/constraints-3.8.txt

# 2.2.4/3.9
# no specific contraints fle for 2.2.4.....

# 2.2.5/3.9 (**success**)
#   removed: mysql,ldap
#   constraints-2-2/constraints-3.9.txt
#   or
#   constraints-2.2.5-fix/constraints-3.9.txt
