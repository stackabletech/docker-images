#!/usr/bin/env bash
set -euo pipefail

# Print out the repo name for an image tag.
# Either set `IMAGE_REPOSITORY_URI`, or provide the URI as the first argument.
#
# Given any of these inputs:
# - `my/image`
# - `my/image:1.0.2`
# - `my/image@sha256:0aee95ea27261801c7bea9326a8b03ab888f6b68fe389c3e6de291adf044c11a`
# - `registry.example.com/my/image`
# - `registry.example.com/my/image:1.0.2`
# - `registry.example.com/my/image@sha256:0aee95ea27261801c7bea9326a8b03ab888f6b68fe389c3e6de291adf044c11a`
#
# The printed output will be: `my/image`

# If not set in the environment, then it must come from the first argument.
IMAGE_REPOSITORY_URI="${IMAGE_REPOSITORY_URI:-$1}"

# Assert that it is not empty
if [ -z "$IMAGE_REPOSITORY_URI" ]; then
    >&2 echo "Error: An image tag must be supplied as the first argument"
    exit 1;
fi

IMAGE_REPOSITORY_URI=$(echo "$IMAGE_REPOSITORY_URI" | cut -d@ -f1)
IMAGE_REPOSITORY_URI=$(echo "$IMAGE_REPOSITORY_URI" | cut -d: -f1)

# Need to cater to images that might not have the registry prefix
# ie: defaulting to hub.docker.com
if grep -q '\.' <(echo "$IMAGE_REPOSITORY_URI"); then
    # Strip off the registry
    echo "$IMAGE_REPOSITORY_URI" | cut -d/ -f2-
else
    # The image uri is already without a registry
    echo "$IMAGE_REPOSITORY_URI"
fi
