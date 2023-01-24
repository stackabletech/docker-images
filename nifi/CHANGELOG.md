# Changelog

## [nifi1.15.3-stackable???] [nifi1.16.3-stackable???] [nifi1.18.0-stackable???] - 2023-??-??

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#xxx]).

[#xxx]: https://github.com/stackabletech/docker-images/pull/xxx
