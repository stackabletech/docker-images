from __future__ import annotations

import logging
from typing import Any

import sqlglot
from openlineage.client.event_v2 import InputDataset, OutputDataset
from openlineage.client.facet_v2 import column_lineage_dataset, schema_dataset
from sqlalchemy.engine import URL
from sqlglot import exp
from sqlglot.lineage import lineage as sqlglot_lineage

from superset_openlineage.naming import dataset_name, dataset_namespace, TableRef

logger = logging.getLogger(__name__)


def _parse(sql: str, dialect: str | None) -> exp.Expression | None:
    try:
        return sqlglot.parse_one(sql, read=dialect)
    except Exception as ex:  # noqa: BLE001 - best-effort parse
        logger.debug("OpenLineage: could not parse SQL: %s", ex)
        return None


def extract_tables(sql: str, dialect: str | None) -> list[TableRef]:
    expression = _parse(sql, dialect)
    if expression is None:
        return []
    cte_names = {cte.alias_or_name for cte in expression.find_all(exp.CTE)}
    seen: set[TableRef] = set()
    refs: list[TableRef] = []
    for tbl in expression.find_all(exp.Table):
        if tbl.name in cte_names:
            continue
        ref = TableRef(
            catalog=tbl.catalog or None,
            schema=tbl.db or None,
            table=tbl.name,
        )
        if ref not in seen:
            seen.add(ref)
            refs.append(ref)
    return refs


def build_input_datasets(sql: str, url: URL, dialect: str | None) -> list[InputDataset]:
    namespace = dataset_namespace(url)
    datasets: list[InputDataset] = []
    for ref in extract_tables(sql, dialect):
        datasets.append(InputDataset(namespace=namespace, name=dataset_name(url, ref)))
    return datasets


def _output_columns(expression: exp.Expression) -> list[str]:
    select = expression.find(exp.Select)
    if select is None:
        return []
    names = []
    for projection in select.expressions:
        name = projection.alias_or_name
        if name and name != "*":
            names.append(name)
    return names


def _table_ref_for(node: Any) -> TableRef | None:
    tbl = node.expression.find(exp.Table) if node.expression else None
    if tbl is None:
        return None
    return TableRef(catalog=tbl.catalog or None, schema=tbl.db or None, table=tbl.name)


def build_column_lineage_facet(
    sql: str, url: URL, dialect: str | None
) -> "column_lineage_dataset.ColumnLineageDatasetFacet | None":
    expression = _parse(sql, dialect)
    if expression is None:
        return None
    try:
        namespace = dataset_namespace(url)
        fields: dict[str, column_lineage_dataset.Fields] = {}
        for out_col in _output_columns(expression):
            input_fields: list[column_lineage_dataset.InputField] = []
            root = sqlglot_lineage(out_col, sql, dialect=dialect)
            for leaf in root.walk():
                # only true leaves (no further upstream) reference a
                # source column of a real table; intermediate/CTE nodes
                # must not be reported as sources (I5).
                if leaf is root or leaf.downstream or not leaf.name:
                    continue
                ref = _table_ref_for(leaf)
                if ref is None or "." not in leaf.name:
                    continue
                col = leaf.name.split(".")[-1]
                input_fields.append(
                    column_lineage_dataset.InputField(
                        namespace=namespace,
                        name=dataset_name(url, ref),
                        field=col,
                    )
                )
            if input_fields:
                fields[out_col] = column_lineage_dataset.Fields(
                    inputFields=input_fields
                )
        return (
            column_lineage_dataset.ColumnLineageDatasetFacet(fields=fields)
            if fields
            else None
        )
    except Exception as ex:  # noqa: BLE001 - column lineage is best-effort
        logger.debug("OpenLineage: column lineage failed: %s", ex)
        return None


def _ctas_target(expression: exp.Expression) -> tuple[TableRef, exp.Expression] | None:
    """Return (target table ref, inner SELECT) for a CTAS/CVAS Create, else None."""
    if not isinstance(expression, exp.Create):
        return None
    query = expression.args.get("expression")
    if query is None or not isinstance(query, exp.Query):
        return None
    tbl = expression.find(exp.Table)
    if tbl is None:
        return None
    ref = TableRef(catalog=tbl.catalog or None, schema=tbl.db or None, table=tbl.name)
    return ref, query


def build_output_datasets(
    sql: str, url: URL, dialect: str | None
) -> list[OutputDataset]:
    """Build the OUTPUT dataset for CTAS/CVAS statements only.

    Column lineage is only meaningful when there is a real output table,
    so plain SELECT queries (and any other non-CTAS/CVAS statement)
    intentionally return an empty list.
    """
    try:
        expression = _parse(sql, dialect)
        if expression is None:
            return []
        target = _ctas_target(expression)
        if target is None:
            return []
        ref, inner_query = target
        namespace = dataset_namespace(url)
        name = dataset_name(url, ref)

        facets: dict[str, Any] = {}
        columns = _output_columns(inner_query)
        if columns:
            facets["schema"] = schema_dataset.SchemaDatasetFacet(
                fields=[
                    schema_dataset.SchemaDatasetFacetFields(name=col) for col in columns
                ]
            )
        column_lineage = build_column_lineage_facet(
            inner_query.sql(dialect=dialect), url, dialect
        )
        if column_lineage is not None:
            facets["columnLineage"] = column_lineage

        return [OutputDataset(namespace=namespace, name=name, facets=facets)]
    except Exception as ex:  # noqa: BLE001 - output lineage is best-effort
        logger.debug("OpenLineage: output dataset extraction failed: %s", ex)
        return []
