"""
Custom Auth manager for Airflow
"""

from airflow.providers.fab.auth_manager.models import User
from airflow.providers.fab.auth_manager.fab_auth_manager import FabAuthManager

from airflow.api_fastapi.auth.managers.base_auth_manager import ResourceMethod
from airflow.configuration import conf
from airflow.api_fastapi.auth.managers.models.resource_details import (
from airflow.providers.fab.auth_manager.models import User
from airflow.providers.fab.auth_manager.fab_auth_manager import FabAuthManager

from airflow.api_fastapi.auth.managers.base_auth_manager import ResourceMethod
from airflow.configuration import conf
from airflow.api_fastapi.auth.managers.models.resource_details import (
    AccessView,
    AssetDetails,
    AssetAliasDetails,
    BackfillDetails,
    AssetDetails,
    AssetAliasDetails,
    BackfillDetails,
    ConfigurationDetails,
    ConnectionDetails,
    DagAccessEntity,
    DagDetails,
    PoolDetails,
    VariableDetails,
)
from airflow.stats import Stats
from airflow.utils.log.logging_mixin import LoggingMixin
from cachetools import TTLCache, cachedmethod
from typing import Optional, Union
from overrides import override
import json
import requests

METRIC_NAME_OPA_CACHE_LIMIT_REACHED = "opa_cache_limit_reached"


class OpaInput:
    """
    Wrapper for the OPA input structure which is hashable so that it can be cached
    """

    def __init__(self, input: dict) -> None:
        self.input = input

    def __eq__(self, other: object) -> bool:
        return isinstance(other, OpaInput) and self.input == other.input

    def __hash__(self) -> int:
        return hash(json.dumps(self.input, sort_keys=True))

    def to_dict(self) -> dict:
        return self.input


class Cache(TTLCache):
    """
    LRU Cache implementation with per-item time-to-live (TTL) value.
    """

    @override
    def popitem(self):
        """
        Remove the least recently used item that has not already expired.
        """

        Stats.incr(METRIC_NAME_OPA_CACHE_LIMIT_REACHED)
        return super().popitem()


class OpaFabAuthManager(FabAuthManager, LoggingMixin):
    """
    Auth manager based on the FabAuthManager which delegates the authorization to an Open Policy
    Agent
    """

    AUTH_OPA_CACHE_MAXSIZE_DEFAULT = 1000
    AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT = 30
    AUTH_OPA_REQUEST_URL_DEFAULT = "http://opa:8081/v1/data/airflow"
    AUTH_OPA_REQUEST_TIMEOUT_DEFAULT = 10

    @override
    def init_flask_resources(self) -> None:
    @override
    def init_flask_resources(self) -> None:
        """
        Run operations when Airflow is initializing.
        """

        super().init_flask_resources()
        super().init_flask_resources()

        Stats.incr(METRIC_NAME_OPA_CACHE_LIMIT_REACHED, count=0)

        self.opa_cache = Cache(
            maxsize=conf.getint(
                "core",
                "AUTH_OPA_CACHE_MAXSIZE",
                fallback=self.AUTH_OPA_CACHE_MAXSIZE_DEFAULT,
            maxsize=conf.getint(
                "core",
                "AUTH_OPA_CACHE_MAXSIZE",
                fallback=self.AUTH_OPA_CACHE_MAXSIZE_DEFAULT,
            ),
            ttl=conf.getint(
                "core",
                "AUTH_OPA_CACHE_TTL_IN_SEC",
                fallback=self.AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT,
            ttl=conf.getint(
                "core",
                "AUTH_OPA_CACHE_TTL_IN_SEC",
                fallback=self.AUTH_OPA_CACHE_TTL_IN_SEC_DEFAULT,
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

        opa_url = conf.get(
            "core", "AUTH_OPA_REQUEST_URL", fallback=self.AUTH_OPA_REQUEST_URL_DEFAULT
        )
        opa_url = conf.get(
            "core", "AUTH_OPA_REQUEST_URL", fallback=self.AUTH_OPA_REQUEST_URL_DEFAULT
        )
        try:
            response = self.call_opa(
                f"{opa_url}/{endpoint}",
                json=input.to_dict(),
                timeout=conf.getint(
                    "core",
                    "AUTH_OPA_REQUEST_TIMEOUT",
                    fallback=self.AUTH_OPA_REQUEST_TIMEOUT_DEFAULT,
                timeout=conf.getint(
                    "core",
                    "AUTH_OPA_REQUEST_TIMEOUT",
                    fallback=self.AUTH_OPA_REQUEST_TIMEOUT_DEFAULT,
                ),
            )
            return response.json().get("result")
        except Exception as e:
            self.log.error("Request to OPA failed", exc_info=e)
            return False

    @override
    def is_authorized_configuration(
        self,
        *,
        method: ResourceMethod,
        details: Optional[ConfigurationDetails] = None,
        user: User,
        user: User,
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

        if not details:
            section = None
        else:
            section = details.section

        return self._is_authorized_in_opa(
            "is_authorized_configuration",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "section": section,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_connection(
        self,
        *,
        method: ResourceMethod,
        details: Optional[ConnectionDetails] = None,
        user: User,
        user: User,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a connection.

        :param method: the method to perform
        :param details: optional details about the connection
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.debug("Check is_authorized_connection")

        if not details:
            conn_id = None
        else:
            conn_id = details.conn_id

        return self._is_authorized_in_opa(
            "is_authorized_connection",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "conn_id": conn_id,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_dag(
        self,
        *,
        method: ResourceMethod,
        access_entity: Optional[DagAccessEntity] = None,
        details: Optional[DagDetails] = None,
        user: User,
        user: User,
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

        if not access_entity:
            entity = None
        else:
            entity = access_entity.name

        if not details:
            dag_id = None
        else:
            dag_id = details.id

        return self._is_authorized_in_opa(
            "is_authorized_dag",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "access_entity": entity,
                        "details": {
                            "id": dag_id,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_backfill(
        self,
        *,
        method: ResourceMethod,
        details: Optional[BackfillDetails] = None,
        user: User,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a backfill.

        :param method: the method to perform
        :param details: optional details about the backfill
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """
        self.log.debug("Check is_authorized_backfill")

        backfill_id = details.id if details else None

        return self._is_authorized_in_opa(
            "is_authorized_backfill",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "id": backfill_id,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_asset(
        self,
        *,
        method: ResourceMethod,
        user: User,
        details: Optional[AssetDetails] = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on an asset.

        :param method: the method to perform
        :param details: optional details about the asset
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """
        self.log.debug("Check is_authorized_asset")

        asset_id = details.id if details else None

        return self._is_authorized_in_opa(
            "is_authorized_asset",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "id": asset_id,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_asset_alias(
        self,
        *,
        method: ResourceMethod,
        user: User,
        details: Optional[AssetAliasDetails] = None,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on an asset alias.

        :param method: the method to perform
        :param details: optional details about the asset alias
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """
        self.log.debug("Check is_authorized_asset_alias")

        alias_id = details.id if details else None

        return self._is_authorized_in_opa(
            "is_authorized_asset_alias",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "id": alias_id,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_pool(
        self,
        *,
        method: ResourceMethod,
        details: Optional[PoolDetails] = None,
        user: User,
        user: User,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a pool.

        :param method: the method to perform
        :param details: optional details about the pool
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.debug("Check is_authorized_pool")

        if not details:
            name = None
        else:
            name = details.name

        return self._is_authorized_in_opa(
            "is_authorized_pool",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "name": name,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_variable(
        self,
        *,
        method: ResourceMethod,
        details: Optional[VariableDetails] = None,
        user: User,
        user: User,
    ) -> bool:
        """
        Return whether the user is authorized to perform a given action on a variable.

        :param method: the method to perform
        :param details: optional details about the variable
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.debug("Check is_authorized_variable")

        if not details:
            key = None
        else:
            key = details.key

        return self._is_authorized_in_opa(
            "is_authorized_variable",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "details": {
                            "key": key,
                        },
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_view(
        self,
        *,
        access_view: AccessView,
        user: User,
        user: User,
    ) -> bool:
        """
        Return whether the user is authorized to access a read-only state of the installation.

        :param access_view: the specific read-only view/state the authorization request is about.
        :param user: the user to perform the action on. If not provided (or None), it uses the
            current user
        """

        self.log.debug("Check is_authorized_view")

        return self._is_authorized_in_opa(
            "is_authorized_view",
            OpaInput(
                {
                    "input": {
                        "access_view": access_view.name,
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )

    @override
    def is_authorized_custom_view(
        self,
        *,
        method: Union[ResourceMethod, str],
        resource_name: str,
        user: User,
        user: User,
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
            "is_authorized_custom_view",
            OpaInput(
                {
                    "input": {
                        "method": method,
                        "resource_name": resource_name,
                        "user": {
                            "id": user.get_id(),
                            "name": user.get_name(),
                        },
                    }
                }
            ),
        )
