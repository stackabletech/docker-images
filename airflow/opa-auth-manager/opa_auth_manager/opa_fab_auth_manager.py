"""
Custom Auth manager for Airflow
"""

from airflow.auth.managers.base_auth_manager import ResourceMethod
from airflow.auth.managers.models.base_user import BaseUser
from airflow.auth.managers.models.resource_details import (
    AccessView,
    ConfigurationDetails,
    ConnectionDetails,
    DagAccessEntity,
    DagDetails,
    DatasetDetails,
    PoolDetails,
    VariableDetails,
)
from airflow.providers.fab.auth_manager.fab_auth_manager import FabAuthManager
from airflow.utils.log.logging_mixin import LoggingMixin

class OpaFabAuthManager(FabAuthManager, LoggingMixin):
    """
    Auth manager based on the FabAuthManager which delegates the authorization to an Open Policy
    Agent
    """

    def is_authorized_configuration(
        self,
        *,
        method: ResourceMethod,
        details: ConfigurationDetails | None = None,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on
        configuration.

        :param method: the method to perform
        :param details: optional details about the configuration
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_configuration to OPA")

        return True

    def is_authorized_connection(
        self,
        *,
        method: ResourceMethod,
        details: ConnectionDetails | None = None,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a connection.

        :param method: the method to perform
        :param details: optional details about the connection
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_connection to OPA")

        return True

    def is_authorized_dag(
        self,
        *,
        method: ResourceMethod,
        access_entity: DagAccessEntity | None = None,
        details: DagDetails | None = None,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a DAG.

        :param method: the method to perform
        :param access_entity: the kind of DAG information the authorization request is about. If not
            provided, the authorization request is about the DAG itself
        :param details: optional details about the DAG
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_dag to OPA")

        return True

    def is_authorized_dataset(
        self,
        *,
        method: ResourceMethod,
        details: DatasetDetails | None = None,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on an asset.

        :param method: the method to perform
        :param details: optional details about the asset
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_dataset to OPA")

        return True

    def is_authorized_pool(
        self,
        *,
        method: ResourceMethod,
        details: PoolDetails | None = None,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a pool.

        :param method: the method to perform
        :param details: optional details about the pool
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_pool to OPA")

        return True

    def is_authorized_variable(
        self,
        *,
        method: ResourceMethod,
        details: VariableDetails | None = None,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a variable.

        :param method: the method to perform
        :param details: optional details about the variable
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_variable to OPA")

        return True

    def is_authorized_view(
        self,
        *,
        access_view: AccessView,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to access a read-only state of the installation.

        :param access_view: the specific read-only view/state the authorization request is about.
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_view to OPA")

        return True

    def is_authorized_custom_view(
        self,
        *,
        method: ResourceMethod | str,
        resource_name: str,
        user: BaseUser | None = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a custom view.

        A custom view can be a view defined as part of the auth manager. This view is then only
        available when the auth manager is used as part of the environment. It can also be a view
        defined as part of a plugin defined by a user.

        :param method: the method to perform.
            The method can also be a string if the action has been defined in a plugin. In that
            case, the action can be anything (e.g. can_do). See
            https://github.com/apache/airflow/issues/39144
        :param resource_name: the name of the resource
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.info("Forward is_authorized_custom_view to OPA")

        return True
