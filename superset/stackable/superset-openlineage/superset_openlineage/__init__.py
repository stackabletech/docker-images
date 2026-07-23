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
