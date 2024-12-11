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
5. Come up with a patch process for such things ( @Lars )
"""
class OpaSupersetSecurityManager(SupersetSecurityManager):

    def get_user_roles(self, user: Optional[User] = None) -> List[Role]:
        if not user:
            user = g.user

        # TODO: Let the operator configure host and port
        client = OpaClient(host = 'simple-opa', port=8081)
        response = client.query_rule(
                input_data = {'username': user.username},
                package_path = 'superset',
                rule_name = 'user_roles')
        role_names = response['result']
        logging.info(f'found opa roles: {role_names}')
        roles = list(map(self.find_role, role_names)) 

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