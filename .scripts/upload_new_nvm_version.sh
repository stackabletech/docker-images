#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo

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

# https://github.com/nvm-sh/nvm/archive/refs/tags/v0.40.3.tar.gz
BASE_URL="https://github.com/nvm-sh/nvm/archive/refs/tags"
ARCHIVE_FILE="v$VERSION.tar.gz"
ARCHIVE_URL="$BASE_URL/$ARCHIVE_FILE"

echo "Downloading nvm $VERSION"
curl --fail -LO --progress-bar "$ARCHIVE_URL"

# nvm maintainers don't produce sum files

echo "Uploading to Nexus"

curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$ARCHIVE_FILE" 'https://repo.stackable.tech/repository/packages/nvm/'

echo "Successfully uploaded nvm $VERSION to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/nvm/"
echo "https://github.com/nvm-sh/nvm/releases/tag/v$VERSION"
