# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- omid: init at 1.1.0 ([#493]).

### Changed

- ubi8-rust-bilder: bump ubi8-minimal image to latest 8.9 ([#514]).
- stackable-base: bump ubi8-minimal image to latest 8.9 ([#514]).
- Bump ubi8-rust-builder toolchain to `1.74.0` ([#517]).
- GH workflows: make preflight an independent manual workflow and update to version 1.7.2 ([#519]).

[#493]: https://github.com/stackabletech/docker-images/pull/493
[#514]: https://github.com/stackabletech/docker-images/pull/514
[#517]: https://github.com/stackabletech/docker-images/pull/517
[#519]: https://github.com/stackabletech/docker-images/pull/519

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
- spark-k8s: Rework spark images to build on top of java-base image.  This fixes the missing tzdata-java package in 0.0.0-dev versions ([#434]).

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
