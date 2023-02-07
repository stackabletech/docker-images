# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Added labels for openshift requirements and licenses for all images ([#95]).
- Added `RELEASE` ARG for all images ([#95]).

### Changed

- Updated java base image to latest ubi8 tag 8.6-994 ([#249]).
- Updated all java-base images to stackable0.2.2 ([#250]).
- Updated all ubi8 base images to latest (8.6-994) ([#250]).
- Updated all internal images to rebuild their base images on demand ([#321]).

### Removed

- Prometheus image, NodeExporter image, Antora ([#95]).
- Retired Java 1.8.0 support ([#248]).

[#95]: https://github.com/stackabletech/docker-images/pull/95
[#248]: https://github.com/stackabletech/docker-images/pull/248
[#249]: https://github.com/stackabletech/docker-images/pull/249
[#250]: https://github.com/stackabletech/docker-images/pull/250
[#321]: https://github.com/stackabletech/docker-images/pull/321
