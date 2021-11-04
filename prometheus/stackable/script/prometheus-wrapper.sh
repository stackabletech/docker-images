#!/usr/bin/env sh
#
# Prometheus wrapper that patches te kubeconfig_file setting for service discovery
# with the contents of the KUBECONFIG environment variable.
#

#set -x

# Returns the value of --config.file=... command line argument if it exists
prometheus_config_file() {
  PROMETHEUS_CONFIG=""
  for opt in ${@}
  do
    PROMETHEUS_CONFIG=$(echo $opt | awk '/--config.file=.+/' | sed 's/--config.file=//')
    test ${PROMETHEUS_CONFIG} && break
  done
  echo ${PROMETHEUS_CONFIG}
}

# Returns the kube config file to use for prometheus discovery.
# Currently it only returns the contents of KUBECONFIG env var, but it can be extended to look
# in additional places for it.
kube_config_file() {
  echo ${KUBECONFIG}
}

substiute_kubeconfig_file() {
  KUBECONFIG_FILE=$1
  PROMETHEUS_CONFIG=$2

  TMP_CONFIG_NAME=$(mktemp)
  if test ${PROMETHEUS_CONFIG}; then
    cat ${PROMETHEUS_CONFIG} | sed s#kubeconfig_file:.*#kubeconfig_file:\ ${KUBECONFIG_FILE}# > ${TMP_CONFIG_NAME}
    ### Overwrite the original prometheus config file (is this a good idea?)
    mv ${TMP_CONFIG_NAME} ${PROMETHEUS_CONFIG}
  fi
}

# Assumes prometheus binary is in the same folder as this script
run_prometheus() {
  $(dirname $0)/prometheus $@
}

# main
{
  PC=$(prometheus_config_file $@)
  if test ${PC}; then
    KC=$(kube_config_file)

    if test $KC; then
      substiute_kubeconfig_file $KC $PC
    else
      >&2 echo 'No kubeconfig file found.'
      exit 1
    fi
  fi
  run_prometheus $@
}


