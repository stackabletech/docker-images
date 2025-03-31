versions = [
    {
        "product": "2.19.1",
        # The performance analyzer works with JDK 17, but not with 21.
        # https://github.com/opensearch-project/performance-analyzer-rca/issues/545
        "java-devel": "17",
        "jdk-base": "17",
    },
]
