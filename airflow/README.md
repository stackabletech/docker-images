# Airflow constraints

To add a new version of Airflow (or python) you can download the applicable constraints file from upstream.

_The script is safe to run from outside of this directory. The file will be downloaded to the same folder that the
script resides. This example will assume you are running from the repository root directory._

```sh
# Specify Airflow and Python versions:
./airflow/download_constraints.sh 3.0.6 3.12
```

Example output:

```output
Downloading constraints file for Airflow 3.0.6 (Python 3.12)
Successfully pulled new constraints file: constraints-3.0.6-python3.12.txt
```

## Airflow providers/extras

The providers are released independently of Airflow.
The list of provider packages are listed in the build configuration file, matching the groups used in the online documentation to make them easier to compare and manage (these will be concatentated into a single list in the Dockerfile).
The expected versions are listed in the constraints files, but these can change over time.
To keep the installation tightly coupled to the associated constraints it is best to only use providers listed in the relevant constraints file.

### Version 3.0.6

Applying the filter above results in the omission of the following providers:

- `apache-atlas`
- `apache-webhdfs`

Other than the above, the only other providers that are currently excluded are:

- `mysql`, as it requires an implementation of: <https://github.com/apache/airflow/blob/main/scripts/docker/install_mysql.sh>
- `apache-spark`, due to the size (roughly 500MB) and the number of high/critical CVEs it adds to the image
