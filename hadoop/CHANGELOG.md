# Changelog

## [Unreleased]

### Added

- Add krb5-workstation and openssl packages - needed for Kerberos support ([#347]).
- Add all necessary components to the image to mount HDFS using FUSE ([#400])

### Changed

- Upgraded to JMX Exporter 0.19 ([#400])

[#347]: https://github.com/stackabletech/docker-images/pull/347
[#400]: https://github.com/stackabletech/docker-images/pull/400

## [3.2.2-stackable0.7.0] [3.3.1-stackable0.7.0] [3.3.3-stackable0.3.0] [3.3.4-stackable0.3.0] - 2022-12-27

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#285]).

[#285]: https://github.com/stackabletech/docker-images/pull/285
