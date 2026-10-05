# Airflow 2 OPA auth manager

Auth manager for Airflow 2 which delegates the authorization to an Open Policy
Agent

Build:

    uv build

Test:

    uv run --python 3.9 --group local pytest --disable-warnings

- `--python` must match the image's `python-version` in
  `airflow/boil-config.toml`; wheel availability differs between Python
  versions.
- The `local` group holds the Airflow and FAB versions to test against. The
  image build ignores it and installs both from
  `airflow/stackable/constraints/<PRODUCT_VERSION>/` instead.

## Updating `uv.lock`

The build runs `uv sync --locked`, so the lock must match `pyproject.toml`.

Run `uv lock --upgrade` after any change to it.

Always `--upgrade`: uv otherwise keeps already-locked versions, which can leave
packages with no wheel for a newer Python — and the builder stage has no C
compiler to build them from source.
