#!/usr/bin/env bash

# The script outputs the image manifest uri with the SHA256 digest, for example:
# oci.stackable.tech/sdp/hello-world@sha256:917f800259ef4915f976e93987b752fd64debf347568610d7f685d20220fc88a
set -euo pipefail

# There is only one input to this script, which contains the entire image
# manifest uri, like: oci.stackable.tech/sdp/hello-world:0.0.1-SNAPSHOT-stackable0.0.0-dev-arm64
IMAGE_MANIFEST_URI="$1"

# Here, we cut off the image manifest tag to get the image repository uri, for
# example: oci.stackable.tech/sdp/hello-world
IMAGE_REPOSITORY_URI="$(echo "$IMAGE_MANIFEST_URI" | cut -d : -f 1)"

IMAGE_REPO_DIGEST=$(
  docker inspect "$IMAGE_MANIFEST_URI" --format json | \
  jq -r \
  --arg IMAGE_REPOSITORY_URI "$IMAGE_REPOSITORY_URI" \
  --arg IMAGE_MANIFEST_URI "$IMAGE_MANIFEST_URI" \
  '
      map(select(.RepoTags[] | contains($IMAGE_MANIFEST_URI)))[0]
      | .RepoDigests[]
      | select(. | startswith($IMAGE_REPOSITORY_URI))
  '
)

# Ensure IMAGE_REPO_DIGEST is not empty
if [[ -z "$IMAGE_REPO_DIGEST" ]]; then
    >&2 echo "Repo Digest is empty, but is required for signing"
    exit 1
fi

echo "$IMAGE_REPO_DIGEST"
