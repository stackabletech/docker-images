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
                'vector': '0.26.0',
            },
            {
                'product': '2.2.4',
                'python': '39',
                'vector': '0.26.0',
            },
            {
                'product': '2.2.5',
                'python': '39',
                'vector': '0.26.0',
            },
            {
                'product': '2.4.1',
                'python': '39',
                'vector': '0.26.0',
            },
        ]
    },
    {
        'name': 'druid',
        'versions': [
            {
                'product': '0.23.0',
                'java-base': '11',
                'authorizer': '0.2.0',
            },
            {
                'product': '24.0.0',
                'java-base': '11',
                'authorizer': '0.4.0',
            }
        ]
    },
    {
        'name': 'hadoop',
        'versions': [
            {'product': '3.2.2', 'java-base': '11'},
            {'product': '3.3.1', 'java-base': '11'},
            {'product': '3.3.3', 'java-base': '11'},
            {'product': '3.3.4', 'java-base': '11'},
        ],
    },
    {
        'name': 'hbase',
        'versions': [
            {
                'product': '2.4.6',
                'java-base': '11',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.8',
                'java-base': '11',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.9',
                'java-base': '11',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.11',
                'java-base': '11',
                'phoenix': '2.4-5.1.2',
            },
            {
                'product': '2.4.12',
                'java-base': '11',
                'phoenix': '2.4-5.1.2',
            },
        ]
    },
    {
        'name': 'hive',
        'versions': [
            {
                'product': '2.3.9',
                'java-base': '11',
                'hadoop_libs': '2.10.1',
                'jackson_dataformat_xml': '2.7.9',
                'aws_java_sdk_bundle': '1.11.271',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
            {
                'product': '3.1.3',
                'java-base': '11',
                'hadoop_libs': '3.3.3',
                'jackson_dataformat_xml': '2.12.3',
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
                'product': '11',
                'vector': '0.26.0',
                '_security_path': '/usr/lib/jvm/jre-11/conf/security/java.security',
            },
            {
                'product': '17',
                'vector': '0.26.0',
                '_security_path': '/usr/lib/jvm/jre-17/conf/security/java.security',
            },
        ],
    },
    {
        'name': 'kafka',
        'versions': [
            {
                'product': '2.7.1',
                'java-base': '11',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '2.8.1',
                'java-base': '11',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '3.1.0',
                'java-base': '11',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '3.2.0',
                'java-base': '11',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
            {
                'product': '3.3.1',
                'java-base': '11',
                'scala': '2.13',
                'opa_authorizer': '1.4.0',
            },
        ]
    },
    {
        'name': 'krb5',
        'versions': [{'product': '1.18.2'}],
    },
    {
        'name': 'vector',
        'versions': [{'product': '0.26.0', 'stackable-base': '1.0.0'}],
    },
    {
        'name': 'nifi',
        'versions': [
            {'product': '1.15.3', 'java-base': '11'},
            {'product': '1.16.3', 'java-base': '11'},
            {'product': '1.18.0', 'java-base': '11'},
        ],
    },
    {
        'name': 'opa',
        'versions': [
            {
                'product': '0.27.1',
                'vector': '0.26.0',
            },
            {
                'product': '0.28.0',
                'vector': '0.26.0',
            },
            {
                'product': '0.37.2',
                'vector': '0.26.0',
            },
            {
                'product': '0.41.0',
                'vector': '0.26.0',
            },
            {
                'product': '0.45.0',
                'vector': '0.26.0',
            },
        ]
    },
    {
        'name': 'pyspark-k8s',
        'versions': [
            {
                'product': '3.2.1',
                'spark': '3.2.1',
                'stackable-base': '1.0.0',
                'python': '39',
                'java': '11',
                'hadoop_short_version': '3.2',
            },
            {
                'product': '3.3.0',
                'spark': '3.3.0',
                'stackable-base': '1.0.0',
                'python': '39',
                'java': '11',
                'hadoop_short_version': '3',
                'hadoop_long_version': '3.3.3',
                'aws_java_sdk_bundle': '1.11.1026',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
            {
                'product': '3.3.0-java17',
                'spark': '3.3.0',
                'stackable-base': '1.0.0',
                'python': '39',
                'java': '17',
                'hadoop_short_version': '3',
                'hadoop_long_version': '3.3.3',
                'aws_java_sdk_bundle': '1.11.1026',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
        ]
    },
    {
        'name': 'spark-k8s',
        'versions': [
            {
                'product': '3.2.1',
                'stackable-base': '1.0.0',
                'hadoop_short_version': '3.2',
            },
            {
                'product': '3.3.0',
                'stackable-base': '1.0.0',
                'hadoop_short_version': '3',
                'hadoop_long_version': '3.3.3',
                'aws_java_sdk_bundle': '1.11.1026',
                'azure_storage': '7.0.1',
                'azure_keyvault_core': '1.0.0',
            },
        ]
    },
    {
        'name': 'stackable-base',
        'versions': [{'product': '1.0.0'}],
    },
    {
        'name': 'superset',
        'versions': [
            {
                'product': '1.3.2',
                'python': '3.8',
                'vector': '0.26.0',
            },
            {
                'product': '1.4.1',
                'python': '3.9',
                'vector': '0.26.0',
            },
            {
                'product': '1.5.1',
                'python': '3.8',
                'vector': '0.26.0',
            },
        ],
    },
    {
        'name': 'trino',
        'versions': [
            {
                'product': '377',
                'java-base': '11',
                'opa_authorizer': '0.1.0',
            },
            {
                'product': '387',
                'java-base': '11',
                'opa_authorizer': '0.1.0'
            },
            {
                'product': '395',
                'java-base': '17',
                'opa_authorizer': 'stackable0.1.0'
            },
            {
                'product': '396',
                'java-base': '17',
                'opa_authorizer': 'stackable0.1.0'
            },
            {
                'product': '403',
                'java-base': '17',
                'opa_authorizer': 'stackable0.1.0'
            },
        ],
    },
    {
        'name': 'testing-tools',
        'versions': [{'product': '0.1.0'}],
    },
    {
        # ZooKeeper must be at least 3.5.0
        'name': 'zookeeper',
        'versions': [
            {'product': '3.5.8', 'java-base': '11'},
            {'product': '3.6.3', 'java-base': '11'},
            {'product': '3.7.0', 'java-base': '11'},
            {'product': '3.8.0', 'java-base': '11'},
        ],
    },
]
