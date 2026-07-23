from __future__ import annotations

import logging
from typing import Any

from superset_openlineage import settings

logger = logging.getLogger(__name__)

_client: Any = None
_initialized: bool = False


def reset_client() -> None:
    global _client, _initialized
    _client = None
    _initialized = False


def get_client() -> Any:
    global _client, _initialized
    if _initialized:
        return _client
    _initialized = True
    if not settings.is_enabled():
        _client = None
        return None
    try:
        from openlineage.client import OpenLineageClient

        _client = OpenLineageClient()
    except Exception as ex:  # noqa: BLE001 - never break Superset on OL init
        logger.warning("OpenLineage: client init failed, disabling: %s", ex)
        _client = None
    return _client


def emit(event: Any) -> None:
    client = get_client()
    if client is None:
        return
    try:
        client.emit(event)
    except Exception as ex:  # noqa: BLE001 - emission must never break a query
        logger.warning("OpenLineage: emit failed, dropping event: %s", ex)
