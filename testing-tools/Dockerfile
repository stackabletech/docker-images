# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# Find the latest version at https://catalog.redhat.com/en/software/containers/ubi10/ubi-minimal/66f1504a379b9c2cf23e145c#get-this-image
# IMPORTANT: Make sure to use the "Manifest List Digest" that references the images for multiple architectures
# rather than just the "Image Digest" that references the image for the selected architecture.
FROM registry.access.redhat.com/ubi10/ubi-minimal@sha256:67aafc6c9c44374e1baf340110d4c835457d59a0444c068ba9ac6431a6d9e7ac

ARG PRODUCT_VERSION
ARG PYTHON_VERSION
ARG KUBECTL_VERSION
ARG RELEASE_VERSION
ARG TARGETARCH
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG STACKABLE_USER_NAME

LABEL name="Stackable Testing Tools" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT_VERSION}" \
      release="${RELEASE_VERSION}" \
      summary="Stackable tools for integration tests." \
      description="Used by Stackable integration tests."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]


# We configure microdnf to not install weak dependencies in this file
# Not doing this caused the content of images to become unpredictable because
# based on which packages get updated by `microdnf update` new weak dependencies
# might be installed that were not present earlier (the ubi base image doesn't
# seem to install weak dependencies)
# This also affects the packages that are installed in our Dockerfiles (java as prime
# example).
# https://github.com/stackabletech/docker-images/pull/533
COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf

# Default curl configuration to avoid forgetting settings and to declutter the Dockerfiles
COPY stackable-base/stackable/curlrc /root/.curlrc

# Base requirements for all testing-tools images
COPY testing-tools/requirements.txt /stackable/requirements.txt
COPY testing-tools/licenses /licenses

RUN <<EOF
microdnf update

microdnf install \
  `# This installs kinit needed in kerberos tests.` \
  krb5-workstation \
  krb5-devel
  gcc \
  make \
  pkg-config \
  openssl \
  openssl-devel \
  libxml2-devel \
  libxslt-devel \
  python${PYTHON_VERSION} \
  python${PYTHON_VERSION}-devel \
  python${PYTHON_VERSION}-pip \
  jq \
  gzip \
  curl \
  tar \
  zip \
  unzip \
  `# Required to do test assertions of files` \
  diffutils

microdnf clean all
rm -rf /var/cache/yum

python3.12 -m pip install --no-cache-dir --upgrade pip
python3.12 -m pip install --no-cache-dir -r /stackable/requirements.txt

ln -s /usr/bin/python3.12 /usr/bin/python

# Install kubectl
# Get latest stable version from curl -L -s https://dl.k8s.io/release/stable.txt
curl -LO "https://dl.k8s.io/release/v${KUBECTL_VERSION}/bin/linux/${TARGETARCH}/kubectl"
install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
rm kubectl

# Added only temporarily to create the user and group, removed again below
microdnf install shadow-utils

groupadd --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}
useradd \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --create-home \
  --home-dir /stackable \
   ${STACKABLE_USER_NAME}

microdnf remove shadow-utils
microdnf clean all

chown -R ${STACKABLE_USER_UID}:0 /stackable
EOF

USER ${STACKABLE_USER_UID}

ENV STACKABLE_PRODUCT_VERSION=${PRODUCT_VERSION}

WORKDIR /stackable
