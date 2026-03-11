# SuperSet Docker Image

## Constraints

The `constraints-<version>.txt` files come from e.g. <https://github.com/apache/superset/blob/x.y.z/requirements/base.txt> and are concatenated with the dependencies provided in <https://github.com/apache/superset/blob/x.y.z/requirements/docker.txt>, where that file exists (i.e. <4.1.0). This differs from version to version (4.1.4 shown here):

In some cases `apispec[yaml]==6.3.0` needed to be adjusted to `apispec==6.3.0` due to `ERROR: Constraints cannot have extras`.

If the constraints file contains an `-e file:.` directive, this can be commented out as we are install Superset using uv from PyPI and do not need to edit it subsequently.
