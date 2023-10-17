# SuperSet Docker Image

## Constraints

The `constraints-<version>.txt` files come from e.g. <https://github.com/apache/superset/blob/2.0.1/requirements/base.txt> and are concatenated with the dependencies provided in <https://github.com/apache/superset/blob/2.0.1/requirements/docker.txt>. This differs from version to version (2.0.1 shown here).

In some cases `apispec[yaml]==3.3.2` needed to be adjusted to `apispec==3.3.2` due to `ERROR: Constraints cannot have extras`.
