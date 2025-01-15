# pylint: disable=missing-module-docstring
import logging
import os

from http.client import HTTPException
from typing import List, Optional, Tuple
from cachetools import cached, TTLCache
from flask import current_app, g
from flask_appbuilder.security.sqla.models import (
    Role,
    User,
)
from opa_client.opa import OpaClient
from superset.security import SupersetSecurityManager


class OpaSupersetSecurityManager(SupersetSecurityManager):
    """
    Custom security manager that syncs user-role mappings from Open Policy Agent to Superset.
    """
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
        logging.info('OPA returned roles: %s', opa_role_names)

        opa_roles = set(map(self.resolve_role, opa_role_names))
        logging.info('Resolved OPA Roles in Database: %s', opa_roles)
        # Ensure that in case of a bad or no response from OPA each user will have
        # at least one role.
        if opa_roles == {None} or opa_roles == set():
            opa_roles = {default_role}

        if set(user.roles) != opa_roles:
            logging.info('Found a diff between %s (Superset) and %s (OPA).',
                    user.roles, opa_roles)
            user.roles = list(opa_roles)
            self.update_user(user)

        return user.roles


    @cached(cache=TTLCache(
        maxsize = 1024,
        ttl = int(os.getenv('STACKABLE_OPA_CACHE_TTL', '10'))
        ))
    def get_opa_user_roles(self, username: str) -> set[str]:
        """
        Queries an Open Policy Agent instance for the roles of a given user.

        :returns: A list of role names or an empty list if an exception during
        the connection to OPA is encountered or if OPA didn't return a list.
        """
        host, port, tls = self.resolve_opa_base_url()
        client = OpaClient(host = host, port = port, ssl = tls)
        try:
            response = client.query_rule(
                  input_data = {'username': username},
                  package_path = current_app.config.get('STACKABLE_OPA_PACKAGE'),
                  rule_name = current_app.config.get('STACKABLE_OPA_RULE'))
        except HTTPException as exception:
            logging.error('Encountered an exception while querying OPA:%s', exception)
            return []
        roles = response.get('result')
        # If OPA didn't return a result or if the result is not a list, return no roles.
        if roles is None or type(roles).__name__ != "list":
            logging.error('The OPA query didn\'t return a list: %s', response)
            return []
        return roles


    def resolve_opa_base_url(self) -> Tuple[str, int, bool]:
        """
        Extracts connection parameters of an Open Policy Agent instance from config.

        :returns: Hostname, port and protocol (http/https).
        """
        opa_base_path = current_app.config.get('STACKABLE_OPA_BASE_URL')
        [protocol, host, port] = opa_base_path.split(":")
        # remove any path be appended to the base url
        port = int(port.split('/')[0])
        return host.lstrip('/'), port, protocol == 'https'


    def resolve_role(self, role_name: str) -> Role:
        """
        Finds a role by name creating it if it doesn't exist in Superset yet.

        :returns: A role.
        """
        role = self.find_role(role_name)
        if role is None:
            logging.info('Creating role %s as it doesn\'t already exist.', role_name)
            self.add_role(role_name)
        return self.find_role(role_name)
