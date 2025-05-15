FROM stackable/image/java-devel
ARG PRODUCT
ARG NIFI_VERSION
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /build

RUN <<EOF
mkdir -p /stackable

curl "https://github.com/stackabletech/nifi-iceberg-bundle/archive/refs/tags/${PRODUCT}.tar.gz" | tar -xzC .
cd nifi-iceberg-bundle-${PRODUCT} || exit

sed -i -e "s/{{ NIFI_VERSION }}/${NIFI_VERSION}/g" pom.xml

mvn \
    --batch-mode \
    --no-transfer-progress\
    clean package \
    -D nifi.version=${NIFI_VERSION} \
    -Dmaven.javadoc.skip=true \
    -Denforcer.skip=true
# We need "-Denforcer.skip=true", as the Maven version is too old

cp ./nifi-iceberg-services-api-nar/target/nifi-iceberg-services-api-nar-${PRODUCT}.nar /stackable
cp ./nifi-iceberg-services-nar/target/nifi-iceberg-services-nar-${PRODUCT}.nar /stackable
cp ./nifi-iceberg-processors-nar/target/nifi-iceberg-processors-nar-${PRODUCT}.nar /stackable
cp ./target/bom.json /stackable/nifi-iceberg-bundle.sbom.json

cd ..
# Save disk space, even for intermediate images
rm -rf nifi-iceberg-bundle-${PRODUCT}

# Set correct groups
chmod g=u /stackable/*.nar
chmod g=u /stackable/*.sbom.json
EOF