#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

ARCHITECTURES=(
    x86_64
    aarch64
)

if [[ $VERSION =~ ^(.+)-(.+)\.el(.+)$ ]]; then
  # INOTIFY_TOOLS_VERSION=${BASH_REMATCH[1]}
  # PACKAGE_VERSION=${BASH_REMATCH[2]}
  EPEL_VERSION=${BASH_REMATCH[3]}
else
  echo 'VERSION must match the pattern "<INOTIFY-TOOLS-VERSION>-<PACKAGE-VERSION>.el<EPEL-VERSION>", e.g. "3.14-19.el8".'
  exit 1
fi

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
    file=inotify-tools-$VERSION.$arch.rpm

    echo "Downloading $file from dl.fedoraproject.org"
    curl --fail -LO --progress-bar "https://dl.fedoraproject.org/pub/epel/$EPEL_VERSION/Everything/$arch/Packages/i/$file"

    echo "Uploading $file to Nexus"
    curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" \
        --upload-file "$file" \
        'https://repo.stackable.tech/repository/packages/inotify-tools/'
done

echo "Successfully uploaded new version $VERSION to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/inotify-tools/"
