FROM local-image/java-devel

ARG JVMKILL_VERSION

WORKDIR /stackable

COPY --chown=0:0 java-base/jvmkill/stackable/patches/patchable.toml /stackable/src/java-base/jvmkill/stackable/patches/patchable.toml
COPY --chown=0:0 java-base/jvmkill/stackable/patches/${JVMKILL_VERSION} /stackable/src/java-base/jvmkill/stackable/patches/${JVMKILL_VERSION}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout java-base/jvmkill ${JVMKILL_VERSION})"

make

cp libjvmkill.so /stackable/libjvmkill.so

# Clean up build artifacts and temporary files to reduce image size
cd /stackable
rm -rf ./src
EOF
