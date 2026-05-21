# boil

boil builds container images in parallel.

- Define versions of container images and version specific values via the `boil-config.toml` file.
- Refer to local images in Containerfiles via `FROM local-image/...`. Nesting is supported.
- Structured output is provided for any potential follow-up tasks.

## Quick Overview

Either compile and run the binary, or use the `cargo boil` alias.

```shell
# Builds all version of the image located in the 'airflow' folder
boil build airflow

# Builds the 3.0.1 version of the image located in the 'airflow' folder
boil build airflow=3.0.1

# Builds both the 3.0.1 and 2.10.5 versions of the image located in the
# 'airflow' folder
boil build airflow=3.0.1,2.10.5

# Builds all versions of the images located in the 'airflow' and 'opa' folder
boil build airflow opa

# Display a list of all images and their declared versions as structured JSON
# output
boil image list

# Display a list of versions of the image located in the 'airflow' folder
boil image list airflow
```

## Advanced Building Options

### Use Remote Cache

> [!NOTE]
> The default builder (with the `docker` driver) doesn't support the registry cache storage backend. You must create
> a new builder using the `docker-container` driver and either set this new builder as the default or pass
> `-- --builder <NAME>` to use it:
>
> ```shell
> docker builder create --name container --driver=docker-container
> boil build airflow --cache-registry oci.example.org -- --builder container
> ```

boil offers to option to automatically pull from and push to a remote cache. This feature can be
enabled by using the `--cache-registry` (and the optional `--cache-namespace`) argument:

```shell
# This will use `oci.example.org/<NAMESPACE>-cache/airflow` to store and retrieve cached layers
boil build airflow --cache-registry oci.example.org

# This will use `oci.example.org/foo/airflow` to store and retrieve cached layers
boil build airflow --cache-registry oci.example.org --cache-namespace foo
```
