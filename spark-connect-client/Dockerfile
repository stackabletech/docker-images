# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7

# spark-builder: provides client libs for spark-connect
FROM stackable/image/spark-k8s AS spark-builder

ARG PRODUCT
ARG PYTHON
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Stackable Spark Connect Examples" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="Spark Connect Examples" \
    description="Spark Connect client libraries for Python and the JVM, including some examples."

# Need root to install setuptools
USER root

COPY --chown=${STACKABLE_USER_UID}:0 spark-connect-client/stackable/spark-connect-examples /stackable/spark-connect-examples
COPY --chown=${STACKABLE_USER_UID}:0 spark-connect-client/stackable/.jupyter /stackable/.jupyter

RUN <<EOF
microdnf update
# python{version}-setuptools: needed to build the pyspark[connect] package
microdnf install --nodocs \
  "python${PYTHON}-setuptools"
microdnf clean all
rm -rf /var/cache/yum

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

USER ${STACKABLE_USER_UID}

# Install python packages.
# Packages are intentionally installed in "user mode" to reduce the container attack surface.
# - pyspark[connect] = spark connect client libs
# - jupyterlab = notebook client used in demos
RUN pip install --no-cache-dir --user \
  "pyspark[connect]==${PRODUCT}" \
  "jupyterlab==4.4.1" \
  "scikit-learn==1.3.1" \
  "matplotlib==3.10.1"

WORKDIR /stackable/spark-connect-examples/python
