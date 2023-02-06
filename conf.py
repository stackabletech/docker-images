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
                'authorizer': '0.2.0',
            },
            {
                'product': '24.0.0',
                'authorizer': '0.4.0',
            }
        ]
    },
    {
        'name': 'hadoop',
        'versions': [{'product': '3.2.2'}, {'product': '3.3.1'}, {'product': '3.3.3'}, {'product': '3.3.4'}],
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
            # Hadoop 2.10.1 is no longer supported
            # {
            #     'product': '2.3.9',
            #     'hadoop': '2.10.1',
            #     'jackson_dataformat_xml': '2.7.9',
            #     'aws_java_sdk_bundle': '1.11.271',
            #     'azure_storage': '7.0.1',
            #     'azure_keyvault_core': '1.0.0',
            # },
            {
                'product': '3.1.3',
                'hadoop': '3.3.3',
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
        'name': 'vector',
        'versions': [{'product': '0.26.0', 'stackable-base': '1.0.0'}],
    },
    {
        'name': 'nifi',
        'versions': [{'product': '1.15.3'}, {'product': '1.16.3'}, {'product': '1.18.0'}],
    },
    {
        'name': 'opa',
        'versions': [{'product': '0.27.1'}, {'product': '0.28.0'}, {'product': '0.37.2'}, {'product': '0.41.0'},
                     {'product': '0.45.0'}],
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
        'name': 'stackable-base',
        'versions': [{'product': '1.0.0'}],
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
                'java_base_image_sha256': '7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e',
                'opa_authorizer': '0.1.0',
            },
            {
                'product': '387',
                'java': '11',
                'java_base_image_sha256': '7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e',
                'opa_authorizer': '0.1.0'
            },
            {
                'product': '395',
                'java': '17',
                'java_base_image_sha256': '2b8d60d1ab50d56240cb6286d6bc377410442afbfc3292d81be5674bc0b51724',
                'opa_authorizer': 'stackable0.1.0'
            },
            {
                'product': '396',
                'java': '17',
                'java_base_image_sha256': '2b8d60d1ab50d56240cb6286d6bc377410442afbfc3292d81be5674bc0b51724',
                'opa_authorizer': 'stackable0.1.0'
            },
            {
                'product': '403',
                'java': '17',
                'java_base_image_sha256': '2b8d60d1ab50d56240cb6286d6bc377410442afbfc3292d81be5674bc0b51724',
                'opa_authorizer': 'stackable0.1.0'
            },
        ],
    },
    {
        'name': 'tools',
        'versions': [{'product': '0.2.0'}],
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
