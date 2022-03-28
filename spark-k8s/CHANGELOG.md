# Changelog

## [spark-k8s-stackable0.4.0] - 2022-03-28

### Changed

- Separate python and non-python images (#[xx])

[#xx]: https://github.com/stackabletech/docker-images/pull/75

## [spark-k8s-stackable0.3.0] - 2022-03-22

### Added

- Added AWS dependencies for Spark (#[xx])

[#xx]: https://github.com/stackabletech/docker-images/pull/75

## [spark-k8s-stackable0.2.0] - 2022-03-21

### Added

- Added Python 3.9 back to the image and set /stackable as home folder. (#[75])

[#75]: https://github.com/stackabletech/docker-images/pull/75

## [spark-k8s-stackable0.1.0] - 2022-03-18

### Added

- Stackable spark-on-kubernetes image compatible with the entrypoint used by the standard Spark kubernetes image. N.B. this involves copying in an external binary file (https://github.com/krallin/tini) that spark uses internally to add a level of security to called processes ([#73]).

[#73]: https://github.com/stackabletech/docker-images/pull/73
