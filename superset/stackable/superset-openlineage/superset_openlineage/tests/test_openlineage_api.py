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
def test_openlineage_symbols_exist() -> None:
    from openlineage.client import OpenLineageClient  # noqa: F401
    from openlineage.client.event_v2 import (  # noqa: F401
        InputDataset,
        Job,
        OutputDataset,
        Run,
        RunEvent,
        RunState,
    )
    from openlineage.client.facet_v2 import (  # noqa: F401
        column_lineage_dataset,
        job_type_job,
        schema_dataset,
        sql_job,
    )
    from openlineage.client.uuid import generate_new_uuid

    assert str(generate_new_uuid())
    assert RunState.START
    assert RunState.COMPLETE


def test_package_imports() -> None:
    import superset_openlineage  # noqa: F401
