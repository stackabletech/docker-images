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
from superset_openlineage.client import emit, get_client, reset_client


def test_disabled_returns_no_client(monkeypatch: Any) -> None:
    reset_client()
    monkeypatch.setenv("SUPERSET_OPENLINEAGE_ENABLED", "false")
    assert get_client() is None


def test_emit_never_raises_when_disabled(monkeypatch: Any) -> None:
    reset_client()
    monkeypatch.setenv("SUPERSET_OPENLINEAGE_ENABLED", "false")
    emit(object())  # must not raise


def test_emit_swallows_client_errors(monkeypatch: Any) -> None:
    reset_client()
    monkeypatch.setenv("SUPERSET_OPENLINEAGE_ENABLED", "true")

    class Boom:
        def emit(self, _event: Any) -> None:
            raise RuntimeError("transport down")

    monkeypatch.setattr(client_mod, "_client", Boom(), raising=False)
    monkeypatch.setattr(client_mod, "_initialized", True, raising=False)
    emit(object())  # must not raise
