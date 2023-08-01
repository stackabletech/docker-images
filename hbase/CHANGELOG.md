# Changelog

## [Unreleased]

### Added

- Add versions `2.4.17`, `2.5.3` and `2.5.3-hadoop3` ([#352]).
- Add krb5-workstation and openssl packages - needed for Kerberos support ([#352]).
- Add `jackson-dataformat-xml` library (used for logging) ([#352]).

### Changed

- Bump to Apache Phoenix `5.1.3` ([#352]).

[#352]: https://github.com/stackabletech/docker-images/pull/352

## [hbase2.4.6-stackable0.9.0] [hbase2.4.8-stackable0.9.0] [hbase2.4.9-stackable0.9.0] [hbase2.4.11-stackable0.9.0] [hbase2.4.12-stackable0.4.0] - 2022-12-21

### Changed

- Upgraded to the base image java-base:11-stackable0.3.0. The java-base image
  contains a layer which provides Vector. The creation of the stackable user
  and group happens in the stackable-base layer and is therefore removed from
  this image ([#283]).

[#283]: https://github.com/stackabletech/docker-images/pull/283

## [hbase2.4.6-stackable0.7.0] [hbase2.4.8-stackable0.7.0] [hbase2.4.9-stackable0.7.0] [hbase2.4.11-stackable0.7.0] [hbase2.4.12-stackable0.2.0] - 2022-08-01

### Added

- Phoenix support added ([#157]).

[#157]: https://github.com/stackabletech/docker-images/pull/157

## [hbase-stackable0.4.0] - 2022-03-22

### Added

- HBase 2.4.11 added ([#76]).

[#76]: https://github.com/stackabletech/docker-images/pull/76

## [hbase-stackable0.3.0] - 2022-02-22

### Added

- HBase 2.4.9 added ([#57]).

[#57]: https://github.com/stackabletech/docker-images/pull/57
