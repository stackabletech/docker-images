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
