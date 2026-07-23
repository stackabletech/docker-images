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
