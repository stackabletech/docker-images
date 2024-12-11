          from flask import g
          from flask_appbuilder.security.sqla.models import (Role, User)
          from opa_client.opa import OpaClient
          from superset.security.manager import SupersetSecurityManager
          from typing import (Optional, List)

          import logging

          logger = logging.get_logger(__name__)

          class OpaSupersetSecurityManager(SupersetSecurityManager):

              # Override to sync new opa roles in Superset... maybe... 
              def sync_role_definitions(self) -> None:
                """
                Syncing roles from opa with available roles in Superset. Override if diffs.
                Leaves initial roles from superset untouched.
                """

                def _get_opa_roles() -> list[str]:
                  logger.info('Retrieve Opa roles')
                  # Not great to invoke OpaClient twice. TODO: Better solution
                  client = OpaClient(host = 'simple-opa', port=8081)
                  policies = client.get_policies_list()
                  logger.info(f'retrieved opa informations: {policies}') 

                  
      
                logger.info("Syncing role definition")

                self.create_custom_permissions()

                pvms = self._get_all_pvms()

                # Creating default roles
                self.set_role("Admin", self._is_admin_pvm, pvms)
                self.set_role("Alpha", self._is_alpha_pvm, pvms)
                self.set_role("Gamma", self._is_gamma_pvm, pvms)
                self.set_role("sql_lab", self._is_sql_lab_pvm, pvms)

                # TODO: OPA part right here
                for role in _get_opa_roles():
                  self.add_role(role)

                # Configure public role
                if current_app.config["PUBLIC_ROLE_LIKE"]:
                    self.copy_role(
                        current_app.config["PUBLIC_ROLE_LIKE"],
                        self.auth_role_public,
                        merge=True,
                    )

                self.create_missing_perms()

                # commit role and view menu updates
                self.get_session.commit()
                self.clean_perms()

              def get_user_roles(self, user: Optional[User] = None) -> List[Role]:
                  if not user:
                      user = g.user

                  client = OpaClient(host = 'simple-opa', port=8081)
                  logging.info(f'client host: {client._host}, client root: {client._root_url}')
                  response = client.check_policy_rule(
                          input_data = {'username': user.username},
                          package_path = 'superset',
                          rule_name = 'user_roles')

                  role_names = response['result']
                  roles = list(map(self.find_role, role_names))

                  user.roles = roles

                  return roles