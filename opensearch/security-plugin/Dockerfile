# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/java-devel AS security-plugin-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 opensearch/security-plugin/stackable/patches/patchable.toml /stackable/src/opensearch/security-plugin/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch/security-plugin/stackable/patches/${PRODUCT_VERSION} /stackable/src/opensearch/security-plugin/stackable/patches/${PRODUCT_VERSION}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout opensearch/security-plugin ${PRODUCT_VERSION})"

# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-security-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz .

# Starting with version 3.6.0.0, the security plugin defaults to crypto.standard=FIPS-140-3 (gradle.properties),
# which makes the build treat the BouncyCastle FIPS jars as compileOnly, expecting the OpenSearch core
# distribution to provide them in lib/.
# The Stackable OpenSearch build is not a FIPS build, so it does not ship those jars, and the plugin fails
# at startup with NoClassDefFoundError: org/bouncycastle/jcajce/provider/BouncyCastleFipsProvider.
# Build in non-FIPS mode (-Pcrypto.standard=any-supported) so the plugin bundles its own BouncyCastle jars.
./gradlew clean assemble -Dbuild.snapshot=false -Pcrypto.standard=any-supported
./gradlew cyclonedxBom --warning-mode=summary -Dbuild.snapshot=false
EOF

RUN <<EOF
# Change the group permissions already in the builder image to reduce the size of the final image.
# see https://github.com/stackabletech/docker-images/issues/961
chmod -R g=u /stackable
EOF
