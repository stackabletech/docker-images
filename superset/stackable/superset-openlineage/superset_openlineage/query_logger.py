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

import hashlib
import logging
import time
from typing import Any, Callable

from sqlalchemy.engine import make_url

from superset_openlineage.context import PendingRun, push_pending
from superset_openlineage.lineage import build_input_datasets, build_output_datasets
from superset_openlineage.naming import sqlglot_dialect

logger = logging.getLogger(__name__)


def _sql_hash(sql: str) -> str:
    return hashlib.sha1(sql.encode("utf-8", "ignore")).hexdigest()[:12]  # noqa: S324


def _source_for(client: str | None) -> str:
    return "sqllab" if client == "superset.sql_lab" else "chart"


def build_query_logger() -> Callable[..., None]:
    def query_logger(
        database: Any,
        query: str,
        schema: str | None = None,
        client: str | None = None,
        security_manager: Any = None,
        log_params: dict[str, Any] | None = None,
        **_: Any,
    ) -> None:
        try:
            from openlineage.client.uuid import generate_new_uuid

            url = make_url(str(database))
            dialect = sqlglot_dialect(url)
            inputs = build_input_datasets(query, url, dialect)
            outputs = build_output_datasets(query, url, dialect)
            username = None
            try:
                username = getattr(
                    getattr(security_manager, "current_user", None),
                    "username",
                    None,
                )
            except Exception:  # noqa: BLE001
                username = None
            log_params = log_params or {}
            catalog_value = url.query.get("catalog") if url.query else None
            catalog: str | None
            if isinstance(catalog_value, (list, tuple)):
                catalog = catalog_value[0] if catalog_value else None
            else:
                catalog = catalog_value
            push_pending(
                PendingRun(
                    run_id=str(generate_new_uuid()),
                    sql=query,
                    dialect=dialect,
                    source=_source_for(client),
                    sql_hash=_sql_hash(query),
                    schema=schema,
                    catalog=catalog,
                    database=url.database,
                    username=username,
                    sql_editor_id=log_params.get("sql_editor_id"),
                    client_id=log_params.get("client_id"),
                    start_time=time.time(),
                    inputs=inputs,
                    outputs=outputs,
                    column_lineage=None,
                )
            )
        except Exception as ex:  # noqa: BLE001 - never break the query
            logger.warning("OpenLineage: query_logger failed: %s", ex)

    return query_logger
