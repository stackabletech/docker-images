from __future__ import annotations

import attr
from openlineage.client.facet_v2 import JobFacet


@attr.define
class SupersetJobFacet(JobFacet):
    source: str | None = None
    dashboardId: int | None = None  # noqa: N815 - OpenLineage uses camelCase
    username: str | None = None
