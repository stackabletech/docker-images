# Changelog

## [hive2.3.9-stackable0.7.0] [hive3.1.3-stackable0.3.0] - 2023-01-16

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#xxx]).

[#xxx]: https://github.com/stackabletech/docker-images/pull/xxx
