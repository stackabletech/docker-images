#!/bin/bash
# Adapted from https://github.com/cloudera/cloudera-scripts-for-log4j/blob/main/hdp_support_scripts/delete_jndi.sh
#
# Applicable Open Source License: Apache License 2.0
#
# --------------------------------------------------------------------------------------

set -eu -o pipefail

shopt -s globstar
shopt -s nullglob

if ! command -v zip &> /dev/null; then
	echo "zip not found. zip is required to run this script."
	exit 1
fi

for targetdir in ${1}
do
  echo "Running on '$targetdir'"

  for jarfile in $targetdir/**/*.jar; do
	if grep -q JndiLookup.class $jarfile; then
		# Rip out class
		echo "Deleting JndiLookup.class from '$jarfile'"
		zip -q -d "$jarfile" \*/JndiLookup.class
	fi
  done

  for warfile in $targetdir/**/*.{war,nar}; do
    doZip=0

    rm -r -f /tmp/unzip_target
	  mkdir /tmp/unzip_target
	  set +e
    unzip -qq $warfile -d /tmp/unzip_target
    set -e
      for jarfile in /tmp/unzip_target/**/*.jar; do
      if grep -q JndiLookup.class $jarfile; then
        # Rip out class
        echo "Deleting JndiLookup.class from '$jarfile'"
        zip -q -d "$jarfile" \*/JndiLookup.class
        doZip=1
      fi
	  done

	if [ 1 -eq $doZip ]; then
	  echo "Updating '$warfile'"
	  pushd /tmp/unzip_target
	  zip -r -q $warfile .
	  popd
	fi

    rm -r -f /tmp/unzip_target
  done
done

echo "Run successful"