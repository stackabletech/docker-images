# SPIKE: Apache Airflow on Red Hat Hummingbird, without any UBI9 stage.
#
# Same approach as zookeeper.Dockerfile: named stages instead of the local-image chain, and a final
# image that is `FROM core-runtime` plus a delta. Build from the repository root (the build context
# must be the repo root):
#
#   docker build -f hummingbird-spike/airflow.Dockerfile -t oci.stackable.tech/sandbox/airflow:3.3.2-humm .
#
# The defaults below are the values of airflow 3.3.2 in airflow/boil-config.toml and its dependencies.
#
# Not covered yet:
# - Which of the packages below exist in the Hummingbird repo is partly unverified, see the comments at
#   the `dnf install` steps. The build fails on missing build dependencies and only reports missing
#   optional runtime packages.
# - nano / iputils (not in the Hummingbird repo)
# - inotify-tools is not in the Hummingbird repo either, see zookeeper.Dockerfile
# - arm64 (only amd64 was inspected)
# - Python comes from `uv` (a python-build-standalone download), as in airflow/Dockerfile. It is not an RPM,
#   so it does not appear in the rpmdb. The alternative would be the `python3.12` RPM of the Hummingbird repo.

ARG DEVEL_BASE=registry.access.redhat.com/hi/rust:latest-builder
# The base of the final image, see zookeeper.Dockerfile. Must be glibc based.
ARG RUNTIME_BASE=registry.access.redhat.com/hi/core-runtime:latest
# TODO: pin the bases by digest, like the other Dockerfiles in this repo do

ARG PRODUCT_VERSION=3.3.2
ARG RELEASE_VERSION=0.0.0-dev
ARG STACKABLE_USER_NAME=stackable
ARG STACKABLE_USER_UID=1000
ARG STACKABLE_USER_GID=1000

# airflow/boil-config.toml, version 3.3.2
ARG PYTHON_VERSION=3.14
ARG GIT_SYNC_VERSION=v4.7.1
ARG S3FS_VERSION=2026.7.0
ARG CYCLONEDX_BOM_VERSION=7.3.0
ARG CDXGEN_VERSION=13.1.0
ARG CDXGEN_NODEJS_VERSION=24.19.0
# boil.toml
ARG CDXGEN_SPEC_VERSION=1.6
ARG TINI_VERSION=0.19.0
ARG UV_VERSION=0.13.4
ARG OPA_AUTH_MANAGER=airflow-3
ARG NODEJS_VERSION=22

# shared/statsd-exporter/boil-config.toml, version 0.31.0
ARG SHARED_STATSD_EXPORTER_VERSION=0.31.0
ARG GOLANG_VERSION=1.25.14
ARG CYCLONEDX_GOMOD_VERSION=1.12.0

# stackable-base/boil-config.toml, vector/boil-config.toml and stackable-devel/boil-config.toml (the "vector-build" variant)
ARG CONFIG_UTILS_VERSION=0.5.1
ARG CONTAINERDEBUG_VERSION=0.5.0
ARG CERT_TOOLS_VERSION=0.1.1
ARG VECTOR_VERSION=0.58.0
ARG VECTOR_RUST_TOOLCHAIN_VERSION=1.95.0
ARG PROTOC_VERSION=36.2
ARG INOTIFY_TOOLS=3.22.1.0-1.el9

# The extras of airflow 3.3.2, see the links in airflow/boil-config.toml
ARG AIRFLOW_EXTRAS_CORE="async,graphviz,kerberos,memray,otel,sentry,standard,statsd"
ARG AIRFLOW_EXTRAS_META="aiobotocore,amazon-aws-auth,cloudpickle,github-enterprise,google-auth,graphviz,gunicorn,ldap,leveldb,pandas,polars,rabbitmq,s3fs,saml,uv"
ARG AIRFLOW_EXTRAS_PROVIDER_APACHE="apache-drill,apache-druid,apache-flink,apache-hdfs,apache-hive,apache-iceberg,apache-impala,apache-kafka,apache-kylin,apache-livy,apache-pig,apache-pinot,apache-tinkerpop"
ARG AIRFLOW_EXTRAS_EXTERNAL_SERVICES="airbyte,akeyless,alibaba,apprise,amazon,asana,atlassian-jira,microsoft-azure,clickhousedb,cloudant,cohere,databricks,datadog,dbt-cloud,dingding,discord,facebook,github,google,hashicorp,openai,opsgenie,pagerduty,pgvector,pinecone,qdrant,salesforce,sendgrid,segment,slack,snowflake,tableau,telegram,vertica,vespa,weaviate,yandex,ydb,zendesk"
ARG AIRFLOW_EXTRAS_LOCALLY_INSTALLED_SOFTWARE="arangodb,celery,cncf-kubernetes,docker,edge3,elasticsearch,exasol,fab,git,github,influxdb,jenkins,mongo,microsoft-mssql,neo4j,odbc,openfaas,oracle,postgres,presto,redis,samba,singularity,teradata,trino"
ARG AIRFLOW_EXTRAS_OTHER="common-ai,common-compat,common-io,common-messaging,common-sql,ftp,grpc,http,imap,jdbc,keycloak,microsoft-psrp,microsoft-winrm,openlineage,opensearch,papermill,sftp,smtp,sqlite,ssh,informatica"

# For updated versions check https://github.com/kubernetes/git-sync/releases
FROM oci.stackable.tech/sdp/git-sync/git-sync:${GIT_SYNC_VERSION} AS gitsync-image

FROM oci.stackable.tech/stackable/astral-sh/uv:${UV_VERSION} AS uv-image

FROM oci.stackable.tech/sdp/library/golang:${GOLANG_VERSION} AS golang-image

# ==============================================================================
# devel: replaces stackable-devel. Hummingbird builder variant + user + Rust + patchable
# (same as in zookeeper.Dockerfile)

FROM ${DEVEL_BASE} AS devel

ARG STACKABLE_USER_NAME
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID

# Build stage only, nothing from it is shipped as is. The final image runs as ${STACKABLE_USER_UID}.
# root is needed because the base image defaults to UID 65532.
# hadolint ignore=DL3002
USER 0
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]
# The base image sets HOME=/home/build
ENV HOME=/root
# The Rust builder image ships its own Rust and (probably) sets these to its own paths. rustup would then
# install there, and the `. "$HOME/.cargo/env"` of all the stages below would not find it.
ENV CARGO_HOME=/root/.cargo
ENV RUSTUP_HOME=/root/.rustup

COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf
COPY stackable-base/stackable/curlrc /root/.curlrc

ARG RUST_DEFAULT_TOOLCHAIN_VERSION=1.99.0
ENV RUST_DEFAULT_TOOLCHAIN_VERSION=${RUST_DEFAULT_TOOLCHAIN_VERSION}
ENV CARGO_CYCLONEDX_CRATE_VERSION=0.5.9
ENV CARGO_AUDITABLE_CRATE_VERSION=0.7.7

# tar, xz and gzip are not part of the builder image
RUN <<EOF
dnf install \
  curl diffutils findutils gcc gettext git gzip hostname make openssl-devel \
  pkgconf pkgconf-pkg-config shadow-utils tar which xz
dnf clean all

groupadd --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}
useradd \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --create-home \
  --home-dir /stackable \
  ${STACKABLE_USER_NAME}

cp /root/.curlrc /stackable/.curlrc
chown ${STACKABLE_USER_UID}:0 /stackable/.curlrc

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_DEFAULT_TOOLCHAIN_VERSION"
. "$HOME/.cargo/env"
cargo --locked --quiet install cargo-cyclonedx@"$CARGO_CYCLONEDX_CRATE_VERSION" cargo-auditable@"$CARGO_AUDITABLE_CRATE_VERSION"
EOF

# patchable: tool for patch management
COPY rust/patchable/ /patchable/rust/patchable
COPY Cargo.* /patchable/
RUN <<EOF
cd /patchable
. "$HOME/.cargo/env"
cargo auditable --quiet build --release
mv /patchable/target/release/patchable /stackable/patchable
chown ${STACKABLE_USER_UID}:0 /stackable/patchable
rm -rf /patchable
EOF

# ==============================================================================
# rust-binaries: config-utils, containerdebug, cert-tools (copied from stackable-base/Dockerfile)

FROM devel AS rust-binaries

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG CONFIG_UTILS_VERSION
ARG CONTAINERDEBUG_VERSION
ARG CERT_TOOLS_VERSION

RUN <<EOF
. "$HOME/.cargo/env"

cd /
git clone --depth 1 --branch "config-utils-${CONFIG_UTILS_VERSION}" https://github.com/stackabletech/operator-rs config-utils
git clone --depth 1 --branch "containerdebug-${CONTAINERDEBUG_VERSION}" https://github.com/stackabletech/operator-rs containerdebug
git clone --depth 1 --branch "cert-tools-${CERT_TOOLS_VERSION}" https://github.com/stackabletech/secret-operator
cd /config-utils
cargo auditable --quiet build --package config-utils --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries --no-build-deps
cd /containerdebug
cargo auditable --quiet build --package containerdebug --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries --no-build-deps
cd /secret-operator
cargo auditable --quiet build --release --package cert-tools && cargo cyclonedx --all --spec-version 1.5 --describe binaries --no-build-deps
EOF

# ==============================================================================
# vector-builder: copied from zookeeper.Dockerfile (which is copied from vector/Dockerfile).
# PRODUCT_VERSION is the Airflow version in this file, so the Vector version is called VECTOR_VERSION.

FROM devel AS vector-builder

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG VECTOR_VERSION
ARG VECTOR_RUST_TOOLCHAIN_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG PROTOC_VERSION

# See zookeeper.Dockerfile for why perl-interpreter, gawk and OPENSSL_NO_VENDOR
ENV OPENSSL_NO_VENDOR=1
RUN <<EOF
dnf install \
    cmake \
    automake \
    perl-interpreter \
    findutils \
    openssl-devel \
    pkgconf \
    pkgconf-pkg-config \
    tar \
    cyrus-sasl-devel \
    clang \
    unzip \
    `# the configure script of tikv-jemalloc-sys needs awk, which the builder image does not have` \
    gawk
dnf clean all
EOF

WORKDIR /opt/protoc
RUN ARCH=$(arch | sed 's/^aarch64$/aarch_64/') \
  && curl --fail --location --output protoc.zip "https://repo.stackable.tech/repository/packages/protoc/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip" \
  && unzip protoc.zip \
  && rm protoc.zip
ENV PROTOC=/opt/protoc/bin/protoc

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/patches/patchable.toml /stackable/src/vector/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/patches/${VECTOR_VERSION} /stackable/src/vector/stackable/patches/${VECTOR_VERSION}

RUN mkdir /app
COPY vector/copy_artifacts.sh /

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout vector ${VECTOR_VERSION})"

NEW_VERSION="${VECTOR_VERSION}-stackable${RELEASE_VERSION}"
tar -czf /stackable/vector-${NEW_VERSION}-src.tar.gz .

. "$HOME/.cargo/env"
# The devel stage has a newer default toolchain, vector uses what upstream pins
rustup default "${VECTOR_RUST_TOOLCHAIN_VERSION}"

# Keep in sync with vector/Dockerfile (single line, cargo-cyclonedx does not split on newlines)
VECTOR_FEATURES="sources-file,sources-internal_logs,transforms-remap,transforms-filter,transforms-route,sinks-vector,sinks-opentelemetry,sinks-console,sinks-blackhole,api,tikv-jemallocator"

cargo auditable --quiet build --locked --release --no-default-features --features "${VECTOR_FEATURES}"

cargo cyclonedx --all --spec-version 1.5 --describe binaries \
  --no-default-features --features "${VECTOR_FEATURES}" --no-build-deps

find target/release \
  -regextype egrep \
  -maxdepth 1 \
  -executable \
  -type f \
  ! -regex ".*\-[a-fA-F0-9]{16,16}$" \
  -exec /copy_artifacts.sh {} \;

echo "The following files will be copied to the runtime image: $(ls /app)"

chmod -R g=u /stackable
EOF

# ==============================================================================
# statsd-exporter: copied from shared/statsd-exporter/Dockerfile (only the FROM lines and the
# package installation differ; tar, gzip and git are part of the devel stage)

FROM devel AS statsd-exporter-builder

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG SHARED_STATSD_EXPORTER_VERSION
ARG CYCLONEDX_GOMOD_VERSION
ARG STACKABLE_USER_UID
ARG TARGETARCH
ARG TARGETOS

ENV CYCLONEDX_GOMOD_VERSION=$CYCLONEDX_GOMOD_VERSION
ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

WORKDIR /statsd_exporter

COPY --from=golang-image /usr/local/go/ /usr/local/go/
ENV PATH="/usr/local/go/bin:${PATH}"

COPY --chown=${STACKABLE_USER_UID}:0 shared/statsd-exporter/stackable/patches/patchable.toml /statsd_exporter/src/shared/statsd-exporter/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/statsd-exporter/stackable/patches/${SHARED_STATSD_EXPORTER_VERSION} /statsd_exporter/src/shared/statsd-exporter/stackable/patches/${SHARED_STATSD_EXPORTER_VERSION}

RUN --mount=type=cache,id=go-statsd-exporter,uid=${STACKABLE_USER_UID},target=/go_cache <<EOF
export GOPATH=/go_cache
go install "github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@v${CYCLONEDX_GOMOD_VERSION}"

(
  cd "$(/stackable/patchable --images-repo-root=src checkout shared/statsd-exporter ${SHARED_STATSD_EXPORTER_VERSION})"

  tar -czf /statsd_exporter/statsd-exporter-${SHARED_STATSD_EXPORTER_VERSION}-src.tar.gz .

  # cyclonedx-gomod needs a git repository to determine the version of statsd_exporter
  rm .git
  git init
  git config user.email "fake.commiter@stackable.tech"
  git config user.name "Fake committer"
  git commit --allow-empty --message "Fake commit, so that we can create a tag"
  git tag "v${SHARED_STATSD_EXPORTER_VERSION}"
  go build -o /statsd_exporter/statsd_exporter
  $GOPATH/bin/cyclonedx-gomod app -json -output-version 1.5 -licenses -assert-licenses -output /statsd_exporter/statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}.cdx.json
)
rm -rf "statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}"
chmod -R g=u /statsd_exporter/
EOF

# ==============================================================================
# tini: the init process of the entrypoint

FROM devel AS tini

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG TINI_VERSION
ARG TARGETARCH

RUN <<EOF
curl -o /tini "https://repo.stackable.tech/repository/packages/tini/tini-${TINI_VERSION}-${TARGETARCH}"
chmod +x /tini
EOF

# ==============================================================================
# opa-auth-manager: builds the wheel and runs its tests (copied from airflow/Dockerfile)

FROM devel AS opa-auth-manager-builder

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG OPA_AUTH_MANAGER
ARG PRODUCT_VERSION
ARG PYTHON_VERSION
ARG STACKABLE_USER_UID

COPY airflow/opa-auth-manager/${OPA_AUTH_MANAGER} /tmp/opa-auth-manager

# The tests need the same constraints the image is built with, see airflow/Dockerfile.
COPY airflow/stackable/constraints/${PRODUCT_VERSION}/constraints-python${PYTHON_VERSION}.txt /tmp/constraints.txt

WORKDIR /tmp/opa-auth-manager

COPY --from=uv-image --chown=${STACKABLE_USER_UID}:0 /uv /uvx /bin/

RUN <<EOF
# This folder is required by the tests to set up an sqlite database
mkdir /root/airflow

uv sync --locked --python "${PYTHON_VERSION}"

uv pip install \
  --python .venv/bin/python \
  --constraint /tmp/constraints.txt \
  "apache-airflow==${PRODUCT_VERSION}" \
  apache-airflow-providers-fab

uv run --no-sync pytest --disable-warnings

uv build
EOF

# ==============================================================================
# airflow-build-image: copied from airflow/Dockerfile. Differences: the package installation, and the
# comments are shortened (the original ones apply)

FROM devel AS airflow-build-image

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG PRODUCT_VERSION
ARG PYTHON_VERSION
ARG TARGETARCH
ARG STACKABLE_USER_UID
ARG S3FS_VERSION
ARG CYCLONEDX_BOM_VERSION
ARG CDXGEN_VERSION
ARG CDXGEN_NODEJS_VERSION
ARG CDXGEN_SPEC_VERSION

ARG AIRFLOW_EXTRAS_CORE
ARG AIRFLOW_EXTRAS_META
ARG AIRFLOW_EXTRAS_PROVIDER_APACHE
ARG AIRFLOW_EXTRAS_EXTERNAL_SERVICES
ARG AIRFLOW_EXTRAS_LOCALLY_INSTALLED_SOFTWARE
ARG AIRFLOW_EXTRAS_OTHER

COPY --from=uv-image --chown=${STACKABLE_USER_UID}:0 /uv /uvx /bin/

# Package names are those of the Hummingbird repo, the Fedora/RHEL names where the images of the
# Hummingbird repo do not install them:
# - cyrus-sasl-devel, krb5-devel, gcc-c++, openssl-devel, make, diffutils, jq: seen in the repo or the findings
# - libpq-devel, openldap-devel, cyrus-sasl-gssapi: checked with `dnf repoquery`, 2026-11-27
# - nodejs22-bin and npm12 are what the Hummingbird nodejs:22 builder image installs. They replace
#   `microdnf module enable nodejs:22` + nodejs + npm (modules do not exist here). `NODEJS_VERSION` is
#   therefore not used, change the package names to change the Node version.
# - unixODBC is left out: it is not in the repo as far as seen. The comment in airflow/Dockerfile
#   says the odbc provider compiles without the headers.
RUN <<EOF
dnf install \
    cyrus-sasl-devel \
    `# Needed for kerberos` \
    cyrus-sasl-gssapi \
    krb5-devel \
    `# Needed to build gevent, see airflow/Dockerfile` \
    gcc-c++ \
    libpq-devel \
    openldap-devel \
    `# Needed for the Airflow UI assets` \
    nodejs22-bin \
    npm12 \
    `# Needed to modify the SBOM` \
    jq \
    `# The Rust builder image has no awk (the extras are de-duplicated with it), sed or grep` \
    gawk \
    grep \
    sed
dnf clean all
EOF

# cdxgen creates the SBOM for the frontend (npm) dependencies of the Airflow UI, see airflow/Dockerfile.
RUN <<EOF
ARCH="${TARGETARCH/amd64/x64}"
mkdir -p /opt/node-cdxgen
curl "https://repo.stackable.tech/repository/packages/node/node-v${CDXGEN_NODEJS_VERSION}-linux-${ARCH}.tar.xz" | \
    tar --extract --xz --directory=/opt/node-cdxgen --strip-components=1
PATH="/opt/node-cdxgen/bin:$PATH" npm install --global --ignore-scripts --before="$(date --utc --date='-7 days' --iso-8601=seconds)" "@cdxgen/cdxgen@${CDXGEN_VERSION}"
EOF

COPY airflow/stackable/constraints/${PRODUCT_VERSION}/constraints-python${PYTHON_VERSION}.txt /tmp/constraints.txt
COPY airflow/stackable/constraints/${PRODUCT_VERSION}/build-constraints-python${PYTHON_VERSION}.txt /tmp/build-constraints.txt
COPY --from=opa-auth-manager-builder /tmp/opa-auth-manager/dist/opa_auth_manager-0.1.0-py3-none-any.whl /tmp/

COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/patches/patchable.toml /stackable/src/airflow/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 airflow/stackable/patches/${PRODUCT_VERSION} /stackable/src/airflow/stackable/patches/${PRODUCT_VERSION}

WORKDIR /stackable

ENV UV_CACHE_DIR=/var/cache/uv
ENV UV_LINK_MODE=copy
# Keeps the uv-managed Python inside /stackable/, so that it is part of the COPY --from in the rootfs stage
ENV UV_PYTHON_INSTALL_DIR=/stackable/python

# This installs the Python runtime
# --managed-python: this base has a system Python (/usr/sbin/python3.14) that uv would otherwise prefer. The venv
# has to use the uv-managed one in /stackable/python, which is copied to the final image.
RUN <<EOF
uv venv --managed-python --python "${PYTHON_VERSION}" --system-site-packages /stackable/app

# See airflow/Dockerfile for why pip is upgraded (GHSA-6q48-6c7w-9gw2)
uv pip install --python "$(uv python find --system --managed-python "${PYTHON_VERSION}")" --break-system-packages pip==26.2.2
EOF

ENV VIRTUAL_ENV=/stackable/app

RUN --mount=type=cache,target=/var/cache/uv <<EOF
AIRFLOW_EXTRAS="$AIRFLOW_EXTRAS_CORE,$AIRFLOW_EXTRAS_META,$AIRFLOW_EXTRAS_PROVIDER_APACHE,$AIRFLOW_EXTRAS_EXTERNAL_SERVICES,$AIRFLOW_EXTRAS_LOCALLY_INSTALLED_SOFTWARE,$AIRFLOW_EXTRAS_OTHER"
AIRFLOW_EXTRAS=$(echo "$AIRFLOW_EXTRAS" | tr ',' '\n' | awk 'NF > 0 {if (!seen[$0]++) print $0}' | tr '\n' ',' | sed 's/,$//')

# Pin virtualenv due to a breaking change in 21.0.0 which is pulled in by hatch.
# See https://github.com/pypa/hatch/issues/2193
uv tool install hatch --with "virtualenv<21"

cd "$(/stackable/patchable --images-repo-root=src checkout airflow ${PRODUCT_VERSION})"

tar -czf /stackable/airflow-${PRODUCT_VERSION}-src.tar.gz .

cd airflow-core/src/airflow/ui

# build front-end assets
npm install -g --ignore-scripts --before="$(date --utc --date='-7 days' --iso-8601=seconds)" pnpm@10.18.2
pnpm install --frozen-lockfile
pnpm run build

# SBOM for the frontend assets, see airflow/Dockerfile for the options
PATH="/opt/node-cdxgen/bin:$PATH" cdxgen \
  --type js \
  --required-only \
  --no-babel \
  --spec-version "${CDXGEN_SPEC_VERSION}" \
  --project-version "${PRODUCT_VERSION}" \
  --output "/stackable/app/airflow-ui-${PRODUCT_VERSION}.cdx.json"

# build airflow wheel from airflow root folder
cd ../../..
/root/.local/bin/hatch build -t wheel
uv pip install --python ${VIRTUAL_ENV}/bin/python apache-airflow[${AIRFLOW_EXTRAS}]==${PRODUCT_VERSION} --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt
uv pip install --python ${VIRTUAL_ENV}/bin/python dist/apache_airflow_core-${PRODUCT_VERSION}-py3-none-any.whl[${AIRFLOW_EXTRAS}] --constraint /tmp/constraints.txt --build-constraints /tmp/build-constraints.txt

uv pip install --python ${VIRTUAL_ENV}/bin/python s3fs==${S3FS_VERSION}
uv pip install --python ${VIRTUAL_ENV}/bin/python Flask_OIDC==2.2.0 Flask-OpenID==1.3.1

uv pip install --python ${VIRTUAL_ENV}/bin/python /tmp/opa_auth_manager-0.1.0-py3-none-any.whl

# Create the SBOM for Airflow
# Important: All `pip install` commands must be above this line, otherwise the SBOM will be incomplete
uv venv /tmp/sbom-venv
uv pip install --python /tmp/sbom-venv/bin/python cyclonedx-bom==${CYCLONEDX_BOM_VERSION}
/tmp/sbom-venv/bin/cyclonedx-py environment --spec-version 1.5 --output-file /tmp/sbom.json "${VIRTUAL_ENV}"
rm -rf /tmp/sbom-venv

# Post-process the SBOM, see airflow/Dockerfile for the reasons
jq 'def normalize_name: ascii_downcase | gsub("[-_.]+"; "-");
.dependencies |= map(if .ref | test("^apache-airflow-providers-") then
    .dependsOn |= map(select(. != "apache-airflow=='${PRODUCT_VERSION}'"))
else
    .
end)
| .components |= map(if (.purl | not) and (.name != null) and (.version != null) then
    .purl = "pkg:pypi/" + (.name | normalize_name) + "@" + .version
else
    .
end
| if (.purl // "") | startswith("pkg:pypi/") then
    .purl |= sub("^pkg:pypi/(?<name>[^@]+)"; "pkg:pypi/" + (.name | normalize_name))
else
    .
end)' /tmp/sbom.json > /stackable/app/airflow-${PRODUCT_VERSION}.cdx.json

cd /stackable
rm -rf ./src
EOF

RUN <<EOF
mkdir -pv /stackable/airflow
mkdir -pv /stackable/airflow/dags
mkdir -pv /stackable/airflow/logs
chmod --recursive g=u /stackable
EOF

# ==============================================================================
# runtime-base: the base of the final image, only used to copy its filesystem

FROM ${RUNTIME_BASE} AS runtime-base

# ==============================================================================
# rootfs: assembles the runtime filesystem into /rootfs
# (replaces stackable-base + vector + the final RUN step of airflow).
# See zookeeper.Dockerfile for why this is a copy of the runtime base plus `dnf --installroot`, and for
# the `additions` part at the end.

FROM ${DEVEL_BASE} AS rootfs

ARG PRODUCT_VERSION
ARG STACKABLE_USER_NAME
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG TARGETARCH
ARG INOTIFY_TOOLS
ARG SHARED_STATSD_EXPORTER_VERSION

# hadolint ignore=DL3002
USER 0
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]
ENV HOME=/root

COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf

COPY --from=runtime-base / /rootfs

# The packages of the runtime stage of airflow/Dockerfile, plus the ones of stackable-base that
# are not nano/iputils (findutils, less, tar). The tools that the scripts in the image use are added
# explicitly because core-runtime does not have them (grep, sed, find, tar, less, curl).
#
# - libstdc++: entrypoint.sh preloads /usr/lib64/libstdc++.so.6 (checked below)
# - krb5-libs: the `kerberos` extra links it
# - cyrus-sasl-lib is the Hummingbird name of what UBI9 splits into cyrus-sasl and cyrus-sasl-lib
# - openssl-pkcs11, socat and unixODBC are not in the Hummingbird repo (checked with `dnf repoquery`, 2026-11-27).
#   The second transaction only reports which of them are missing, so that one missing package does not stop the build.
# The stage's SHELL is bash, hadolint does not take it into account for the array here
# hadolint ignore=SC3054
RUN <<EOF
dnf install tar xz diffutils findutils grep shadow-utils

dnf --installroot=/rootfs --use-host-config install \
  curl findutils gawk git grep krb5-libs less libpq libstdc++ openssl-libs sed tar \
  cyrus-sasl-lib openldap openldap-clients openssh-clients

OPTIONAL=(openssl-pkcs11 socat unixODBC)
dnf --installroot=/rootfs --use-host-config install --skip-unavailable "${OPTIONAL[@]}"
for p in "${OPTIONAL[@]}"; do
  rpm --root /rootfs -q "$p" > /dev/null || echo "WARNING: optional package $p is not in the Hummingbird repo, it is missing from the image"
done

# See zookeeper.Dockerfile: the full package replaces the ca-certificates-bundle of the runtime base
dnf --installroot=/rootfs --use-host-config install --allowerasing ca-certificates

# Unlike in the ZooKeeper image, the user has to have a passwd entry: Airflow resolves the user name
# (getpass.getuser() falls back to the passwd database) and entrypoint.sh only creates an entry if
# /etc/passwd is writable. `-R` edits the files below /rootfs, shadow-utils is not installed there.
groupadd -R /rootfs --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}
useradd -R /rootfs \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --no-create-home \
  --home-dir /stackable \
  ${STACKABLE_USER_NAME}
mkdir -p /rootfs/stackable

dnf clean all
EOF

# The vector sidecar waits for a shutdown file using `inotifywait`, see zookeeper.Dockerfile
RUN <<EOF
rpm --root /rootfs --import https://dl.fedoraproject.org/pub/epel/RPM-GPG-KEY-EPEL-9
rpm --root /rootfs -q gpg-pubkey-ff8ad1344597106ece813b918a3872bf3228467c

ARCH="${TARGETARCH/amd64/x86_64}"
ARCH="${ARCH/arm64/aarch64}"
rpm --root /rootfs --install \
    "https://repo.stackable.tech/repository/packages/inotify-tools/inotify-tools-${INOTIFY_TOOLS}.${ARCH}.rpm"
EOF

# CVE-2023-37920: the Hummingbird ca-certificates package still contains the e-Tugra roots
COPY stackable-base/stackable/ca-cert-blocklist/ /rootfs/etc/pki/ca-trust/source/blocklist/
RUN <<EOF
chroot /rootfs update-ca-trust
if [ "$(chroot /rootfs trust list --filter=ca-anchors | chroot /rootfs grep -c 'E-Tugra')" != "0" ]; then
  echo "Still found E-Tugra root certificates, this should not happen!"
  exit 1;
fi

# The checks and the link run in the chroot because these are absolute symlinks that would resolve against the build host.
# On Hummingbird /etc/pki/tls/certs only has OpenSSL hash links, not the ca-bundle.crt of UBI. The system bundle is
# generated at /etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem. The link restores the UBI path, which
# GRPC_DEFAULT_SSL_ROOTS_FILE_PATH (see the final stage) and DAGs that hardcode it use.
chroot /rootfs test -s /etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem
chroot /rootfs ln -s /etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem /etc/pki/tls/certs/ca-bundle.crt
chroot /rootfs test -s /etc/pki/tls/certs/ca-bundle.crt

# entrypoint.sh sets LD_PRELOAD to this file
chroot /rootfs test -e /usr/lib64/libstdc++.so.6 || { echo "missing: /usr/lib64/libstdc++.so.6"; ls -l /rootfs/usr/lib64/libstdc* || true; exit 1; }
EOF

COPY stackable-base/stackable/curlrc /rootfs/stackable/.curlrc

# Rust utilities, licenses, permission check script
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /config-utils/target/release/config-utils /rootfs/stackable/config-utils
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /config-utils/crates/config-utils/config-utils_bin.cdx.xml /rootfs/stackable/config-utils.cdx.xml
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /containerdebug/target/release/containerdebug /rootfs/stackable/containerdebug
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /containerdebug/crates/containerdebug/containerdebug_bin.cdx.xml /rootfs/stackable/containerdebug_bin.cdx.xml
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /secret-operator/target/release/cert-tools /rootfs/stackable/cert-tools
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /secret-operator/rust/cert-tools/cert-tools_bin.cdx.xml /rootfs/stackable/cert-tools_bin.cdx.xml
COPY --chown=${STACKABLE_USER_UID}:0 shared/checks/check-permissions-ownership.sh /rootfs/usr/bin/check-permissions-ownership.sh

# vector (same layering order for /licenses as the original chain: vector, airflow)
COPY --chown=${STACKABLE_USER_UID}:0 vector/licenses /rootfs/licenses
COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/bin /rootfs/stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 --from=vector-builder /app/* /rootfs/usr/local/bin/

# Airflow
COPY airflow/licenses /rootfs/licenses
# Like in airflow/Dockerfile this takes everything below /stackable of the build image
COPY --from=airflow-build-image --chown=${STACKABLE_USER_UID}:0 /stackable/ /rootfs/stackable/

COPY --chown=${STACKABLE_USER_UID}:0 --chmod=0755 airflow/stackable/utils/entrypoint.sh /rootfs/entrypoint.sh
COPY --chown=${STACKABLE_USER_UID}:0 --chmod=0755 airflow/stackable/utils/run-airflow.sh /rootfs/run-airflow.sh

COPY --from=statsd-exporter-builder --chown=${STACKABLE_USER_UID}:0 /statsd_exporter/statsd_exporter /rootfs/stackable/statsd_exporter
COPY --from=statsd-exporter-builder --chown=${STACKABLE_USER_UID}:0 /statsd_exporter/statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}.cdx.json /rootfs/stackable/statsd_exporter-${SHARED_STATSD_EXPORTER_VERSION}.cdx.json
COPY --from=gitsync-image --chown=${STACKABLE_USER_UID}:0 /git-sync /rootfs/stackable/git-sync

COPY --from=uv-image --chown=${STACKABLE_USER_UID}:0 /uv /uvx /rootfs/usr/bin/
COPY --from=tini --chmod=0755 /tini /rootfs/usr/bin/tini

RUN <<EOF
# dnf state and caches have no use in the final image (the rpmdb is kept)
rm -rf /rootfs/var/cache /rootfs/var/lib/dnf /rootfs/var/log/*

chown -R ${STACKABLE_USER_UID}:0 /rootfs/stackable
chmod -R g=u /rootfs/stackable

chroot /rootfs /usr/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# The `additions` part, see zookeeper.Dockerfile
# hadolint ignore=SC3001
RUN --mount=type=bind,from=runtime-base,target=/base <<EOF
mkdir /additions
cd /rootfs
meta() { stat -c '%u:%g:%a' "$1"; }
: > /tmp/delta.lst
while IFS= read -r -d '' p; do
  b="/base/$p"
  if [ ! -e "$b" ] && [ ! -L "$b" ]; then
    printf '%s\0' "$p" >> /tmp/delta.lst
  elif [ -L "$p" ]; then
    if [ ! -L "$b" ] || [ "$(readlink "$p")" != "$(readlink "$b")" ]; then printf '%s\0' "$p" >> /tmp/delta.lst; fi
  elif [ -d "$p" ]; then
    if [ ! -d "$b" ] || [ "$(meta "$p")" != "$(meta "$b")" ]; then printf '%s\0' "$p" >> /tmp/delta.lst; fi
  elif [ -f "$p" ]; then
    if [ ! -f "$b" ] || [ "$(meta "$p")" != "$(meta "$b")" ] || ! cmp -s "$p" "$b"; then printf '%s\0' "$p" >> /tmp/delta.lst; fi
  else
    printf '%s\0' "$p" >> /tmp/delta.lst
  fi
done < <(find . -mindepth 1 -print0)

echo "Entries added or changed compared to the runtime base: $(tr -cd '\0' < /tmp/delta.lst | wc -c)"
tar -C /rootfs --null --no-recursion -T /tmp/delta.lst -cf - | tar -C /additions --same-owner -xpf -

# Files of the runtime base that are replaced (preflight may report them as modified files)
echo "Files of the runtime base that were changed:"
tr '\0' '\n' < /tmp/delta.lst | while IFS= read -r p; do
  if [ -e "/base/$p" ] && [ ! -d "/base/$p" ]; then echo "  $p"; fi
done
EOF

# ==============================================================================
# FINAL AIRFLOW IMAGE

# hadolint ignore=DL3006,DL3007
FROM ${RUNTIME_BASE}

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

COPY --from=rootfs /additions /

LABEL name="Apache Airflow" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT_VERSION}" \
    release="${RELEASE_VERSION}" \
    summary="The Stackable image for Apache Airflow." \
    description="This image is deployed by the Stackable Operator for Apache Airflow."

LABEL org.opencontainers.image.authors="info@stackable.tech"
LABEL org.opencontainers.image.url="https://stackable.tech"
LABEL org.opencontainers.image.source="https://github.com/stackabletech/docker-images/"
LABEL org.opencontainers.image.vendor="Stackable GmbH"
LABEL org.opencontainers.image.licenses="Apache-2.0"

# C.UTF-8 is built into glibc >= 2.35, see zookeeper.Dockerfile
ENV LANG=C.UTF-8
ENV HOME=/stackable
ENV VIRTUAL_ENV=$HOME/app
ENV AIRFLOW_USER_HOME_DIR=$HOME
ENV PATH=$HOME/app/bin:$PATH:/bin:/stackable
ENV AIRFLOW_HOME=$HOME/airflow
# grpcio bundles its own roots.pem instead of using the system trust store. Point it to the system
# trust store, so it does not trust the compromised "emSign ECC Root CA - G3" root certificate
# (GHSA-6q48-6c7w-9gw2), which the system trust store no longer contains.
ENV GRPC_DEFAULT_SSL_ROOTS_FILE_PATH=/etc/pki/tls/certs/ca-bundle.crt

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

ENTRYPOINT ["/usr/bin/tini", "--", "/run-airflow.sh"]
CMD []
