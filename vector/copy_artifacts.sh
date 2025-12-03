#!/usr/bin/env bash

# NOTE: This is a modified version of shared/copy_artifacts.sh

# Copy over the binary
cp "$1" /app

# And now try to find a BOM file named like the binary + _bin.cdx.xml and copy it over as well if it exists
base=$(basename "$1")
find /src/ -type f -name "${base}_bin.cdx.xml" -exec cp {} /app \;
