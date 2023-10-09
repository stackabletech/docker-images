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

downloaded_bin_file=opa_linux_amd64_static
download_url="https://openpolicyagent.org/downloads/v${VERSION}/${downloaded_bin_file}"
bin_file=opa
bin_file_zipped="${bin_file}"_"${VERSION}".tar.gz

# We need to keep the original bin file name for now to have the easy diff on the checksum working
# Will rename it to just "opa" later
echo "Downloading OPA from ${download_url}"
curl --fail -L -o "${downloaded_bin_file}" "${download_url}"
echo "Downloading OPA checksum from ${download_url}.sha256"
curl --fail -L -o "${bin_file_zipped}".sha256 "${download_url}.sha256"

echo "Validating SHA256 Checksum"
if ! (sha256sum "${downloaded_bin_file}" | diff - "${bin_file_zipped}".sha256); then
  echo "ERROR: One of the SHA256 sums does not match"
  exit 1
fi

echo "Rename downloaded bin file ${downloaded_bin_file} to ${bin_file}"
mv ${downloaded_bin_file} ${bin_file}

echo "Set execute permissions on bin file ${bin_file}"
chmod 755 "${bin_file}"

echo "Compressing ${bin_file} bin file to ${bin_file_zipped}"
tar -czf "${bin_file_zipped}" "${bin_file}"

echo "Recreating SHA512 checksum for compressed bin file ${bin_file_zipped}"
sha512sum "${bin_file_zipped}" > "${bin_file_zipped}".sha512

echo "Uploading everything to Nexus"
EXIT_STATUS=0
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file_zipped}" 'https://repo.stackable.tech/repository/packages/opa/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file_zipped}".sha512 'https://repo.stackable.tech/repository/packages/opa/' || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version $VERSION of OPA to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/opa/"
