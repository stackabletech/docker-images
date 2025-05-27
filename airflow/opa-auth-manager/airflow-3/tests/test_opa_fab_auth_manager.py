#
# To make these tests relevant we would have to package the OPA auth manager as
# an Airflow provider and use `breeze` to set up a docker environment with Airflow
# and an SQLite database.
#
# Then we could run these tests against the Airflow instance and use the Airflow API to
# actually test the effect of Rego policies on user authorization.
#
from unittest import mock
from unittest.mock import Mock

import pytest
from airflow.api_fastapi.auth.managers.models.resource_details import (
    DagAccessEntity,
    DagDetails,
)
from airflow.providers.fab.www.extensions.init_appbuilder import init_appbuilder
from airflow.providers.fab.www.security.permissions import (
    ACTION_CAN_CREATE,
    ACTION_CAN_DELETE,
    ACTION_CAN_EDIT,
    ACTION_CAN_READ,
    RESOURCE_DAG,
    RESOURCE_DAG_RUN,
    RESOURCE_TASK_INSTANCE,
)
from flask import Flask
from tests_common.test_utils.config import conf_vars

from opa_auth_manager.opa_fab_auth_manager import OpaFabAuthManager


@pytest.fixture
def flask_app():
    with conf_vars(
        {
            (
                "core",
                "auth_manager",
            ): "opa_auth_manager.opa_fab_auth_manager.OpaFabAuthManager",
        }
    ):
        yield Flask(__name__)


@pytest.fixture
def auth_manager_with_appbuilder(flask_app):
    flask_app.config["AUTH_RATE_LIMITED"] = False
    flask_app.config["SERVER_NAME"] = "localhost"

    appbuilder = init_appbuilder(flask_app, enable_plugins=False)
    auth_manager = OpaFabAuthManager()
    auth_manager.appbuilder = appbuilder
    auth_manager.init_flask_resources()
    return auth_manager


class TestOpaFabAuthManager:
    @pytest.mark.parametrize(
        "method, dag_access_entity, dag_details, user_permissions, expected_opa_result, expected_result",
        [
            # Scenario 1 #
            # With global permissions on Dags
            (
                "GET",
                None,
                None,
                [(ACTION_CAN_READ, RESOURCE_DAG)],
                True,
                True,
            ),
            # On specific DAG with global permissions on Dags
            (
                "GET",
                None,
                DagDetails(id="test_dag_id"),
                [(ACTION_CAN_READ, RESOURCE_DAG)],
                True,
                True,
            ),
            # With permission on a specific DAG
            (
                "GET",
                None,
                DagDetails(id="test_dag_id"),
                [
                    (ACTION_CAN_READ, "DAG:test_dag_id"),
                    (ACTION_CAN_READ, "DAG:test_dag_id2"),
                ],
                True,
                True,
            ),
            # Without permission on a specific DAG (wrong method)
            (
                "POST",
                None,
                DagDetails(id="test_dag_id"),
                [(ACTION_CAN_READ, "DAG:test_dag_id")],
                False,
                False,
            ),
            # Without permission on a specific DAG
            (
                "GET",
                None,
                DagDetails(id="test_dag_id2"),
                [(ACTION_CAN_READ, "DAG:test_dag_id")],
                False,
                False,
            ),
            # Without permission on DAGs
            (
                "GET",
                None,
                None,
                [(ACTION_CAN_READ, "resource_test")],
                False,
                False,
            ),
            # Scenario 2 #
            # With global permissions on DAGs
            (
                "GET",
                DagAccessEntity.RUN,
                DagDetails(id="test_dag_id"),
                [(ACTION_CAN_READ, RESOURCE_DAG), (ACTION_CAN_READ, RESOURCE_DAG_RUN)],
                True,
                True,
            ),
            # Without read permissions on a specific DAG
            (
                "GET",
                DagAccessEntity.TASK_INSTANCE,
                DagDetails(id="test_dag_id"),
                [(ACTION_CAN_READ, RESOURCE_TASK_INSTANCE)],
                False,
                False,
            ),
            # With read permissions on a specific DAG but not on the DAG run
            (
                "GET",
                DagAccessEntity.TASK_INSTANCE,
                DagDetails(id="test_dag_id"),
                [
                    (ACTION_CAN_READ, "DAG:test_dag_id"),
                    (ACTION_CAN_READ, RESOURCE_TASK_INSTANCE),
                ],
                False,
                False,
            ),
            # With read permissions on a specific DAG but not on the DAG run
            (
                "GET",
                DagAccessEntity.TASK_INSTANCE,
                DagDetails(id="test_dag_id"),
                [
                    (ACTION_CAN_READ, "DAG:test_dag_id"),
                    (ACTION_CAN_READ, RESOURCE_TASK_INSTANCE),
                    (ACTION_CAN_READ, RESOURCE_DAG_RUN),
                ],
                True,
                True,
            ),
            # With edit permissions on a specific DAG and read on the DAG access entity
            (
                "DELETE",
                DagAccessEntity.TASK,
                DagDetails(id="test_dag_id"),
                [
                    (ACTION_CAN_EDIT, "DAG:test_dag_id"),
                    (ACTION_CAN_DELETE, RESOURCE_TASK_INSTANCE),
                ],
                True,
                True,
            ),
            # With edit permissions on a specific DAG and read on the DAG access entity
            (
                "POST",
                DagAccessEntity.RUN,
                DagDetails(id="test_dag_id"),
                [
                    (ACTION_CAN_EDIT, "DAG:test_dag_id"),
                    (ACTION_CAN_CREATE, RESOURCE_DAG_RUN),
                ],
                True,
                True,
            ),
            # Without permissions to edit the DAG
            (
                "POST",
                DagAccessEntity.RUN,
                DagDetails(id="test_dag_id"),
                [(ACTION_CAN_CREATE, RESOURCE_DAG_RUN)],
                False,
                False,
            ),
            # Without read permissions on a specific DAG
            (
                "GET",
                DagAccessEntity.TASK_LOGS,
                DagDetails(id="test_dag_id"),
                [(ACTION_CAN_READ, RESOURCE_TASK_INSTANCE)],
                False,
                False,
            ),
        ],
    )
    def test_is_authorized_dag(
        self,
        method,
        dag_access_entity,
        dag_details,
        user_permissions,
        expected_opa_result,
        expected_result,
        auth_manager_with_appbuilder,
    ):
        user = Mock()
        user.perms = user_permissions
        with mock.patch.object(
            OpaFabAuthManager, "_is_authorized_in_opa"
        ) as mock_is_authorized_in_opa:
            mock_is_authorized_in_opa.return_value = expected_opa_result
            result = auth_manager_with_appbuilder.is_authorized_dag(
                method=method,
                access_entity=dag_access_entity,
                details=dag_details,
                user=user,
            )
            assert result == expected_result
