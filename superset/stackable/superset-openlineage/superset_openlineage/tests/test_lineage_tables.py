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
from sqlalchemy.engine import make_url

from superset_openlineage.lineage import build_input_datasets, extract_tables
from superset_openlineage.naming import TableRef


def test_extract_tables_simple_join() -> None:
    sql = "SELECT * FROM public.orders o JOIN public.customers c ON o.cid = c.id"
    refs = set(extract_tables(sql, "postgres"))
    assert TableRef(None, "public", "orders") in refs
    assert TableRef(None, "public", "customers") in refs


def test_extract_tables_excludes_ctes() -> None:
    sql = "WITH recent AS (SELECT * FROM public.orders) SELECT * FROM recent"
    refs = extract_tables(sql, "postgres")
    names = {r.table for r in refs}
    assert "orders" in names
    assert "recent" not in names


def test_extract_tables_bad_sql_returns_empty() -> None:
    assert extract_tables("this is not sql {{{", "postgres") == []


def test_build_input_datasets_names_and_namespaces() -> None:
    url = make_url("postgresql://u@h:5432/analytics")
    inputs = build_input_datasets("SELECT id FROM public.orders", url, "postgres")
    assert len(inputs) == 1
    assert inputs[0].namespace == "postgres://h:5432"
    assert inputs[0].name == "analytics.public.orders"
