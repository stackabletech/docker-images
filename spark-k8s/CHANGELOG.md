# Changelog

## [spark-k8s-stackable0.1.0] - 2022-03-21

### Added

- Added Python 3.9 back to the image and set /stackable as home folder. (#[xx])

[#xx]: https://github.com/stackabletech/docker-images/pull/xx

## [spark-k8s-stackable0.1.0] - 2022-03-18

### Added

- Stackable spark-on-kubernetes image compatible with the entrypoint used by the standard Spark kubernetes image. N.B. this involves copying in an external binary file (https://github.com/krallin/tini) that spark uses internally to add a level of security to called processes ([#73]).

[#73]: https://github.com/stackabletech/docker-images/pull/73
