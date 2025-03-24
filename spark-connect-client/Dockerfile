# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

# hadoop-builder: Provides Hadoop libraries
FROM stackable/image/spark-k8s AS spark-builder

FROM stackable/image/java-base AS final

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
    description="Spark Connect Examples"


ENV HOME=/stackable
ENV SPARK_HOME=/stackable/spark
ENV PATH=$SPARK_HOME:$PATH:/bin:$JAVA_HOME/bin:$JAVA_HOME/jre/bin:$HOME/.local/bin
ENV PYSPARK_PYTHON=/usr/bin/python
ENV PYTHONPATH=$SPARK_HOME/python

COPY spark-k8s/stackable /stackable
COPY spark-k8s/licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 --from=spark-builder /stackable/spark /stackable/spark
COPY --from=spark-builder /usr/bin/tini /usr/bin/tini

RUN <<EOF
microdnf update
# java-*-openjdk-devel: This is needed by the Spark UI to display process information using jps and jmap
#                       The spark-connect server also needs it to compile auto-generated classes on the fly.
#                       Copying just the binaries from the builder stage failed.
#
# python{version}-setuptools: needed to build the pyspark[connect] package
microdnf install \
  gzip \
  hostname \
  "python${PYTHON}" \
  "python${PYTHON}-pip" \
  "python${PYTHON}-setuptools" \
  zip \
  "java-${JAVA_VERSION}-openjdk-devel"
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

# The base image (java-base) defines this to point to a JRE installation.
# Spark Connect requires it to point to a JDK installation.
ENV JAVA_HOME="/usr/lib/jvm/java-${JAVA_VERSION}-openjdk"


# ----------------------------------------
# Attention: We are changing the group of all files in /stackable directly above
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}

WORKDIR /stackable/spark
ENTRYPOINT [ "/stackable/run-spark.sh" ]
