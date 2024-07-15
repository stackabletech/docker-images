# statsd_exporter

This is the Docker image that builds statsd_exporter.

This image does _not_ need to be built manually.
It will be built automatically if specified as a dependency in our `conf.py` file.
Which will then be transformed to a dependency in a Docker `buildx bake` file.
