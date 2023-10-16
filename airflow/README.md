# Airflow constraints

The constraints file can be downloaded from https://raw.githubusercontent.com/apache/airflow/constraints-2.7.2/constraints-3.9.txt where `2.7.2` is the airflow version and `3.9` is the python version.

Due to the build in the docker image, the python version must be specified without the `dot`. E.g. `3.9` must be renamed to `39` in the constraints file name.
