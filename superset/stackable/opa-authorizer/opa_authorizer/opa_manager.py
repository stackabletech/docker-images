"""
Custom security manager for Superset.

Assigns OPA roles to a user. The roles and their permissions must exist in the
Superset database.
"""

import logging
from dataclasses import dataclass

import requests
from cachetools import TTLCache, cachedmethod
from flask import current_app
from flask_appbuilder import AppBuilder
from flask_appbuilder.security.sqla.models import Role, User
from overrides import override
from sqlalchemy.orm.session import Session
from superset.security import SupersetSecurityManager

log = logging.getLogger(__name__)


class OpaError(Exception):
    pass


class SupersetError(Exception):
    pass


@dataclass
class OpaResponse:
    roles: list[str]


def opa_response_from_json(json: dict[str, object]) -> OpaResponse:
    """Converts a JSON object to an OpaResponse object."""
    if "result" in json:
        if type(json["result"]) is list:
            return OpaResponse(roles=json["result"])

    raise OpaError(f"Invalid OPA response: [{json}]")


class OpaSupersetSecurityManager(SupersetSecurityManager):
    """
    Custom security manager that syncs role mappings from Open Policy Agent to Superset.
    """

    AUTH_OPA_CACHE_MAXSIZE_DEFAULT: int = 1000
    AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT: int = 30
    AUTH_OPA_REQUEST_URL_DEFAULT: str = "http://opa:8081/"
    AUTH_OPA_REQUEST_TIMEOUT_DEFAULT: int = 10
    AUTH_OPA_PACKAGE_DEFAULT: str = "superset"
    AUTH_OPA_RULE_DEFAULT: str = "user_roles"

    def __init__(self, appbuilder: AppBuilder):
        super().__init__(appbuilder)

        config = appbuilder.get_app.config

        self.role_cache: TTLCache[str, set[Role]] = TTLCache(
            maxsize=config.get(
                "AUTH_OPA_CACHE_MAXSIZE", self.AUTH_OPA_CACHE_MAXSIZE_DEFAULT
            ),
            ttl=config.get(
                "AUTH_OPA_CACHE_TTL_IN_SEC", self.AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT
            ),
        )

        self.auth_opa_url: str = config.get(
            "AUTH_OPA_REQUEST_URL", self.AUTH_OPA_REQUEST_URL_DEFAULT
        )
        self.auth_opa_rule: str = config.get(
            "AUTH_OPA_RULE", self.AUTH_OPA_RULE_DEFAULT
        )
        self.auth_opa_request_timeout: int = current_app.config.get(
            "AUTH_OPA_REQUEST_TIMEOUT", self.AUTH_OPA_REQUEST_TIMEOUT_DEFAULT
        )

        self.opa_session: requests.Session = requests.Session()

    @override
    def update_user_auth_stat(self, user, success=True):
        """
        Update user authentication stats upon successful/unsuccessful
        authentication attempts.
        Additionally, retrieve the roles of a successfully authenticated
        user from an Open Policy Agent instance and update the user-role
        mapping in the database.
        """
        if success:
            resolved_opa_roles = self.roles(user)
            user.roles = resolved_opa_roles

        super().update_user_auth_stat(user, success)

    @cachedmethod(lambda self: self.role_cache)
    def roles(self, user: User) -> list[Role]:
        """
        Retrieves a user's role names from an Open Policy Agent instance and
        maps them to existing Role objects in the Superset database.
        The result is cached.
        """
        opa_role_names = self.opa_get_user_roles(user.username)
        result: list[Role] = self.resolve_user_roles(user, opa_role_names)
        return result

    def opa_get_user_roles(self, username: str) -> list[str]:
        """
        Queries an Open Policy Agent instance for the roles of a given user.

        :returns: A list of Role objects assigned to the user or an empty list.
        """
        input = {"input": {"username": username}}
        try:
            req_url = f"{self.auth_opa_url}/{self.auth_opa_rule}"
            response = self.call_opa(
                url=req_url,
                json=input,
                timeout=self.auth_opa_request_timeout,
            )

            opa_response: OpaResponse = response.json(
                object_hook=opa_response_from_json
            )

            log.info(f"OPA role names for user [{username}]: [{opa_response.roles}]")

            return opa_response.roles

        except Exception as e:
            log.error("Failed to get OPA role names", exc_info=e)
            return []

    def call_opa(self, url: str, json: dict, timeout: int) -> requests.Response:
        return self.opa_session.post(
            url=url,
            json=json,
            timeout=timeout,
        )

    def resolve_user_roles(self, user: User, roles: list[str]) -> list[Role]:
        """
        Given a user object and a list of OPA role names, return the Role objects
        that must be assigned to this user.

        The user object is only needed to ensure that the Role objects are resolved
        using the same SQLAlchemy session as the user object.

        The Session object assigned to the SecurityManager is apparently not the same
        Session as the one used by the FAB login.
        """
        result: list[Role] = list()
        sqla_session = Session.object_session(user)
        superset_roles = sqla_session.query(Role).all()
        for role_name in roles:
            found = False

            for role in superset_roles:
                if role.name == role_name:
                    result.append(role)
                    log.debug(f"Resolved Superset role [{role_name}].")
                    found = True

            if not found:
                log.error(f"Superset role [{role_name}] does not exist.")
        return result
