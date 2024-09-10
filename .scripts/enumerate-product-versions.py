import sys
import os

# NOTE: This script (used in the release workflow as of 2024-07-23) relies on conf.py being in its parent folder. Should either file be moved or the structure changed in any way remember to update this script as well.
# Add parent directory to the sys.path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import conf

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

for product in conf.products:
    product_name = product["name"]
    if product_name not in PRODUCTS:
        continue

    for version in product["versions"]:
        print(f"{product_name}#{version['product']}")
