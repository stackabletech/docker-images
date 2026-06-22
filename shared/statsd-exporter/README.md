# statsd_exporter

This is the Docker image that builds statsd_exporter.

This image does _not_ need to be built manually.
It will be built automatically if specified as a dependency in our `boil-config.toml` file.
Which will then be transformed to a dependency in a Docker `buildx bake` file.

## Updating

This image uses patchable for applying patch sets before building from source.
Our mirror can be found here:

To update:

> [!CAUTION]
> Check the changes since the last version, eg:
> <https://github.com/prometheus/statsd_exporter/compare/v0.29.0...v0.30.0>

```shell
STATSD_VERSION=0.29.0
cargo patchable init version "shared/statsd-exporter" "$STATSD_VERSION" --base="v$STATSD_VERSION" --mirror
```

Apply any necessary patches (see previous version's patch set).
