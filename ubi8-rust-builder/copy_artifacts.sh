#!/usr/bin/env bash

# Copy over the binary
cp "$1" /app

# And now try to find a BOM file named like the binary + .cdx.xml and copy it over as well if it exists
base=$(basename "$1")
find /src/rust/ -type f -name "$base.cdx.xml" -exec cp {} /app \;
