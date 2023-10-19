"""
Configuration file for the Stackable image-tools: https://github.com/stackabletech/image-tools.

Application images will be created for products and associated versions configured here.
"""

products = [
    {
        "name": "airflow",
        "versions": [
            {
                "product": "2.2.3",
                "git_sync": "v3.6.8",
                "python": "38",
                "statsd_exporter": "v0.24.0",
                "tini": "0.19.0",
                "vector": "0.33.0",
            },
            {
                "product": "2.2.4",
                "git_sync": "v3.6.8",
                "python": "39",
                "statsd_exporter": "v0.24.0",
                "tini": "0.19.0",
                "vector": "0.33.0",
            },
            {
                "product": "2.2.5",
                "git_sync": "v3.6.8",
                "python": "39",
                "statsd_exporter": "v0.24.0",
                "tini": "0.19.0",
                "vector": "0.33.0",
            },
            {
                "product": "2.4.1",
                "git_sync": "v3.6.8",
                "python": "39",
                "statsd_exporter": "v0.24.0",
                "tini": "0.19.0",
                "vector": "0.33.0",
            },
            {
                "product": "2.6.1",
                "git_sync": "v3.6.8",
                "python": "39",
                "statsd_exporter": "v0.24.0",
                "tini": "0.19.0",
                "vector": "0.33.0",
            },
        ],
    },
    {
        "name": "druid",
        "versions": [
            {
                "product": "0.23.0",
                "java-base": "11",
                "jackson_dataformat_xml": "2.10.5",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "authorizer": "0.2.0",
            },
            {
                "product": "24.0.0",
                "java-base": "11",
                "jackson_dataformat_xml": "2.10.5",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "authorizer": "0.4.0",
            },
            {
                "product": "26.0.0",
                "java-base": "11",
                "jackson_dataformat_xml": "2.10.5",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "authorizer": "0.5.0",
            },
        ],
    },
    {
        "name": "hadoop",
        "versions": [
            {"product": "3.2.2", "java-base": "11", "jmx_exporter": "0.20.0"},
            {"product": "3.2.4", "java-base": "11", "jmx_exporter": "0.20.0"},
            {"product": "3.3.4", "java-base": "11", "jmx_exporter": "0.20.0"},
            {"product": "3.3.6", "java-base": "11", "jmx_exporter": "0.20.0"},
        ],
    },
    {
        "name": "hbase",
        "versions": [
            {
                "product": "2.4.12",
                "java-base": "11",
                "phoenix": "2.4-5.1.2",
                "hadoop": "3.3.6",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "2.4.17",
                "java-base": "11",
                "phoenix": "2.4-5.1.3",
                "hadoop": "3.3.6",
                "jmx_exporter": "0.20.0",
            },
        ],
    },
    {
        "name": "hello-world",
        "versions": [
            {
                "product": "0.0.1-SNAPSHOT",
                "java-base": "17",
            },
        ],
    },
    {
        "name": "hive",
        "versions": [
            {
                "product": "3.1.3",
                "jmx_exporter": "0.20.0",
                "java-base": "11",
                "hadoop_libs": "3.3.3",
                "jackson_dataformat_xml": "2.12.3",
                "aws_java_sdk_bundle": "1.11.1026",
                "azure_storage": "7.0.1",
                "azure_keyvault_core": "1.0.0",
            },
        ],
    },
    {
        "name": "java-base",
        "versions": [
            {
                "product": "11",
                "vector": "0.33.0",
            },
            {
                "product": "17",
                "vector": "0.33.0",
            },
        ],
    },
    {
        "name": "kafka",
        "versions": [
            {
                "product": "2.8.1",
                "java-base": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.4.0",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "2.8.2",
                "java-base": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.4.0",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.4.0",
                "java-base": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.5.1",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.4.1",
                "java-base": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.5.1",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.5.1",
                "java-base": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.5.1",
                "jmx_exporter": "0.20.0",
            },
        ],
    },
    {
        "name": "krb5",
        "versions": [{"product": "1.18.2"}],
    },
    {
        "name": "vector",
        "versions": [
            {
                "product": "0.33.0",
                "rpm_release": "1",
                "stackable-base": "1.0.0"
            }
        ],
    },
    {
        "name": "nifi",
        "versions": [
            {"product": "1.21.0", "java-base": "11"},
            {"product": "1.23.2", "java-base": "11"},
        ],
    },
    {
        "name": "opa",
        "versions": [
            {
                "product": "0.51.0",
                "vector": "0.33.0",
                "bundle_builder_version": "1.1.0",
            },
            {
                "product": "0.57.0",
                "vector": "0.33.0",
                "bundle_builder_version": "1.1.0",
            },
        ],
    },
    {
        "name": "spark-k8s",
        "versions": [
            {
                "product": "3.2.1",
                "spark": "3.2.1",
                "java-base": "11",
                "python": "39",
                "hadoop_short_version": "3.2",
                "hadoop_long_version": "3.3.1",
                "aws_java_sdk_bundle": "1.11.901",
                "azure_storage": "7.0.1",
                "azure_keyvault_core": "1.0.0",
                "jackson_dataformat_xml": "2.12.3",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "vector": "0.33.0",
            },
            {
                "product": "3.3.0",
                "spark": "3.3.0",
                "java-base": "11",
                "python": "39",
                "hadoop_short_version": "3",
                "hadoop_long_version": "3.3.3",
                "aws_java_sdk_bundle": "1.11.1026",
                "azure_storage": "7.0.1",
                "azure_keyvault_core": "1.0.0",
                "jackson_dataformat_xml": "2.13.3",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "vector": "0.33.0",
            },
            {
                "product": "3.3.0-java17",
                "spark": "3.3.0",
                "java-base": "17",
                "python": "39",
                "hadoop_short_version": "3",
                "hadoop_long_version": "3.3.3",
                "aws_java_sdk_bundle": "1.11.1026",
                "azure_storage": "7.0.1",
                "azure_keyvault_core": "1.0.0",
                "jackson_dataformat_xml": "2.13.3",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "vector": "0.33.0",
            },
            {
                "product": "3.4.0",
                "spark": "3.4.0",
                "java-base": "11",
                "python": "3.11",
                "hadoop_short_version": "3",
                "hadoop_long_version": "3.3.4",
                "aws_java_sdk_bundle": "1.12.262",
                "azure_storage": "7.0.1",
                "azure_keyvault_core": "1.0.0",
                "jackson_dataformat_xml": "2.14.2",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.5.0",
                "vector": "0.33.0",
            },
            {
                "product": "3.4.0-java17",
                "spark": "3.4.0",
                "java-base": "17",
                "python": "3.11",
                "hadoop_short_version": "3",
                "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/1db2f5c36b120c213432fc658c9fd24fc73cb45e/pom.xml#L122
                "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
                "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
                "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
                "jackson_dataformat_xml": "2.14.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.4.0
                "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
                "woodstox_core": "6.5.0",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
                "vector": "0.33.0",
            },
        ],
    },
    {
        "name": "stackable-base",
        "versions": [{"product": "1.0.0"}],
    },
    {
        "name": "superset",
        "versions": [
            {
                "product": "2.1.0",
                "python": "3.9",
                "vector": "0.33.0",
                "statsd_exporter": "v0.24.0",
                "authlib": "0.15.4"  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.0/requirements-extra.txt#L10
            },
            {
                "product": "2.1.1",
                "python": "3.9",
                "vector": "0.33.0",
                "statsd_exporter": "v0.24.0",
                "authlib": "0.15.4"  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.0/requirements-extra.txt#L10
            },
            {
                "product": "3.0.0",
                "python": "3.9",
                "vector": "0.33.0",
                "statsd_exporter": "v0.24.0",
                "authlib": "0.15.4"  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.6/requirements-extra.txt#L7
            },
        ],
    },
    {
        "name": "trino",
        "versions": [
            {"product": "377", "java-base": "11", "opa_authorizer": "0.1.0", "jmx_exporter": "0.16.1"},
            {"product": "387", "java-base": "11", "opa_authorizer": "0.1.0", "jmx_exporter": "0.16.1"},
            {"product": "395", "java-base": "17", "opa_authorizer": "stackable0.1.0", "jmx_exporter": "0.16.1"},
            {"product": "396", "java-base": "17", "opa_authorizer": "stackable0.1.0", "jmx_exporter": "0.16.1"},
            {"product": "403", "java-base": "17", "opa_authorizer": "stackable0.1.0", "jmx_exporter": "0.16.1"},
            {"product": "414", "java-base": "17", "opa_authorizer": "stackable0.2.0", "jmx_exporter": "0.18.0", "storage_connector": "414"},
        ],
    },
    {
        "name": "testing-tools",
        "versions": [{"product": "0.2.0"}],
    },
    {
        "name": "zookeeper",
        "versions": [
            {"product": "3.8.1", "java-base": "11", "jmx_exporter": "0.20.0"},
            {"product": "3.8.3", "java-base": "11", "jmx_exporter": "0.20.0"},
        ],
    },
    {
        "name": "tools",
        "versions": [
            {
                "product": "1.0.0",
                "kubectl_version": "1.26.2",
                "jq_version": "1.6",
                "stackable-base": "1.0.0",
            },
        ],
    },
]

open_shift_projects = {
    "airflow": {"id": "62613f498ccb9938ba3cfde6"},
    "druid": {"id": "626140028ccb9938ba3cfde7"},
    "hadoop": {"id": "6261407f887d6e0b8614660c"},
    "hbase": {"id": "62614109992bac3f9a4a24b8"},
    "hive": {"id": "626140028ccb9938ba3cfde7"},
    "kafka": {"id": "625ff25b91bdcd4b49c823a4"},
    "nifi": {"id": "625586a32e9e14bc8118e203"},
    "opa": {"id": "6255838bea1feb8bec4aaaa3"},
    "spark-k8s": {"id": "62613e81f8ce82a2f247dda5"},
    "superset": {"id": "62557e5fea1feb8bec4aaaa0"},
    "tools": {"id": "62557cd575ab7e30884aaaa0"},
    "trino": {"id": "62557c4a0030f6483318e203"},
    "zookeeper": {"id": "62552b0aadd9d54d56cda11d"},
}
