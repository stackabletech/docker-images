versions = [
    # Also do not merge java-base with java below as "JAVA-BASE is not a valid identifier" in Dockerfiles, it's unfortunate but to fix this would require a bigger refactoring of names or the image tools
    # hbase-thirdparty is used to build the hbase-operator-tools and should be set to the version defined in the POM of HBase.
    {
        "product": "2.6.1",
        "hbase/hbase-operator-tools": "1.3.0-fd5a5fb",
        "hbase/phoenix": "5.2.1",
        "hbase/hbase-opa-authorizer": "0.1.0",  # only for HBase 2.6.1
        "hadoop": "3.3.6",
        "java-base": "11",
        "java-devel": "11",
        "hbase_profile": "2.6",
        "async_profiler": "2.9",
        "delete_caches": "true",
    },
    {
        "product": "2.6.2",
        "hbase/hbase-operator-tools": "1.3.0-fd5a5fb",
        "hbase/phoenix": "5.2.1",
        "hbase/hbase-opa-authorizer": "0.1.0",  # only for HBase 2.6.1
        "hadoop": "3.4.1",
        "java-base": "11",
        "java-devel": "11",
        "hbase_profile": "2.6",
        "async_profiler": "2.9",
        "delete_caches": "true",
    },
]
