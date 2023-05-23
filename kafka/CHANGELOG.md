# Changelog

## [Unreleased]

## [3.4.0] - 2023-05-23

### Added

- Added Kafka version `3.4.0` ([#376]).

### Changed

- Make soft link for `jmx-exporter` e.g. `jmx_prometheus_javaagent-<version>.jar` -> `jmx_prometheus_javaagent.jar` ([#376]).
- Update `jmx-exporter` to `0.18.0` ([#376]).
- Update `kafka-opa-authorizer` to `1.5.1` ([#376]).

[#306]: https://github.com/stackabletech/docker-images/pull/376

## [kafka-stackable0.3.0] - 2022-03-04

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#306]).

[#306]: https://github.com/stackabletech/docker-images/pull/306

## [kafka-stackable0.3.0] - 2022-03-04

### Removed

- Image for Kafka 2.6.2. ([#61])

### Changed

- Use the Kafka OPA plugin 1.4.0 for Kafka 2.7.1+. ([#61])
- Only build images for Kafka 2.7.1 and up. ([#61])

[#61]: https://github.com/stackabletech/docker-images/pull/61
