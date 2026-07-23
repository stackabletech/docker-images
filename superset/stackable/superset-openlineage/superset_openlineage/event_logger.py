from __future__ import annotations

import logging
from datetime import datetime, timezone
from typing import Any

from openlineage.client.event_v2 import Job, Run, RunEvent, RunState
from openlineage.client.facet_v2 import job_type_job, sql_job

from superset.utils.log import DBEventLogger
from superset_openlineage import settings
from superset_openlineage.client import emit
from superset_openlineage.context import PendingRun, pop_pending
from superset_openlineage.facets import SupersetJobFacet
from superset_openlineage.jobname import render_job_name, resolve_identity

logger = logging.getLogger(__name__)


class OpenLineageEventLogger(DBEventLogger):
    """DBEventLogger that also emits OpenLineage run events on query success."""

    def log(  # type: ignore[override]
        self, user_id: int | None, action: str, *args: Any, **kwargs: Any
    ) -> None:
        # Always preserve default DB audit logging.
        self._persist(user_id, action, *args, **kwargs)
        if action != "execute_sql":
            return
        try:
            self._emit_lineage(kwargs)
        except Exception as ex:  # noqa: BLE001 - emission must never break a query
            logger.warning("OpenLineage: event emission failed: %s", ex)

    def _persist(
        self, user_id: int | None, action: str, *args: Any, **kwargs: Any
    ) -> None:
        super().log(user_id, action, *args, **kwargs)

    def _emit_lineage(self, kwargs: dict[str, Any]) -> None:
        pending = pop_pending()
        if pending is None:
            return
        job = self._build_job(pending, kwargs)
        run = Run(runId=pending.run_id)
        producer = settings.get_producer()
        start_iso = (
            datetime.fromtimestamp(pending.start_time, tz=timezone.utc).isoformat()
            if pending.start_time
            else datetime.now(timezone.utc).isoformat()
        )
        end_iso = datetime.now(timezone.utc).isoformat()
        emit(
            RunEvent(
                eventType=RunState.START,
                eventTime=start_iso,
                run=run,
                job=job,
                inputs=pending.inputs,
                outputs=pending.outputs,
                producer=producer,
            )
        )
        emit(
            RunEvent(
                eventType=RunState.COMPLETE,
                eventTime=end_iso,
                run=run,
                job=job,
                inputs=pending.inputs,
                outputs=pending.outputs,
                producer=producer,
            )
        )

    def _build_job(self, pending: PendingRun, kwargs: dict[str, Any]) -> Job:
        values = {
            "source": pending.source,
            "sql_editor_id": pending.sql_editor_id,
            "client_id": pending.client_id,
            "slice_id": kwargs.get("slice_id"),
            "dashboard_id": kwargs.get("dashboard_id"),
            "sql_hash": pending.sql_hash,
            "schema": pending.schema,
            "catalog": pending.catalog,
            "database": pending.database,
            "username": pending.username,
        }
        values["identity"] = resolve_identity(values)
        name = render_job_name(settings.get_job_name_template(), values)
        job_facets: dict[str, Any] = {
            "jobType": job_type_job.JobTypeJobFacet(
                processingType="BATCH",
                integration="SUPERSET",
                jobType="QUERY",
            ),
            "sql": sql_job.SQLJobFacet(
                query=pending.sql, dialect=pending.dialect or ""
            ),
            "superset": SupersetJobFacet(
                source=pending.source,
                dashboardId=kwargs.get("dashboard_id"),
                username=pending.username,
            ),
        }
        return Job(namespace=settings.get_namespace(), name=name, facets=job_facets)
