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
