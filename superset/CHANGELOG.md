# Changelog

## [Unreleased]

### Changed

- Upgraded to the vector base image ([#328]).
- setuptools pinned to version 66.1.1 because newer versions are not
  compatible with the supported Superset versions. With version 67.0.0,
  the requirements of Superset cannot be parsed anymore (see also
  <https://github.com/pypa/setuptools/pull/3790>) ([#307]).

[#307]: https://github.com/stackabletech/docker-images/pull/307
[#328]: https://github.com/stackabletech/docker-images/pull/328

## [superset1.5.1-stackable0.2.0] - 2022-07-13

- Add Trino sqlalchemy library ([#153]).

[#153]: https://github.com/stackabletech/docker-images/pull/153

## [superset1.5.1-stackable0.1.0] - 2022-06-30

### Added

- Superset 1.5.1 added ([#144]).

[#144]: https://github.com/stackabletech/docker-images/pull/144

## [superset-stackable2.1.0] - 2022-05-04

### Added

- Python package pydruid added ([#110]).

[#110]: https://github.com/stackabletech/docker-images/pull/110

## [superset-stackable2.0.1] - 2022-05-03

### Fixed

- Version label set ([#108]).

[#108]: https://github.com/stackabletech/docker-images/pull/108

## [superset-stackable2.0.0] - 2022-05-02

### Changed

- BREAKING: Replace base image with the Red Hat Univeral Base Image. The
  location for the Superset configuration file changed to
  `/stackable/app/pythonpath` ([#103]).

[#103]: https://github.com/stackabletech/docker-images/pull/103

## [superset-stackable1.0.0] - 2022-04-14

### Removed

- BREAKING: Custom Superset configuration removed ([#85]).

[#85]: https://github.com/stackabletech/docker-images/pull/85

## [superset-stackable0.3.0] - 2022-02-21

### Added

- Superset 1.4.1 added ([#56]).

### Changed

- Base image tag of Superset 1.3.2 set to the commit in the official Superset
  repository which is tagged with the version 1.3.2. The correct version is now
  displayed in the web UI instead of 0.0.0-dev ([#56]).

[#56]: https://github.com/stackabletech/docker-images/pull/56

## [superset-stackable0.2.0] - 2022-02-15

### Changed

- Base image made configurable to allow different Superset versions ([#48]).

[#48]: https://github.com/stackabletech/docker-images/pull/48

## [superset-stackable0.1.0] - 2021-11-12

### Added

- Superset 1.3.2 added using the official Docker image from 2021-11-02 as base
  image ([#6]).

[#6]: https://github.com/stackabletech/docker-images/pull/6
