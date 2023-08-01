# Changelog

## [Unreleased]

### Changed

- Updated Vector to 0.31 ([#429]).
- Updated git-sync to 3.6.8 ([#431]).
- Updated statsd-exporter to 0.24, this was accidentally moved to a very old version previously (0.3.0) ([#431]).

[#429]: https://github.com/stackabletech/docker-images/pull/429
[#431]: https://github.com/stackabletech/docker-images/pull/431

## [23.7.0]

### Added

- Added new airflow version `2.6.1` ([#379]).

[#379]: https://github.com/stackabletech/docker-images/pull/379

## [23.4.0]

### Added

- Added git-sync functionality to the airflow image ([#337]).

### Changed

- Upgraded to the base image vector:0.26.0-stackable1.1.0. The new base image
  provides Vector. The creation of the stackable user and group happens in the
  stackable-base layer and is therefore removed from this image ([#291]).

[#291]: https://github.com/stackabletech/docker-images/pull/291
[#337]: https://github.com/stackabletech/docker-images/pull/337

## [airflow-stackable0.5.0] - 2022-11-30

### Added

- Add Trino dependency and reduce image size ([#264])

[#264]: https://github.com/stackabletech/docker-images/pull/264

## [airflow-stackable0.4.0] - 2022-10-12

### Added

- Support for Airflow 2.4.1 ([#215])

[#215]: https://github.com/stackabletech/docker-images/pull/215

## [airflow-stackable0.3.0] - 2022-04-28

### Changed

- Replace airflow/debian-based image with Redhat-UBI-based image ([#97]).

[#97]: https://github.com/stackabletech/docker-images/pull/97

## [airflow-stackable0.2.0] - 2022-03-21

### Added

- Airflow 2.2.4 with Python 3.9 added ([#74]).

[#74]: https://github.com/stackabletech/docker-images/pull/74
