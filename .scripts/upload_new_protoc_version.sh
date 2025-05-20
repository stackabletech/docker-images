#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

ARCHITECTURES=(
    aarch_64
    x86_64
)

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

# https://stackoverflow.com/questions/4632028/how-to-create-a-temporary-directory
# Find the directory name of the script
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# the temp directory used, within $DIR
WORK_DIR=$(mktemp -d -p "$DIR")

# check if tmp dir was created
if [[ ! "$WORK_DIR" || ! -d "$WORK_DIR" ]]; then
  echo "Could not create temp dir"
  exit 1
fi

# deletes the temp directory
function cleanup {
  rm -rf "$WORK_DIR"
}

# register the cleanup function to be called on the EXIT signal
trap cleanup EXIT

cd "$WORK_DIR" || exit

for arch in "${ARCHITECTURES[@]}"; do
  # protoc does not currently publish signatures or SBOMs

  DOWNLOAD_URL="https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-linux-$arch.zip"

    echo "Downloading protoc"
  if ! curl --fail -Ls -O "$DOWNLOAD_URL"; then
    echo "Failed to download from $DOWNLOAD_URL"
    exit 1
  fi

  FILE_NAME=$(basename "$DOWNLOAD_URL")

  echo "Uploading protoc to Nexus"
  if ! curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$FILE_NAME" 'https://repo.stackable.tech/repository/packages/protoc/'; then
    echo "Failed to upload protoc to Nexus"
    exit 1
  fi

done

echo "Successfully uploaded new version of protoc ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/protoc/"
