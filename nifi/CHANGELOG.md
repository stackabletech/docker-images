# Changelog

## [Unreleased]

## [23.4.0] - 2023-04-17

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#297]).

[#297]: https://github.com/stackabletech/docker-images/pull/297
