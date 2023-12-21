#!/bin/bash
# Adapted from https://github.com/cloudera/cloudera-scripts-for-log4j/blob/main/hdp_support_scripts/delete_jndi.sh
#
# Applicable Open Source License: Apache License 2.0
#
# --------------------------------------------------------------------------------------

function delete_jndi_from_jar_files {
  targetdir=${1}
  echo "Running on '$targetdir'"

  shopt -s globstar
  for jarfile in $targetdir/**/*.jar; do
    if grep -q JndiLookup.class $jarfile; then
      # Rip out class
      echo "Deleting JndiLookup.class from '$jarfile'"
      zip -q -d "$jarfile" \*/JndiLookup.class
    fi
  done

  echo "Completed removing JNDI from jar files"

  for warfile in $targetdir/**/*.{war,nar}; do
    doZip=0

    rm -r -f /tmp/unzip_target
    mkdir /tmp/unzip_target
    unzip -qq $warfile -d /tmp/unzip_target
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

  echo "Completed removing JNDI from nar files"
}

function delete_jndi_from_targz_file {
  tarfile=$1
  if [ ! -f "$tarfile" ]; then
    echo "Tar file '$tarfile' not found"
    exit 1
  fi

  echo "Patching '$tarfile'"
  tempfile=$(mktemp)
  tempdir=$(mktemp -d)
  tempbackupdir=$(mktemp -d)

  tar xf "$tarfile" -C "$tempdir"
  delete_jndi_from_jar_files "$tempdir" "$tempbackupdir"

  echo "Recompressing"
  (cd "$tempdir" && tar czf "$tempfile" --owner=1000 --group=100 .)

  # Restore old permissions before replacing original
  chown --reference="$tarfile" "$tempfile"
  chmod --reference="$tarfile" "$tempfile"

  mv "$tempfile" "$tarfile"

  rm -f $tempfile
  rm -rf $tempdir
  rm -rf $tempbackupdir

  echo "Completed removing JNDI from $tarfile"
}


# Begin main program
set -eu -o pipefail

shopt -s globstar
shopt -s nullglob

if ! command -v unzip &> /dev/null; then
  echo "unzip not found. unzip is required to run this script."
  exit 1
fi

if ! command -v zgrep &> /dev/null; then
  echo "zgrep not found. zgrep is required to run this script."
  exit 1
fi

if ! command -v zip &> /dev/null; then
  echo "zip not found. zip is required to run this script."
  exit 1
fi

for targetdir in ${1}
do
  echo "Removing JNDI from jar files in $targetdir"
  delete_jndi_from_jar_files "$targetdir"
done

for targetdir in ${1}
do
  echo "Removing JNDI from tar.gz files in $targetdir"
  for targzfile in $(find "$targetdir" -name '*.tar.gz') ; do
    delete_jndi_from_targz_file "$targzfile"
  done
done

echo "Run successful"

