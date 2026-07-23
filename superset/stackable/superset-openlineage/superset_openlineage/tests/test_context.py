from superset_openlineage.context import (
    clear_pending,
    PendingRun,
    pop_pending,
    push_pending,
)


def _run(run_id: str) -> PendingRun:
    return PendingRun(
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


def test_push_then_pop_is_lifo() -> None:
    clear_pending()
    push_pending(_run("a"))
    push_pending(_run("b"))
    top = pop_pending()
    assert top is not None
    assert top.run_id == "b"
    next_ = pop_pending()
    assert next_ is not None
    assert next_.run_id == "a"
    assert pop_pending() is None


def test_clear_empties_stack() -> None:
    push_pending(_run("a"))
    clear_pending()
    assert pop_pending() is None
