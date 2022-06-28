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
        ]
    },
    {
        'name': 'druid',
        'versions': [
            {
                'product': '0.23.0',
                'authorizer': '0.2.0',
            }
        ]
    },
    {
        'name': 'hadoop',
        'versions': ['3.2.2', '3.3.1'],
    },
    {
        'name': 'hbase',
        'versions': ['2.4.6', '2.4.8', '2.4.9', '2.4.11'],
    },
    {
        'name': 'hive',
        'versions': ['2.3.9'],
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
        ]
    },
    {
        'name': 'nifi',
        'versions': ['1.13.2', '1.15.0', '1.15.1', '1.15.2', '1.15.3', '1.16.0', '1.16.1', '1.16.2', '1.16.3'],
    },
    {
        'name': 'opa',
        'versions': ['0.27.1', '0.28.0', '0.37.2'],
    },
    {
        'name': 'pyspark-k8s',
        'versions': [
            {
                'product': '3.2.1',
                'hadoop': '3.2',
                'python': '39',
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
                'hadoop': '3.2',
            },
        ]
    },
    {
        'name': 'superset',
        'versions': [
            {
                'product': '1.3.2',
                'python': '38',
            },
            {
                'product': '1.4.1',
                'python': '39',
            },
        ],
    },
    {
        'name': 'trino',
        'versions': [
            {
                'product': '377',
                'opa_authorizer': '0.1.0'
            },
        ],
    },
    {
        'name': 'tools',
        'versions': ['0.2.0'],
    },
    {
        # ZooKeeper must be at least 3.5.0
        'name': 'zookeeper',
        'versions': ['3.5.8', '3.6.3', '3.7.0', '3.8.0'],
    },
 ]
