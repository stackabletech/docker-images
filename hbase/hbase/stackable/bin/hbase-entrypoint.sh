#!/usr/bin/env bash
#
# Entrypoint for HBase that ensures services are shutdown gracefuly
#
# Expects the following env vars:
# - RUN_REGION_MOVER: if set to true, the region mover will be run before region server shutdown
# - REGION_MOVER_OPTS: additional options for the region mover
#
set -x
set -euo pipefail

# master, regionserver, rest
HBASE_ROLE_NAME="$1"
# 16010 for master, 16020 for regionservers etc.
HBASE_ROLE_SERVICE_PORT="$2"
# master, regionserver, rest_http, rest_https
HBASE_PORT_NAME="$3"
# ui-http or ui-https
HBASE_UI_PORT_NAME="$4"

# Needed for regionmover service and for hbase-site.xml (see below)
HBASE_SERVICE_HOST=$(cat /stackable/listener/default-address/address)

REGION_MOVER_OPTS="--regionserverhost ${HBASE_SERVICE_HOST}:${HBASE_ROLE_SERVICE_PORT} --operation unload ${REGION_MOVER_OPTS}"

prepare_signal_handlers() {
  unset term_child_pid
  unset term_kill_needed
  trap handle_term_signal TERM
}

handle_term_signal() {
  if [ "${term_child_pid}" ]; then
    if [ "regionserver" == "${HBASE_ROLE_NAME}" ] && [ "true" == "${RUN_REGION_MOVER}" ]; then
      echo "Start pre-shutdown"
      # REGION_MOVER_OPTS is a string that contains multiple arguments and needs to be spliced here
      # therefore disable shellcheck for this line
      # shellcheck disable=SC2086
      /stackable/hbase/bin/hbase org.apache.hadoop.hbase.util.RegionMover ${REGION_MOVER_OPTS}
      echo "Done pre-shutdown"
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

# Kerberos
if [ -f /stackable/kerberos/krb5.conf ]; then
  KERBEROS_REALM=$(grep -oP 'default_realm = \K.*' /stackable/kerberos/krb5.conf)
  export KERBEROS_REALM
  sed -i -e s/\$\{env\.KERBEROS_REALM\}/"${KERBEROS_REALM}"/g /stackable/conf/core-site.xml
  sed -i -e s/\$\{env\.KERBEROS_REALM\}/"${KERBEROS_REALM}"/g /stackable/conf/hbase-site.xml
  sed -i -e s/\$\{env\.KERBEROS_REALM\}/"${KERBEROS_REALM}"/g /stackable/conf/hdfs-site.xml
fi

# Service endpoints
HBASE_SERVICE_PORT=$(cat /stackable/listener/default-address/ports/"${HBASE_PORT_NAME}")
HBASE_INFO_PORT=$(cat /stackable/listener/default-address/ports/"${HBASE_UI_PORT_NAME}")
HBASE_LISTENER_ENDPOINT="$HBASE_SERVICE_HOST:$HBASE_INFO_PORT"

sed -i -e s/\$\{HBASE_SERVICE_HOST\}/"${HBASE_SERVICE_HOST}"/g /stackable/conf/hbase-site.xml
sed -i -e s/\$\{HBASE_SERVICE_PORT\}/"${HBASE_SERVICE_PORT}"/g /stackable/conf/hbase-site.xml
sed -i -e s/\$\{HBASE_LISTENER_ENDPOINT\}/"${HBASE_LISTENER_ENDPOINT}"/g /stackable/conf/hbase-site.xml
sed -i -e s/\$\{HBASE_INFO_PORT\}/"${HBASE_INFO_PORT}"/g /stackable/conf/hbase-site.xml

rm -f "${STACKABLE_LOG_DIR}/_vector/shutdown"
prepare_signal_handlers
/stackable/containerdebug --output="${STACKABLE_LOG_DIR}/containerdebug-state.json" --loop &
/stackable/hbase/bin/hbase "${HBASE_ROLE_NAME}" start &
wait_for_termination $!
mkdir -p "${STACKABLE_LOG_DIR}/_vector" && touch "${STACKABLE_LOG_DIR}/_vector/shutdown"
