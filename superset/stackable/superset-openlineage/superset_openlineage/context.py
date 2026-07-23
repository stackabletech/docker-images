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

from contextvars import ContextVar
from dataclasses import dataclass, field
from typing import Any

_PENDING: ContextVar[list["PendingRun"] | None] = ContextVar("ol_pending", default=None)


@dataclass
class PendingRun:
    run_id: str
    sql: str
    dialect: str | None
    source: str
    sql_hash: str
    schema: str | None
    catalog: str | None
    database: str | None
    username: str | None
    sql_editor_id: str | None
    client_id: str | None
    start_time: float
    inputs: list[Any] = field(default_factory=list)
    outputs: list[Any] = field(default_factory=list)
    column_lineage: Any = None


def _stack() -> list[PendingRun]:
    stack = _PENDING.get()
    if stack is None:
        stack = []
        _PENDING.set(stack)
    return stack


def push_pending(run: PendingRun) -> None:
    stack = list(_stack())
    stack.append(run)
    _PENDING.set(stack)


def pop_pending() -> PendingRun | None:
    stack = list(_stack())
    if not stack:
        return None
    run = stack.pop()
    _PENDING.set(stack)
    return run


def clear_pending() -> None:
    _PENDING.set([])
