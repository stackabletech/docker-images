# Druid Docker Image

## Prometheus Extension

We ship druid with the Prometheus Emitter extension. It is a community extension and so it is not shipped by default, [the documentation](https://druid.apache.org/docs/latest/development/extensions.html#loading-community-extensions) explains how to load community extensions.  It can be found on [maven](https://search.maven.org/artifact/org.apache.druid.extensions.contrib/prometheus-emitter), the codebase is in the official druid [github repository](https://github.com/apache/druid/tree/master/extensions-contrib/prometheus-emitter).
