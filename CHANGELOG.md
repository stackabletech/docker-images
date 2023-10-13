# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- hadoop: Add all necessary components to the image to mount HDFS using FUSE ([#400])
- java-base: Add needed tzdata-java package ([#425]).
- testing-tools: Add java, tzdata-java, unzip ([#464], [#465], [#466]).

- zookeeper: add version 3.8.2 ([#470]).
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

### Removed

- airflow: Remove unused environment variable `AIRFLOW_UID` ([#429]).
- java-base: Remove hard-coded JVM security properties containing DNS cache settings. Going forward operators will configure DNS cache settings ([#433])
- pyspark-k8s: The PySpark image has been removed completely. Python is now installed with the Spark image ([#436])
- Removed all product specific changelogs and updated the root file ([#440])

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
[#479]: https://github.com/stackabletech/docker-images/pull/479

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
