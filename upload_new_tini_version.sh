#!/usr/bin/env bash

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

ARCHITECTURES=(
    arm64
    amd64
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
  # tini does not currently publish signatures or SBOMs
  # renaming binary since original file name has no version
  echo "Downloading TINI"
  curl --fail -Ls -o tini-$VERSION-$arch "https://github.com/krallin/tini/releases/download/v$VERSION/tini-$arch" 

  echo "Uploading to Nexus"
  curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "tini-$arch" 'https://repo.stackable.tech/repository/packages/tini/'

echo "Successfully uploaded new version of TINI ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/tini/"
