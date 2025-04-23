#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

ARCHITECTURES=(
    x64
    arm64
)

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

# async-profiler does not currently publish signatures or SBOMs (as of
# 2024-01-30, latest version at this point v3.0)

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
    file=async-profiler-$VERSION-linux-$arch.tar.gz

    echo "Downloading $file from github.com"
    curl --fail -LO --progress-bar "https://github.com/async-profiler/async-profiler/releases/download/v$VERSION/$file"

    echo "Uploading $file to Nexus"
    curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" \
        --upload-file "$file" \
        'https://repo.stackable.tech/repository/packages/async-profiler/'
done

echo "Successfully uploaded new version $VERSION to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/async-profiler/"
