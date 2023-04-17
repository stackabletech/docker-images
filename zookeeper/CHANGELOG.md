# Changelog

## [Unreleased]

## [23.4.0] - 2023-04-17

## [zookeeper-stackable0.9.0] - 2022-12-12

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#268]).

[#268]: https://github.com/stackabletech/docker-images/pull/268

## [zookeeper-stackable0.7.1] - 2022-06-02

### Changed

- Disabled Java DNS caching ([#126]).

[#126]: https://github.com/stackabletech/docker-images/pull/126

## [zookeeper-stackable0.4.0] - 2022-04-07

### Added

- ZooKeeper version 3.8.0 added ([#86]).

[#86]: https://github.com/stackabletech/docker-images/pull/86
