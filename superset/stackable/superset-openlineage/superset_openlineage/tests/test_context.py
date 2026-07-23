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
from superset_openlineage.context import (
    clear_pending,
    PendingRun,
    pop_pending,
    push_pending,
)


def _run(run_id: str) -> PendingRun:
    return PendingRun(
        run_id=run_id,
        sql="SELECT 1",
        dialect="postgres",
        source="sqllab",
        sql_hash="abc",
        schema=None,
        catalog=None,
        database="d",
        username="u",
        sql_editor_id=None,
        client_id=None,
        start_time=0.0,
        inputs=[],
        outputs=[],
        column_lineage=None,
    )


def test_push_then_pop_is_lifo() -> None:
    clear_pending()
    push_pending(_run("a"))
    push_pending(_run("b"))
    top = pop_pending()
    assert top is not None
    assert top.run_id == "b"
    next_ = pop_pending()
    assert next_ is not None
    assert next_.run_id == "a"
    assert pop_pending() is None


def test_clear_empties_stack() -> None:
    push_pending(_run("a"))
    clear_pending()
    assert pop_pending() is None
