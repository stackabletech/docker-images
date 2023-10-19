#!/usr/bin/env bash

# Enable error handling and unset variable checking
set -eu
set -o pipefail

# Check if $1 (PRODUCT) is provided
if [ -z "${1-}" ]; then
  echo "Please provide a value for PRODUCT as the first argument."
  exit 1
fi

PRODUCT="$1"

# Iterate through patch files in the PATCH_DIR
# TODO: Make it more deterministic, patch files should have a sequence number in their name to apply them in order
for patch_file in patches/*; do
  echo "Checking $patch_file"
  # Check if the patch file matches the PRODUCT - this could probably also be more robust
  if [[ $patch_file == *"$PRODUCT"* ]]; then
    echo "Applying $patch_file"
    git apply --directory "hbase-${PRODUCT}-src" "$patch_file"
  fi
done
