versions = [
    # Also do not merge java-base with java below as "JAVA-BASE is not a valid identifier" in Dockerfiles, it's unfortunate but to fix this would require a bigger refactoring of names or the image tools
    # hbase-thirdparty is used to build the hbase-operator-tools and should be set to the version defined in the POM of HBase.
    {
        "product": "2.6.1",
        "hadoop/hadoop": "3.3.6",
        "java-base": "11",
        "java-devel": "11",
        "async_profiler": "2.9",
        "delete_caches": "true",
    },
    {
        "product": "2.6.2",
        "hadoop/hadoop": "3.4.1",
        "java-base": "11",
        "java-devel": "11",
        "async_profiler": "2.9",
        "delete_caches": "true",
    },
]
