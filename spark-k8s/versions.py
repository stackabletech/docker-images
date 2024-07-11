versions = [
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
        "vector": "0.39.0",
        "jmx_exporter": "1.0.1",
        "tini": "0.19.0",
    },
    {
        "product": "3.4.3",
        "java-base": "11",
        "java-devel": "11",
        "python": "3.11",
        "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/1db2f5c36b120c213432fc658c9fd24fc73cb45e/pom.xml#L122
        "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
        "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
        "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
        "jackson_dataformat_xml": "2.14.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.4.3
        "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
        "woodstox_core": "6.5.0",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.14.2
        "vector": "0.39.0",
        "jmx_exporter": "1.0.1",
        "tini": "0.19.0",
    },    
    {
        "product": "3.5.1",
        "java-base": "17",
        "java-devel": "17",
        "python": "3.11",
        "hadoop_long_version": "3.3.4",  # https://github.com/apache/spark/blob/6a5747d66e53ed0d934cdd9ca5c9bd9fde6868e6/pom.xml#L125
        "aws_java_sdk_bundle": "1.12.262",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.3.4
        "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.3.4
        "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
        "jackson_dataformat_xml": "2.15.2",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/3.5.1
        "stax2_api": "4.2.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.15.2
        "woodstox_core": "6.5.1",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.15.2
        "vector": "0.39.0",
        "jmx_exporter": "1.0.1",
        "tini": "0.19.0",
    },
    {
        "product": "4.0.0-preview1",
        "java-base": "17",
        "java-devel": "17",
        "python": "3.11",
        "hadoop_long_version": "3.4.0",  # https://github.com/apache/spark/blob/7a7a8bc4bab591ac8b98b2630b38c57adf619b82/pom.xml#L125
        # NOTE: The "aws_java_sdk_bundle" jar now is only called bundle-x.x.x instead of aws-java-sdk-bundle-x.x.x and was renamed before uploading to nexus
        "aws_java_sdk_bundle": "2.23.19",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-aws/3.4.0
        "azure_storage": "7.0.1",  # https://mvnrepository.com/artifact/org.apache.hadoop/hadoop-azure/3.4.0
        "azure_keyvault_core": "1.0.0",  # https://mvnrepository.com/artifact/com.microsoft.azure/azure-storage/7.0.1
        "jackson_dataformat_xml": "2.17.1",  # https://mvnrepository.com/artifact/org.apache.spark/spark-core_2.13/4.0.0-preview1
        "stax2_api": "4.2.2",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.17.1
        "woodstox_core": "6.6.2",  # https://mvnrepository.com/artifact/com.fasterxml.jackson.dataformat/jackson-dataformat-xml/2.17.1
        "vector": "0.39.0",
        "jmx_exporter": "1.0.1",
        "tini": "0.19.0",
    },
]
