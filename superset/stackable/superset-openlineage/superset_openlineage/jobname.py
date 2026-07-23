from __future__ import annotations

import re
from string import Template


def resolve_identity(values: dict[str, object]) -> str:
    source = values.get("source")
    if source == "chart":
        candidates = [values.get("slice_id"), values.get("sql_hash")]
    else:  # sqllab (and any other source)
        candidates = [
            values.get("sql_editor_id"),
            values.get("client_id"),
            values.get("sql_hash"),
        ]
    for candidate in candidates:
        if candidate:
            return str(candidate)
    return ""


def _normalize(name: str) -> str:
    # collapse repeated dots and strip leading/trailing separators
    name = re.sub(r"\.{2,}", ".", name)
    return name.strip(". ")


def render_job_name(template: str, values: dict[str, object]) -> str:
    safe = {k: ("" if v is None else str(v)) for k, v in values.items()}
    rendered = Template(template).safe_substitute(safe)
    normalized = _normalize(rendered)
    if not normalized:
        fallback = f"{safe.get('source', 'superset')}.{safe.get('sql_hash', '')}"
        return _normalize(fallback)
    return normalized
