from flask import g
from flask_appbuilder.security.sqla.models import (Role, User)
from opa_client.opa import OpaClient
from superset.security.manager import SupersetSecurityManager
from typing import (Optional, List)

import logging

# logger = logging.get_logger(__name__)

"""
We want OPA to sync roles.
1. Role Sync via OPA
2. Automated sync ( how and where to sync configurable [Decision Role sync policy] )
3. CRD option to turn sync on, off. [ ~Decision~ Standardized through op-rs ]
4. CRD option for auto delete roles from OPA and how ( Prefix maybe ) [ Decision ]
  --> Maybe we don't want that as we could reach permission states in dashboards, charts etc. which
  had been RBAC but now the role is gone. What now? Unsecure state.
5. Come up with a patch process for such things ( @Lars )
"""
class OpaSupersetSecurityManager(SupersetSecurityManager):

    """
    This is called:
    as get_user_permissions() in FlaskApplicationBuilder
    - bootstrap_user_data() in superset views (REST APIs)
    as get_user_roles
    - get_rls_filter() -> row-level filter on tables
    - dashboard rbac filter
    - is_admin() -> used in many places as admin role in special

    Important!
    user.roles can also be called directly, looks like you don't have to use the getter...

    Seems to not use user.roles:
    - resource ownership (looks at owner attribute, not roles)
    """
    def get_user_roles(self, user: Optional[User] = None) -> List[Role]:
        if not user:
            user = g.user

        # TODO: Let the operator configure host and port
        client = OpaClient(host = 'simple-opa', port=8081)
        response = client.query_rule(
                input_data = {'username': user.username},
                package_path = 'superset',
                rule_name = 'user_roles')
        logging.info(f'Query: {response}')
        role_names = response['result']
        logging.info(f'found opa roles: {role_names}')
        roles    = list(map(self.find_role, role_names))

        # fairly primitive check if roles are already in database
        # TODO: Sophisticate
        for i, role in enumerate(roles):
          if role == None:
            logging.info(f'Found None: {role}, adding role {role_names[i]}')
            self.add_role(role_names[i])

        roles = list(map(self.find_role, role_names))

        # TODO: See if you want to delete roles and how
        if set(user.roles) != set(roles):
          logging.info(f'found diff in {user.roles} vs. {roles}')
          user.roles = roles
          self.update_user(user)

        logging.info(f'found user roles: {user.roles}')

        return roles