# pylint: disable=missing-module-docstring
import logging

from typing import List, Optional
from cachetools import cachedmethod, TTLCache
from flask import current_app, g
from flask_appbuilder.security.sqla.models import (
    Role,
    User,
)
import requests
from superset.security import SupersetSecurityManager


class OpaSupersetSecurityManager(SupersetSecurityManager):
    """
    Custom security manager that syncs user-role mappings from Open Policy Agent to Superset.
    """

    AUTH_OPA_CACHE_MAXSIZE_DEFAULT = 1000
    AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT = 30
    AUTH_OPA_REQUEST_URL_DEFAULT = "http://opa:8081/"
    AUTH_OPA_REQUEST_TIMEOUT_DEFAULT = 10
    AUTH_OPA_PACKAGE_DEFAULT = "superset"
    AUTH_OPA_RULE_DEFAULT = "user_roles"

    def __init__(self, appbuilder):
        self.appbuilder = appbuilder
        super().__init__(self.appbuilder)
        config = self.appbuilder.get_app.config
        self.opa_cache = self.opa_cache = TTLCache(
            maxsize=config.get(
                "AUTH_OPA_CACHE_MAXSIZE", self.AUTH_OPA_CACHE_MAXSIZE_DEFAULT
            ),
            ttl=config.get(
                "AUTH_OPA_CACHE_TTL_IN_SEC", self.AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT
            ),
        )
        self.opa_session = requests.Session()

    def get_user_roles(self, user: Optional[User] = None) -> List[Role]:
        """
        Retrieves a user's roles from an Open Policy Agent instance updating the
        user-role mapping in Superset's database in the process.

        :returns: A list of roles.
        """
        if not user:
            user = g.user

        default_role = self.resolve_role(
            current_app.config.get("AUTH_USER_REGISTRATION_ROLE")
        )

        opa_role_names = self.get_opa_user_roles(user.username)
        logging.info("OPA returned roles: %s", opa_role_names)

        opa_roles = set(map(self.resolve_role, opa_role_names))
        logging.info("Resolved OPA Roles in Database: %s", opa_roles)
        # Ensure that in case of a bad or no response from OPA each user will have
        # at least one role.
        if opa_roles == {None} or opa_roles == set():
            opa_roles = {default_role}

        if set(user.roles) != opa_roles:
            logging.info(
                "Found a diff between %s (Superset) and %s (OPA).",
                user.roles,
                opa_roles,
            )
            user.roles = list(opa_roles)
            self.update_user(user)

        return user.roles

    @cachedmethod(lambda self: self.opa_cache)
    def get_opa_user_roles(self, username: str) -> set[str]:
        """
        Queries an Open Policy Agent instance for the roles of a given user.

        :returns: A list of role names or an empty list if an exception during
        the connection to OPA is encountered or if OPA didn't return a list.
        """

        opa_url = current_app.config.get(
            "AUTH_OPA_REQUEST_URL", self.AUTH_OPA_REQUEST_URL_DEFAULT
        )
        package = current_app.config.get(
            "AUTH_OPA_PACKAGE", self.AUTH_OPA_PACKAGE_DEFAULT
        )
        rule = current_app.config.get("AUTH_OPA_RULE", self.AUTH_OPA_RULE_DEFAULT)
        timeout = current_app.config.get(
            "AUTH_OPA_REQUEST_TIMEOUT", self.AUTH_OPA_REQUEST_TIMEOUT_DEFAULT
        )
        input = {"input": {"username": username}}
        response = self.call_opa(
            url=f"{opa_url}/v1/data/{package}/{rule}",
            json=input,
            timeout=timeout,
        )

        if response.status_code is None or response.status_code != 200:
            logging.error("Error while querying OPA.")
            return []

        roles = response.json().get("result")
        # If OPA didn't return a result or if the result is not a list, return no roles.
        if roles is None or type(roles).__name__ != "list":
            logging.error("The OPA query didn't return a list: %s", response.json())
            return []
        return roles

    def call_opa(self, url: str, json: dict, timeout: int) -> requests.Response:
        return self.opa_session.post(
            url=url,
            json=json,
            timeout=timeout,
        )

    def resolve_role(self, role_name: str) -> Role:
        """
        Finds a role by name creating it if it doesn't exist in Superset yet.

        :returns: A role.
        """
        role = self.find_role(role_name)
        if role is None:
            logging.info("Creating role %s as it doesn't already exist.", role_name)
            self.add_role(role_name)
        return self.find_role(role_name)
