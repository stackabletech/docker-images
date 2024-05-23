#!/bin/bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}
PRODUCT=${3:?"Missing product argument (arg 3)"}
REGISTRY="registry.k8s.io/sig-storage"
STACKABLE_REGISTRY="docker.stackable.tech/k8s/sig-storage"

ARCHITECTURES=(
    amd64
    arm64
)

function pull_retag_and_push_images() {
    for arch in "${ARCHITECTURES[@]}"; do
        echo "Pulling ${REGISTRY}/${PRODUCT}:v${VERSION} for ${arch}"
        docker pull "${REGISTRY}/${PRODUCT}:v${VERSION}" --platform "linux/${arch}"

        echo "Retagging image for ${arch}"
        docker tag "${REGISTRY}/${PRODUCT}:v${VERSION}" "${STACKABLE_REGISTRY}/${PRODUCT}:v${VERSION}-${arch}"

        echo "Pushing retagged image for ${arch}"
        PUSH_OUTPUT=$(docker push "${STACKABLE_REGISTRY}/${PRODUCT}:v${VERSION}-${arch}")
        echo "$PUSH_OUTPUT"
        DIGEST=$(echo "$PUSH_OUTPUT" | awk "/: digest: sha256:[a-f0-9]{64} size: [0-9]+$/ { print \$3 }")

        echo "Sigining image ${DIGEST} for ${arch}"
        cosign sign -y "${STACKABLE_REGISTRY}/${PRODUCT}@${DIGEST}"
    done
}

function create_and_push_manifest_list() {
    echo "Creating manifest list"
    docker manifest create "${STACKABLE_REGISTRY}/${PRODUCT}:v${VERSION}" \
        --amend "${STACKABLE_REGISTRY}/${PRODUCT}:v${VERSION}-amd64" \
        --amend "${STACKABLE_REGISTRY}/${PRODUCT}:v${VERSION}-arm64"

    echo "Pushing manifest list"
    LIST_DIGEST=$(docker manifest push "${STACKABLE_REGISTRY}/${PRODUCT}:v${VERSION}")

    echo "Signing manifest list"
    cosign sign -y "${STACKABLE_REGISTRY}/${PRODUCT}@${LIST_DIGEST}"
}

function docker_login() {
    read -r -s -p "Nexus Password: " NEXUS_PASSWORD
    echo
    echo "Logging in to Docker registry"
    docker login -u "$NEXUS_USER" -p "$NEXUS_PASSWORD" docker.stackable.tech
}

function main() {
    docker_login
    pull_retag_and_push_images
    create_and_push_manifest_list
}

main "$@"
