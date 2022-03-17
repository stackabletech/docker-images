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
        'versions': ['2.4.6', '2.4.8', '2.4.9'],
    },
    {
        'name': 'hive',
        'versions': ['2.3.9'],
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
        'versions': ['1.13.2', '1.15.0'],
    },
    {
        'name': 'opa',
        'versions': ['0.27.1', '0.28.0', '0.37.2'],
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
                '_base_image_tag': '286ba5d37d4e240d01bbefd2307a816829cf512d',
            },
            {
                'product': '1.4.1',
                '_base_image_tag': '26545a26d195b79ab6e838631e86cb0bf20f3ced',
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
