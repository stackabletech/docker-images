"""
Custom security manager for Superset
"""

import logging
from dataclasses import dataclass
from typing import Optional

import requests
from cachetools import TTLCache, cachedmethod
from flask import current_app, g
from flask_appbuilder.security.sqla.models import Role, User
from overrides import override
from superset.security import SupersetSecurityManager

log = logging.getLogger(__name__)


class OpaError(Exception):
    pass


class SupersetError(Exception):
    pass


@dataclass
class OpaResponse:
    roles: list[str]


def opa_response_from_json(json: dict) -> OpaResponse:
    if "result" in json:
        if type(json["result"]) is list:
            return OpaResponse(roles=json["result"])

    raise OpaError(f"Invalid OPA response: [{json}]")


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

        resolved_opa_roles = self.roles(user.username)

        # user.roles = resolved_opa_roles

        return resolved_opa_roles

    @cachedmethod(lambda self: self.role_cache)
    def roles(self, username: str) -> list[Role]:
        opa_role_names = self.opa_get_user_roles(username)
        result = self.resolve_user_roles(opa_role_names)
        return result

    def opa_get_user_roles(self, username: str) -> list[str]:
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

    def resolve_user_roles(self, roles: list[str]) -> list[Role]:
        result: list[Role] = list()
        for role_name in roles:
            if resolved_role := self.find_role(role_name):
                log.info(f"Resolved Superset role [{role_name}].")
                result.append(resolved_role)
            else:
                raise SupersetError(f"Superset role [{role_name}] does not exist.")
        return result
