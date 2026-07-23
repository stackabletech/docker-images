from __future__ import annotations

import os

_PRODUCER_DEFAULT = "https://github.com/apache/superset"


def get_namespace() -> str:
    return os.environ.get("SUPERSET_OPENLINEAGE_NAMESPACE", "superset")


def get_producer() -> str:
    return os.environ.get("SUPERSET_OPENLINEAGE_PRODUCER", _PRODUCER_DEFAULT)


def get_job_name_template() -> str:
    return os.environ.get("SUPERSET_OPENLINEAGE_JOB_NAME", "$source.$identity")


def is_enabled() -> bool:
    return os.environ.get("SUPERSET_OPENLINEAGE_ENABLED", "true").lower() not in (
        "false",
        "0",
        "no",
    )
