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
from superset_openlineage.jobname import render_job_name, resolve_identity


def _vals(**over: object) -> dict[str, object]:
    base: dict[str, object] = {
        "source": "chart",
        "sql_editor_id": "",
        "client_id": "",
        "slice_id": "",
        "dashboard_id": "",
        "sql_hash": "deadbeef",
        "schema": "public",
        "catalog": "",
        "database": "analytics",
        "username": "alice",
    }
    base.update(over)
    return base


def test_identity_prefers_slice_id_for_chart() -> None:
    assert resolve_identity(_vals(source="chart", slice_id="42")) == "42"


def test_identity_prefers_editor_then_client_for_sqllab() -> None:
    assert (
        resolve_identity(_vals(source="sqllab", sql_editor_id="7", client_id="c1"))
        == "7"
    )
    assert (
        resolve_identity(_vals(source="sqllab", sql_editor_id="", client_id="c1"))
        == "c1"
    )


def test_identity_falls_back_to_sql_hash() -> None:
    assert resolve_identity(_vals(source="sqllab")) == "deadbeef"


def test_default_template() -> None:
    vals = _vals(source="chart", slice_id="42")
    vals["identity"] = resolve_identity(vals)
    assert render_job_name("$source.$identity", vals) == "chart.42"


def test_missing_placeholder_collapses_separators() -> None:
    vals = _vals(source="chart", slice_id="42", dashboard_id="")
    vals["identity"] = resolve_identity(vals)
    assert render_job_name("$source.$dashboard_id.$slice_id", vals) == "chart.42"


def test_unknown_placeholder_is_safe() -> None:
    vals = _vals(source="chart", slice_id="42")
    vals["identity"] = resolve_identity(vals)
    # $nope is left untouched by safe_substitute, then normalized away only if empty;
    # here it stays literal, proving no exception is raised.
    assert "chart" in render_job_name("$source.$nope", vals)


def test_empty_result_falls_back_to_source_hash() -> None:
    vals = _vals(source="chart", slice_id="", sql_hash="deadbeef")
    vals["identity"] = resolve_identity(vals)
    assert render_job_name("${slice_id}", vals) == "chart.deadbeef"
