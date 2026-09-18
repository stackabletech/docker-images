# Airflow 3 OPA auth manager

Auth manager for Airflow 3 which delegates the authorization to an Open Policy
Agent

Build:

    uv build

Test:

    uv run --python 3.14 --group local pytest --disable-warnings

- `--python` must match the newest supported image's `python-version` in
  `airflow/boil-config.toml`; wheel availability differs between Python
  versions.
- The `local` group holds the Airflow and FAB versions to test against. The
  image build ignores it and installs both from
  `airflow/stackable/constraints/<PRODUCT_VERSION>/` instead.

## Updating `uv.lock`

The build runs `uv sync --locked`, so the lock must match `pyproject.toml`.

When adding an Airflow version to `airflow/boil-config.toml`:

1. In `pyproject.toml`, set the `local` group to that image's `apache-airflow`
   version, and its `apache-airflow-providers-fab` version from the constraints
   file.
2. Run `uv lock --upgrade`.

After any other change to `pyproject.toml`, step 2 alone.

Always `--upgrade`: uv otherwise keeps already-locked versions, which can leave
packages with no wheel for a newer Python — and the builder stage has no C
compiler to build them from source.
