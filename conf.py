"""
Configuration file for the Stackable image-tools: https://github.com/stackabletech/image-tools.

Application images will be created for products and associated versions configured here.
"""

products = [
    {
        "name": "airflow",
        "versions": [
            {
                "product": "2.6.3",
                "python": "3.9",
                "git_sync": "v4.2.1",
                "statsd_exporter": "0.26.0",
                "tini": "0.19.0",
                "vector": "0.35.0",
            },
            {
                "product": "2.7.2",
                "python": "3.9",
                "git_sync": "v4.2.1",
                "statsd_exporter": "0.26.0",
                "tini": "0.19.0",
                "vector": "0.35.0",
            },
            {
                "product": "2.7.3",
                "python": "3.9",
                "git_sync": "v4.2.1",
                "statsd_exporter": "0.26.0",
                "tini": "0.19.0",
                "vector": "0.35.0",
            },
            {
                "product": "2.8.1",
                "python": "3.9",
                "git_sync": "v4.2.1",
                "statsd_exporter": "0.26.0",
                "tini": "0.19.0",
                "vector": "0.35.0",
            },
            {
                "product": "2.8.3",
                "python": "3.9",
                "git_sync": "v4.2.1",
                "statsd_exporter": "0.26.0",
                "tini": "0.19.0",
                "vector": "0.35.0",
            },
        ],
    },
    {
        "name": "druid",
        "versions": [
            {
                "product": "26.0.0",
                "java-base": "11",
                "java-devel": "11",
                "jackson_dataformat_xml": "2.10.5",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "authorizer": "0.5.0",
            },
            {
                "product": "27.0.0",
                "java-base": "11",
                "java-devel": "11",
                "jackson_dataformat_xml": "2.10.5",
                "stax2_api": "4.2.1",
                "woodstox_core": "6.2.1",
                "authorizer": "0.5.0",
            },
            {
                "product": "28.0.1",
                # Java 17 should be fully supported as of 27.0.0 https://github.com/apache/druid/releases#27.0.0-highlights-java-17-support
                # Did not work in a quick test due to reflection error:
                # Caused by: java.lang.reflect.InaccessibleObjectException: Unable to make protected final java.lang.Class
                # java.lang.ClassLoader.defineClass(java.lang.String,byte[],int,int,java.security.ProtectionDomain) throws java.lang.ClassFormatError
                "java-base": "11",
                "java-devel": "11",
                "jackson_dataformat_xml": "2.12.7",  # from https://github.com/apache/druid/blob/b8201e31aa6b124049a61764309145baaad78db7/pom.xml#L100
                "stax2_api": "4.2.2",
                "woodstox_core": "6.6.0",
                "authorizer": "0.5.0",
            },
        ],
    },
    {
        "name": "hadoop",
        "versions": [
            {
                "product": "3.3.4",
                "java-base": "11",
                "java-devel": "11",
                "async_profiler": "2.9",
                "jmx_exporter": "0.20.0",
                "protobuf": "3.7.1",
                "hdfs_utils": "0.2.1",
                "topology_provider": "0.3.0",
            },
            {
                "product": "3.3.6",
                "java-base": "11",
                "java-devel": "11",
                "async_profiler": "2.9",
                "jmx_exporter": "0.20.0",
                "protobuf": "3.7.1",
                "hdfs_utils": "0.2.1",
                "topology_provider": "0.3.0",
            },
        ],
    },
    {
        "name": "hbase",
        "versions": [
            # Also do not merge java-base with java below as "JAVA-BASE is not a valid identifier" in Dockerfiles, it's unfortunate but to fix this would require a bigger refactoring of names or the image tools
            # hbase-thirdparty is used to build the hbase-operator-tools and should be set to the version defined in the POM of HBase.
            {
                "product": "2.4.17",
                "hbase_thirdparty": "4.1.5",
                "hbase_operator_tools": "1.2.0",
                "java-base": "11",
                "java-devel": "11",
                "async_profiler": "2.9",
                "phoenix": "5.2.0",
                "hbase_profile": "2.4",
                "hadoop": "3.3.6",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "2.6.0",
                "hbase_thirdparty": "4.1.7",
                "hbase_operator_tools": "1.3.0-SNAPSHOT",
                "java-base": "11",
                "java-devel": "11",
                "async_profiler": "2.9",
                "phoenix": "5.3.0-SNAPSHOT",
                "hbase_profile": "2.6",
                "hadoop": "3.3.6",
                "jmx_exporter": "",  # 2.6 exports jmx and prometheus metrics by default
                "jackson_dataformat_xml": "2.17.0",  # only for HBase 2.6.0
                "opa_authorizer": "initial-setup",  # only for HBase 2.6.0
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
                # Hive must be bult with Java 8 but will run on Java 11
                "java-base": "11",
                "java-devel": "1.8.0",
                "hadoop": "3.3.4",
                "jackson_dataformat_xml": "2.12.3",
                # No longer bundled with the hadoop-yarn/mapreduce libraries (2.12.7 corresponds to the hadoop build for 3.3.4).
                "jackson_jaxb_annotations": "2.12.7",
                # Normally Hive 3.1.3 ships with "postgresql-9.4.1208.jre7.jar", but as this is old enough it does only support
                # MD5 based authentication. Because of this, it does not work against more recent PostgresQL versions.
                # See https://github.com/stackabletech/hive-operator/issues/170 for details.
                "postgres_driver": "42.7.2",
                "aws_java_sdk_bundle": "1.12.262",
                "azure_storage": "7.0.1",
                "azure_keyvault_core": "1.0.0",
            },
        ],
    },
    {
        "name": "java-base",
        "versions": [
            {
                "product": "1.8.0",
                "vector": "0.35.0",
            },
            {
                "product": "11",
                "vector": "0.35.0",
            },
            {
                "product": "17",
                "vector": "0.35.0",
            },
            {
                "product": "21",
                "vector": "0.35.0",
            },
        ],
    },
    {
        "name": "java-devel",
        "versions": [
            {
                "product": "1.8.0",
                "stackable-base": "1.0.0",
            },
            {
                "product": "11",
                "stackable-base": "1.0.0",
            },
        ],
    },
    {
        "name": "kafka",
        "versions": [
            {
                "product": "3.4.1",
                "java-base": "11",
                "java-devel": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.5.1",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.5.2",
                "java-base": "11",
                "java-devel": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.5.1",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.6.1",
                "java-base": "11",
                "java-devel": "11",
                "scala": "2.13",
                "kcat": "1.7.0",
                "opa_authorizer": "1.5.1",
                "jmx_exporter": "0.20.0",
            },
        ],
    },
    {
        "name": "krb5",
        "versions": [{"product": "1.21.1"}],
    },
    {
        "name": "vector",
        "versions": [
            {
                "product": "0.35.0",
                "rpm_release": "1",
                "stackable-base": "1.0.0",
                "inotify_tools": "3.14-19.el8",
            }
        ],
    },
    {
        "name": "nifi",
        "versions": [
            {"product": "1.21.0", "java-base": "11", "java-devel": "11"},
            {"product": "1.23.2", "java-base": "11", "java-devel": "11"},
            {"product": "1.25.0", "java-base": "21", "java-devel": "11"},
        ],
    },
    {
        "name": "omid",
        "versions": [
            {
                "product": "1.1.0",
                "java-base": "11",
                "java-devel": "11",
                "jmx_exporter": "0.20.0",
            },
        ],
    },
    {
        "name": "opa",
        "versions": [
            {
                "product": "0.57.0",
                "vector": "0.35.0",
                "bundle_builder_version": "1.1.2",
                "stackable-base": "1.0.0",
            },
            {
                "product": "0.61.0",
                "vector": "0.35.0",
                "bundle_builder_version": "1.1.2",
                "stackable-base": "1.0.0",
            },
        ],
    },
    {
        "name": "spark-k8s",
        "versions": [
            {
                "product": "3.4.1",
                "java-base": "11",
                "java-devel": "11",
                "python": "3.11",
                "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/1db2f5c36b120c213432fc658c9fd24fc73cb45e/pom.xml#L122
                "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
                "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
                "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
                "jackson_dataformat_xml": "2.14.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.4.1
                "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
                "woodstox_core": "6.5.0",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
                "vector": "0.35.0",
                "jmx_exporter": "0.20.0",
                "tini": "0.19.0",
            },
            {
                "product": "3.4.2",
                "java-base": "11",
                "java-devel": "11",
                "python": "3.11",
                "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/1db2f5c36b120c213432fc658c9fd24fc73cb45e/pom.xml#L122
                "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
                "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
                "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
                "jackson_dataformat_xml": "2.14.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.4.2
                "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
                "woodstox_core": "6.5.0",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
                "vector": "0.35.0",
                "jmx_exporter": "0.20.0",
                "tini": "0.19.0",
            },
            {
                "product": "3.5.0",
                "java-base": "11",
                "java-devel": "11",
                "python": "3.11",
                "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/6a5747d66e53ed0d934cdd9ca5c9bd9fde6868e6/pom.xml#L125
                "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
                "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
                "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
                "jackson_dataformat_xml": "2.15.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.5.0
                "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.15.2
                "woodstox_core": "6.5.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.15.2
                "vector": "0.35.0",
                "jmx_exporter": "0.20.0",
                "tini": "0.19.0",
            },
            {
                "product": "3.5.1",
                "spark": "3.5.1",
                "java-base": "11",
                "java-devel": "11",
                "python": "3.11",
                "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/6a5747d66e53ed0d934cdd9ca5c9bd9fde6868e6/pom.xml#L125
                "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
                "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
                "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
                "jackson_dataformat_xml": "2.15.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.5.1
                "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.15.2
                "woodstox_core": "6.5.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.15.2
                "vector": "0.35.0",
                "jmx_exporter": "0.20.0",
                "tini": "0.19.0",
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
                "product": "2.1.1",
                "python": "3.9",
                "vector": "0.35.0",
                "statsd_exporter": "0.26.0",
                "authlib": "0.15.4",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.0/requirements-extra.txt#L10
            },
            {
                "product": "2.1.3",
                "python": "3.9",
                "vector": "0.35.0",
                "statsd_exporter": "0.26.0",
                "authlib": "0.15.4",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.0/requirements-extra.txt#L10
            },
            {
                "product": "3.0.1",
                "python": "3.9",
                "vector": "0.35.0",
                "statsd_exporter": "0.26.0",
                "authlib": "0.15.4",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.7/requirements-extra.txt#L7
            },
            {
                "product": "3.0.3",
                "python": "3.9",
                "vector": "0.35.0",
                "statsd_exporter": "0.26.0",
                "authlib": "1.2.1",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.10/requirements-extra.txt#L7
            },
            {
                "product": "3.1.0",
                "python": "3.9",
                "vector": "0.35.0",
                "statsd_exporter": "0.26.0",
                "authlib": "1.2.1",  # https://github.com/dpgaspar/Flask-AppBuilder/blob/v4.3.10/requirements-extra.txt#L7
            },
        ],
    },
    {
        "name": "trino-cli",
        "versions": [
            {
                "product": "442",
                "java-base": "21",
            },
        ],
    },
    {
        "name": "trino",
        "versions": [
            {
                "product": "414",
                "java-base": "17",
                "opa_authorizer": "stackable0.2.0",
                "jmx_exporter": "0.20.0",
                "storage_connector": "414",
            },
            {
                "product": "428",
                "java-base": "17",
                "opa_authorizer": "stackable0.3.0",
                "jmx_exporter": "0.20.0",
                "storage_connector": "428-jackson",
            },
            {
                "product": "442",
                "java-base": "21",
                "jmx_exporter": "0.20.0",
                "storage_connector": "442",
                "opa_authorizer": "",
            },
        ],
    },
    {
        "name": "kafka-testing-tools",
        "versions": [
            {
                "product": "1.0.0",
                "kcat": "1.7.0",
                "java-base": "11",
                "stackable-base": "1.0.0",
            }
        ],
    },
    {
        "name": "kcat",
        "versions": [
            {
                "product": "1.7.0",
                "java-base": "11",
                "stackable-base": "1.0.0",
            }
        ],
    },
    {
        "name": "testing-tools",
        "versions": [
            {
                "product": "0.2.0",
                "keycloak_version": "23.0.0",
            }
        ],
    },
    {
        "name": "zookeeper",
        "versions": [
            {
                "product": "3.8.3",
                "java-base": "11",
                "java-devel": "11",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.8.4",
                "java-base": "11",
                "java-devel": "11",
                "jmx_exporter": "0.20.0",
            },
            {
                "product": "3.9.2",
                "java-base": "11",
                "java-devel": "11",
                "jmx_exporter": "0.20.0",
            },
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
