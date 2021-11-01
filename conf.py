"""
Application images will be created for products and associated versions configured here.
"""
products = [
    {
        'name': 'zookeeper',
        'versions': ['3.5.8', '3.7.0'],
    },
    {
        'name': 'opa',
        'versions': ['0.34.0'],
    },
    {
        'name': 'kafka',
        'versions': [
            {
                'product_version': '2.8.0',
                'scala_version': '2.12',
            },
            {
                'product_version': '2.8.0',
                'scala_version': '2.13',
            },
        ]
    },
    {
        'name': 'nifi',
        'versions': ['1.13.2', '1.14.0'],
    },
    {
        'name': 'trino',
        'versions': ['362'],
    },
    {
        'name': 'spark',
        'versions': [
            {
                'product_version': '3.0.1',
                'hadoop_version': '2.7',
            },
            {
                'product_version': '3.1.1',
                'hadoop_version': '2.7',
            },
        ]
    },
    {
        'name': 'hadoop',
        'versions': ['3.2.2', '3.3.1'],
    },
    {
        'name': 'hive',
        'versions': ['2.3.9'],
    },
    {
        'name': 'hbase',
        'versions': ['2.4.6'],
    },
 ]

