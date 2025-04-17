#!/bin/bash

eval "$_STACKABLE_PRE_HOOK"

containerdebug --output=/stackable/log/containerdebug-state.json --loop &

/stackable/spark/sbin/start-connect-server.sh \
--deploy-mode client \
--master k8s://https://${KUBERNETES_SERVICE_HOST}:${KUBERNETES_SERVICE_PORT_HTTPS} \
--properties-file /stackable/spark/conf/spark-defaults.conf "$@"

result=$?

eval "$_STACKABLE_POST_HOOK"

exit $result
