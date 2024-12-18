from http.client import HTTPException
import os
from flask import g
from flask_appbuilder.security.sqla.models import (Role, User)
from opa_client.opa import OpaClient
from superset.security.manager import SupersetSecurityManager
from superset import conf
from typing import (Optional, List, Tuple)

import logging

# logger = logging.get_logger(__name__)
class OpaSupersetSecurityManager(SupersetSecurityManager):

    def get_user_roles(self, user: Optional[User] = None) -> List[Role]:
        if not user:
            user = g.user
        
        default_role = self.resolve_role(self.get_default_role())

        opa_role_names = self.get_opa_user_roles(user.username)
        logging.info(f'OPA returned roles: {opa_role_names}')

        opa_roles = set(map(self.resolve_role, opa_role_names))
        # Ensure that in case of a bad or no reponse from OPA each user will have at least one role.
        opa_roles.add(default_role)
        
        if set(user.roles) != opa_roles:
          logging.info(f'Found diff in {user.roles} vs. {opa_roles}')
          user.roles = list(opa_roles)
          self.update_user(user)

        return user.roles
    

    def get_opa_user_roles(self, username: str) -> set[str]:
        """
        Queries an Open Policy Agent instance for the roles of a given user and returns a list of role names.
        Returns an empty list if an exception during the connection to Open Policy Agent is encountered or if the query result
        is not a list.
        """
        host, port, tls = self.resolve_opa_endpoint()
        print(host)
        print(port)
        print(tls)
        client = OpaClient(host = host, port = port, ssl = tls)
        try:
          response = client.query_rule(
                  input_data = {'username': username},
                  package_path = 'superset',
                  rule_name = 'user_roles')
        except HTTPException as e:
           logging.error(f'Encountered an exception while querying OPA:\n{e}')
           return []
        roles = response.get('result')
        # If OPA didn't return a result or if the result is not a list, return no roles.
        if roles is None or type(roles).__name__ != "list":
          logging.error(f'The OPA query didn\'t return a list: {response}')
          return []
        return roles
    

    def resolve_opa_endpoint(self) -> Tuple[str, int, bool]:
       opa_endpoint = os.getenv('STACKABLE_OPA_ENDPOINT')
       [protocol, host, port] = opa_endpoint.split(":")
       return host.lstrip('/'), int(port.rstrip('/')), protocol == 'https'
    

    def resolve_role(self, role_name: str) -> Role:
      role = self.find_role(role_name)
      if role is None:
        logging.info(f'Creating role {role_name} as it doesn\'t already exist.')
        self.add_role(role_name)
      return self.find_role(role_name)
         
  
    def get_default_role(self) -> str:
      return conf["AUTH_USER_REGISTRATION_ROLE"] if conf["AUTH_USER_REGISTRATION_ROLE"] else "Public"
