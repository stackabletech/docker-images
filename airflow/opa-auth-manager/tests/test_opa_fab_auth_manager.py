from typing import Any, Generator, override
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
from airflow.www.extensions.init_appbuilder import init_appbuilder
from flask import Flask
from flask_login import login_user
from opa_auth_manager.opa_fab_auth_manager import OpaFabAuthManager
import pytest
import requests
import time

class MockedOpaFabAuthManager(OpaFabAuthManager):

    def mock_opa_call(self, url: str, json: dict, timeout: int):
        self.url = url
        self.json = json
        self.timeout = timeout

    @override
    def call_opa(self, url: str, json: dict, timeout: int) -> requests.Response:
        assert url == self.url
        assert json == self.json
        assert timeout == self.timeout

        response = requests.Response()
        response.status_code = 200
        response._content = '{"result": true}'.encode()
        return response

class User(BaseUser):

    def __init__(self, user_id: str, username: str) -> None:
        self.user_id = user_id
        self.username = username

    def get_id(self) -> str:
        return self.user_id

    def get_name(self) -> str:
        return self.username

    def is_anonymous(self) -> bool:
        return False

@pytest.fixture
def auth_manager(request) -> Generator[MockedOpaFabAuthManager, Any, None]:
    flask_app = Flask(__name__)
    flask_app.config["SECRET_KEY"] = "secret" # required for test_request_context below
    flask_app.config.from_mapping(request.param)

    appbuilder = init_appbuilder(flask_app)
    auth_manager = MockedOpaFabAuthManager(appbuilder)
    auth_manager.init()

    with flask_app.test_request_context():
        login_user(User(user_id='1', username='testuser'))
        yield auth_manager

class TestOpaFabAuthManager:

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_configuration_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager,
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_configuration',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'section': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_configuration(
            method='GET',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_configuration_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager,
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_configuration',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'section': 'core',
                    },
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_configuration(
            method='GET',
            details=ConfigurationDetails(section='core'),
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_connection_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_connection',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'conn_id': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_connection(
            method='GET',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_connection_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_connection',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'conn_id': 'postgres_default',
                    },
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_connection(
            method='GET',
            details=ConnectionDetails(conn_id="postgres_default"),
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_dag_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_dag',
            {
                'input': {
                    'method': 'GET',
                    'access_entity': None,
                    'details': {
                        'id': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_dag(
            method='GET',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_dag_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_dag',
            {
                'input': {
                    'method': 'GET',
                    'access_entity': 'RUN',
                    'details': {
                        'id': 'example_trigger_target_dag',
                    },
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_dag(
            method='GET',
            access_entity=DagAccessEntity.RUN,
            details=DagDetails(id="example_trigger_target_dag"),
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_dataset_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_dataset',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'uri': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_dataset(
            method='GET',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_dataset_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_dataset',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'uri': 's3://bucket/my-task',
                    },
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_dataset(
            method='GET',
            details=DatasetDetails(uri='s3://bucket/my-task'),
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_pool_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_pool',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'name': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_pool(
            method='GET',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_pool_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_pool',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'name': 'default_pool',
                    },
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_pool(
            method='GET',
            details=PoolDetails(name='default_pool'),
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_variable_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_variable',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'key': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_variable(
            method='GET',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_variable_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_variable',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'key': 'myVar',
                    },
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_variable(
            method='GET',
            details=VariableDetails(key='myVar'),
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_view_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_view',
            {
                'input': {
                    'access_view': 'WEBSITE',
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_view(
            access_view=AccessView.WEBSITE,
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_view_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_view',
            {
                'input': {
                    'access_view': 'WEBSITE',
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_view(
            access_view=AccessView.WEBSITE,
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_custom_view_with_defaults(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_custom_view',
            {
                'input': {
                    'method': 'GET',
                    'resource_name': 'Users',
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_custom_view(
            method='GET',
            resource_name='Users',
        )

    @pytest.mark.parametrize('auth_manager', [{}], indirect=True)
    def test_is_authorized_custom_view_with_all_settings(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'http://opa:8081/v1/data/airflow/is_authorized_custom_view',
            {
                'input': {
                    'method': 'TRACE',
                    'resource_name': 'Users',
                    'user': {
                        'id': '2',
                        'name': 'otheruser',
                    },
                }
            },
            10
        )

        assert auth_manager.is_authorized_custom_view(
            method='TRACE',
            resource_name='Users',
            user=User(user_id='2', username='otheruser'),
        )

    @pytest.mark.parametrize(
        'auth_manager',
        [{
            'AUTH_OPA_REQUEST_URL': 'https://other-endpoint',
            'AUTH_OPA_REQUEST_TIMEOUT': 60,
        }],
        indirect=True,
    )
    def test_opa_request_configurations(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        auth_manager.mock_opa_call(
            'https://other-endpoint/is_authorized_configuration',
            {
                'input': {
                    'method': 'GET',
                    'details': {
                        'section': None,
                    },
                    'user': {
                        'id': '1',
                        'name': 'testuser',
                    },
                }
            },
            60
        )

        assert auth_manager.is_authorized_configuration(
            method='GET',
        )

    @pytest.mark.parametrize(
        'auth_manager',
        [{
            'AUTH_OPA_CACHE_MAXSIZE': 3,
        }],
        indirect=True,
    )
    def test_opa_cache_maxsize_configuration(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        def add_to_cache(section: str) -> None:
            auth_manager.mock_opa_call(
                'http://opa:8081/v1/data/airflow/is_authorized_configuration',
                {
                    'input': {
                        'method': 'GET',
                        'details': {
                            'section': section,
                        },
                        'user': {
                            'id': '1',
                            'name': 'testuser',
                        },
                    }
                },
                10
            )

            assert auth_manager.is_authorized_configuration(
                method='GET',
                details=ConfigurationDetails(section=section),
            )

        def is_cached(section: str) -> bool:
            auth_manager.mock_opa_call('', {}, 0)
            return auth_manager.is_authorized_configuration(
                method='GET',
                details=ConfigurationDetails(section=section),
            )

        add_to_cache('core')
        add_to_cache('database')
        add_to_cache('logging')

        assert is_cached('core')
        assert is_cached('database')
        assert is_cached('logging')

        add_to_cache('metrics')

        assert is_cached('database')
        assert is_cached('logging')
        assert is_cached('metrics')
        # The log output "ERROR - Request to OPA failed" is expected.
        assert not is_cached('core')

    @pytest.mark.parametrize(
        'auth_manager',
        [{
            'AUTH_OPA_CACHE_TTL_IN_SEC': 3,
        }],
        indirect=True,
    )
    def test_opa_cache_ttl_in_sec_configuration(
        self,
        auth_manager: MockedOpaFabAuthManager
    ) -> None:
        def add_to_cache(section: str) -> None:
            auth_manager.mock_opa_call(
                'http://opa:8081/v1/data/airflow/is_authorized_configuration',
                {
                    'input': {
                        'method': 'GET',
                        'details': {
                            'section': section,
                        },
                        'user': {
                            'id': '1',
                            'name': 'testuser',
                        },
                    }
                },
                10
            )

            assert auth_manager.is_authorized_configuration(
                method='GET',
                details=ConfigurationDetails(section=section),
            )

        def is_cached(section: str) -> bool:
            auth_manager.mock_opa_call('', {}, 0)
            return auth_manager.is_authorized_configuration(
                method='GET',
                details=ConfigurationDetails(section=section),
            )

        add_to_cache('core')

        assert is_cached('core')
        time.sleep(1)
        assert is_cached('core')
        time.sleep(1)
        assert is_cached('core')
        time.sleep(2)
        # The log output "ERROR - Request to OPA failed" is expected.
        assert not is_cached('core')
