from airflow.auth.managers.models.resource_details import AccessView
import pytest

from airflow.www.extensions.init_appbuilder import init_appbuilder
from flask import Flask
from opa_auth_manager.opa_fab_auth_manager import OpaFabAuthManager

@pytest.fixture
def flask_app():
    return Flask(__name__)

@pytest.fixture
def auth_manager(flask_app):
    appbuilder = init_appbuilder(flask_app)
    return OpaFabAuthManager(appbuilder)

@pytest.mark.db_test
class TestOpaFabAuthManager:

    def test_is_authorized_configuration(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_configuration(
            method="GET",
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_connection(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_connection(
            method="GET",
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_dag(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_dag(
            method="GET",
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_dataset(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_dataset(
            method="GET",
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_pool(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_pool(
            method="GET",
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_variable(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_variable(
            method="GET",
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_view(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_view(
            access_view=AccessView.JOBS,
        )
        expected_result = True
        assert result == expected_result

    def test_is_authorized_custom_view(self, auth_manager: OpaFabAuthManager):
        result = auth_manager.is_authorized_custom_view(
            method="GET",
            resource_name="test",
        )
        expected_result = True
        assert result == expected_result
