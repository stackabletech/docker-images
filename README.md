# Stackable Docker Images

This repository contains Dockerfiles and scripts to build base images for use within Stackable.

<!-- start:badges: autogenerated by .scripts/update_readme_badges.sh -->
| | | | |
| -: | -: | -: | -: |
| [![Build Airflow]][dev_airflow.yaml] | [![Build Druid]][dev_druid.yaml] | [![Build Hadoop]][dev_hadoop.yaml] | [![Build HBase]][dev_hbase.yaml] |
| [![Build Hello-World]][dev_hello-world.yaml] | [![Build Hive]][dev_hive.yaml] | [![Build Java Base]][dev_java-base.yaml] | [![Build Java Development]][dev_java-devel.yaml] |
| [![Build Kafka Testing Tools]][dev_kafka-testing-tools.yaml] | [![Build Kafka]][dev_kafka.yaml] | [![Build kcat]][dev_kcat.yaml] | [![Build Krb5]][dev_krb5.yaml] |
| [![Build NiFi]][dev_nifi.yaml] | [![Build Omid]][dev_omid.yaml] | [![Build OPA]][dev_opa.yaml] | [![Build Spark K8s]][dev_spark-k8s.yaml] |
| [![Build Stackable Base]][dev_stackable-base.yaml] | [![Build Superset]][dev_superset.yaml] | [![Build Testing Tools]][dev_testing-tools.yaml] | [![Build Tools]][dev_tools.yaml] |
| [![Build Trino CLI]][dev_trino-cli.yaml] | [![Build Trino]][dev_trino.yaml] | [![Build Vector]][dev_vector.yaml] | [![Build ZooKeeper]][dev_zookeeper.yaml] |
<!-- end:badges -->

## Prerequisites

* [Stackable Image Tools](https://github.com/stackabletech/image-tools) (`pip install image-tools-stackabletech`)
* Docker including the [`buildx` plugin](https://github.com/docker/buildx)
* Optional: [OpenShift preflight tool](https://github.com/redhat-openshift-ecosystem/openshift-preflight) to verify an image for OpenShift

## Build Product Images

Product images are published to the `docker.stackable.tech` registry under the `stackable` organization by default.

### Build single products locally

To build and push product images to the default repository use this command:

```sh
bake --product zookeeper --image 0.0.0-dev --push
```

This will build images for Apache ZooKeeper versions as defined in the `conf.py` file, tag them with the `image-version` 0.0.0-dev and push them to the registry.

You can select a specific version of a product to build using the syntax `product=version` e.g. to build Hive 3.1.3 you can use this command:

```sh
bake --product hive=3.1.3 -i 0.0.0-dev
```

> [!NOTE]
> `-i` is the shorthand for `--image` (i.e. the resulting image tag)

### Build all products locally

To build all products in all versions locally you can use this command

```sh
bake --image-version 0.0.0-dev
```

### Build everything in GitHub

The GitHub action called `Build (and optionally publish) 0.0.0-dev images` can be triggered manually to do build all images in all versions.
When triggered manually it will _not_ push the images to the registry.

## Verify Product Images

To verify if Apache Zookeeper validate against OpenShift preflight, run:

```sh
check-container --product zookeeper --image 0.0.0-dev
```

## ubi8-rust-builder / ubi9-rust-builder

These images are meant to be used in multi-stage builds as a base image for projects building Rust projects.
They are automatically rebuilt and pushed every night and also on every push to the main branch, in addition a build can be triggered using GitHub Actions.

The image will run `cargo build --release` in the current context and copy all binaries to an `/app` directory.

This will bake in the current stable Rust version at the time this image was built, which means it should be rebuilt (and tagged) for every release of Rust.

## Example usage

```dockerfile
FROM docker.stackable.tech/ubi9-rust-builder AS builder

FROM registry.access.redhat.com/ubi9/ubi-minimal AS operator
LABEL maintainer="Stackable GmbH"

# Update image
RUN microdnf update \
  && microdnf install \
    shadow-utils \
  && rm -rf /var/cache/yum

COPY --from=builder /app/stackable-zookeeper-operator /

RUN groupadd -g 1000 stackable && adduser -u 1000 -g stackable -c 'Stackable Operator' stackable

USER 1000:1000

ENTRYPOINT ["/stackable-zookeeper-operator"]
```

<!-- start:links: autogenerated by .scripts/update_readme_badges.sh -->
[Build Airflow]: https://github.com/stackabletech/docker-images/actions/workflows/dev_airflow.yaml/badge.svg
[dev_airflow.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_airflow.yaml
[Build Druid]: https://github.com/stackabletech/docker-images/actions/workflows/dev_druid.yaml/badge.svg
[dev_druid.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_druid.yaml
[Build Hadoop]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hadoop.yaml/badge.svg
[dev_hadoop.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hadoop.yaml
[Build HBase]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hbase.yaml/badge.svg
[dev_hbase.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hbase.yaml
[Build Hello-World]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hello-world.yaml/badge.svg
[dev_hello-world.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hello-world.yaml
[Build Hive]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hive.yaml/badge.svg
[dev_hive.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_hive.yaml
[Build Java Base]: https://github.com/stackabletech/docker-images/actions/workflows/dev_java-base.yaml/badge.svg
[dev_java-base.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_java-base.yaml
[Build Java Development]: https://github.com/stackabletech/docker-images/actions/workflows/dev_java-devel.yaml/badge.svg
[dev_java-devel.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_java-devel.yaml
[Build Kafka Testing Tools]: https://github.com/stackabletech/docker-images/actions/workflows/dev_kafka-testing-tools.yaml/badge.svg
[dev_kafka-testing-tools.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_kafka-testing-tools.yaml
[Build Kafka]: https://github.com/stackabletech/docker-images/actions/workflows/dev_kafka.yaml/badge.svg
[dev_kafka.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_kafka.yaml
[Build kcat]: https://github.com/stackabletech/docker-images/actions/workflows/dev_kcat.yaml/badge.svg
[dev_kcat.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_kcat.yaml
[Build Krb5]: https://github.com/stackabletech/docker-images/actions/workflows/dev_krb5.yaml/badge.svg
[dev_krb5.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_krb5.yaml
[Build NiFi]: https://github.com/stackabletech/docker-images/actions/workflows/dev_nifi.yaml/badge.svg
[dev_nifi.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_nifi.yaml
[Build Omid]: https://github.com/stackabletech/docker-images/actions/workflows/dev_omid.yaml/badge.svg
[dev_omid.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_omid.yaml
[Build OPA]: https://github.com/stackabletech/docker-images/actions/workflows/dev_opa.yaml/badge.svg
[dev_opa.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_opa.yaml
[Build Spark K8s]: https://github.com/stackabletech/docker-images/actions/workflows/dev_spark-k8s.yaml/badge.svg
[dev_spark-k8s.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_spark-k8s.yaml
[Build Stackable Base]: https://github.com/stackabletech/docker-images/actions/workflows/dev_stackable-base.yaml/badge.svg
[dev_stackable-base.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_stackable-base.yaml
[Build Superset]: https://github.com/stackabletech/docker-images/actions/workflows/dev_superset.yaml/badge.svg
[dev_superset.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_superset.yaml
[Build Testing Tools]: https://github.com/stackabletech/docker-images/actions/workflows/dev_testing-tools.yaml/badge.svg
[dev_testing-tools.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_testing-tools.yaml
[Build Tools]: https://github.com/stackabletech/docker-images/actions/workflows/dev_tools.yaml/badge.svg
[dev_tools.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_tools.yaml
[Build Trino CLI]: https://github.com/stackabletech/docker-images/actions/workflows/dev_trino-cli.yaml/badge.svg
[dev_trino-cli.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_trino-cli.yaml
[Build Trino]: https://github.com/stackabletech/docker-images/actions/workflows/dev_trino.yaml/badge.svg
[dev_trino.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_trino.yaml
[Build Vector]: https://github.com/stackabletech/docker-images/actions/workflows/dev_vector.yaml/badge.svg
[dev_vector.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_vector.yaml
[Build ZooKeeper]: https://github.com/stackabletech/docker-images/actions/workflows/dev_zookeeper.yaml/badge.svg
[dev_zookeeper.yaml]: https://github.com/stackabletech/docker-images/actions/workflows/dev_zookeeper.yaml
<!-- end:links -->