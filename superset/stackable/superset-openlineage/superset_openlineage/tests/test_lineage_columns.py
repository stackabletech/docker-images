# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
from __future__ import annotations

from typing import Any, Iterator

from sqlalchemy.engine import make_url
from sqlglot import exp

import superset_openlineage.lineage as lineage_mod
from superset_openlineage.lineage import build_column_lineage_facet


class _FakeNode:
    """Duck-typed stand-in for sqlglot.lineage.Node used to force an
    intermediate (non-leaf) node whose expression resolves to a real
    exp.Table, so we can prove it is excluded from the reported lineage."""

    def __init__(
        self,
        name: str,
        expression: exp.Expression | None = None,
        downstream: list["_FakeNode"] | None = None,
    ) -> None:
        self.name = name
        self.expression = expression
        self.downstream = downstream or []

    def walk(self) -> Iterator["_FakeNode"]:
        yield self
        for child in self.downstream:
            yield from child.walk()


def test_column_lineage_maps_output_to_source() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    sql = "SELECT o.amount AS revenue FROM public.orders o"
    facet = build_column_lineage_facet(sql, url, "postgres")
    assert facet is not None
    assert "revenue" in facet.fields
    inputs = facet.fields["revenue"].inputFields
    assert any(
        i.name == "analytics.public.orders" and i.field == "amount" for i in inputs
    )


def test_column_lineage_bad_sql_returns_none() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    assert build_column_lineage_facet("not sql {{{", url, "postgres") is None


def test_column_lineage_skips_non_leaf_nodes(monkeypatch: Any) -> None:
    # I5: an intermediate node that merely passes through a real table
    # (i.e. it still has downstream nodes) must not be reported as a
    # source column -- only the true leaf (no downstream) should be.
    url = make_url("postgresql://u@h:5432/analytics")

    leaf_table = exp.Table(
        this=exp.Identifier(this="orders"), db=exp.Identifier(this="public")
    )
    mid_table = exp.Table(
        this=exp.Identifier(this="staging"), db=exp.Identifier(this="public")
    )
    leaf_node = _FakeNode(name="orders.amount", expression=leaf_table, downstream=[])
    mid_node = _FakeNode(
        name="staging.amount", expression=mid_table, downstream=[leaf_node]
    )
    root_node = _FakeNode(name="revenue", expression=None, downstream=[mid_node])

    monkeypatch.setattr(lineage_mod, "sqlglot_lineage", lambda *a, **k: root_node)

    sql = "SELECT amount AS revenue FROM public.orders"
    facet = build_column_lineage_facet(sql, url, "postgres")
    assert facet is not None
    inputs = facet.fields["revenue"].inputFields
    names = {i.name for i in inputs}
    assert names == {"analytics.public.orders"}
    assert "analytics.public.staging" not in names
