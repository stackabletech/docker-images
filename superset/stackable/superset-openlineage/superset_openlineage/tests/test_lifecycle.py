from __future__ import annotations

from typing import Any, Callable

from superset_openlineage import context
from superset_openlineage.lifecycle import init_app


class FakeApp:
    """Stand-in for a Flask app, capturing the registered teardown callback."""

    def __init__(self) -> None:
        self.handler: Callable[..., Any] | None = None

    def teardown_appcontext(self, f: Callable[..., Any]) -> Callable[..., Any]:
        self.handler = f
        return f


def _run(run_id: str) -> context.PendingRun:
    return context.PendingRun(
        run_id=run_id,
        sql="SELECT 1",
        dialect="postgres",
        source="sqllab",
        sql_hash="abc",
        schema=None,
        catalog=None,
        database="d",
        username="u",
        sql_editor_id=None,
        client_id=None,
        start_time=0.0,
        inputs=[],
        outputs=[],
        column_lineage=None,
    )


def test_init_app_registers_teardown_handler() -> None:
    fake = FakeApp()
    init_app(fake)
    assert callable(fake.handler)


def test_teardown_handler_clears_pending_stash() -> None:
    fake = FakeApp()
    init_app(fake)
    context.push_pending(_run("leaked"))

    assert fake.handler is not None
    fake.handler(None)

    assert context.pop_pending() is None
