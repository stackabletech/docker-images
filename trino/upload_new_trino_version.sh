#!/usr/bin/env bash

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

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

bin_file=trino-server-$VERSION.tar.gz

echo "Downloading Trino (this can take a while, it is intentionally downloading from a slow mirror that contains all old versions)"
curl --fail -LOs "https://repo1.maven.org/maven2/io/trino/trino-server/${VERSION}/${bin_file}"
curl --fail -LOs "https://repo1.maven.org/maven2/io/trino/trino-server/${VERSION}/${bin_file}.asc"
curl --fail -LOs "https://repo1.maven.org/maven2/io/trino/trino-server/${VERSION}/${bin_file}.sha1"

# It is probably redundant to check both the checksum and the signature but it's cheap and why not
echo "Validating SHA1 Checksum"
if ! (sha1sum "${bin_file}" | cut -d " " -f 1 | diff -Z - "${bin_file}.sha1"); then
  echo "ERROR: The SHA1 sum does not match"
  exit 1
fi

echo "Validating signature"
echo '--> NOTE: Make sure you have added the public key (https://app.nuclino.com/Stackable/Knowledge-Base/Finding-GPG-keys-for-Maven-packages-6b7b0324-8f0f-4b0f-a6e8-aa97cea5512c) to GPG: https://www.apache.org/info/verification.html'

if ! (gpg --verify "${bin_file}.asc" "${bin_file}" 2> /dev/null); then
  echo "ERROR: The signature could not be verified"
  exit 1
fi

echo "Uploading everything to Nexus"
EXIT_STATUS=0
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file}" 'https://repo.stackable.tech/repository/packages/trino-server/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file}.asc" 'https://repo.stackable.tech/repository/packages/trino-server/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file}.sha1" 'https://repo.stackable.tech/repository/packages/trino-server/' || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version ${VERSION} of Trino to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/trino-server/"
