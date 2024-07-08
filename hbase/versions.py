versions = [
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
        "jmx_exporter": "1.0.1",
        "delete_caches": "true",
    },
    {
        "product": "2.4.18",
        "hbase_thirdparty": "4.1.5",
        "hbase_operator_tools": "1.2.0",
        "java-base": "11",
        "java-devel": "11",
        "async_profiler": "2.9",
        "phoenix": "5.2.0",
        "hbase_profile": "2.4",
        "hadoop": "3.3.6",
        "jmx_exporter": "1.0.1",
        "delete_caches": "true",
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
        "delete_caches": "true",
    },
]
