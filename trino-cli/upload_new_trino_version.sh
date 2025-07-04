#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

# https://stackoverflow.com/questions/4632028/how-to-create-a-temporary-directory
# Find the directory name of the script
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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

bin_file=trino-cli-${VERSION}-executable.jar

echo "Downloading Trino (this can take a while, it is intentionally downloading from a slow mirror that contains all old versions)"
curl --fail -LO --progress-bar "https://repo1.maven.org/maven2/io/trino/trino-cli/${VERSION}/${bin_file}"
curl --fail -LO --progress-bar "https://repo1.maven.org/maven2/io/trino/trino-cli/${VERSION}/${bin_file}.asc"
curl --fail -LO --progress-bar "https://repo1.maven.org/maven2/io/trino/trino-cli/${VERSION}/${bin_file}.sha1"

# It is probably redundant to check both the checksum and the signature but it's cheap and why not
echo "Validating SHA1 Checksum"
if ! (sha1sum "${bin_file}" | cut -d " " -f 1 | diff -Z - "${bin_file}.sha1"); then
  echo "ERROR: The SHA1 sum does not match"
  exit 1
fi

# echo "Adding pinned public key for signature"
# # We lock the public key here until trino has a better workflow for signing
# gpg --no-default-keyring --keyring "${WORK_DIR}"/keyring --import <<-EOF
# -----BEGIN PGP PUBLIC KEY BLOCK-----
# Comment: Hostname:
# Version: Hockeypuck 2.1.0-223-gdc2762b
#
# xsBNBErpKk4BCADmT8y00Y1BcsB6KqEww68wyfH7129Izs2wHikHOEcPQWz2ROrb
# ug16Y+uvJkAzjD1EgJ9t/CSU4JbAT11I7u7oHMYlgtyEg06nAruQlwchs1vMrcPj
# mJ2aK3dOMHmYtiZ2Qq2ZUxnGvv1T+ywOsukV/4idZ1X9Z4qlQTg8jtp7gceRJ3Ct
# yiRoZ4lV+H+dV07dk9gEqiMJyCaf96IT+CIB5Xv6Z1jpKLxOAONzdENa5K9cfI+m
# HflrDmqg6apZ3obwzmEC+88K78A9w/+PK/VgK7OCaCkp816Vr2ej2e0qxpXz1xN4
# gyo/JUEqutf8gMKvmPuU2BnbdoNapf9f8LttABEBAAHNJU1hcnRpbiBUcmF2ZXJz
# byA8bXRyYXZlcnNvQGdtYWlsLmNvbT7CwHgEEwECACIFAkrpKk4CGwMGCwkIBwMC
# BhUIAgkKCwQWAgMBAh4BAheAAAoJEA62n3b9FxU4cVwH/jR9rwRB2uA7+bSuJBCM
# Ak2JWPwo2Ek8RjHb4VMlbKsPDW15nX8JriINesQ5ELecOMVgHKV24Mv31c/2Yh6Y
# SuEuYvauGdtPbREo7evZU/R3r54uCcNaK8ZpLeZQXRMNKKwBUKRnF1G+lRuVvORj
# abkbrUSfIS/cFKFzcCVzKLCbTpbfJ5JJmjulg5p8KpRS2/R63mAn7JRRDuSa1SJQ
# FgerzSoe4/t0GBCusBs8TsnEQ2X4OdQP95nBL3TANwMUupdX9dBa1h8c8gps1Uak
# xRYsbAANoCPfVUUpLT6WpsYk5X28+sXngVK2BJfoaq5zi2ATfQdBHIedRihCQrTk
# VjXOwE0ESukqTgEIAMNTWjnhzQeyUjGvvcuczhiKWj8lPlLCpN8AF168PNQDFoDC
# Uxdi7S5OKiwwDxm1cUy/gbij6qLazIAgSBRrW5C4dNK+SIAcgtNfLbT5Z/4mlOfg
# ErYH+lAxCNO3k+AzfVU/n0ZShhEuNhVgHc8pDiI/MXCZZKsAJxPFVu7pxiEM7LdT
# sSunzM/jZDrfIU7KBjZdlz0FK8L624+tAJD7WomQ8Ddx3MpOju+ShpP3YqddU0Hx
# jjP1eGkcZibJzKmByQw0r5WX9ePFJ0K86ovWNKTcnDJbUEwhq2s/lJsIsvbcbNaI
# LaOTEiQI2EBy0OB72zrvUcw0Xwaipft0BCwUIN0AEQEAAcLAXwQYAQIACQUCSukq
# TgIbDAAKCRAOtp92/RcVOBcYB/9KXC+CV3GBFZNViBJdPAzGFD5FIcr83riwy2RK
# cbehekBjETfLjSfNzB60HnAeU/l+vIOTsLLu1dk0XehG6Laq4325kIZGmRIIqIzZ
# qMNG/DLmqMwicSnbw+4hJLU6GQdLNXu0fGDjK4NuZ0yRur0e2JHbgKNgFDnttJx/
# ER6Q1SfaIKZSKSd46EFYX2f63Uu7w+yIgvpQCaRUG7Lqz7NJVxxCiF+qRdVEY2E3
# hhyG1DGAMMXETV2Hp7SoBmQjAqqwAy1aLwyyNgn1Ft38T+6/IBMGQHMnBcWfOd41
# LoKR7XroVADNIdggJawYzZNyU6clw/O1if5vSURumLeul13T
# =p7ZF
# -----END PGP PUBLIC KEY BLOCK-----
# EOF
#
# echo "Validating signature"
# if ! (gpgv --keyring "${WORK_DIR}"/keyring "${bin_file}.asc" "${bin_file}" 2> /dev/null); then
#   echo "ERROR: The signature could not be verified"
#   exit 1
# fi

echo "Uploading everything to Nexus"
EXIT_STATUS=0
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file}" 'https://repo.stackable.tech/repository/packages/trino-cli/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file}.asc" 'https://repo.stackable.tech/repository/packages/trino-cli/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${bin_file}.sha1" 'https://repo.stackable.tech/repository/packages/trino-cli/' || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version ${VERSION} of Trino CLI to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/trino-cli/"
