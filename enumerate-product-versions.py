import conf

PRODUCTS = [
    # "airflow",
    # "druid",
    # "hadoop",
    # "hbase",
    # "hive",
    # "kafka",
    # "kafka-testing-tools",
    "krb5",
    # "nifi",
    # "opa",
    # "spark-k8s",
    # "superset",
    # "testing-tools",
    # "trino",
    # "trino-cli",
    # "tools",
    # "zookeeper",
]

for product in conf.products:
    product_name = product['name']
    if product_name not in PRODUCTS:
        continue

    for version in product['versions']:
        print(f"{product_name}#{version['product']}")
