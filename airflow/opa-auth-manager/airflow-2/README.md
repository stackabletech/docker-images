# Airflow 2 OPA auth manager

Auth manager for Airflow 2 which delegates the authorization to an Open Policy
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

## Updating `uv.lock`

The build runs `uv sync --locked`, so the lock must match `pyproject.toml` or
the build fails. After any change to it, run `uv lock --upgrade`.

This tree serves only Airflow 2.9.3, so there is no per-release Airflow bump;
the `airflow-3` project alongside it does have one.

`--upgrade`, not plain `uv lock`: uv keeps already-locked versions while they
still resolve. The tests run on the image's `PYTHON_VERSION`, and packages
locked against an older interpreter may ship no wheel for the new one — the
builder stage has no C compiler to build them from source.
