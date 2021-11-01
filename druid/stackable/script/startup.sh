#!/bin/bash -eu

if [ "$#" -gt 2 ] || [ "$#" -lt 2 ]
then
  >&2 echo "usage: $0 <service> conf-dir"
  exit 1
fi

PWD="$(pwd)"
WHEREAMI="$(dirname "$0")"
WHATAMI="$1"
CONFDIR="$(cd "$2" && pwd)"
DRUID_BIN="$(cd "$WHEREAMI"/../bin && pwd)"
JAVA_BIN="$(source "$DRUID_BIN"/java-util && get_java_bin_dir)"
if [ -z "$JAVA_BIN" ]; then
  >&2 echo "Could not find java - please run $DRUID_BIN/verify-java to confirm it is installed."
  exit 1
fi
cd "$WHEREAMI/.."
exec "$JAVA_BIN"/java `cat "$CONFDIR"/jvm.config | xargs` \
  -cp "$CONFDIR":"$WHEREAMI/../lib/*" \
  org.apache.druid.cli.Main server "$WHATAMI"