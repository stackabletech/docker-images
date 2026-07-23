from __future__ import annotations

from typing import Any

from superset_openlineage.context import clear_pending


def init_app(app: Any) -> None:
    """Register a teardown hook that clears the request-scoped OpenLineage
    stash at the end of each app context, preventing cross-request leaks/
    mis-attribution on reused worker threads."""

    @app.teardown_appcontext
    def _clear_openlineage_stash(exc: BaseException | None = None) -> None:
        clear_pending()
