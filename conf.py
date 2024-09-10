"""
Configuration file for the Stackable image-tools: https://github.com/stackabletech/image-tools.

Application images will be created for products and associated versions configured here.
"""

# NOTE: The .scripts/enumerate-product-versions.py script (used in the release workflow as of 2024-07-23) imports this file and it relies on conf.py being in its parent folder. Should this file be moved or the structure changed in any way remember to update that script as well!

# NOTE (@NickLarsenNZ): Unfortunately, some directories have hyphens, so they need
# importing in a special way. For consistency, we'll do them all the same way.
import importlib

airflow = importlib.import_module("airflow.versions")
druid = importlib.import_module("druid.versions")
hadoop = importlib.import_module("hadoop.versions")
hbase = importlib.import_module("hbase.versions")
hello_world = importlib.import_module("hello-world.versions")
hive = importlib.import_module("hive.versions")
java_base = importlib.import_module("java-base.versions")
java_devel = importlib.import_module("java-devel.versions")
kafka = importlib.import_module("kafka.versions")
krb5 = importlib.import_module("krb5.versions")
vector = importlib.import_module("vector.versions")
nifi = importlib.import_module("nifi.versions")
omid = importlib.import_module("omid.versions")
opa = importlib.import_module("opa.versions")
spark_k8s = importlib.import_module("spark-k8s.versions")
stackable_base = importlib.import_module("stackable-base.versions")
superset = importlib.import_module("superset.versions")
trino_cli = importlib.import_module("trino-cli.versions")
trino = importlib.import_module("trino.versions")
kafka_testing_tools = importlib.import_module("kafka-testing-tools.versions")
kcat = importlib.import_module("kcat.versions")
testing_tools = importlib.import_module("testing-tools.versions")
zookeeper = importlib.import_module("zookeeper.versions")
tools = importlib.import_module("tools.versions")
statsd_exporter = importlib.import_module("statsd_exporter.versions")

products = [
    {"name": "airflow", "versions": airflow.versions},
    {"name": "druid", "versions": druid.versions},
    {"name": "hadoop", "versions": hadoop.versions},
    {"name": "hbase", "versions": hbase.versions},
    {"name": "hello-world", "versions": hello_world.versions},
    {"name": "hive", "versions": hive.versions},
    {"name": "java-base", "versions": java_base.versions},
    {"name": "java-devel", "versions": java_devel.versions},
    {"name": "kafka", "versions": kafka.versions},
    {"name": "krb5", "versions": krb5.versions},
    {"name": "vector", "versions": vector.versions},
    {"name": "nifi", "versions": nifi.versions},
    {"name": "omid", "versions": omid.versions},
    {"name": "opa", "versions": opa.versions},
    {"name": "spark-k8s", "versions": spark_k8s.versions},
    {"name": "stackable-base", "versions": stackable_base.versions},
    {"name": "superset", "versions": superset.versions},
    {"name": "trino-cli", "versions": trino_cli.versions},
    {"name": "trino", "versions": trino.versions},
    {"name": "kafka-testing-tools", "versions": kafka_testing_tools.versions},
    {"name": "kcat", "versions": kcat.versions},
    {"name": "testing-tools", "versions": testing_tools.versions},
    {"name": "zookeeper", "versions": zookeeper.versions},
    {"name": "tools", "versions": tools.versions},
    {"name": "statsd_exporter", "versions": statsd_exporter.versions},
]

open_shift_projects = {
    "airflow": {"id": "62613f498ccb9938ba3cfde6"},
    "druid": {"id": "626140028ccb9938ba3cfde7"},
    "hadoop": {"id": "6261407f887d6e0b8614660c"},
    "hbase": {"id": "62614109992bac3f9a4a24b8"},
    "hive": {"id": "626140806812078a392dceaa"},
    "kafka": {"id": "625ff25b91bdcd4b49c823a4"},
    "nifi": {"id": "625586a32e9e14bc8118e203"},
    "opa": {"id": "6255838bea1feb8bec4aaaa3"},
    "spark-k8s": {"id": "62613e81f8ce82a2f247dda5"},
    "superset": {"id": "62557e5fea1feb8bec4aaaa0"},
    "tools": {"id": "62557cd575ab7e30884aaaa0"},
    "trino": {"id": "62557c4a0030f6483318e203"},
    "zookeeper": {"id": "62552b0aadd9d54d56cda11d"},
}

cache = [
    {
        "type": "registry",
        "ref_prefix": "build-repo.stackable.tech:8083/sandbox/cache",
        "mode": "max",
        "compression": "zstd",
        "ignore-error": "true",
    },
]

args = {
    "STACKABLE_USER_NAME": "stackable",
    "STACKABLE_USER_UID": "1000",
    "STACKABLE_USER_GID": "1000"
}
