#!/usr/bin/env bash
#
# Entrypoint for HBase that ensures services are shutdown gracefuly
#
set -x
set -euo pipefail

# master, regionserver, rest
HBASE_ROLE_NAME="$1"
# k8s service name for this role+group combo
HBASE_ROLE_SERVICE_NAME="$2"
# 16010 for master, 16020 for regionservers etc.
HBASE_ROLE_SERVICE_PORT="$3"

# TODO: remove hard coded svc.cluster.local
HBASE_ROLE_SERVICE_HOST="${HOSTNAME}.${HBASE_ROLE_SERVICE_NAME}.${NAMESPACE}.svc.cluster.local"

REGION_MOVER_OPTS="--regionserverhost ${HBASE_ROLE_SERVICE_HOST}:${HBASE_ROLE_SERVICE_PORT} --operation unload ${REGION_MOVER_OPTS}"

prepare_signal_handlers() {
  unset term_child_pid
  unset term_kill_needed
  trap handle_term_signal TERM
}

handle_term_signal() {
  if [ "${term_child_pid}" ]; then
    if [ "regionserver" == "${HBASE_ROLE_NAME}" ]; then
      echo "Running pre-shutdown command: /stackable/hbase/bin/hbase org.apache.hadoop.hbase.util.RegionMover ${REGION_MOVER_OPTS}"
      # REGION_MOVER_OPTS is a string that contains multiple arguments and needs to be spliced here
      # therefore disable shellcheck for this line
      # shellcheck disable=SC2086
      /stackable/hbase/bin/hbase org.apache.hadoop.hbase.util.RegionMover ${REGION_MOVER_OPTS}
      echo "Done pre-shutdown command"
    fi
    kill -TERM "${term_child_pid}" 2>/dev/null
  else
    term_kill_needed='yes'
  fi
}

wait_for_termination() {
  set +e
  term_child_pid=$1
  if [[ -v term_kill_needed ]]; then
    kill -TERM "${term_child_pid}" 2>/dev/null
  fi
  wait "${term_child_pid}" 2>/dev/null
  trap - TERM
  wait "${term_child_pid}" 2>/dev/null
  set -e
}

# ##################################################################################################
# main
# ##################################################################################################
mkdir -p /stackable/conf
cp /stackable/tmp/hdfs/hdfs-site.xml /stackable/conf
cp /stackable/tmp/hdfs/core-site.xml /stackable/conf
cp /stackable/tmp/hbase/* /stackable/conf
cp /stackable/tmp/log_config/log4j* /stackable/conf

rm -f /stackable/log/_vector/shutdown
prepare_signal_handlers
/stackable/hbase/bin/hbase "${HBASE_ROLE_NAME}" start &
wait_for_termination $!
mkdir -p /stackable/log/_vector && touch /stackable/log/_vector/shutdown
