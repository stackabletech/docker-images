# Airflow constraints

To add a new version of Airflow (or python) you can download the applicable constraints file from upstream.

_The script is safe to run from outside of this directory. The file will be downloaded to the same folder that the
script resides. This example will assume you are running from the repository root directory._

```sh
# Use default Python version (specified in the script):
./airflow/download_constraints.sh 2.8.3

# Use a specific Python version:
./airflow/download_constraints.sh 2.8.3 3.11
```

Example output:

```output
Downloading constraints file for Airflow 2.8.3 (Python 3.9)
Successfully pulled new constraints file: constraints-2.8.3-python39.txt
```
