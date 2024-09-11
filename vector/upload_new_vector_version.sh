#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
RELEASE=${2:?"Missing release number argument (arg 2)"}
NEXUS_USER=${3:?"Missing Nexus username argument (arg 3)"}

ARCHITECTURES=(
    x86_64
    aarch64
)

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

# Vector does not currently publish signatures or SBOMs (as of
# 2023-10-11, latest version at this point 0.33.0)
# But there are SHA256 sums in Github Releases. Maybe we should download from there?

for arch in "${ARCHITECTURES[@]}"; do
    file="vector-$VERSION-$RELEASE.$arch.rpm"

    echo "Downloading $file from timber.io"
    curl -LOs "https://packages.timber.io/vector/$VERSION/$file"

    echo "Uploading $file to Nexus"
    curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" \
        --upload-file "$file" \
        'https://repo.stackable.tech/repository/packages/vector/'

    echo "Removing downloaded $file"
    rm "$file"
done

echo "Successfully uploaded new version of Vector ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/vector/"
