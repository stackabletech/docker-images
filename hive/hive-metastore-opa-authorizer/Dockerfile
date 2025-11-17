# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/hadoop/hadoop AS hadoop-builder

FROM local-image/java-devel

ARG AUTHORIZER_VERSION
ARG HIVE_VERSION
ARG HADOOP_HADOOP_VERSION
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP_VERSION}
ARG STACKABLE_USER_UID
# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 hive/hive-metastore-opa-authorizer/stackable/patches/patchable.toml /stackable/src/hive/hive-metastore-opa-authorizer/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hive/hive-metastore-opa-authorizer/stackable/patches/${AUTHORIZER_VERSION} /stackable/src/hive/hive-metastore-opa-authorizer/stackable/patches/${AUTHORIZER_VERSION}

COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/patched-libs /stackable/patched-libs

RUN --mount=type=cache,id=maven-hive-metastore-opa-authorizer-${AUTHORIZER_VERSION},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
# for moving nested artifacts out of target folder
mkdir -p /stackable/opa-authorizer-bin
# for moving sources out of target folder
mkdir -p /stackable/opa-authorizer-src

cd "$(/stackable/patchable --images-repo-root=src checkout hive/hive-metastore-opa-authorizer ${AUTHORIZER_VERSION})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/opa-authorizer-src/hive-metastore-opa-authorizer-${AUTHORIZER_VERSION}-hive-${HIVE_VERSION}-hadoop-${HADOOP_VERSION}-src.tar.gz .

# Make Maven aware of custom Stackable libraries
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

# Set version in the output jars(s)
mvn versions:set -DnewVersion=${AUTHORIZER_VERSION}

# The if part can be removed once we do no longer support Hive 3.x.x
if [[ "${HIVE_VERSION}" =~ ^3 ]]; then
  mvn clean package -DskipTests -Dhive.version=${HIVE_VERSION} -Dhadoop.version=${HADOOP_VERSION} -f hms-v3/pom.xml

  mv hms-v3/target/com.bosch.bdps.hms3-${HIVE_VERSION}-${HADOOP_VERSION}-dev.jar /stackable/opa-authorizer-bin
else
  mvn clean package -DskipTests -Dhive.version=${HIVE_VERSION} -Dhadoop.version=${HADOOP_VERSION} -f hms-v4/pom.xml

  # The hive-metastore-opa-authorizer offers a shaded jar from version 4.x.x. Using the shaded jar leads to problems with schema tool at pod startup.
  # mv hms-v4/target/com.bosch.bdps.hms4-${HIVE_VERSION}-${HADOOP_VERSION}-dev.jar /stackable/opa-authorizer-bin
  mv hms-v4/target/hms4-dev.jar /stackable/opa-authorizer-bin
fi

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
  rm -rf /stackable/.npm/*
  rm -rf /stackable/.cache/*
  rm -rf /stackable/src
fi

# fix permissions
chmod --recursive g=u /stackable/opa-authorizer-bin
EOF
