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
