#!/usr/bin/env bash
#
# Example usage:
# upload_new_vector_version.sh 0.41.1 1 nexus-username /var/lib/rpm

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
RELEASE=${2:?"Missing release number argument (arg 2)"}
NEXUS_USER=${3:?"Missing Nexus username argument (arg 3)"}
RPM_PACKAGE_DB_PATH=${4:-"/var/lib/rpm"}

ARCHITECTURES=(
    x86_64
    aarch64
)

major_version="${VERSION%%.*}"

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

for arch in "${ARCHITECTURES[@]}"; do
    file="vector-$VERSION-$RELEASE.$arch.rpm"

    echo "Downloading $file from yum.vector.dev"
    curl \
        --fail \
        --location \
        --remote-name \
        --silent \
        "https://yum.vector.dev/stable/vector-$major_version/$arch/$file"

    echo "Validating signature"
    EXIT_STATUS=0
    # `rpmkeys --checksig` also succeeds if the digests of an unsigned
    # package are okay. Therefore, test explicitly if the output
    # contains "digests signatures OK" to ensure that the package is
    # signed.
    rpmkeys \
        --checksig \
        --dbpath "$RPM_PACKAGE_DB_PATH" \
        "$file" | \
        grep "^$file: digests signatures OK\$" || \
        EXIT_STATUS=$?
    if [ $EXIT_STATUS -ne 0 ]; then
      echo "ERROR: The signature could not be verified."
    echo "--> NOTE: Make sure you have downloaded and added Datadog's \
public key (https://keys.datadoghq.com/DATADOG_RPM_KEY_B01082D3.public) \
to the RPM package database:
rpmkeys --import --dbpath $RPM_PACKAGE_DB_PATH DATADOG_APT_KEY_CURRENT.public"
      exit 1
    fi

    echo "Uploading $file to Nexus"
    curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" \
        --upload-file "$file" \
        'https://repo.stackable.tech/repository/packages/vector/'

    echo "Removing downloaded $file"
    rm "$file"
done

echo "Successfully uploaded new version of Vector ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/vector/"
