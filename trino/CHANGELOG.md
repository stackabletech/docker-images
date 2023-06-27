# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Add `htpasswd` tool ([#385]).
- Experimental: Add Snowlift Trino Storage Connector (https://github.com/snowlift/trino-storage) ([#397])

[#385]: https://github.com/stackabletech/docker-images/pull/385
[#397]: https://github.com/stackabletech/docker-images/pull/397

## [414] - 2023-04-26

### Added

- Added Trino version 414 ([#367]).
- Make soft link for `jmx-exporter` e.g. `jmx_prometheus_javaagent-<version>.jar` -> `jmx_prometheus_javaagent.jar` ([#367]).

### Changed

- Make `jmx-exporter` configurable in `conf.py` ([#367]).

[#367]: https://github.com/stackabletech/docker-images/pull/367

## [396-stackable0.1.0] - 2022-09-16

### Added

- Added Trino version 396 ([#175]).

[#175]: https://github.com/stackabletech/docker-images/pull/175

## [395-stackable0.1.0] - 2022-09-15

### Added

- Added Trino version 395 (using Java 17) ([#173]).

[#173]: https://github.com/stackabletech/docker-images/pull/173
