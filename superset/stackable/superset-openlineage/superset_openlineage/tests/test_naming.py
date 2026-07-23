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
from sqlalchemy.engine import make_url

from superset_openlineage.naming import (
    dataset_name,
    dataset_namespace,
    sqlglot_dialect,
    TableRef,
)


def test_postgres_namespace_and_name() -> None:
    url = make_url("postgresql://u:p@db.example:5432/analytics")
    assert dataset_namespace(url) == "postgres://db.example:5432"
    ref = TableRef(catalog=None, schema="public", table="orders")
    assert dataset_name(url, ref) == "analytics.public.orders"


def test_mysql_name_has_no_schema() -> None:
    url = make_url("mysql://u:p@h:3306/sales")
    ref = TableRef(catalog=None, schema=None, table="orders")
    assert dataset_namespace(url) == "mysql://h:3306"
    assert dataset_name(url, ref) == "sales.orders"


def test_trino_uses_catalog_schema_table() -> None:
    url = make_url("trino://u@h:8080/hive")
    ref = TableRef(catalog="hive", schema="default", table="events")
    assert dataset_namespace(url) == "trino://h:8080"
    assert dataset_name(url, ref) == "hive.default.events"


def test_generic_fallback() -> None:
    url = make_url("somedb://u@h:1234/thedb")
    ref = TableRef(catalog=None, schema="s", table="t")
    assert dataset_namespace(url) == "somedb://h:1234"
    assert dataset_name(url, ref) == "thedb.s.t"


def test_dialect_mapping() -> None:
    assert sqlglot_dialect(make_url("postgresql://h/d")) == "postgres"
    assert sqlglot_dialect(make_url("mysql://h/d")) == "mysql"
    assert sqlglot_dialect(make_url("trino://h/d")) == "trino"
