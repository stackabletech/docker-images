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
from superset_openlineage.context import clear_pending, pop_pending
from superset_openlineage.query_logger import build_query_logger


def test_sqllab_source_pushes_pending_with_inputs() -> None:
    clear_pending()
    log = build_query_logger()
    log(
        "postgresql://u@h:5432/analytics",
        "SELECT id FROM public.orders",
        "public",
        "superset.sql_lab",
        None,
        None,
    )
    pending = pop_pending()
    assert pending is not None
    assert pending.source == "sqllab"
    assert pending.run_id
    assert any(d.name == "analytics.public.orders" for d in pending.inputs)


def test_chart_source_detected_from_client() -> None:
    clear_pending()
    log = build_query_logger()
    log(
        "postgresql://u@h:5432/analytics",
        "SELECT 1",
        None,
        "superset.models.core",
    )
    pending = pop_pending()
    assert pending is not None
    assert pending.source == "chart"


def test_ctas_pushes_pending_with_output_dataset() -> None:
    clear_pending()
    log = build_query_logger()
    log(
        "postgresql://u@h:5432/analytics",
        "CREATE TABLE analytics.public.summary AS "
        "SELECT o.amount AS revenue FROM public.orders o",
        "public",
        "superset.sql_lab",
        None,
        None,
    )
    pending = pop_pending()
    assert pending is not None
    assert pending.column_lineage is None
    assert len(pending.outputs) == 1
    output = pending.outputs[0]
    assert output.name == "analytics.public.summary"
    assert "columnLineage" in output.facets


def test_plain_select_pushes_pending_with_no_outputs() -> None:
    clear_pending()
    log = build_query_logger()
    log(
        "postgresql://u@h:5432/analytics",
        "SELECT id FROM public.orders",
        "public",
        "superset.sql_lab",
        None,
        None,
    )
    pending = pop_pending()
    assert pending is not None
    assert pending.outputs == []


def test_bad_input_never_raises_and_pushes_nothing_useful() -> None:
    clear_pending()
    log = build_query_logger()
    # a non-URL database value must not raise
    log(None, "SELECT 1", None, "superset.sql_lab")
    # either nothing pushed, or a pending with empty inputs — but no exception
    pending = pop_pending()
    if pending is not None:
        assert pending.inputs == []
