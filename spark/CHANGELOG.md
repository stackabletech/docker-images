# Changelog

## spark-stackable0.3.0 - 2022-02-21

### Added
- Added python requirements (python, pip and related libraries) to the docker image so that pyspark (bundled with the spark code) can be used. The python libraries are linked to the spark version, with a separate requirements.txt for each spark release ([#53]).

[#53]: https://github.com/stackabletech/docker-images/pull/53
