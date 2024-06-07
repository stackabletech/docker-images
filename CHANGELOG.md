# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- ubi9-rust-builder: A builder image using UBI9 instead of the current UBI8 ([#583])
- Build all `0.0.0-dev` product images as multi-arch and push them to Nexus and Harbor.
  Also SBOMs are generated and everything is signed ([#614], [#616]).
- hbase: Enable snapshot exports to S3; The HBase image depends now on
  the Hadoop image. The required AWS JARs are copied from the Hadoop
  image to the HBase image. The script `export-snapshot-to-s3` makes
  exporting easier ([#621]).
- kafka: Build from source ([#659], [#661]).
- omid: Include Apache Omid in all workflows such as building and releasing images ([#635]).
- java-devel: New image to serve as base layer for builder stages ([#665]).
- hdfs: Exclude YARN and Mapreduce projects from build ([#667]).
- stackable-base: Mitigate CVE-2023-37920 by removing e-Tugra root certificates ([#673]).

### Changed

- all: Switched all product builds from UBI8 to UBI9.4 ([#628])
- hbase: Remove the symlink `/stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar`
  which is unused since SDP 23.11 ([#621]).
- hive: Only build and ship Hive metastore. This reduces the image size from `2.63GB` to `1.9GB` and should also reduce the number of dependencies ([#619], [#622]).
- ubi8-rust-builder: Bump `protoc` from `21.5` to `26.1` ([#624]).
- pass platform argument to preflight check ([#626]).
- nifi: provision stackable-bcrypt from Maven ([#663])
- hadoop: use java-devel as base layer for the builder stage ([#665])
- hive: use java-devel as base layer for the builder stage ([#665])
- zookeeper: use java-devel as base layer for the builder stage ([#665])
- hbase: use java-devel as base layer for the builder stage ([#665])
- omid: use java-devel as base layer for the builder stage ([#665])
- kafka: use java-devel as base layer for the builder stage ([#665])
- opa-bundle-builder: Bump image to 1.1.2 ([#666])
- opa: Build from source ([#676])
- trino: Build from source ([#687]).

### Fixed

- superset: Let Superset 3.1.0 build on ARM by adding `make` and `diffutils` ([#611]).
- airflow: Let Airflow 2.8.x and 2.9.x build on ARM by adding `make` and `diffutils` ([#612]).
- python:3.11 manifest list fixed. Added proper hash ([#613]).
- trino-cli: Include the trino-cli in the CI build process ([#614]).
- hive: Fix compilation on ARM by back-porting [HIVE-21939](https://issues.apache.org/jira/browse/HIVE-21939) from [this](https://github.com/apache/hive/commit/2baf21bb55fcf33d8522444c78a8d8cab60e7415) commit ([#617]).
- hive: Fix compilation on ARM in CI as well ([#619]).
- hive: Fix compilation of x86 in CI due to lower disk usage to prevent disk running full ([#619]).

### Removed

- zookeeper: Remove unsupported version 3.9.1 ([#628]).
- java-base: Remove openjdk-devel rpm package again to reduce the vulnerability surface ([#665])

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
[#676]: https://github.com/stackabletech/docker-images/pull/676
[#673]: https://github.com/stackabletech/docker-images/pull/673
[#687]: https://github.com/stackabletech/docker-images/pull/687

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
