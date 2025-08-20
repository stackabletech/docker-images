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
boil show images

# Display a list of versions of the image located in the 'airflow' folder
boil show images airflow

# Soon (hopefully) implemented
boil show graph
```
