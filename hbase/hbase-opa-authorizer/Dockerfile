FROM stackable/image/java-devel

ARG PRODUCT
ARG DELETE_CACHES
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-opa-authorizer/stackable/patches/patchable.toml /stackable/src/hbase/hbase-opa-authorizer/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hbase/hbase-opa-authorizer/stackable/patches/${PRODUCT} /stackable/src/hbase/hbase-opa-authorizer/stackable/patches/${PRODUCT}

RUN --mount=type=cache,id=maven-opa,uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
###
### OPA Authorizer (only for 2.6 upwards)
###
if [[ -n "$PRODUCT" ]]; then
    cd "$(/stackable/patchable --images-repo-root=src checkout hbase/hbase-opa-authorizer ${PRODUCT})"

    # Create snapshot of the source code including custom patches
    tar -czf /stackable/hbase-opa-authorizer-${PRODUCT}-src.tar.gz .
    mvn \
      --batch-mode \
      --no-transfer-progress \
      -DskipTests \
      -Dmaven.test.skip=true \
      package
else
  # Create a dummy jar to avoid errors when copying it to the final image
  mkdir -p target
  touch target/hbase-opa-authorizer.jar
  touch /stackable/hbase-opa-authorizer-${PRODUCT}-src.tar.gz
fi

mkdir /stackable/hbase-opa-authorizer
mv target /stackable/hbase-opa-authorizer

if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
fi

# set correct groups
chmod --recursive g=u /stackable
EOF
