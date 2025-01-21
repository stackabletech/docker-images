# pylint: disable=invalid-name, unused-argument, redefined-outer-name, missing-module-docstring
import pytest
from flask import Flask
from flask_appbuilder.security.sqla.models import Role, User
from pytest_mock import MockFixture

from superset.extensions import appbuilder
from opa_authorizer.opa_manager import OpaSupersetSecurityManager


@pytest.fixture
def app():
    """
    Returns an empty Flask app with no db connection.
    """
    return Flask(__name__)


@pytest.fixture
def opa_security_manager(
    mocker: MockFixture,
    app: Flask,
) -> OpaSupersetSecurityManager:
    """
    Returns an OpaSupersetSecurityManager for an empty Flask app with no db connection.
    """

    with app.app_context():
        mocker.patch(
            "flask_appbuilder.security.sqla.manager.SecurityManager.create_db",
            return_value=None,
        )
        return OpaSupersetSecurityManager(appbuilder)


@pytest.fixture
def user() -> User:
    """
    Return a user.
    """
    user = User()
    user.id = 1234
    user.first_name = "mock"
    user.last_name = "mock"
    user.username = "mock"
    user.email = "mock@mock.com"

    return user


def test_opa_security_manager(opa_security_manager: OpaSupersetSecurityManager) -> None:
    """
    Test that the OPA security manager can be built.
    """
    assert opa_security_manager


def test_add_roles(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
    user: User,
) -> None:
    """
    Test that roles are correctly added to a user.
    """
    opa_roles = ["Test1", "Test2", "Test3"]

    with app.app_context():
        mocker.patch(
            "flask_appbuilder.security.sqla.manager.SecurityManager.update_user",
            return_value=True,
        )
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.get_opa_user_roles",
            return_value=opa_roles,
        )
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.resolve_role",
            wraps=mock_resolve_role,
        )

        assert set(
            map(lambda r: r.name, opa_security_manager.get_user_roles(user))
        ) == set(opa_roles)


def test_change_roles(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
    user: User,
) -> None:
    """
    Test that roles are correcty changed on a user.
    """
    opa_roles = ["Test4"]

    with app.app_context():
        mocker.patch(
            "flask_appbuilder.security.sqla.manager.SecurityManager.update_user",
            return_value=True,
        )
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.get_opa_user_roles",
            return_value=opa_roles,
        )
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.resolve_role",
            wraps=mock_resolve_role,
        )
        user_roles = ["Test1", "Test2", "Test3"]
        user.roles = list(map(opa_security_manager.resolve_role, user_roles))

        assert set(
            map(lambda r: r.name, opa_security_manager.get_user_roles(user))
        ) == set(opa_roles)


def test_no_roles(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
    user: User,
) -> None:
    """
    Test that only the default role is assigned if OPA returns no roles.
    """
    opa_roles = []

    with app.app_context():
        mocker.patch(
            "flask_appbuilder.security.sqla.manager.SecurityManager.update_user",
            return_value=True,
        )
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.get_opa_user_roles",
            return_value=opa_roles,
        )
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.resolve_role",
            wraps=mock_resolve_role,
        )

        assert set(
            map(lambda r: r.name, opa_security_manager.get_user_roles(user))
        ) == {"Public"}


def test_get_opa_roles(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
) -> None:
    """
    Test that roles are correctly extracted from the OPA response.
    """
    roles = ["Test1", "Test2", "Test3"]
    response = {"result": roles}

    with app.app_context():
        mocker.patch("opa_client.opa.OpaClient.query_rule", return_value=response)
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.resolve_opa_base_url",
            return_value=("opa-instance", 8081, False),
        )

        assert opa_security_manager.get_opa_user_roles("User1") == roles


def test_get_opa_roles_no_result(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
) -> None:
    """
    Test that no roles are returned if the OPA response doesn't contain a query result.
    """
    response = {"error": "error occurred"}

    with app.app_context():
        mocker.patch("opa_client.opa.OpaClient.query_rule", return_value=response)
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.resolve_opa_base_url",
            return_value=("opa-instance", 8081, False),
        )

        assert opa_security_manager.get_opa_user_roles("User1") == []


def test_get_opa_roles_not_a_list(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
) -> None:
    """
    Test that no roles are returned if the query result doesn't contain a list.
    """
    response = {"result": "['Test1', 'Test2', 'Test3']"}  # type string not list

    with app.app_context():
        mocker.patch("opa_client.opa.OpaClient.query_rule", return_value=response)
        mocker.patch(
            "opa_authorizer.opa_manager.OpaSupersetSecurityManager.resolve_opa_base_url",
            return_value=("opa-instance", 8081, False),
        )

        assert opa_security_manager.get_opa_user_roles("User1") == []


def test_resolve_opa_base_url(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
) -> None:
    """
    Test that only OPA base URL parts correctly resolved from the config.
    """
    with app.app_context():
        mocker.patch(
            "flask.current_app.config.get", return_value="http://opa-instance:8081"
        )
        assert opa_security_manager.resolve_opa_base_url() == (
            "opa-instance",
            8081,
            False,
        )


def test_resolve_opa_base_url_with_path(
    mocker: MockFixture,
    app: Flask,
    opa_security_manager: OpaSupersetSecurityManager,
) -> None:
    """
    Test that only OPA base URL parts correctly resolved from the config when the URL
    also contains a path.
    """
    with app.app_context():
        mocker.patch(
            "flask.current_app.config.get",
            return_value="http://opa-instance:8081/v1/data/superset",
        )
        assert opa_security_manager.resolve_opa_base_url() == (
            "opa-instance",
            8081,
            False,
        )


def mock_resolve_role(role_name: str) -> Role:
    """
    Returns a role without interacting with the db.
    """
    role = Role()
    role.id = 123
    role.name = role_name

    return role
