# Airflow 3 OPA auth manager

Auth manager for Airflow 3 which delegates the authorization to an Open Policy
Agent

[uv](https://docs.astral.sh/uv/) is used to build the project:

    uv build

The unit tests can be run as follows:

    # Create directory for an SQLite database used by the test suite
    mkdir ~/airflow

    uv run pytest
