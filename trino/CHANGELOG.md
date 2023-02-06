# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed

- Upgraded to the base image `java-base:11-stackable0.3.0` and `java-base:17-stackable0.3.0`. The java-base image contains a layer which provides Vector. The creation of the stackable user and group happens in the stackable-base layer and is therefore removed from this image ([#320]).

[#320]: https://github.com/stackabletech/docker-images/pull/320

## [396-stackable0.1.0] - 2022-09-16

### Added

- Added Trino version 396 ([#175]).

[#175]: https://github.com/stackabletech/docker-images/pull/175

## [395-stackable0.1.0] - 2022-09-15

### Added

- Added Trino version 395 (using Java 17) ([#173]).

[#173]: https://github.com/stackabletech/docker-images/pull/173
