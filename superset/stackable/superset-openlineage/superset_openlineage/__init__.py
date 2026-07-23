"""OpenLineage integration for Apache Superset (config-only plugin)."""

__all__ = ["build_query_logger", "OpenLineageEventLogger", "init_app"]


# lazy re-export to avoid import cost at config load
def __getattr__(name: str) -> object:
    if name == "build_query_logger":
        from superset_openlineage.query_logger import build_query_logger

        return build_query_logger
    if name == "OpenLineageEventLogger":
        from superset_openlineage.event_logger import OpenLineageEventLogger

        return OpenLineageEventLogger
    if name == "init_app":
        from superset_openlineage.lifecycle import init_app

        return init_app
    raise AttributeError(name)
