"""
Application images will be created for products and associated versions configured here.
"""
products = [
    {
        'name': 'airflow',
        'versions': [
            {
                'product': '2.2.3',
                'python': '38',
            },
            {
                'product': '2.2.4',
                'python': '39',
            },
            {
                'product': '2.2.5',
                'python': '39',
            },
            {
                'product': '2.4.1',
                'python': '39',
            },
        ]
    },
    {
        'name': 'druid',
        'versions': [
            {
                'product': '0.23.0',
                'authorizer': '0.2.0',
            },
            {
                'product': '24.0.0',
                'authorizer': '0.3.0',
            }
        ]
    },
    {
        'name': 'hadoop',
        'versions': ['3.2.2', '3.3.1', '3.3.3', '3.3.4'],
    },
    {
        'name': 'hbase',
        'versions': [
            {
                'product': '2.4.6',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.8',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.9',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.11',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.12',
                'phoenix': '2.4-5.1.2',
            },
        ]
    },
    {
        'name': 'hive',
        'versions': [
            {
                'product': '2.3.9',
                'hadoop': '2.10.1',
                'aws_java_sdk_bundle': '1.11.271',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
            {
                'product': '3.1.3',
                'hadoop': '3.3.3',
                'aws_java_sdk_bundle': '1.11.1026',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
        ],
    },
    {
        'name': 'java-base',
        'versions': [
            {
                'product': '1.8.0',
                '_security_path': '/usr/lib/jvm/jre-1.8.0/lib/security/java.security',
            },
            {
                'product': '11',
                '_security_path': '/usr/lib/jvm/jre-11/conf/security/java.security',
            },
            {
                'product': '17',
                '_security_path': '/usr/lib/jvm/jre-17/conf/security/java.security',
            },
        ],
    },
    {
        'name': 'kafka',
        'versions': [
            {
                'product': '2.7.1',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '2.8.1',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '3.1.0',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '3.2.0',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '3.3.1',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
        ]
    },
    {
        'name': 'nifi',
        'versions': ['1.13.2', '1.15.0', '1.15.1', '1.15.2', '1.15.3', '1.16.0', '1.16.1', '1.16.2', '1.16.3', '1.18.0'],
    },
    {
        'name': 'opa',
        'versions': ['0.27.1', '0.28.0', '0.37.2', '0.41.0'],
    },
    {
        'name': 'pyspark-k8s',
        'versions': [
            {
                'product': '3.2.1',
                'python': '39',
                'hadoop_short_version': '3.2',
            },
            {
                'product': '3.3.0',
                'python': '39',
                'hadoop_short_version': '3',
                'hadoop_long_version': '3.3.3',
                'aws_java_sdk_bundle': '1.11.1026',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
        ]
    },
    {
        'name': 'spark',
        'versions': [
            {
                'product': '3.0.1',
                'hadoop': '2.7',
            },
            {
                'product': '3.1.1',
                'hadoop': '2.7',
            },
        ]
    },
    {
        'name': 'spark-k8s',
        'versions': [
            {
                'product': '3.2.1',
                'hadoop_short_version': '3.2',
            },
            {
                'product': '3.3.0',
                'hadoop_short_version': '3',
                'hadoop_long_version': '3.3.3',
                'aws_java_sdk_bundle': '1.11.1026',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
        ]
    },
    {
        'name': 'superset',
        'versions': [
            {
                'product': '1.3.2',
                'python': '3.8',
            },
            {
                'product': '1.4.1',
                'python': '3.9',
            },
            {
                'product': '1.5.1',
                'python': '3.8',
            },
        ],
    },
    {
        'name': 'trino',
        'versions': [
            {
                'product': '377',
                'java': '11',
                'opa_authorizer': '0.1.0',
            },
            {
                'product': '387',
                'java': '11',
                'opa_authorizer': '0.1.0'
            },
            {
                'product': '395',
                'java': '17',
                'opa_authorizer': 'stackable0.1.0'
            },
            {
                'product': '396',
                'java': '17',
                'opa_authorizer': 'stackable0.1.0'
            },
        ],
    },
    {
        'name': 'tools',
        'versions': ['0.2.0'],
    },
    {
        'name': 'testing-tools',
        'versions': ['0.1.0'],
    },
    {
        # ZooKeeper must be at least 3.5.0
        'name': 'zookeeper',
        'versions': ['3.5.8', '3.6.3', '3.7.0', '3.8.0'],
    },
]
