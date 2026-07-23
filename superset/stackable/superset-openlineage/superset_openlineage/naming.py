from __future__ import annotations

from dataclasses import dataclass

from sqlalchemy.engine import URL

# SQLAlchemy backend name -> OpenLineage namespace scheme.
_SCHEME = {
    "postgresql": "postgres",
    "postgres": "postgres",
    "mysql": "mysql",
    "redshift": "redshift",
    "snowflake": "snowflake",
    "bigquery": "bigquery",
    "trino": "trino",
    "presto": "presto",
}

# SQLAlchemy backend name -> sqlglot dialect.
_DIALECT = {
    "postgresql": "postgres",
    "postgres": "postgres",
    "mysql": "mysql",
    "redshift": "redshift",
    "snowflake": "snowflake",
    "bigquery": "bigquery",
    "trino": "trino",
    "presto": "presto",
    "sqlite": "sqlite",
    "mssql": "tsql",
    "oracle": "oracle",
}


@dataclass(frozen=True)
class TableRef:
    catalog: str | None
    schema: str | None
    table: str


def _backend(url: URL) -> str:
    # e.g. "postgresql+psycopg2" -> "postgresql"
    return url.get_backend_name()


def dataset_namespace(url: URL) -> str:
    scheme = _SCHEME.get(_backend(url), _backend(url))
    host = url.host or "localhost"
    return f"{scheme}://{host}:{url.port}" if url.port else f"{scheme}://{host}"


def dataset_name(url: URL, ref: TableRef) -> str:
    parts = [ref.catalog, ref.schema, ref.table]
    if url.database and ref.catalog is None:
        # Prefix the connection database when the table has no explicit catalog.
        parts = [url.database, ref.schema, ref.table]
    return ".".join(p for p in parts if p)


def sqlglot_dialect(url: URL) -> str | None:
    return _DIALECT.get(_backend(url))
