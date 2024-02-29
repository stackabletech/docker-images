#!/usr/bin/env bash

# Enable error handling and unset variable checking
set -eu
set -o pipefail

# Check if $1 (VERSION) is provided
if [ -z "${1-}" ]; then
  echo "Please provide a value for VERSION as the first argument."
  exit 1
fi

VERSION="$1"
PATCH_DIR="patches/$VERSION"

# Check if version-specific patches directory exists
if [ ! -d "$PATCH_DIR" ]; then
  echo "Patches directory '$PATCH_DIR' does not exist."
  exit 1
fi

# Create an array to hold the patches in sorted order
declare -a patch_files
patch_files=()

echo "Applying patches from ${PATCH_DIR}" now

# Read the patch files into the array
while IFS= read -r -d $'\0' file; do
  patch_files+=("$file")
done < <(find "$PATCH_DIR" -name "*.patch" -print0 | sort -zV)

echo "Found ${#patch_files[@]} patches, applying now"

# Check if any patches were found
if [ ${#patch_files[@]} -eq 0 ]; then
  echo "No patches found in $PATCH_DIR, nothing to apply."
  exit 0
fi

# Iterate through sorted patch files
for patch_file in "${patch_files[@]}"; do
  echo "Applying $patch_file"
  git apply --directory "hive-${VERSION}-src" "$patch_file" || {
    echo "Failed to apply $patch_file"
    exit 1
  }
done

echo "All patches applied successfully."
