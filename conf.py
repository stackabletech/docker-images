"""
Application images will be created for products and associated versions configured here.
"""
products = [
    {
        'name': 'druid',
        'versions': ['0.22.1'],
    },
    {
        'name': 'hadoop',
        'versions': ['3.2.2', '3.3.1'],
    },
    {
        'name': 'hbase',
        'versions': ['2.4.6', '2.4.8'],
    },
    {
        'name': 'hive',
        'versions': ['2.3.9'],
    },
    {
        # Opa authorizer 1.1.0 for Kafka version < 3.0.0
        # Opa authorizer 1.2.0 for Kafka version >= 3.0.0
        'name': 'kafka',
        'versions': [
            {
                'product': '2.6.2',
                'scala': '2.13',
                'opa_authorizer': '1.1.0',
            },
            {
                'product': '2.7.1',
                'scala': '2.13',
                'opa_authorizer': '1.1.0',
            },
            {
                'product': '2.8.1',
                'scala': '2.13',
                'opa_authorizer': '1.1.0',
            },
        ]
    },
    {
        'name': 'nginx',
        'versions': ['1.21.6'],
    },
    {
        'name': 'nifi',
        'versions': ['1.13.2', '1.15.0'],
    },
    {
        'name': 'opa',
        'versions': ['0.27.1', '0.28.0'],
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
        'name': 'superset',
        'versions': [
            {
                'product': '1.3.2',
                '_base_image_tag': '9515ba68dc560307758774d1618c885e379d2011',
            },
        ],
    },
    {
        'name': 'trino',
        'versions': ['362'],
    },
    {
        'name': 'tools',
        'versions': ['0.2.0'],
    },
    {
        'name': 'airflow',
        'versions': [
            {
                'product': '2.2.3',
                'python': '3.8',
            },
        ]
    },
    {
        # ZooKeeper must be at least 3.5.0
        'name': 'zookeeper',
        'versions': ['3.5.8', '3.6.3', '3.7.0'],
    },
 ]

