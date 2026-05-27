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
    AccessView,
    DagAccessEntity,
    DagDetails,
)
from airflow.api_fastapi.common.types import MenuItem
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
    def test_init_wires_opa_cache_for_fastapi_apiserver(self):
        # The FastAPI api-server calls auth_manager.init() instead of
        # init_flask_resources(). Without wiring the cache from init() too,
        # any is_authorized_* call from a REST handler crashes with
        # AttributeError: 'OpaFabAuthManager' object has no attribute 'opa_cache'.
        auth_manager = OpaFabAuthManager()
        auth_manager.init()

        assert auth_manager.opa_cache is not None
        assert auth_manager.opa_session is not None

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

    def test_get_authorized_dag_ids_uses_opa_not_fab_db(
        self, auth_manager_with_appbuilder
    ):
        # Repro for the OPA listing bug: a user with no FAB permissions
        # (e.g. the default Public role) must still see the DAGs that OPA
        # allows. The FabAuthManager base override would return set() here
        # because it reads roles from the metadata DB.
        user = Mock()
        user.id = 1
        user.perms = []

        all_dag_ids = {"allowed_dag", "denied_dag"}
        session = Mock()
        session.execute.return_value = [Mock(dag_id=dag_id) for dag_id in all_dag_ids]

        def fake_is_authorized_dag(*, method, details=None, access_entity=None, user):
            return details is not None and details.id == "allowed_dag"

        with mock.patch.object(
            auth_manager_with_appbuilder,
            "is_authorized_dag",
            side_effect=fake_is_authorized_dag,
        ):
            result = auth_manager_with_appbuilder.get_authorized_dag_ids(
                user=user, method="GET", session=session
            )

        assert result == {"allowed_dag"}

    def test_get_authorized_dag_ids_provides_session_when_caller_omits_it(
        self, auth_manager_with_appbuilder
    ):
        # Real callers (api_fastapi/core_api/security.py) don't pass `session`.
        # Our override must rely on @provide_session to inject one; previously
        # it forwarded the default NEW_SESSION (None) and crashed with
        # 'NoneType' has no attribute 'execute'.
        user = Mock()
        user.perms = []

        session = Mock()
        session.execute.return_value = [Mock(dag_id="allowed_dag")]

        with (
            mock.patch("airflow.utils.session.create_session") as mock_create_session,
            mock.patch.object(
                auth_manager_with_appbuilder, "is_authorized_dag", return_value=True
            ),
        ):
            mock_create_session.return_value.__enter__.return_value = session
            result = auth_manager_with_appbuilder.get_authorized_dag_ids(
                user=user, method="GET"
            )

        assert result == {"allowed_dag"}
        mock_create_session.assert_called_once()

    @pytest.mark.parametrize(
        "menu_item, expected_method, expected_kwargs",
        [
            (MenuItem.ASSETS, "is_authorized_asset", {"method": "GET"}),
            (
                MenuItem.AUDIT_LOG,
                "is_authorized_dag",
                {"method": "GET", "access_entity": DagAccessEntity.AUDIT_LOG},
            ),
            (MenuItem.CONFIG, "is_authorized_configuration", {"method": "GET"}),
            (MenuItem.CONNECTIONS, "is_authorized_connection", {"method": "GET"}),
            (MenuItem.DAGS, "is_authorized_dag", {"method": "GET"}),
            (MenuItem.DOCS, "is_authorized_view", {"access_view": AccessView.DOCS}),
            (
                MenuItem.PLUGINS,
                "is_authorized_view",
                {"access_view": AccessView.PLUGINS},
            ),
            (MenuItem.POOLS, "is_authorized_pool", {"method": "GET"}),
            (
                MenuItem.PROVIDERS,
                "is_authorized_view",
                {"access_view": AccessView.PROVIDERS},
            ),
            (MenuItem.VARIABLES, "is_authorized_variable", {"method": "GET"}),
            (
                MenuItem.XCOMS,
                "is_authorized_dag",
                {"method": "GET", "access_entity": DagAccessEntity.XCOM},
            ),
        ],
    )
    def test_filter_authorized_menu_items_routes_through_opa(
        self,
        menu_item,
        expected_method,
        expected_kwargs,
        auth_manager_with_appbuilder,
    ):
        # Each MenuItem must trigger the matching OPA-backed is_authorized_*
        # call, so menu visibility is OPA-driven rather than FAB-DB-driven.
        user = Mock()
        user.perms = []

        with mock.patch.object(
            auth_manager_with_appbuilder, expected_method, return_value=True
        ) as mocked:
            allowed = auth_manager_with_appbuilder.filter_authorized_menu_items(
                [menu_item], user=user
            )
        assert allowed == [menu_item]
        mocked.assert_called_once_with(user=user, **expected_kwargs)

        with mock.patch.object(
            auth_manager_with_appbuilder, expected_method, return_value=False
        ):
            denied = auth_manager_with_appbuilder.filter_authorized_menu_items(
                [menu_item], user=user
            )
        assert denied == []

    def test_filter_authorized_menu_items_preserves_order_and_filters(
        self, auth_manager_with_appbuilder
    ):
        user = Mock()
        user.perms = []

        def fake_is_authorized_dag(*, method, details=None, access_entity=None, user):
            return access_entity is None

        with (
            mock.patch.object(
                auth_manager_with_appbuilder,
                "is_authorized_dag",
                side_effect=fake_is_authorized_dag,
            ),
            mock.patch.object(
                auth_manager_with_appbuilder,
                "is_authorized_connection",
                return_value=True,
            ),
            mock.patch.object(
                auth_manager_with_appbuilder,
                "is_authorized_view",
                return_value=False,
            ),
        ):
            result = auth_manager_with_appbuilder.filter_authorized_menu_items(
                [MenuItem.DAGS, MenuItem.DOCS, MenuItem.CONNECTIONS, MenuItem.XCOMS],
                user=user,
            )

        assert result == [MenuItem.DAGS, MenuItem.CONNECTIONS]
