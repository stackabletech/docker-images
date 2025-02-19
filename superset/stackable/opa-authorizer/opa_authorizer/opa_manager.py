"""
Custom security manager for Superset
"""

import logging
from typing import Optional

import requests
from cachetools import TTLCache, cachedmethod
from flask import current_app, g
from flask_appbuilder.security.sqla.models import Role, User
from overrides import override
from superset.security import SupersetSecurityManager

log = logging.getLogger(__name__)


class OpaSupersetSecurityManager(SupersetSecurityManager):
    """
    Custom security manager that syncs role mappings from Open Policy Agent to Superset.
    """

    AUTH_OPA_CACHE_MAXSIZE_DEFAULT = 1000
    AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT = 30
    AUTH_OPA_REQUEST_URL_DEFAULT = "http://opa:8081/"
    AUTH_OPA_REQUEST_TIMEOUT_DEFAULT = 10
    AUTH_OPA_PACKAGE_DEFAULT = "superset"
    AUTH_OPA_RULE_DEFAULT = "user_roles"

    def __init__(self, appbuilder):
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
        self.auth_opa_package: str = config.get(
            "AUTH_OPA_PACKAGE", self.AUTH_OPA_PACKAGE_DEFAULT
        )
        self.auth_opa_rule: str = config.get(
            "AUTH_OPA_RULE", self.AUTH_OPA_RULE_DEFAULT
        )
        self.auth_opa_request_timeout: int = current_app.config.get(
            "AUTH_OPA_REQUEST_TIMEOUT", self.AUTH_OPA_REQUEST_TIMEOUT_DEFAULT
        )
        self.opa_session = requests.Session()

    @override
    def get_user_roles(self, user: Optional[User] = None) -> list[Role]:
        """
        Retrieves a user's roles from an Open Policy Agent instance updating the
        user-role mapping in Superset's database in the process.

        :returns: A list of roles.
        """
        if not user:
            user = g.user

        resolved_opa_roles = self.resolve_opa_roles(user.username)

        if not resolved_opa_roles:
            log.error(
                f"No OPA roles for user [{user.username}], defaulting to role AUTH_USER_REGISTRATION_ROLE"
            )
            return []

        user_role_set = set(user.roles)
        log.debug(f"Superset roles for user [{user.username}]: {user_role_set}")

        if user_role_set != resolved_opa_roles:
            log.info(
                f"Superset and OPA roles diverge. Updating OPA roles for user [{user.username}]"
            )
            user.roles = list(resolved_opa_roles)
            if not self.update_user(user):
                log.error(f"Failed to update user roles for user {user.username}")

        return user.roles

    def get_opa_user_roles(self, username: str) -> list[str]:
        """
        Queries an Open Policy Agent instance for the roles of a given user.

        :returns: A list of Role objects assigned to the user or an empty list.
        """
        input = {"input": {"username": username}}
        try:
            req_url = f"{self.auth_opa_url}/v1/data/{self.auth_opa_package}/{self.auth_opa_rule}"
            response = self.call_opa(
                url=req_url,
                json=input,
                timeout=self.auth_opa_request_timeout,
            )

            if response.status_code != 200:
                log.error(
                    f"OPA request [{req_url}] for user [{username}] failed with response code [{response.status_code}]"
                )
                return []

            roles: list[str] = response.json().get("result")
            if type(roles) is not list:
                log.error(f"Expected a list or role names from OPA but got [{roles}]")
                return []

            if not roles:
                log.error(
                    "Expected a list or role names from OPA but got an empty list"
                )
                return []

            log.debug(f"Retrieved OPA role names for user [{username}]: [{roles}]")
            return roles

        except Exception as e:
            log.error("Failed to get OPA role names", exc_info=e)
            return []

    @cachedmethod(lambda self: self.role_cache)
    def resolve_opa_roles(self, username: str) -> set[Role]:
        """
        Queries an Open Policy Agent instance for the roles of a given user.

        Then maps the OPA role names to Superset Role objects.

        The result is cached.

        :returns: A set of Role objects assigned to the user or an empty set.
        """
        try:
            roles = self.get_opa_user_roles(username)
            resolved_opa_roles = self.resolve_role_list(roles)

            log.debug(
                f"Resolved OPA Roles for user [{username}]: [{resolved_opa_roles}]"
            )

            return resolved_opa_roles

        except Exception as e:
            log.error("Failed to resolve OPA roles", exc_info=e)
            return set()

    def call_opa(self, url: str, json: dict, timeout: int) -> requests.Response:
        return self.opa_session.post(
            url=url,
            json=json,
            timeout=timeout,
        )

    def resolve_role(self, role_name: str) -> Optional[Role]:
        """
        Finds a role by name creating it if it doesn't exist in Superset yet.

        :returns: A role or None.
        """
        role: Optional[Role] = self.find_role(role_name)
        if not role:
            log.info(
                f"Failed to resolve role {role_name}, attempting to create it with no permissions"
            )
            role = self.add_role(role_name)
            if not role:
                log.error(f"Failed to create new role {role_name}")
            else:
                log.info(f"Resolved role name [{role_name}] to new role.")

        log.debug(f"Resolved role name [{role_name}] to existing role.")

        return role

    def resolve_role_list(self, roles: list[str]) -> set[Role]:
        result: set[Role] = set()
        for role_name in roles:
            if resolved_role := self.resolve_role(role_name):
                result.add(resolved_role)
        return result
