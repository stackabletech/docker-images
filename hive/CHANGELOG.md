# Changelog

## [Unreleased]

## [23.4.0] - 2023-04-17

### Added

- Added `jackson-dataformat-xml-2.7.9.jar` (2.3.9) and `jackson-dataformat-xml-2.12.3.jar` (3.1.3) for XmlFormat conversion for logging ([#293]).

[#293]: https://github.com/stackabletech/docker-images/pull/293

## [hive2.3.9-stackable0.7.0] [hive3.1.3-stackable0.3.0] - 2023-01-16

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#287]).

[#287]: https://github.com/stackabletech/docker-images/pull/287
