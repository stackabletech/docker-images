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
from typing import Any

import superset_openlineage.event_logger as el_mod
from superset_openlineage.context import clear_pending, PendingRun, push_pending
from superset_openlineage.event_logger import OpenLineageEventLogger


def _pending() -> PendingRun:
    return PendingRun(
        run_id="11111111-1111-7111-8111-111111111111",
        sql="SELECT id FROM public.orders",
        dialect="postgres",
        source="chart",
        sql_hash="deadbeef",
        schema="public",
        catalog=None,
        database="analytics",
        username="alice",
        sql_editor_id=None,
        client_id=None,
        start_time=0.0,
        inputs=[],
        outputs=[],
        column_lineage=None,
    )


def test_execute_sql_emits_start_and_complete(monkeypatch: Any) -> None:
    clear_pending()
    push_pending(_pending())
    emitted = []
    monkeypatch.setattr(el_mod, "emit", lambda e: emitted.append(e))
    # do not touch the DB in the parent logger
    monkeypatch.setattr(
        OpenLineageEventLogger, "_persist", lambda self, *a, **k: None, raising=False
    )

    logger = OpenLineageEventLogger()
    logger.log(
        user_id=1,
        action="execute_sql",
        records=[{}],
        dashboard_id=9,
        slice_id=42,
        duration_ms=120,
        curated_payload={},
        curated_form_data={},
    )
    assert len(emitted) == 2
    assert emitted[0].job.name == "chart.42"


def test_non_execute_sql_emits_nothing(monkeypatch: Any) -> None:
    clear_pending()
    emitted = []
    monkeypatch.setattr(el_mod, "emit", lambda e: emitted.append(e))
    monkeypatch.setattr(
        OpenLineageEventLogger, "_persist", lambda self, *a, **k: None, raising=False
    )
    logger = OpenLineageEventLogger()
    logger.log(user_id=1, action="DashboardRestApi.get", records=[{}])
    assert emitted == []


def test_emit_failure_does_not_propagate(monkeypatch: Any) -> None:
    clear_pending()
    push_pending(_pending())

    def boom(_e: Any) -> None:
        raise RuntimeError("down")

    monkeypatch.setattr(el_mod, "emit", boom)
    monkeypatch.setattr(
        OpenLineageEventLogger, "_persist", lambda self, *a, **k: None, raising=False
    )
    logger = OpenLineageEventLogger()
    # must not raise
    logger.log(user_id=1, action="execute_sql", records=[{}], slice_id=42)
