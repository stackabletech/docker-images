# Changelog

## [Unreleased]

### changed

- Update ubi-minimal base image from 8.6 to 8.7@sha256:3e1adcc31c6073d010b8043b070bd089d7bf37ee2c397c110211a6273453433f
- Update ubi-minimal base image from 8.7 to 8.8@sha256:621f5245fb3e8597a626163cdf1229e1f8311e07ab71bb1e9332014b51c59f9c

## [stackable-base-stackable1.0.0] - 2022-12-12

### Added

- Image stackable-base added which is based on the ubi-minimal:8.6 image. This
  image creates the stackable user and group and the working directory
  /stackable ([#268]).

[#268]: https://github.com/stackabletech/docker-images/pull/268
