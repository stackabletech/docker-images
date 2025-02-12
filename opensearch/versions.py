versions = [
    {
        "product": "2.18.0",
        # The performance analyzer works with JDK 17, but not with 21.
        # https://github.com/opensearch-project/performance-analyzer-rca/issues/545
        "java-base": "17",
        "java-devel": "17",
    },
]
