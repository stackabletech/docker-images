# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true;skip=InvalidDefaultArgInFrom

ARG GOLANG

FROM oci.stackable.tech/sdp/library/golang:${GOLANG} AS golang-image

FROM stackable/image/stackable-devel AS elasticsearch-exporter-builder

ARG PRODUCT
ARG PROMU
ARG RELEASE
ARG STACKABLE_USER_UID
ARG TARGETARCH
ARG TARGETOS

ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

# tar - used to archive the elasticsearch-exporter source
# git - needed by patchable and the cyclonedx-gomod tool to determine the version of elasticsearch-exporter
RUN <<EOF
microdnf update
microdnf install \
  git \
  tar
microdnf clean all
EOF

# Manually install Go since the dnf package is sometimes not recent enough
COPY --from=golang-image /usr/local/go/ /usr/local/go/
ENV PATH="/usr/local/go/bin:${PATH}"

COPY --chown=${STACKABLE_USER_UID}:0 opensearch/elasticsearch-exporter/stackable/patches/patchable.toml /stackable/src/opensearch/elasticsearch-exporter/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 opensearch/elasticsearch-exporter/stackable/patches/${PRODUCT} /stackable/src/opensearch/elasticsearch-exporter/stackable/patches/${PRODUCT}

WORKDIR /stackable

RUN <<EOF
# We use version 1.7.0, since a newer version of cyclonedx-gomod is not compatible with the version of Golang (>= 1.23.1)
go install github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@v1.7.0
# prometheus utility
go install github.com/prometheus/promu@v0.17.0

cd "$(/stackable/patchable --images-repo-root=src checkout opensearch/elasticsearch-exporter ${PRODUCT})"

ORIGINAL_VERSION=${PRODUCT}
NEW_VERSION="${PRODUCT}-stackable${RELEASE}"

# Create snapshot of the source code including custom patches
tar -czf /stackable/opensearch-elasticsearch-exporter-${NEW_VERSION}-src.tar.gz .

# Unfortunately, we need to create a dummy Git repository to allow cyclonedx-gomod to determine the version of elasticsearch-exporter
rm .git
git init
git config user.email "fake.commiter@stackable.tech"
git config user.name "Fake commiter"
git commit --allow-empty --message "Fake commit, so that we can create a tag"
git tag "v${NEW_VERSION}"
# required by promu
git remote add origin https://github.com/stackabletech/elasticsearch_exporter.git
make build
# move artifact to /stackable/*/ to copy in final image
~/go/bin/cyclonedx-gomod app -json -output-version 1.5 -output "/stackable/opensearch-elasticsearch-exporter-${NEW_VERSION}.cdx.json" -packages -files
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" "/stackable/opensearch-elasticsearch-exporter-${NEW_VERSION}.cdx.json"
# move artifact to /stackable/* to copy in final image
mv elasticsearch_exporter /stackable/elasticsearch_exporter
# set correct groups
chmod -R g=u /stackable/elasticsearch_exporter /stackable/opensearch-elasticsearch-exporter-${NEW_VERSION}-src.tar.gz /stackable/opensearch-elasticsearch-exporter-${NEW_VERSION}.cdx.json
EOF
