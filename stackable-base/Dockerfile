# 8.9-1029 as of 2023-12-01
# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi8/ubi-minimal:8.9@sha256:87bcbfedfd70e67aab3875fff103bade460aeff510033ebb36b7efa009ab6639

# intentionally unused
ARG PRODUCT

# We configure microdnf to not install weak dependencies in this file
# Not doing this caused the content of images to become unpredictable because
# based on which packages get updated by `microdnf update` new weak dependencies
# might be installed that were not present earlier (the ubi8 base image doesn't
# seem to install weak dependencies)
# This also affects the packages that are installed in our Dockerfiles (java as prime
# example).
# https://github.com/stackabletech/docker-images/pull/533
COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf

RUN microdnf update && \
    microdnf --assumeyes install \
      # To make debugging easier, includes things like ping \
      # 2024:02: We cannot find any vulnerabilities in the past years \
      # https://github.com/iputils/iputils \
      iputils \
      # To make debugging easier \
      # less has seen three vulnerabilities between 2004 and 2022 which is a risk we're willing to accept for the added convenience \
      # https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&search_type=all&isCpeNameSearch=false&cpe_vendor=cpe%3A%2F%3Agnu&cpe_product=cpe%3A%2F%3A%3Aless \
      # cpe:2.3:a:gnu:less:*:*:*:*:*:*:*:*
      less \
      # To make debugging and changing things easier \
      # We checked and it has not seen any vulnerabilities since 2010 (as of 2024-03) we decided to accept it into our base image \
      # https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&query=cpe%3A2.3%3Aa%3Agnu%3Anano&search_type=all&isCpeNameSearch=false
      # cpe:2.3:a:gnu:nano:*:*:*:*:*:*:*:*
      nano \
      # Added only temporarily, removed again below
      shadow-utils && \
    groupadd --gid 1000 --system stackable && \
    useradd --gid stackable --uid 1000 --system stackable -d /stackable && \
    mkdir /stackable && \
    chown stackable:stackable /stackable && \
    microdnf remove shadow-utils && \
    microdnf clean all

RUN echo "alias ll='ls -alF --color=auto'" >> /stackable/.bashrc && \
    echo "alias ls='ls --color=auto'" >> /stackable/.bashrc && \
    echo "alias ..='cd ..'" >> /stackable/.bashrc && \
    echo "export PS1='\u@\[\e[36m\]\H\[\e[m\] \[\e[32m\]$(pwd)\[\e[m\] \\$ '" >> /stackable/.bashrc && \
    echo -e "if [ -f ~/.bashrc ]; then\n\tsource ~/.bashrc\nfi" >> /stackable/.profile && \
    chown stackable:stackable /stackable/.bashrc && \
    chown stackable:stackable /stackable/.profile \
