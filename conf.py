"""
Application images will be created for products and associated versions configured here.
"""
products = [
    {
        'name': 'druid',
        'versions': ['0.22.0'],
    },
    {
        'name': 'hadoop',
        'versions': ['3.2.2', '3.3.1'],
    },
    {
        'name': 'hbase',
        'versions': ['2.4.6'],
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
                'product': '2.8.0',
                'scala': '2.12',
                'opa_authorizer': '1.1.0',
            },
            {
                'product': '2.8.0',
                'scala': '2.13',
                'opa_authorizer': '1.1.0',
            },
        ]
    },
    {
        'name': 'nifi',
        'versions': ['1.13.2'],
    },
    {
        'name': 'opa',
        'versions': ['0.27.1'],
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
        'versions': ['1.3.2'],
    },
    {
        'name': 'trino',
        'versions': ['362'],
    },
    {
        # ZooKeeper PRODUCT must be at least 3.5.0 in order to work
        'name': 'zookeeper',
        'versions': ['3.5.8', '3.7.0'],
    },
 ]

