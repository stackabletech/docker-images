# superset-openlineage

OpenLineage integration for Apache Superset, shipped in the Stackable Superset
image. It emits OpenLineage events when SQL Lab queries run and when dashboard
charts fetch data.

It is **config-only**: no changes are made to the Superset source tree, and the
plugin stays completely inert unless it is explicitly enabled in
`superset_config.py`. The package is installed into the image (see the
`superset/Dockerfile` build stage), but activation is left to the operator/admin.

## Enable it

Add the following to `superset_config.py`:

```python
from superset_openlineage import (
    build_query_logger,
    OpenLineageEventLogger,
    init_app,
)

QUERY_LOGGER = build_query_logger()
EVENT_LOGGER = OpenLineageEventLogger()

# Registers a teardown_request that clears the request-scoped stash
# (required to avoid cross-request lineage mis-attribution on reused workers).
def FLASK_APP_MUTATOR(app):
    init_app(app)
```

## Configure the transport

Point the client at a lineage backend with standard OpenLineage configuration
(environment variables or `openlineage.yml`):

```bash
export OPENLINEAGE_URL=http://marquez:5000
export OPENLINEAGE_API_KEY=<token>        # optional
# or set OPENLINEAGE_DISABLED=true to turn emission off entirely
```

## Plugin settings

| Env var | Default | Meaning |
|---|---|---|
| `SUPERSET_OPENLINEAGE_ENABLED` | `true` | Master switch for the plugin |
| `SUPERSET_OPENLINEAGE_NAMESPACE` | `superset` | OpenLineage job namespace |
| `SUPERSET_OPENLINEAGE_PRODUCER` | Superset repo URL | `producer` URI on events |
| `SUPERSET_OPENLINEAGE_JOB_NAME` | `$source.$identity` | Job-name template (`string.Template`) |

### Job-name template

A Python `string.Template`. Placeholders: `$source`, `$identity`,
`$sql_editor_id`, `$client_id`, `$slice_id`, `$dashboard_id`, `$sql_hash`,
`$schema`, `$catalog`, `$database`, `$username`. Placeholders that do not apply
in a given context render empty and surrounding separators collapse; an empty
result falls back to `$source.$sql_hash`.

## Known limitations

- Failed queries and cache hits emit no events (the config-only hooks provide no
  failure or cache signal).
- Column-level lineage is emitted only for `CTAS`/`CVAS` statements (which have a
  real output table); plain `SELECT` queries (charts/dashboards and most SQL Lab)
  carry table-level inputs only.
- Queries executed via Superset's newer SQL execution engine
  (`superset/sql/execution/*`) are not covered yet — they invoke `QUERY_LOGGER`
  but emit no completion event, so they produce no lineage (their pending stash
  entries are reclaimed by the teardown hook).

## Tests

Unit tests live under `superset_openlineage/tests/` and require
`openlineage-python`, `sqlglot`, `SQLAlchemy` and `pytest`. They are not
installed into the image.
