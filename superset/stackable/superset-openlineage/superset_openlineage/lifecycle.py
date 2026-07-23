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

from typing import Any

from superset_openlineage.context import clear_pending


def init_app(app: Any) -> None:
    """Register a teardown hook that clears the request-scoped OpenLineage
    stash at the end of each app context, preventing cross-request leaks/
    mis-attribution on reused worker threads."""

    @app.teardown_appcontext
    def _clear_openlineage_stash(exc: BaseException | None = None) -> None:
        clear_pending()
