FROM stackable/image/java-devel

ARG PRODUCT
ARG DELETE_CACHES
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

RUN --mount=type=cache,id=maven-opa,uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
###
### OPA Authorizer (only for 2.6 upwards)
###
if [[ -n "$PRODUCT" ]]; then
    git clone --depth 1 --branch "$PRODUCT" https://github.com/stackabletech/hbase-opa-authorizer.git
    mvn \
      --batch-mode \
      --no-transfer-progress \
      -DskipTests \
      -Dmaven.test.skip=true \
      -fhbase-opa-authorizer \
      package
else
  # Create a dummy jar to avoid errors when copying it the final image
  mkdir -p hbase-opa-authorizer/target
  touch hbase-opa-authorizer/target/hbase-opa-authorizer.jar
fi

if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
fi

# set correct groups
chmod --recursive g=u /stackable
EOF