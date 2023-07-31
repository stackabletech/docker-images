# Changelog

## [Unreleased]

### Changed

- Update ubi-minimal base image from 8.6 to 8.7@sha256:3e1adcc31c6073d010b8043b070bd089d7bf37ee2c397c110211a6273453433f

- Update ubi-minimal base image from 8.7@sha256:3e1adcc31c6073d010b8043b070bd089d7bf37ee2c397c110211a6273453433f to registry.access.redhat.com/ubi8/ubi-minimal:8.8@sha256:14b404f4181904fb5edfde1a7a6b03fe1b0bb4dad1f5c02e16f797d5eea8c0cb ([#420]).

## [stackable-base-stackable1.0.0] - 2022-12-12

### Added

- Image stackable-base added which is based on the ubi-minimal:8.6 image. This
  image creates the stackable user and group and the working directory
  /stackable ([#268]).

[#268]: https://github.com/stackabletech/docker-images/pull/268
[#420]: https://github.com/stackabletech/docker-images/pull/420
