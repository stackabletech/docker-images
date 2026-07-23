from __future__ import annotations

from openlineage.client.facet_v2 import column_lineage_dataset, schema_dataset
from sqlalchemy.engine import make_url

from superset_openlineage.lineage import build_output_datasets


def test_ctas_produces_output_dataset_with_column_lineage() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    sql = (
        "CREATE TABLE analytics.public.summary AS "
        "SELECT o.amount AS revenue FROM public.orders o"
    )
    outputs = build_output_datasets(sql, url, "postgres")
    assert len(outputs) == 1
    output = outputs[0]
    assert output.name == "analytics.public.summary"
    facets = output.facets
    assert facets is not None
    assert "columnLineage" in facets
    column_lineage_facet = facets["columnLineage"]
    assert isinstance(
        column_lineage_facet, column_lineage_dataset.ColumnLineageDatasetFacet
    )
    fields = column_lineage_facet.fields
    assert "revenue" in fields
    inputs = fields["revenue"].inputFields
    assert any(
        i.name == "analytics.public.orders" and i.field == "amount" for i in inputs
    )
    assert "schema" in facets
    schema_facet = facets["schema"]
    assert isinstance(schema_facet, schema_dataset.SchemaDatasetFacet)
    assert schema_facet.fields is not None
    schema_fields = {f.name for f in schema_facet.fields}
    assert schema_fields == {"revenue"}


def test_cvas_produces_output_dataset() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    sql = (
        "CREATE VIEW analytics.public.summary_v AS "
        "SELECT o.amount AS revenue FROM public.orders o"
    )
    outputs = build_output_datasets(sql, url, "postgres")
    assert len(outputs) == 1
    assert outputs[0].name == "analytics.public.summary_v"


def test_plain_select_produces_no_output_dataset() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    sql = "SELECT id FROM public.orders"
    assert build_output_datasets(sql, url, "postgres") == []


def test_bad_sql_never_raises_and_returns_empty() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    assert build_output_datasets("not sql {{{", url, "postgres") == []


def test_create_table_without_query_returns_empty() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    assert build_output_datasets("CREATE TABLE t (id int)", url, "postgres") == []
