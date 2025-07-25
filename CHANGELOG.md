# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed

- all: Use our build-repo to cache NPM dependencies ([#1219])

[#1219]: https://github.com/stackabletech/docker-images/pull/1219

## [25.7.0] - 2025-07-23

## [25.7.0-rc1] - 2025-07-18

### Added

- airflow: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1054]).
- airflow: Add `2.10.5` ([#1108]).
- airflow: Add `3.0.1` ([#1122]).
- druid: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1039]).
- druid: Add `33.0.0` ([#1110]).
- hadoop: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1029]).
- hbase: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1028]).
- hbase: provide patches to implement listener endpoints ([#1159]).
- hive: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1040]).
- spark-connect-client: A new image for Spark connect tests and demos ([#1034])
- kafka: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1041]).
- kafka: build kafka-opa-plugin from source ([#1177]).
- nifi: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1027]).
- nifi: Add [nifi-iceberg-bundle] for NiFi `2.4.0` ([#1060], [#1106]).
- nifi: Add `2.4.0` ([#1114]).
- nifi: Add git-sync ([#1107]).
- opa: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1038]).
- opa: Add `1.4.2` ([#1103]).
- spark-k8s: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1055]).
- superset: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1053]).
- superset: Add version `4.1.2` ([#1102]).
- trino: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1025]).
- trino: Add `476` ([#1095]).
- trino-storage-connector: Add `476` ([#1095]).
- zookeeper: check for correct permissions and ownerships in /stackable folder via
  `check-permissions-ownership.sh` provided in stackable-base image ([#1043]).
- nifi: Build and add OPA authorizer plugin nar ([#1058]).
- nifi: Add [nifi-iceberg-bundle](https://github.com/stackabletech/nifi-iceberg-bundle) for NiFi `2.2.0` ([#1060], [#1106]).
- java: Add JDK 24 ([#1097]).
- ci: Add golang image to mirror workflow ([#1103]).
- omid: bump version to 1.1.3 ([#1105]).
- hbase: add 2.6.2 and upgrade dependencies ([#1101]).
- kafka: Add `4.0.0` ([#1117]).
- Include `.tar.gz` snapshots of the product source code in container images ([#1126]).
- airflow: OPA authorizer for Airflow 3.x ([#1127]).
- kafka: Add `3.9.1` ([#1149]).
- spark-k8s: Add `3.5.6` ([#1142]).
- spark-connect-client: Add `3.5.6` ([#1142]).
- git-sync: Bump version to 4.4.1 ([#1151]).
- zookeeper: bump jetty version for CVE-2024-13009 in 3.9.3 ([#1179])
- zookeeper: bump netty version for CVE-2025-24970 in 3.9.3 ([#1180])
- hadoop: backport HADOOP-19352, HADOOP-19335, HADOOP-19465, HADOOP-19456 and HADOOP-19225 to fix vulnerabilities in Hadoop `3.4.1` ([#1184])
- hadoop: Backport HADOOP-18583 to make OpenSSL 3.x work with the native hadoop libraries ([#1209]).
- spark: backport [SPARK-51311] Promote bcprov-jdk18on to compile scope ([#1212]).

### Changed

- airflow,superset: Use `uv` to build the product ([#1116]).
- ubi-rust-builder: Bump Rust toolchain to 1.85.0, cargo-cyclonedx to 0.5.7, and cargo-auditable to 0.6.6 ([#1050]).
- ubi9-rust-builder: Bump base image and update protoc to `30.2` ([#1091], [#1163]).
- stackable-base: Bump ubi9 base image ([#1163]).
- stackable-devel: Bump ubi9 base image ([#1103], [#1137], [#1163]).
- spark-k8s: Include spark-connect jars, replace OpenJDK with Temurin JDK, cleanup ([#1034]).
- spark-connect-client: Image is now completely based on spark-k8s and includes JupyterLab and other demo dependencies ([#1071]).
- jmx_exporter: Bump products to use `1.3.0` ([#1090], [#1156]).
- kubectl: Bump products to use `1.33.0` ([#1090]).
- yq: Bump products to use `4.45.2` ([#1090]).
- cyclonedx-bom: Bump airflow and superset to use `6.0.0` ([#1090]).
- trino-cli: Bump to `476` ([#1095]).
- vector: Bump to `0.46.1` ([#1098]).
- spark: update dependencies for 3.5.5 ([#1094]).
- nifi: include NAR SBOMs ([#1119]).
- nifi: update patch allowing to bypass host header validation starting with NiFi 2.4.0 ([#1125]).
- BREAKING: kcat: Stop building kcat image ([#1124]).
- containerdebug updated to 0.2.0 ([#1128]).
- Build Hadoop as `stackable` and configure the Stackable Nexus build-repo for the `root` user ([#1133]).
- patchable: The base branch is now configured as the git upstream branch ([#1131]).
- airflow: Updates the entrypoint script and removes the check for GID == 0 ([#1138]).
- druid: Bump druiod-opa-authorizer to `0.7.0` ([#1139]).
- vector: Bump to `0.47.0` ([#1152]).
- zookeeper: backport ZOOKEEPER-4846, ZOOKEEPER-4921, ZOOKEEPER-4925 into Zookeeper 3.9.3 ([#1150]).
- testing-tools: Update base image ([#1165]).
- trino: Enable custom versions ([#1168]).
- zookeeper: Enable custom versions ([#1169]).
- opa: Enable custom versions ([#1170]).
- use custom product versions for Hadoop, HBase, Phoenix, hbase-operator-tools, Druid, Hive and Spark ([#1173]).
- hbase: Bump dependencies to the latest patch level for HBase `2.6.1` and `2.6.2` ([#1185]).
- hadoop: Separate Dockerfiles for Hadoop build and HDFS image ([#1186]).
- ubi-rust-builder: Bump Rust toolchain to 1.87.0, cargo-auditable to 0.7.0 and protoc to 31.1 ([#1197]).
- stackable-base, stackable-devel, ubi-rust-builder: Update `ubi-minimal` base image ([#1197]).
- testing-tools: Update `python` 3.12-slim-bullseye base image ([#1197]).

### Fixed

- airflow: Pin Cython version ([#1116]).
- druid: reduce docker image size by removing the recursive chown/chmods in the final image ([#1039]).
- hadoop: reduce docker image size by removing the recursive chown/chmods in the final image ([#1029]).
- hadoop: adapt the JMX exporter configuration to also export boolean metrics ([#1140]).
- hbase: reduce docker image size by removing the recursive chown/chmods in the final image ([#1028]).
- hive: reduce docker image size by removing the recursive chown/chmods in the final image ([#1040]).
- kafka: reduce docker image size by removing the recursive chown/chmods in the final image ([#1041]).
- Add `--locked` flag to `cargo install` commands for reproducible builds ([#1044]).
- nifi: reduce docker image size by removing the recursive chown/chmods in the final image ([#1027]).
- opa: reduce docker image size by removing the recursive chown/chmods in the final image ([#1038]).
- opa: Manually install Go 1.23.9 ([#1103]).
- spark-k8s: reduce docker image size by removing the recursive chown/chmods in the final image ([#1042]).
- superset: Pin Cython version ([#1116]).
- trino: reduce docker image size by removing the recursive chown/chmods in the final image ([#1025]).
- zookeeper: reduce docker image size by removing the recursive chown/chmods in the final image ([#1043]).
- Fixed two hardcoded username references ([#1052]).
- ubi9-rust-builder: Use pinned `rustup` version ([#1121]).
- hive: Patch for postgres CVE-2024-1597 ([#1100]).
- bump image-tools (for `bake`) and nixpkgs (for `nodejs_20`, used by pre-commit) ([#1100]).
- bump image-tools (for `bake`) to fix `RELEASE` arg ([#1188]).
- nifi: automatically determine NiFi version create reporting task script ([#1189]).

### Removed

- ci: Remove Nexus steps from build, mirror and release workflows ([#1056]).
  Also remove the old release workflow.
- trino: Remove `455` ([#1095]).
- trino-storage-connector: Remove `455` ([#1095]).
- zookeeper: Remove 3.9.2 ([#1093]).
- Remove ubi8-rust-builder image ([#1091]).
- spark: remove 3.5.2 ([#1094]).
- hadoop: Remove `3.3.4` and `3.4.0` ([#1099]).
- opa: Remove `0.67.1` ([#1103]).
- opa: Remove legacy bundle-builder from container build ([#1103]).
- omid: Remove 1.1.3-SNAPSHOT ([#1105]).
- hbase: Remove 2.4.18 ([#1101])
- druid: Remove `30.0.0` ([#1110]).
- nifi: Remove `2.2.0` ([#1114]).
- kafka: Remove `3.7.1` and `3.8.0` ([#1117]).
- spark-connect-client: Remove `3.5.5` ([#1142]).
- nifi: Enable custom versions ([#1172]).
- kafka: Enable custom versions ([#1171]).
- omid: Enable custom versions ([#1174]).

[nifi-iceberg-bundle]: https://github.com/stackabletech/nifi-iceberg-bundle
[#1025]: https://github.com/stackabletech/docker-images/pull/1025
[#1027]: https://github.com/stackabletech/docker-images/pull/1027
[#1028]: https://github.com/stackabletech/docker-images/pull/1028
[#1029]: https://github.com/stackabletech/docker-images/pull/1029
[#1034]: https://github.com/stackabletech/docker-images/pull/1034
[#1038]: https://github.com/stackabletech/docker-images/pull/1038
[#1039]: https://github.com/stackabletech/docker-images/pull/1039
[#1040]: https://github.com/stackabletech/docker-images/pull/1040
[#1041]: https://github.com/stackabletech/docker-images/pull/1041
[#1042]: https://github.com/stackabletech/docker-images/pull/1042
[#1043]: https://github.com/stackabletech/docker-images/pull/1043
[#1044]: https://github.com/stackabletech/docker-images/pull/1044
[#1050]: https://github.com/stackabletech/docker-images/pull/1050
[#1052]: https://github.com/stackabletech/docker-images/pull/1052
[#1053]: https://github.com/stackabletech/docker-images/pull/1053
[#1054]: https://github.com/stackabletech/docker-images/pull/1054
[#1055]: https://github.com/stackabletech/docker-images/pull/1055
[#1056]: https://github.com/stackabletech/docker-images/pull/1056
[#1058]: https://github.com/stackabletech/docker-images/pull/1058
[#1060]: https://github.com/stackabletech/docker-images/pull/1060
[#1090]: https://github.com/stackabletech/docker-images/pull/1090
[#1091]: https://github.com/stackabletech/docker-images/pull/1091
[#1093]: https://github.com/stackabletech/docker-images/pull/1093
[#1094]: https://github.com/stackabletech/docker-images/pull/1094
[#1095]: https://github.com/stackabletech/docker-images/pull/1095
[#1097]: https://github.com/stackabletech/docker-images/pull/1097
[#1098]: https://github.com/stackabletech/docker-images/pull/1098
[#1099]: https://github.com/stackabletech/docker-images/pull/1099
[#1100]: https://github.com/stackabletech/docker-images/pull/1100
[#1101]: https://github.com/stackabletech/docker-images/pull/1101
[#1102]: https://github.com/stackabletech/docker-images/pull/1102
[#1103]: https://github.com/stackabletech/docker-images/pull/1103
[#1105]: https://github.com/stackabletech/docker-images/pull/1105
[#1106]: https://github.com/stackabletech/docker-images/pull/1106
[#1107]: https://github.com/stackabletech/docker-images/pull/1107
[#1108]: https://github.com/stackabletech/docker-images/pull/1108
[#1110]: https://github.com/stackabletech/docker-images/pull/1110
[#1114]: https://github.com/stackabletech/docker-images/pull/1114
[#1116]: https://github.com/stackabletech/docker-images/pull/1116
[#1117]: https://github.com/stackabletech/docker-images/pull/1117
[#1119]: https://github.com/stackabletech/docker-images/pull/1119
[#1121]: https://github.com/stackabletech/docker-images/pull/1121
[#1122]: https://github.com/stackabletech/docker-images/pull/1122
[#1124]: https://github.com/stackabletech/docker-images/pull/1124
[#1125]: https://github.com/stackabletech/docker-images/pull/1125
[#1126]: https://github.com/stackabletech/docker-images/pull/1126
[#1127]: https://github.com/stackabletech/docker-images/pull/1127
[#1128]: https://github.com/stackabletech/docker-images/pull/1128
[#1131]: https://github.com/stackabletech/docker-images/pull/1131
[#1133]: https://github.com/stackabletech/docker-images/pull/1133
[#1137]: https://github.com/stackabletech/docker-images/pull/1137
[#1138]: https://github.com/stackabletech/docker-images/pull/1138
[#1139]: https://github.com/stackabletech/docker-images/pull/1139
[#1142]: https://github.com/stackabletech/docker-images/pull/1142
[#1149]: https://github.com/stackabletech/docker-images/pull/1149
[#1150]: https://github.com/stackabletech/docker-images/pull/1150
[#1151]: https://github.com/stackabletech/docker-images/pull/1151
[#1152]: https://github.com/stackabletech/docker-images/pull/1152
[#1156]: https://github.com/stackabletech/docker-images/pull/1156
[#1159]: https://github.com/stackabletech/docker-images/pull/1159
[#1163]: https://github.com/stackabletech/docker-images/pull/1163
[#1165]: https://github.com/stackabletech/docker-images/pull/1165
[#1168]: https://github.com/stackabletech/docker-images/pull/1168
[#1169]: https://github.com/stackabletech/docker-images/pull/1169
[#1170]: https://github.com/stackabletech/docker-images/pull/1170
[#1171]: https://github.com/stackabletech/docker-images/pull/1171
[#1173]: https://github.com/stackabletech/docker-images/pull/1173
[#1174]: https://github.com/stackabletech/docker-images/pull/1174
[#1177]: https://github.com/stackabletech/docker-images/pull/1177
[#1179]: https://github.com/stackabletech/docker-images/pull/1179
[#1180]: https://github.com/stackabletech/docker-images/pull/1180
[#1184]: https://github.com/stackabletech/docker-images/pull/1184
[#1185]: https://github.com/stackabletech/docker-images/pull/1185
[#1186]: https://github.com/stackabletech/docker-images/pull/1186
[#1188]: https://github.com/stackabletech/docker-images/pull/1188
[#1189]: https://github.com/stackabletech/docker-images/pull/1189
[#1197]: https://github.com/stackabletech/docker-images/pull/1197
[#1209]: https://github.com/stackabletech/docker-images/pull/1209
[#1212]: https://github.com/stackabletech/docker-images/pull/1212

## [25.3.0] - 2025-03-21

### Added

- omid: Added 1.1.3-SNAPSHOT to allow for easier scanning pre-release
- airflow: Add OPA support to Airflow ([#978]).
- nifi: Activate `include-hadoop` profile for NiFi version 2 ([#958]).
- nifi: Add NiFi hadoop Azure and GCP libraries ([#943]).
- superset: Add role mapping from OPA ([#979]).
- base: Add containerdebug tool ([#928], [#959]).
- tools: Add the package util-linux-core ([#952]).
  util-linux-core contains a basic set of Linux utilities, including the
  command logger which allows to enter messages into the system log.
- vector: Add version 0.43.1 ([#980]).
- druid: Add version 30.0.1 and 31.0.1 ([#984])
- opa: Add version 1.0.1 ([#981], [#1000]).
- statsd-exporter: Bump version to 0.28.0 ([#982]).
- git-sync: Bump version to 4.4.0 ([#990]).
- kafka: Add versions 3.7.2 and 3.9.0 ([#988]).
- java: Add JDK/JRE 23 ([#992]).
- trino: Add 469 ([#993]).
- trino-cli: Add version 469 ([#993]).
- hbase: Support for 2.6.1 ([#997]).
- trino-storage-connector: Add version 469 ([#996]).
- trino: Add 470 ([#999]).
- trino-cli: Add version 470 ([#999]).
- trino-storage-connector: Add version 470 ([#999]).
- superset: Add version `4.1.1` ([#991]).
- Add Patchable patch management tool ([#1003], [#1005], [#1007], [#1026]).
- nifi: Add 1.28.1, 2.2.0 ([#1006]).
- airflow: Add 2.10.4 ([#1021]).
- hadoop: Add 3.4.1, use jmx_export 1.1.0 ([#1021]).
- hive: Add 4.0.1, use jmx_export 1.1.0 ([#1021]).
- zookeeper: Add 3.9.3, use jmx_export 1.1.0 ([#1021]).
- hbase: Use jmx_export 1.1.0 ([#1021]).
- omid: Use jmx_export 1.1.0 ([#1021]).
- spark: Add 3.5.5 ([#1022]).
- trino: Add libstdc++ package, needed by snappy and duckdb ([#1015]).
- hive: Revert the removal of 4.0.0 ([#1031]).

### Changed

- stackable-base: Bump cargo-cyclonedx to 0.5.7 ([#1013]).
- Update registry references to oci ([#989]).
- trino-storage-connector: Move the build out of trino/ for easier patching ([#996]).
- phoenix: Bump 5.2.0 to 5.2.1 ([#997], [#1009]).
- BREAKING: druid: Bump opa-authorizer to 0.6.0 for all versions ([#984]).

### Removed

- kafka: Remove `kubectl`, as we are now using listener-op ([#884]).
- vector: remove version 0.41.1 ([#980]).
- opa: remove version 0.66.0 ([#981]).
- trino: Remove 469 ([#999]).
- trino-cli: Remove version 469 ([#999]).
- trino-storage-connector: Remove version 469 ([#999]).
- nifi: Remove 2.0.0 ([#1006]).
- druid: Remove 26.0.0 ([#984])
- airflow: Remove 2.9.2, and 2.10.2 ([#1021]).
- hive: Remove 4.0.0 ([#1021]).
- spark: Remove 3.5.1 ([#1022]).

### Fixed

- stackable-base: Install missing rust toolchains ([#1013]).
- opa: Install missing rust toolchains ([#1013]).
- druid: Fix CVE-2023-34455 in Druid `30.0.0` by deleting a dependency ([#935]).
- hadoop: Fix the JMX exporter configuration for metrics suffixed with
  `_total`, `_info` and `_created` ([#962]).
- nix: Move `pkg-config` to nativeBuildInputs ([#1021]).
- hadoop: Use the fast mirror in the download script ([#1021]).

[#884]: https://github.com/stackabletech/docker-images/pull/884
[#928]: https://github.com/stackabletech/docker-images/pull/928
[#943]: https://github.com/stackabletech/docker-images/pull/943
[#958]: https://github.com/stackabletech/docker-images/pull/958
[#959]: https://github.com/stackabletech/docker-images/pull/959
[#935]: https://github.com/stackabletech/docker-images/pull/935
[#962]: https://github.com/stackabletech/docker-images/pull/962
[#978]: https://github.com/stackabletech/docker-images/pull/978
[#979]: https://github.com/stackabletech/docker-images/pull/979
[#980]: https://github.com/stackabletech/docker-images/pull/980
[#981]: https://github.com/stackabletech/docker-images/pull/981
[#982]: https://github.com/stackabletech/docker-images/pull/982
[#984]: https://github.com/stackabletech/docker-images/pull/984
[#988]: https://github.com/stackabletech/docker-images/pull/988
[#989]: https://github.com/stackabletech/docker-images/pull/989
[#990]: https://github.com/stackabletech/docker-images/pull/990
[#991]: https://github.com/stackabletech/docker-images/pull/991
[#992]: https://github.com/stackabletech/docker-images/pull/992
[#993]: https://github.com/stackabletech/docker-images/pull/993
[#996]: https://github.com/stackabletech/docker-images/pull/996
[#997]: https://github.com/stackabletech/docker-images/pull/997
[#999]: https://github.com/stackabletech/docker-images/pull/999
[#1000]: https://github.com/stackabletech/docker-images/pull/1000
[#1003]: https://github.com/stackabletech/docker-images/pull/1003
[#1005]: https://github.com/stackabletech/docker-images/pull/1005
[#1006]: https://github.com/stackabletech/docker-images/pull/1006
[#1007]: https://github.com/stackabletech/docker-images/pull/1007
[#1009]: https://github.com/stackabletech/docker-images/pull/1009
[#1013]: https://github.com/stackabletech/docker-images/pull/1013
[#1015]: https://github.com/stackabletech/docker-images/pull/1015
[#1021]: https://github.com/stackabletech/docker-images/pull/1021
[#1022]: https://github.com/stackabletech/docker-images/pull/1022
[#1026]: https://github.com/stackabletech/docker-images/pull/1026

## [24.11.1] - 2025-01-14

### Changed

- ci: Fix various static analysis errors ([#955]).
- all java products: These now use the Stackable Nexus build-repo by default instead of pulling from Maven central ([#953]).
- all java products: Maven is now consistently run with `--batch-mode` and `--no-transfer-progress` to reduce noise ([#953]).

### Added

- tools: Add the package util-linux-core ([#952]).
  util-linux-core contains a basic set of Linux utilities, including the
  command logger which allows to enter messages into the system log.

[#952]: https://github.com/stackabletech/docker-images/pull/952
[#953]: https://github.com/stackabletech/docker-images/pull/953
[#955]: https://github.com/stackabletech/docker-images/pull/955

## [24.11.0] - 2024-11-18

### Added

- ci: Add mirror workflow, add new helper scripts ([#819]).
- opa: Add version `0.67.1` ([#797]).
- vector: Add version `0.41.1` ([#802], [#815], [#867]).
- airflow: Add version `2.9.3` ([#809]).
- airflow: Add version `2.10.2` ([#877]).
- kafka: Add version `3.8.0` ([#813]).
- hive: Add version `4.0.0` ([#818]).
- trino: Add version `455` ([#822]).
- trino-cli: Add version `455` ([#822]).
- spark: Add version `3.5.2` ([#848]).
- statsd-exporter: Bump version to `0.27.1` ([#866], [#879]).
- hadoop: Add patch "HADOOP-18516: Support Fixed SAS Token for ABFS Authentication" ([#852]).
- hbase: Add hadoop-azure.jar to the lib directory to support the Azure Blob Filesystem and
  the Azure Data Lake Storage ([#853]).
- kafka: Add cyrus-sasl-gssapi package for kerberos ([#874]).
- spark: Add HBase connector ([#878], [#882]).
- hbase: hbase-entrypoint.sh script to start and gracefully stop services ([#898]).
- tools: install yq command line tool for YAML manipulation ([#912]).
- nifi: Add version 2.0.0 ([#917]).

### Changed

- ci: Rename local actions, adjust action inputs and outputs, add definition
  README file ([#819]).
- Update cargo-cyclonedx to 0.5.5 and build CycloneDX 1.5 files ([#783]).
- Enable [Docker build checks](https://docs.docker.com/build/checks/) ([#872]).
- java: migrate to temurin jdk/jre ([#894]).
- tools: bump kubectl to `1.31.1` and jq to `1.7.1` ([#896]).
- Make username, user id, group id configurable, use numeric ids everywhere, change group of all files to 0 ([#849], [#890], [#897]).
- ci: Bump `stackabletech/actions` to 0.2.0 ([#901], [#903], [#907], [#910], [#913]).
- ubi-rust-builder: Bump Rust toolchain to 1.81.0 ([#902]).
- ci: Handle release builds in the same build workflows ([#913]).
- hadoop: Bump to `hdfs-utils` 0.4.0 ([#914]).
- superset: Fix `CVE-2024-1135` by upgrading `gunicorn` from 21.2.0 to 22.0.0 ([#919]).
- jmx_exporter: Updated to a custom-built version of 1.0.1 to fix performance regressions ([#920]).

### Removed

- opa: Remove version `0.61.0` ([#797]).
- vector: Remove version `0.39.0` ([#802]).
- airflow: Remove versions `2.6.3`, `2.8.1`, `2.8.4` ([#809]).
- kafka: Remove versions `3.4.1`, `3.6.1`, `3.6.2` ([#813]).
- trino: Remove versions `414`, `442` ([#822]).
- trino-cli: Remove version `451` ([#822]).
- hbase: Remove `2.4.17` ([#846]).
- omid: Remove `1.1.0` and `1.1.1` ([#846]).
- spark: Remove `3.4.2` and `3.4.3` ([#848]).
- statsd-exporter: Remove `0.26.1` ([#866]).
- superset: Remove `2.1.3`, `3.1.0` and `3.1.3` ([#866]).
- zookeeper: Remove `3.8.4` ([#851]).
- nifi: Remove `1.21.0` and `1.25.0` ([#868]).
- druid: Remove `28.0.1` ([#880]).
- nifi: Removed binaries from upload nifi script ([#917]).

### Fixed

- hbase: link to phoenix server jar ([#811]).
- spark: Fix CVE-2024-36114 in Spark 3.5.1 by upgrading a dependency.
  Spark 3.5.2 is not affected. ([#921])
- trino: Correctly report Trino version ([#881]).
- hive: Fix CVE-2024-36114 in Hive `3.1.3` and `4.0.0` by upgrading a dependency. ([#922]).
- nifi: Fix CVE-2024-36114 in NiFi `1.27.0` and `2.0.0` by upgrading a dependency. ([#924]).
- hbase: Fix CVE-2024-36114 in HBase `2.6.0` by upgrading a dependency. ([#925]).
- druid: Fix CVE-2024-36114 in Druid `26.0.0` and `30.0.0` by upgrading a dependency ([#926]).
- hbase: Fix CVE-2023-34455 in HBase `2.4.18` by upgrading a dependency. ([#934]).

[#783]: https://github.com/stackabletech/docker-images/pull/783
[#797]: https://github.com/stackabletech/docker-images/pull/797
[#802]: https://github.com/stackabletech/docker-images/pull/802
[#809]: https://github.com/stackabletech/docker-images/pull/809
[#811]: https://github.com/stackabletech/docker-images/pull/811
[#813]: https://github.com/stackabletech/docker-images/pull/813
[#815]: https://github.com/stackabletech/docker-images/pull/815
[#818]: https://github.com/stackabletech/docker-images/pull/818
[#819]: https://github.com/stackabletech/docker-images/pull/819
[#822]: https://github.com/stackabletech/docker-images/pull/822
[#846]: https://github.com/stackabletech/docker-images/pull/846
[#848]: https://github.com/stackabletech/docker-images/pull/848
[#849]: https://github.com/stackabletech/docker-images/pull/849
[#851]: https://github.com/stackabletech/docker-images/pull/851
[#852]: https://github.com/stackabletech/docker-images/pull/852
[#853]: https://github.com/stackabletech/docker-images/pull/853
[#866]: https://github.com/stackabletech/docker-images/pull/866
[#867]: https://github.com/stackabletech/docker-images/pull/867
[#868]: https://github.com/stackabletech/docker-images/pull/868
[#872]: https://github.com/stackabletech/docker-images/pull/872
[#874]: https://github.com/stackabletech/docker-images/pull/874
[#877]: https://github.com/stackabletech/docker-images/pull/877
[#878]: https://github.com/stackabletech/docker-images/pull/878
[#879]: https://github.com/stackabletech/docker-images/pull/879
[#880]: https://github.com/stackabletech/docker-images/pull/880
[#881]: https://github.com/stackabletech/docker-images/pull/881
[#882]: https://github.com/stackabletech/docker-images/pull/882
[#890]: https://github.com/stackabletech/docker-images/pull/890
[#894]: https://github.com/stackabletech/docker-images/pull/894
[#896]: https://github.com/stackabletech/docker-images/pull/896
[#897]: https://github.com/stackabletech/docker-images/pull/897
[#898]: https://github.com/stackabletech/docker-images/pull/898
[#901]: https://github.com/stackabletech/docker-images/pull/901
[#902]: https://github.com/stackabletech/docker-images/pull/902
[#903]: https://github.com/stackabletech/docker-images/pull/903
[#907]: https://github.com/stackabletech/docker-images/pull/907
[#910]: https://github.com/stackabletech/docker-images/pull/910
[#912]: https://github.com/stackabletech/docker-images/pull/912
[#913]: https://github.com/stackabletech/docker-images/pull/913
[#914]: https://github.com/stackabletech/docker-images/pull/914
[#917]: https://github.com/stackabletech/docker-images/pull/917
[#919]: https://github.com/stackabletech/docker-images/pull/919
[#920]: https://github.com/stackabletech/docker-images/pull/920
[#921]: https://github.com/stackabletech/docker-images/pull/921
[#922]: https://github.com/stackabletech/docker-images/pull/922
[#924]: https://github.com/stackabletech/docker-images/pull/924
[#925]: https://github.com/stackabletech/docker-images/pull/925
[#926]: https://github.com/stackabletech/docker-images/pull/926
[#934]: https://github.com/stackabletech/docker-images/pull/934

## [24.7.0] - 2024-07-24

### Added

- omid: Add version `1.1.1` & `1.1.2` ([#553]).
- ubi9-rust-builder: A builder image using UBI9 instead of the current UBI8 ([#583]).
- Build all `0.0.0-dev` product images as multi-arch and push them to Nexus and Harbor.
  Also SBOMs are generated and everything is signed ([#614], [#616]).
- hbase: Enable snapshot exports to S3; The HBase image depends now on
  the Hadoop image. The required AWS JARs are copied from the Hadoop
  image to the HBase image. The script `export-snapshot-to-s3` makes
  exporting easier ([#621]).
- kafka: Build from source ([#659], [#661]).
- kafka: Add jmx broker config to builder image ([#703]).
- nifi: Build from source ([#678]).
- omid: Include Apache Omid in all workflows such as building and releasing images ([#635]).
- java-devel: New image to serve as base layer for builder stages ([#665]).
- stackable-base: Mitigate CVE-2023-37920 by removing e-Tugra root certificates ([#673]).
- hadoop: Exclude YARN and Mapreduce projects from build ([#667]).
- hadoop: Exclude unused jars and mitigate snappy-java CVEs by bumping dependency ([#682]).
- hadoop: Add version `3.4.0` ([#743]).
- druid: Build from source ([#684], [#696]).
- opa: Add log processing script to opa for decision logging ([#695], [#704]).
- hbase: Added new image for 2.6.0 including the new OPA authorizer ([#701]).
- stackable-base: Add [config-utils](https://github.com/stackabletech/config-utils) ([#706]).
- omid: Include Apache Omid Examples to simplify testing ([#721]).
- nifi: Add support for 1.27.0 and 2.0.0-M4 ([#744], [#767]).
- kafka: Add versions `3.6.2` and `3.7.1` ([#745]).
- trino & trino-cli: Add version 451 ([#758]).
- airflow: Add version `2.8.4` and `2.9.2` ([#762]).
- superset: Add version `3.1.3` and `4.0.2` ([#768]).
- druid: Support for 30.0.0 using Java 17 ([#731]).
- hbase: Support for HBase 2.4.18 ([#740]).
- spark-k8s: Support for `3.5.1` using Java 17 ([#771]).
- airflow & superset: Build `statsd_exporter` from source ([#777]).

### Changed

- all: Switched all product builds from UBI8 to UBI9.4 ([#628])
- hbase: Remove the symlink `/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar`
  which is unused since SDP 23.11 ([#621]).
- hive: Only build and ship Hive metastore. This reduces the image size from `2.63GB` to `1.9GB` and should also reduce the number of dependencies ([#619], [#622]).
- ubi8-rust-builder: Bump `protoc` from `21.5` to `26.1` ([#624]).
- pass platform argument to preflight check ([#626]).
- nifi: provision stackable-bcrypt from Maven ([#663])
- nifi: move /bin/stackable-bcrypt to /stackable/stackable-bcrypt and added softlink for backwards compatibility ([#678]).
- nifi: patch nifi-assembly pom file to not zip binaries after the build to save disk space ([#685]).
- hadoop: use java-devel as base layer for the builder stage ([#665])
- hive: use java-devel as base layer for the builder stage ([#665])
- zookeeper: use java-devel as base layer for the builder stage ([#665])
- hbase: use java-devel as base layer for the builder stage ([#665])
- omid: use java-devel as base layer for the builder stage ([#665])
- kafka: use java-devel as base layer for the builder stage ([#665])
- opa-bundle-builder: Bump image to 1.1.2 ([#666])
- opa: Build from source ([#676])
- trino: Build from source ([#687]).
- spark: Build from source ([#679])
- all: Moved the LOG4J_FORMAT_MSG_NO_LOOKUPS env variable from the individual Dockerfiles to `java-base` and `java-devel` ([#727])
- all: Move product versions into product directory in preparation for individual product build workflows ([#732])
- all: Bump rustc 1.79.0, protoc 27.2, git-sync 4.2.3, statsd-exporter 0.26.1, vector 0.39.0, jmx-exporter 1.0.1, inotify_tools 3.22.1.0-1.el9 ([#624], [#737])

### Fixed

- superset: Let Superset 3.1.0 build on ARM by adding `make` and `diffutils` ([#611]).
- airflow: Let Airflow 2.8.x and 2.9.x build on ARM by adding `make` and `diffutils` ([#612]).
- python:3.11 manifest list fixed. Added proper hash ([#613]).
- trino-cli: Include the trino-cli in the CI build process ([#614]).
- hive: Fix compilation on ARM by back-porting [HIVE-21939](https://issues.apache.org/jira/browse/HIVE-21939) from [this](https://github.com/apache/hive/commit/2baf21bb55fcf33d8522444c78a8d8cab60e7415) commit ([#617]).
- hive: Fix compilation on ARM in CI as well ([#619]).
- hive: Fix compilation of x86 in CI due to lower disk usage to prevent disk running full ([#619]).
- hive: Provide logging dependency previously bundled with the hadoop yarn client ([#688]).
- all: Use correct hbase versions ([#734])
- airflow: fix missing libstdc++.so.6 error message when running the image ([#778])

### Removed

- zookeeper: Remove unsupported version 3.8.3 and 3.9.1 ([#628], [#736]).
- java-base: Remove openjdk-devel rpm package again to reduce the vulnerability surface ([#665])
- trino: Remove unsupported version 428 ([#687]).
- nifi: Remove unsupported version 1.23.2 ([#744]).
- kafka: Remove unsupported version `3.5.2` ([#745]).
- airflow: Remove unsupprted version `2.7.2`, `2.7.3` and `2.8.3` ([#762]).
- superset: Remove version `2.1.1`, `3.0.1` and `3.0.3` ([#768]).
- druid: Remove support for 27.0.0 ([#731])
- spark-k8s: Remove support for `3.4.1` and `3.5.0` ([#771]).

[#583]: https://github.com/stackabletech/docker-images/pull/583
[#611]: https://github.com/stackabletech/docker-images/pull/611
[#612]: https://github.com/stackabletech/docker-images/pull/612
[#613]: https://github.com/stackabletech/docker-images/pull/613
[#614]: https://github.com/stackabletech/docker-images/pull/614
[#616]: https://github.com/stackabletech/docker-images/pull/616
[#617]: https://github.com/stackabletech/docker-images/pull/617
[#619]: https://github.com/stackabletech/docker-images/pull/619
[#621]: https://github.com/stackabletech/docker-images/pull/621
[#622]: https://github.com/stackabletech/docker-images/pull/622
[#624]: https://github.com/stackabletech/docker-images/pull/624
[#626]: https://github.com/stackabletech/docker-images/pull/626
[#628]: https://github.com/stackabletech/docker-images/pull/628
[#635]: https://github.com/stackabletech/docker-images/pull/635
[#659]: https://github.com/stackabletech/docker-images/pull/659
[#661]: https://github.com/stackabletech/docker-images/pull/661
[#663]: https://github.com/stackabletech/docker-images/pull/663
[#665]: https://github.com/stackabletech/docker-images/pull/665
[#666]: https://github.com/stackabletech/docker-images/pull/666
[#667]: https://github.com/stackabletech/docker-images/pull/667
[#673]: https://github.com/stackabletech/docker-images/pull/673
[#676]: https://github.com/stackabletech/docker-images/pull/676
[#678]: https://github.com/stackabletech/docker-images/pull/678
[#679]: https://github.com/stackabletech/docker-images/pull/679
[#682]: https://github.com/stackabletech/docker-images/pull/682
[#684]: https://github.com/stackabletech/docker-images/pull/684
[#685]: https://github.com/stackabletech/docker-images/pull/685
[#687]: https://github.com/stackabletech/docker-images/pull/687
[#688]: https://github.com/stackabletech/docker-images/pull/688
[#696]: https://github.com/stackabletech/docker-images/pull/696
[#695]: https://github.com/stackabletech/docker-images/pull/695
[#701]: https://github.com/stackabletech/docker-images/pull/701
[#703]: https://github.com/stackabletech/docker-images/pull/703
[#704]: https://github.com/stackabletech/docker-images/pull/704
[#706]: https://github.com/stackabletech/docker-images/pull/706
[#721]: https://github.com/stackabletech/docker-images/pull/721
[#727]: https://github.com/stackabletech/docker-images/pull/727
[#731]: https://github.com/stackabletech/docker-images/pull/731
[#732]: https://github.com/stackabletech/docker-images/pull/732
[#734]: https://github.com/stackabletech/docker-images/pull/734
[#736]: https://github.com/stackabletech/docker-images/pull/736
[#737]: https://github.com/stackabletech/docker-images/pull/737
[#740]: https://github.com/stackabletech/docker-images/pull/740
[#743]: https://github.com/stackabletech/docker-images/pull/743
[#744]: https://github.com/stackabletech/docker-images/pull/744
[#745]: https://github.com/stackabletech/docker-images/pull/745
[#758]: https://github.com/stackabletech/docker-images/pull/758
[#762]: https://github.com/stackabletech/docker-images/pull/762
[#767]: https://github.com/stackabletech/docker-images/pull/767
[#768]: https://github.com/stackabletech/docker-images/pull/768
[#553]: https://github.com/stackabletech/docker-images/pull/553
[#771]: https://github.com/stackabletech/docker-images/pull/771
[#777]: https://github.com/stackabletech/docker-images/pull/777
[#778]: https://github.com/stackabletech/docker-images/pull/778

## [24.3.0] - 2024-03-20

### Added

- omid: init at 1.1.0 ([#493]).
- hadoop: Allow datanodes to override their registration addresses ([#506], [#544]).
- hadoop: Add async-profiler and backport HADOOP-18055 and HADOOP-18077
  to support it ([#540]).
- hadoop: Add `tar` package, so that `kubectl cp` can be used to copy
  log files and profiler flamegraphs ([#540]).
- hbase: Add async-profiler and backport HBASE-28242 to support it
  ([#540]).
- hbase: Allow multiple certificates in the KeyStores which is required for
  rotating CA certificates. Because of this, HBASE-27027 was backported to
  HBase version 2.4.12 ([#540]).
- nifi: Add Apache Iceberg extensions ([#529]).
- testing-tools: Add krb5-user library for Kerberos tests ([#531]).
- testing-tools: Add the Python library Beautiful Soup 4 ([#536]).
- java-base: Add `openjdk-devel` package for tool such as `jps` or `jmap` ([#537]).
- java-base: Add JDK 21 ([#547]).
- airflow: Add `2.7.3`, `2.8.1`, `2.8.3` ([#562], [#593]).
- druid: Add `28.0.1` ([#558]).
- kafka: Add `3.5.2`, `3.6.1` ([#559]).
- nifi: Add version `1.25.0` using java 21 ([#552]).
- opa: Add version `0.61.0` ([#538]).
- spark: Add version `3.4.2` ([#560]).
- superset: Add version `2.1.3`,`3.0.3`,`3.1.0` ([#563]).
- trino: Add version `442` ([#597]).
- vector: Switch from version `0.33.0` to `0.35.0` ([#547], [#549]).
- zookeeper: Add version `3.8.4` ([#591]).
- zookeeper: Add version `3.9.1`, `3.9.2` ([#551], [#592]).
- hadoop: Add hdfs-utils ([#566]).
- testing-tools: add pytest `8.0.1` ([#575]).
- trino-cli: Command line for Trino version 442 ([#597])
- kafka-testing-tools (incorporating kcat): New image. Command line utility for interacting with Kafka ([#590])
- spark: Add version `3.5.1` ([#588]).

### Changed

- kafka: Pulling kcat from Nexus rather than GitHub ([#534]).
- Reworking architecture selection mechanism for binaries ([#534]).
- Fixing base images to multi-architecture lists ([#534]).
- airflow and superset: Pull statsd-exporter as binary from Nexus instead of extracting out of the official docker image ([#534]).
- changed microdnf configuration to not install weak dependencies by adding `install_weak_deps=0` ([#533])
- ubi8-rust-builder: bump ubi8-minimal image to latest 8.9 ([#514]).
- stackable-base: bump ubi8-minimal image to latest 8.9 ([#514]).
- ubi8-rust-builder: bump rust toolchain to `1.75.0` ([#542], [#517]).
- GH workflows: make preflight an independent manual workflow and update to version 1.7.2 ([#519]).
- hadoop: Build from source ([#526]).
- superset: Add patch that fixes saved queries export ([#539]).
- inotify-tools: Download from Nexus instead of using the EPEL 8 repository ([#549]).
- hadoop: Add patches to fix missing operationType for some operations in authorizer ([#555], [#564]).
- airflow: bump git-sync to `4.2.1` ([#562]).
- hdfs: bump topology-provider to `0.2.0` ([#565]).
- java-base: Add `krb5-workstation` for all Java based products, as it is used by at least Zookeeper (in the future),
  HDFS, HBase, Trino, Spark, Druid ([#572]).
- hdfs: bump topology-provider to `0.3.0` ([#579]).
- ubi8-rust-builder: bump rust toolchain to `1.76.0` ([#584]).
- opa: bump bundle builder to version 1.1.1 ([#585]).

### Removed

- airflow: Remove support for `2.6.1` ([#562]).
- hadoop: Remove support for version `3.2.2` and `3.2.4` (this ends the `3.2` line) ([#540], [#571]).
- hbase: Remove support for version `2.4.12` ([#567]).
- kafka: Remove support for version `2.8.2`, `3.4.0`, `3.5.1` ([#559]).
- opa: Remove support for version `0.51.0` ([#547]).
- spark: Remove support for version `3.4.0`, `3.4.0-java17` ([#560]).
- superset: Remove support for version `2.1.0` ([#563]).
- zookeeper: Remove support for version `3.8.1` ([#551]).

[#493]: https://github.com/stackabletech/docker-images/pull/493
[#506]: https://github.com/stackabletech/docker-images/pull/506
[#514]: https://github.com/stackabletech/docker-images/pull/514
[#517]: https://github.com/stackabletech/docker-images/pull/517
[#519]: https://github.com/stackabletech/docker-images/pull/519
[#526]: https://github.com/stackabletech/docker-images/pull/526
[#529]: https://github.com/stackabletech/docker-images/pull/529
[#531]: https://github.com/stackabletech/docker-images/pull/531
[#533]: https://github.com/stackabletech/docker-images/pull/533
[#534]: https://github.com/stackabletech/docker-images/pull/534
[#536]: https://github.com/stackabletech/docker-images/pull/536
[#537]: https://github.com/stackabletech/docker-images/pull/537
[#538]: https://github.com/stackabletech/docker-images/pull/538
[#539]: https://github.com/stackabletech/docker-images/pull/539
[#540]: https://github.com/stackabletech/docker-images/pull/540
[#542]: https://github.com/stackabletech/docker-images/pull/542
[#544]: https://github.com/stackabletech/docker-images/pull/544
[#547]: https://github.com/stackabletech/docker-images/pull/547
[#549]: https://github.com/stackabletech/docker-images/pull/549
[#551]: https://github.com/stackabletech/docker-images/pull/551
[#552]: https://github.com/stackabletech/docker-images/pull/552
[#555]: https://github.com/stackabletech/docker-images/pull/555
[#558]: https://github.com/stackabletech/docker-images/pull/558
[#559]: https://github.com/stackabletech/docker-images/pull/559
[#560]: https://github.com/stackabletech/docker-images/pull/560
[#562]: https://github.com/stackabletech/docker-images/pull/562
[#563]: https://github.com/stackabletech/docker-images/pull/563
[#564]: https://github.com/stackabletech/docker-images/pull/564
[#565]: https://github.com/stackabletech/docker-images/pull/565
[#566]: https://github.com/stackabletech/docker-images/pull/566
[#567]: https://github.com/stackabletech/docker-images/pull/567
[#571]: https://github.com/stackabletech/docker-images/pull/571
[#572]: https://github.com/stackabletech/docker-images/pull/572
[#575]: https://github.com/stackabletech/docker-images/pull/575
[#579]: https://github.com/stackabletech/docker-images/pull/579
[#585]: https://github.com/stackabletech/docker-images/pull/585
[#588]: https://github.com/stackabletech/docker-images/pull/588
[#590]: https://github.com/stackabletech/docker-images/pull/590
[#591]: https://github.com/stackabletech/docker-images/pull/591
[#592]: https://github.com/stackabletech/docker-images/pull/592
[#593]: https://github.com/stackabletech/docker-images/pull/593
[#597]: https://github.com/stackabletech/docker-images/pull/597

## [23.11.0] - 2023-11-30

### Added

- hadoop: Added Stackable topology provider jar to enable k8s-based rack awareness ([#509])
- hadoop: Add all necessary components to the image to mount HDFS using FUSE ([#400])
- hbase: Add hbase-operator-tools ([#497], [#498]).
- java-base: Add needed tzdata-java package ([#425]).
- testing-tools: Add java, tzdata-java, unzip ([#464], [#465], [#466]).

- airflow: added support for 2.6.3, 2.7.2 ([#477]).
- druid: added support for 27.0.0 ([#485]).
- hadoop: added support for 3.2.4, 3.3.6 ([#478]).
- hbase: added new version 2.4.17 ([#494]).
- hbase: use jmx-exporter 0.20.0 ([#494]).
- hbase: added hadoop native compression ([#494]).
- hive: added upload new version script ([#472]).
- hive: Update postgresql driver in Hive metastore 3.1.3 to 42.6.0 ([#505]).
- kafka: add support for versions 3.4.1, 3.5.1 ([#476]).
- nifi: added support for version 1.23.2 ([#473]).
- opa: add version 0.57.0 ([#471]).
- opa: add new version upload script ([#471]).
- spark: added versions 3.4.1, 3.5.0 ([#475]).
- superset: add new version 2.1.1, 3.0.1 ([#482], [#489]).
- superset: add tzdata library as ubi-minimal has removed it ([#499]).
- trino: removed support for versions 428 ([#487]).
- zookeeper: add version 3.8.3 ([#470]).
- zookeeper: add upload script ([#470]).

### Changed

- Extract image tools their own [repository](https://github.com/stackabletech/image-tools) ([#437])
- Bump ubi8-rust-builder toolchain to 1.71.0 ([#419]).
- BREAKING: Upgrade Vector in all product images to version 0.33.0. The
  integration tests of the operators must be adapted because the metric
  `processedEventsTotal` was replaced by `receivedEventsTotal` ([#429],
  [#479]).
- BREAKING: Use RPM instead of tar.gz for Vector. Because of that, the
  location of the Vector executable changed, and the operator-rs version
  0.45.0 or newer is required ([#429]).
- spark-k8s: Rework spark images to build on top of java-base image. This fixes the missing tzdata-java package in 0.0.0-dev versions ([#434]).

- airflow: Updated git-sync to 3.6.8 ([#431]).
- airflow: Updated statsd-exporter to 0.24, this was accidentally moved to a very old version previously (0.3.0) ([#431]).
- airflow: Added wrapper script to allow the triggering of pre/post hook actions ([#435]).
- hadoop: bumped jmx-exporter version to 0.20.0 ([#478]).
- hbase: added soft link for jmx-exporter ([#494]).
- hbase: rename jmx_exporter configs to match rolenames in operators ([#494]).
- hive: bump jmx-exporter to 0.20.0 ([#472]).
- spark: bump jmx-exporter to 0.20.0 and access via softlink ([#475]).
- superset: removed patches that are obsolete since 2.0.0 ([#482]).
- superset: bump statsd_exporter to 0.24.0 and set via conf.py ([#482]).
- trino: using new OPA authorizer from <https://github.com/bloomberg/trino/tree/add-open-policy-agent> for version 428 ([#487]).
- zookeeper: bumped jmx-exporter version to 0.20.0 ([#470]).

### Removed

- airflow: Remove unused environment variable `AIRFLOW_UID` ([#429]).
- java-base: Remove hard-coded JVM security properties containing DNS cache settings. Going forward operators will configure DNS cache settings ([#433])
- pyspark-k8s: The PySpark image has been removed completely. Python is now installed with the Spark image ([#436])
- Removed all product specific changelogs and updated the root file ([#440])

- airflow: removed support for 2.2.3, 2.2.4, 2.2.5, 2.4.1 ([#477]).
- druid: removed support for 0.23.0, 24.0.0 ([#485]).
- hadoop: removed support for 3.3.1, 3.3.3 ([#478]).
- hive: remove version 2.3.9 ([#472]).
- kafka: removed support for versions 2.7.1, 3.1.0, 3.2.0, 3.3.1 ([#476]).
- nifi: removed support for version 1.15.x, 1.16.x, 1.18.x, 1.20.x ([#473]).
- nifi: removed openssl from image ([#473]).
- opa: removed versions 0.27.1, 0.28.0, 0.37.2, 0.41.0, 0.45.0 ([#471]).
- spark: removed versions 3.2.1, 3.3.0 versions ([#475]).
- superset: removed versions 1.3.2, 1.4.1, 1.4.2, 1.5.1, 1.5.3, 2.0.1 ([#482]).
- trino: removed support for versions 377, 387, 395, 396, 403 ([#487]).
- zookeeper: removed versions 3.5.8, 3.6.3, 3.7.0, 3.8.0 ([#470]).

[#400]: https://github.com/stackabletech/docker-images/pull/400
[#419]: https://github.com/stackabletech/docker-images/pull/419
[#425]: https://github.com/stackabletech/docker-images/pull/425
[#429]: https://github.com/stackabletech/docker-images/pull/429
[#431]: https://github.com/stackabletech/docker-images/pull/431
[#433]: https://github.com/stackabletech/docker-images/pull/433
[#434]: https://github.com/stackabletech/docker-images/pull/434
[#435]: https://github.com/stackabletech/docker-images/pull/435
[#436]: https://github.com/stackabletech/docker-images/pull/436
[#437]: https://github.com/stackabletech/docker-images/pull/437
[#440]: https://github.com/stackabletech/docker-images/pull/440
[#464]: https://github.com/stackabletech/docker-images/pull/464
[#465]: https://github.com/stackabletech/docker-images/pull/465
[#466]: https://github.com/stackabletech/docker-images/pull/466
[#470]: https://github.com/stackabletech/docker-images/pull/470
[#471]: https://github.com/stackabletech/docker-images/pull/471
[#472]: https://github.com/stackabletech/docker-images/pull/472
[#473]: https://github.com/stackabletech/docker-images/pull/473
[#475]: https://github.com/stackabletech/docker-images/pull/475
[#476]: https://github.com/stackabletech/docker-images/pull/476
[#477]: https://github.com/stackabletech/docker-images/pull/477
[#478]: https://github.com/stackabletech/docker-images/pull/478
[#479]: https://github.com/stackabletech/docker-images/pull/479
[#482]: https://github.com/stackabletech/docker-images/pull/482
[#485]: https://github.com/stackabletech/docker-images/pull/485
[#487]: https://github.com/stackabletech/docker-images/pull/487
[#489]: https://github.com/stackabletech/docker-images/pull/489
[#494]: https://github.com/stackabletech/docker-images/pull/494
[#497]: https://github.com/stackabletech/docker-images/pull/497
[#498]: https://github.com/stackabletech/docker-images/pull/498
[#499]: https://github.com/stackabletech/docker-images/pull/499
[#505]: https://github.com/stackabletech/docker-images/pull/505
[#509]: https://github.com/stackabletech/docker-images/pull/509

## [23.7.0] - 2023-07-14

### Added

- airflow: Support for version `2.6.1` ([#379]).
- druid: Support for version `26.0.0` ([#384]).
- hadoop: Add krb5-workstation and openssl packages - needed for Kerberos support ([#347]).
- hive: Added `jackson-dataformat-xml-2.7.9.jar` (2.3.9) and `jackson-dataformat-xml-2.12.3.jar` (3.1.3) for XmlFormat conversion for logging ([#293]).
- nifi: Support for version `1.20.0`, `1.21.0` ([#365]).
- trino: Add `htpasswd` tool ([#385]).
- trino: [EXPERIMENTAL] Add [Snowlift Trino Storage Connector](https://github.com/snowlift/trino-storage), but only for Trino version 414 ([#397])
- zooKeeper: Support for version `3.8.1` ([#381]).

### Changed

- nifi: Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#297]).
- opa: Add multilog (daemontools) to opa image ([#333]).
- opa: Upgraded to the vector base image ([#329]).
- opa: Support for version `0.51.0`` ([#382]).
- stackable-base: Update ubi-minimal base image from 8.7@sha256:3e1adcc31c6073d010b8043b070bd089d7bf37ee2c397c110211a6273453433f to registry.access.redhat.com/ubi8/ubi-minimal:8.8@sha256:14b404f4181904fb5edfde1a7a6b03fe1b0bb4dad1f5c02e16f797d5eea8c0cb ([#420]).
- zookeeper: Make soft link for `jmx-exporter` e.g. `jmx_prometheus_javaagent-<version>.jar` -> `jmx_prometheus_javaagent.jar` ([#381]).

[#293]: https://github.com/stackabletech/docker-images/pull/293
[#297]: https://github.com/stackabletech/docker-images/pull/297
[#329]: https://github.com/stackabletech/docker-images/pull/329
[#333]: https://github.com/stackabletech/docker-images/pull/333
[#347]: https://github.com/stackabletech/docker-images/pull/347
[#365]: https://github.com/stackabletech/docker-images/pull/365
[#379]: https://github.com/stackabletech/docker-images/pull/379
[#381]: https://github.com/stackabletech/docker-images/pull/381
[#382]: https://github.com/stackabletech/docker-images/pull/382
[#384]: https://github.com/stackabletech/docker-images/pull/384
[#385]: https://github.com/stackabletech/docker-images/pull/385
[#397]: https://github.com/stackabletech/docker-images/pull/397

## [23.4.0] - 2023-04-17

### Added

- Package inotify-tools added ([#291]).
- Added krb5 image ([#338]).

### Changed

- Updated all internal images to rebuild their base images on demand
  ([#321]).
- Unpinned testing-tools dependencies ([#326]).

### Removed

- Tools image ([#325]).
- Replace `build_product_images.py` with the `image_tools` package and
  add OpenShift preflight checks for images ([#339])

[#291]: https://github.com/stackabletech/docker-images/pull/291
[#321]: https://github.com/stackabletech/docker-images/pull/321
[#325]: https://github.com/stackabletech/docker-images/pull/325
[#326]: https://github.com/stackabletech/docker-images/pull/326
[#338]: https://github.com/stackabletech/docker-images/pull/338
[#339]: https://github.com/stackabletech/docker-images/pull/339

## [23.1.0] - 2023-01-23

### Added

- Image vector added which installs Vector and is based on the
  stackable-base image. ([#268]).

### Changed

- Updated java base image to latest ubi8 tag 8.6-994 ([#249]).
- Updated all java-base images to stackable0.2.2 ([#250]).
- Updated all ubi8 base images to latest (8.6-994) ([#250]).

### Removed

- Retired Java 1.8.0 support ([#248]).

[#248]: https://github.com/stackabletech/docker-images/pull/248
[#249]: https://github.com/stackabletech/docker-images/pull/249
[#250]: https://github.com/stackabletech/docker-images/pull/250
[#268]: https://github.com/stackabletech/docker-images/pull/268
