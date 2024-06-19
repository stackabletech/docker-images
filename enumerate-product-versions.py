import conf
import sys

PRODUCTS = [
    "airflow",
    "druid",
    "hadoop",
    "hbase",
    "hive",
    "kafka",
    "kafka-testing-tools",
    "krb5",
    "nifi",
    "opa",
    "omid",
    "spark-k8s",
    "superset",
    "testing-tools",
    "trino",
    "trino-cli",
    "tools",
    "zookeeper",
]

single_product = None
if len(sys.argv) > 1:
    single_product = sys.argv[1]
    if single_product not in PRODUCTS:
        print(f"Error: {single_product} is not a valid product.")
        sys.exit(1)

for product in conf.products:
    product_name = product['name']
    if product_name not in PRODUCTS:
        continue
    if single_product and product_name != single_product:
        continue

    for version in product['versions']:
        print(f"{product_name}#{version['product']}")
