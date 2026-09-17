# Airflow 3 OPA auth manager

Auth manager for Airflow 3 which delegates the authorization to an Open Policy
Agent

[uv](https://docs.astral.sh/uv/) is used to build the project:

    uv build

The unit tests can be run as follows:

    # Create directory for an SQLite database used by the test suite
    mkdir ~/airflow

    uv run --group local pytest

The `local` dependency group supplies the Airflow version to test against.
The image build does not use it: it installs Airflow and the FAB provider
from `airflow/stackable/constraints/<PRODUCT_VERSION>/`, so the tests there
run against the versions that image actually ships.
