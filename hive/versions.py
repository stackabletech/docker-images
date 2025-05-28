versions = [
    {
        "product": "3.1.3",
        "jmx_exporter": "1.2.0",
        # Hive 3 must be built with Java 8 but will run on Java 11
        "java-base": "11",
        "java-devel": "8",
        "hadoop": "3.3.6",
        # Keep consistent with the dependency from Hadoop: https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.6
        "aws_java_sdk_bundle": "1.12.367",
        "azure_storage": "7.0.1",
        "azure_keyvault_core": "1.0.0",
    },
    {
        "product": "4.0.0",
        "jmx_exporter": "1.2.0",
        # Hive 4 must be built with Java 8 (according to GitHub README) but seems to run on Java 11
        "java-base": "11",
        "java-devel": "8",
        "hadoop": "3.3.6",
        # Keep consistent with the dependency from Hadoop: https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.6
        "aws_java_sdk_bundle": "1.12.367",
        "azure_storage": "7.0.1",
        "azure_keyvault_core": "1.0.0",
    },
    {
        "product": "4.0.1",
        "jmx_exporter": "1.2.0",
        # Hive 4 must be built with Java 8 (according to GitHub README) but seems to run on Java 11
        "java-base": "11",
        "java-devel": "8",
        "hadoop": "3.3.6",
        # Keep consistent with the dependency from Hadoop: https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.6
        "aws_java_sdk_bundle": "1.12.367",
        "azure_storage": "7.0.1",
        "azure_keyvault_core": "1.0.0",
    },
]
