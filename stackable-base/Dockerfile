# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# 9.4-949.1714662671 as of 2024-05-08
# https://catalog.redhat.com/software/containers/ubi9/ubi-minimal/615bd9b4075b022acc111bf5?image=6633b5136d81a5634a616c41&architecture=amd64

# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi9/ubi-minimal@sha256:2636170dc55a0931d013014a72ae26c0c2521d4b61a28354b3e2e5369fa335a3 AS builder

# intentionally unused
ARG PRODUCT

# Sets the default shell to Bash with strict error handling and robust pipeline processing.
# "-e": Exits immediately if a command exits with a non-zero status
# "-u": Treats unset variables as an error, preventing unexpected behavior from undefined variables.
# "-o pipefail": Causes a pipeline to return the exit status of the last command in the pipe that failed, ensuring errors in any part of a pipeline are not ignored.
# "-c": Allows the execution of commands passed as a string
# This is automatically inherited in all other Dockerfiles that use this unless it is overwritten
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

RUN microdnf update --assumeyes && \
    microdnf --assumeyes install \
      # To make debugging easier, includes things like ping \
      # Added 2024-03: We cannot find any vulnerabilities in the past years \
      # https://github.com/iputils/iputils \
      iputils \
      # To make debugging easier \
      # Added 2024-03: less has seen three vulnerabilities between 2004 and 2022 which is a risk we're willing to accept for the added convenience \
      # https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&search_type=all&isCpeNameSearch=false&cpe_vendor=cpe%3A%2F%3Agnu&cpe_product=cpe%3A%2F%3A%3Aless \
      # cpe:2.3:a:gnu:less:*:*:*:*:*:*:*:*
      less \
      # To make debugging and changing things easier \
      # Added 2024-03: We checked and it has not seen any vulnerabilities since 2010 (as of 2024-03) we decided to accept it into our base image \
      # https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&query=cpe%3A2.3%3Aa%3Agnu%3Anano&search_type=all&isCpeNameSearch=false
      # cpe:2.3:a:gnu:nano:*:*:*:*:*:*:*:*
      nano \
      # To enable kubectl cp \
      # Added 2024-03: We checked and it has seen eight vulnerabilities since 2001, mostly minor and it's not in executable path so we decided to accept the risk \
      # https://nvd.nist.gov/vuln/search/results?adv_search=true&isCpeNameSearch=true&query=cpe%3A2.3%3Aa%3Agnu%3Atar%3A-%3A*%3A*%3A*%3A*%3A*%3A*%3A*
      # cpe:2.3:a:gnu:tar:-:*:*:*:*:*:*:* \
      tar \
      # Added only temporarily, removed again below
      shadow-utils && \
    groupadd --gid 1000 --system stackable && \
    useradd --gid stackable --uid 1000 --system stackable -d /stackable && \
    mkdir /stackable && \
    chown stackable:stackable /stackable && \
    microdnf remove shadow-utils && \
    microdnf clean all && \
    echo "alias ll='ls -alF --color=auto'" >> /stackable/.bashrc && \
    echo "alias ls='ls --color=auto'" >> /stackable/.bashrc && \
    echo "alias ..='cd ..'" >> /stackable/.bashrc && \
    echo "export PS1='\u@\[\e[36m\]\H\[\e[m\] \[\e[32m\]\$(pwd)\[\e[m\] \\$ '" >> /stackable/.bashrc && \
    echo -e "if [ -f ~/.bashrc ]; then\n\tsource ~/.bashrc\nfi" >> /stackable/.profile && \
    chown stackable:stackable /stackable/.bashrc && \
    chown stackable:stackable /stackable/.profile

# CVE-2023-37920: Remove "e-Tugra" root certificates
# e-Tugra's root certificates were subject to an investigation prompted by reporting of security issues in their systems
# The package ca-certificates 2023.07.22 fixes the problem, until ubi9-minimal updates to it, we should remove them
# manually.

RUN if [ "$(rpm -qa ca-certificates)" != "ca-certificates-2023.2.60_v7.0.306-90.1.el9_2.noarch" ]; then \
    echo "The ca-certificates package was updated. Please check if the e-Tugra root certificates are present. \
When they have been removed, manually blacklisting them should be removed" && \
    echo "Let me help you by running trust list --filter=ca-anchors | grep 'E-Tugra'" && \
    trust list --filter=ca-anchors | grep 'E-Tugra' && \
    exit 1; \
  fi

COPY stackable-base/stackable/ca-cert-blocklist/ /etc/pki/ca-trust/source/blocklist/

RUN update-ca-trust && \
  if [ "$(trust list --filter=ca-anchors | grep -c 'E-Tugra')" != "0" ]; then \
    echo "Still found E-Tugra root certificates, this should not happen!" && \
    exit 1; \
  fi
