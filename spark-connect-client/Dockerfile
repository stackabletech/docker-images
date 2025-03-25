# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5

# spark-builder: provides client libs for spark-connect
FROM stackable/image/spark-k8s AS spark-builder

FROM stackable/image/java-base

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


ENV HOME=/stackable

COPY spark-connect-client/stackable/spark-connect-examples /stackable/spark-connect-examples
COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-builder /stackable/spark/connect /stackable/spark/connect

RUN <<EOF
microdnf update
# python{version}-setuptools: needed to build the pyspark[connect] package
microdnf install --nodocs \
  "python${PYTHON}" \
  "python${PYTHON}-pip" \
  "python${PYTHON}-setuptools"
microdnf clean all
rm -rf /var/cache/yum

ln -s /usr/bin/python${PYTHON} /usr/bin/python
ln -s /usr/bin/pip-${PYTHON} /usr/bin/pip

# Install python libraries for the spark connect client
# shellcheck disable=SC2102
pip install --no-cache-dir pyspark[connect]==${PRODUCT}

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

# ----------------------------------------
# Attention: We are changing the group of all files in /stackable directly above
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}

WORKDIR /stackable/spark-connect-examples/python
