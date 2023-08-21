# Changelog

## [Unreleased]

### Added

- Rework spark images to build on top of java-base image.
  This fixes the missing tzdata-java package in 0.0.0-dev versions ([#434]).

[#434]: https://github.com/stackabletech/docker-images/pull/434

## [23.4.0]

### Added

- `jackson-dataformat-xml`, `stax2-api`, and `woodstox-core` added which are
  used for XML log output ([#342]).

### Changed

- Decoupled product from the archive name so that product can run on different java versions ([#327]).

[#327]: https://github.com/stackabletech/docker-images/pull/327
[#342]: https://github.com/stackabletech/docker-images/pull/342

## [3.3.0-stackable0.2.0] - 2022-09-13

### Added

- Added s3a and abfs libs ([#168])

[#168]: https://github.com/stackabletech/docker-images/pull/168

## [pyspark-k8s-stackable0.1.0] - 2022-03-28

### Added

- Separate python and non-python images ([#80])

[#80]: https://github.com/stackabletech/docker-images/pull/80
