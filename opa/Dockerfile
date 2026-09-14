# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true;skip=InvalidDefaultArgInFrom

ARG GOLANG_VERSION
FROM oci.stackable.tech/sdp/library/golang:${GOLANG_VERSION} AS golang-image

FROM local-image/stackable-devel AS multilog-builder

ARG DAEMONTOOLS_VERSION=0.76

COPY opa/daemontools /daemontools

RUN <<EOF
microdnf update
microdnf install \
  gcc \
  gzip \
  make \
  patch \
  tar
microdnf clean all
rm -rf /var/cache/yum
EOF

RUN <<EOF
cd /daemontools
tar xzf daemontools-${DAEMONTOOLS_VERSION}.tar.gz

cd /daemontools/admin/daemontools-${DAEMONTOOLS_VERSION}/src
patch < /daemontools/conf-cc.patch
patch multilog.c < /daemontools/multilog_max_file_size.patch

cd /daemontools/admin/daemontools-${DAEMONTOOLS_VERSION}
package/install

# set correct groups
chmod g=u /daemontools/admin/daemontools/command/multilog
EOF

FROM local-image/stackable-devel AS opa-builder

ARG CYCLONEDX_GOMOD_VERSION
ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG TARGETARCH
ARG TARGETOS

ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

# gzip, tar - used to unpack the OPA source
# git - needed by the cyclonedx-gomod tool to determine the version of OPA
RUN <<EOF
microdnf update
microdnf install \
  git \
  gzip \
  tar
microdnf clean all
EOF

COPY --chown=${STACKABLE_USER_UID}:0 opa/stackable/bin /stackable/opa/bin

# Manually install Go since the dnf package is sometimes not recent enough
COPY --from=golang-image /usr/local/go/ /usr/local/go/
ENV PATH="/usr/local/go/bin:${PATH}"

COPY --chown=${STACKABLE_USER_UID}:0 opa/stackable/patches/patchable.toml /stackable/src/opa/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opa/stackable/patches/${PRODUCT_VERSION} /stackable/src/opa/stackable/patches/${PRODUCT_VERSION}

WORKDIR /stackable

RUN <<EOF
go install "github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@v${CYCLONEDX_GOMOD_VERSION}"

cd "$(/stackable/patchable --images-repo-root=src checkout opa ${PRODUCT_VERSION})"

ORIGINAL_VERSION=${PRODUCT_VERSION}
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

sed -i 's/var Version = "'${ORIGINAL_VERSION}'"/var Version = "'${NEW_VERSION}'"/g' v1/version/version.go

# Create snapshot of the source code including custom patches
tar -czf /stackable/opa-${NEW_VERSION}-src.tar.gz .

# Unfortunately, we need to create a dummy Git repository to allow cyclonedx-gomod to determine the version of OPA
rm .git
git init
git config user.email "fake.commiter@stackable.tech"
git config user.name "Fake commiter"
git commit --allow-empty --message "Fake commit, so that we can create a tag"
git tag "v${NEW_VERSION}"
go build -o opa -buildmode=exe
# move artifact to /stackable/*/ to copy in final image
# -licenses detects the license of every module from the LICENSE file in the
# module cache, and -assert-licenses puts the result into `licenses` instead of
# `evidence`. syft and mergebom only read `licenses`, so without
# -assert-licenses none of it reaches the published image SBOM.
# -packages and -files are deliberately not used. They describe the module at Go
# package and file level, which multiplies the component count without adding anything
# a vulnerability scanner can use.
# -std is not used either: cyclonedx-gomod emits the standard library as
# `pkg:golang/std@<go version>` without a license, while syft already reports it
# as `pkg:golang/stdlib@<version>` with BSD-3-Clause. The purls differ, so
# mergebom cannot deduplicate them and the image would carry a license-less
# duplicate.
~/go/bin/cyclonedx-gomod app -json -output-version 1.5 -licenses -assert-licenses -output /stackable/opa/"opa_${NEW_VERSION}.cdx.json"
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" /stackable/opa/"opa_${NEW_VERSION}.cdx.json"
# move artifact to /stackable/* to copy in final image
mv opa /stackable/opa/
# set correct groups
chmod -R g=u /stackable/opa /stackable/opa-${NEW_VERSION}-src.tar.gz
EOF

FROM local-image/vector

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

LABEL name="Open Policy Agent" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT_VERSION}" \
  release="${RELEASE_VERSION}" \
  summary="The Stackable image for Open Policy Agent (OPA)." \
  description="This image is deployed by the Stackable Operator for OPA."

COPY --chown=${STACKABLE_USER_UID}:0 opa/licenses /licenses

COPY --from=opa-builder --chown=${STACKABLE_USER_UID}:0 /stackable/opa /stackable/opa
COPY --from=opa-builder --chown=${STACKABLE_USER_UID}:0 /stackable/opa-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /stackable/opa-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz
COPY --from=multilog-builder --chown=${STACKABLE_USER_UID}:0 /daemontools/admin/daemontools/command/multilog /stackable/multilog

RUN <<EOF
microdnf update
# jq: Required for filtering logs
microdnf install \
  jq
microdnf clean all
rm -rf /var/cache/yum

# fix missing permissions
chmod g=u /stackable/opa
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/opa

ENV PATH="${PATH}:/stackable/opa:/stackable/opa/bin"

CMD ["opa", "run", "-s"]
