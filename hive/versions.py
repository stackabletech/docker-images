versions = [
    {
        "product": "3.1.3",
        "jmx_exporter": "1.0.1",
        # Hive must be built with Java 8 but will run on Java 11
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
]
