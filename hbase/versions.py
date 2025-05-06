versions = [
    # Also do not merge java-base with java below as "JAVA-BASE is not a valid identifier" in Dockerfiles, it's unfortunate but to fix this would require a bigger refactoring of names or the image tools
    # hbase-thirdparty is used to build the hbase-operator-tools and should be set to the version defined in the POM of HBase.
    {
        "product": "2.6.1",
        "hbase_thirdparty": "4.1.9",
        "hbase_operator_tools": "1.3.0-fd5a5fb",
        "java-base": "11",
        "java-devel": "11",
        "async_profiler": "2.9",
        "phoenix": "5.2.1",
        "hbase_profile": "2.6",
        "hadoop": "3.4.1",
        "jmx_exporter": "",  # 2.6 exports jmx and prometheus metrics by default
        "opa_authorizer": "0.1.0",  # only for HBase 2.6.1
        "delete_caches": "true",
    },
    {
        "product": "2.6.2",
        "hbase_thirdparty": "4.1.10",
        "hbase_operator_tools": "1.3.0-fd5a5fb",
        "java-base": "11",
        "java-devel": "11",
        "async_profiler": "2.9",
        "phoenix": "5.2.1",
        "hbase_profile": "2.6",
        "hadoop": "3.4.1",
        "jmx_exporter": "",  # 2.6 exports jmx and prometheus metrics by default
        "opa_authorizer": "0.1.0",  # only for HBase 2.6.1
        "delete_caches": "true",
    },
]
