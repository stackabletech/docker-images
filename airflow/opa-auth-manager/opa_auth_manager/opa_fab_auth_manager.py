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
from cachetools import TTLCache, cachedmethod
from typing import override
import json
import requests

class OpaInput:
    """
    Wrapper for the OPA input structure which is hashable so that it can be cached
    """

    def __init__(self, input: dict) -> None:
        self.input = input

    def __eq__(self, other: object) -> bool:
        return isinstance(other, OpaInput) \
            and self.input == other.input

    def __hash__(self) -> int:
        return hash(json.dumps(self.input, sort_keys=True))

    def to_dict(self) -> dict:
        return self.input

class OpaFabAuthManager(FabAuthManager, LoggingMixin):
    """
    Auth manager based on the FabAuthManager which delegates the authorization to an Open Policy
    Agent
    """

    AUTH_OPA_CACHE_MAXSIZE_DEFAULT=1000
    AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT=30
    AUTH_OPA_REQUEST_URL_DEFAULT='http://opa:8081/v1/data/airflow'
    AUTH_OPA_REQUEST_TIMEOUT_DEFAULT=10

    def init(self) -> None:
        """
        Run operations when Airflow is initializing.
        """

        super().init()

        config = self.appbuilder.get_app.config
        self.opa_cache = TTLCache(
            maxsize=config.get(
                'AUTH_OPA_CACHE_MAXSIZE',
                self.AUTH_OPA_CACHE_MAXSIZE_DEFAULT
            ),
            ttl=config.get(
                'AUTH_OPA_CACHE_TTL_IN_SEC',
                self.AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT
            ),
        )
        self.opa_session = requests.Session()

    def call_opa(self, url: str, json: dict, timeout: int) -> requests.Response:
        """
        Send a POST request to OPA.

        This function can be overriden in tests.

        :param url: URL for the OPA rule
        :param json: json to send in the body
        """

        return self.opa_session.post(url=url, json=json, timeout=timeout)

    @cachedmethod(lambda self: self.opa_cache)
    def _is_authorized_in_opa(self, endpoint: str, input: OpaInput) -> bool:
        """
        Forward an authorization request to OPA.

        :param endpoint: the OPA rule
        :param input: the input structure for OPA
        """

        self.log.debug("Forward authorization request to OPA")

        config = self.appbuilder.get_app.config
        opa_url = config.get(
            'AUTH_OPA_REQUEST_URL',
            self.AUTH_OPA_REQUEST_URL_DEFAULT
        )
        try:
            response = self.call_opa(
                f'{opa_url}/{endpoint}',
                json=input.to_dict(),
                timeout=config.get(
                    'AUTH_OPA_REQUEST_TIMEOUT',
                    self.AUTH_OPA_REQUEST_TIMEOUT_DEFAULT
                )
            )
            return response.json().get('result')
        except Exception as e:
            self.log.error("Request to OPA failed", exc_info=e)
            return False

    @override
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

        self.log.debug("Check is_authorized_configuration")

        if not user:
            user = self.get_user()

        if not details:
            section = None
        else:
            section = details.section

        return self._is_authorized_in_opa(
            'is_authorized_configuration',
            OpaInput({
                'input': {
                    'method': method,
                    'details': {
                        'section': section,
                    },
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_connection")

        if not user:
            user = self.get_user()

        if not details:
            conn_id = None
        else:
            conn_id = details.conn_id

        return self._is_authorized_in_opa(
            'is_authorized_connection',
            OpaInput({
                'input': {
                    'method': method,
                    'details': {
                        'conn_id': conn_id,
                    },
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_dag")

        if not user:
            user = self.get_user()

        if not access_entity:
            entity = None
        else:
            entity = access_entity.name

        if not details:
            dag_id = None
        else:
            dag_id = details.id

        return self._is_authorized_in_opa(
            'is_authorized_dag',
            OpaInput({
                'input': {
                    'method': method,
                    'access_entity': entity,
                    'details': {
                        'id': dag_id,
                    },
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_dataset")

        if not user:
            user = self.get_user()

        if not details:
            uri = None
        else:
            uri = details.uri

        return self._is_authorized_in_opa(
            'is_authorized_dataset',
            OpaInput({
                'input': {
                    'method': method,
                    'details': {
                        'uri': uri,
                    },
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_pool")

        if not user:
            user = self.get_user()

        if not details:
            name = None
        else:
            name = details.name

        return self._is_authorized_in_opa(
            'is_authorized_pool',
            OpaInput({
                'input': {
                    'method': method,
                    'details': {
                        'name': name,
                    },
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_variable")

        if not user:
            user = self.get_user()

        if not details:
            key = None
        else:
            key = details.key

        return self._is_authorized_in_opa(
            'is_authorized_variable',
            OpaInput({
                'input': {
                    'method': method,
                    'details': {
                        'key': key,
                    },
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_view")

        if not user:
            user = self.get_user()

        return self._is_authorized_in_opa(
            'is_authorized_view',
            OpaInput({
                'input': {
                    'access_view': access_view.name,
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )

    @override
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

        self.log.debug("Check is_authorized_custom_view")

        if not user:
            user = self.get_user()

        return self._is_authorized_in_opa(
            'is_authorized_custom_view',
            OpaInput({
                'input': {
                    'method': method,
                    'resource_name': resource_name,
                    'user': {
                        'id': user.get_id(),
                        'name': user.get_name(),
                    },
                }
            })
        )
