#!/bin/bash
#
# Purpose
#
# Checks that permissions and ownership in the provided directory are set according to:
#
# chown -R ${STACKABLE_USER_UID}:0 /stackable
# chmod -R g=u /stackable
#
# Will error out and print directories / files that do not match the required permissions or ownership.
#
# Usage
#
# ./check-permissions-ownership.sh <directory> <uid> <gid>
# ./check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
#

if [[ $# -ne 3 ]]; then
    echo "Wrong number of parameters supplied. Usage:"
    echo "$0 <directory> <uid> <gid>"
    echo "$0 /stackable 1000 0"
    exit 1
fi

DIRECTORY=$1
EXPECTED_UID=$2
EXPECTED_GID=$3

error_flag=0

# Check ownership
while IFS= read -r -d '' file; do
    uid=$(stat -c "%u" "$file")
    gid=$(stat -c "%g" "$file")

    if [[ "$uid" -ne "$EXPECTED_UID" || "$gid" -ne "$EXPECTED_GID" ]]; then
        echo "Ownership mismatch:  $file (Expected: $EXPECTED_UID:$EXPECTED_GID, Found: $uid:$gid)"
        error_flag=1
    fi
done < <(find "$DIRECTORY" -print0)

# Check permissions
while IFS= read -r -d '' file; do
    perms=$(stat -c "%A" "$file")
    owner_perms="${perms:1:3}"
    group_perms="${perms:4:3}"

    if [[ "$owner_perms" != "$group_perms" ]]; then
        echo "Permission mismatch: $file (Owner: $owner_perms, Group: $group_perms)"
        error_flag=1
    fi
done < <(find "$DIRECTORY" -print0)

if [[ $error_flag -ne 0 ]]; then
    echo "Permission and Ownership checks failed for $DIRECTORY!"
    exit 1
fi

echo "Permission and Ownership checks succeeded for $DIRECTORY!"
