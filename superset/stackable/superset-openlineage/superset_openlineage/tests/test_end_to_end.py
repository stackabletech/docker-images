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

import superset_openlineage.client as client_mod
from superset_openlineage.client import reset_client
from superset_openlineage.context import clear_pending
from superset_openlineage.event_logger import OpenLineageEventLogger
from superset_openlineage.query_logger import build_query_logger


def test_sqllab_query_emits_events_via_file_transport(
    tmp_path: Any, monkeypatch: Any
) -> None:
    reset_client()
    clear_pending()
    # capture emitted events in-memory instead of a real transport
    events = []
    monkeypatch.setattr(client_mod, "_initialized", True, raising=False)
    monkeypatch.setattr(
        client_mod,
        "_client",
        type("C", (), {"emit": lambda self, e: events.append(e)})(),
        raising=False,
    )
    monkeypatch.setattr(
        OpenLineageEventLogger, "_persist", lambda self, *a, **k: None, raising=False
    )

    log_query = build_query_logger()
    log_query(
        "postgresql://u@h:5432/analytics",
        "SELECT id FROM public.orders",
        "public",
        "superset.sql_lab",
        None,
        {"sql_editor_id": "7"},
    )
    OpenLineageEventLogger().log(
        user_id=1, action="execute_sql", records=[{}], duration_ms=50
    )

    assert len(events) == 2
    job = events[0].job
    assert job.namespace == "superset"
    assert job.name == "sqllab.7"
    assert any(d.name == "analytics.public.orders" for d in events[0].inputs)
