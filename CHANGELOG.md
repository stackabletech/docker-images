# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed

- Bump ubi8-rust-builder toolchain to 1.71.0 ([#419]).

[#419]: https://github.com/stackabletech/docker-images/pull/419

## [23.7.0] - 2023-07-14

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
