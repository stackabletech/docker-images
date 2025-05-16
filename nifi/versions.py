versions = [
    {
        "product": "1.27.0",
        "java-base": "11",
        "java-devel": "11",  # There is an error when trying to use the jdk 21 (since nifi 1.26.0)
        "nifi_opa_authorizer_plugin": "0.1.0",
    },
    {
        "product": "1.28.1",
        "java-base": "11",
        "java-devel": "11",
        "nifi_opa_authorizer_plugin": "0.1.0",
    },
    {
        "product": "2.4.0",
        "java-base": "21",
        "java-devel": "21",
        "nifi_iceberg_bundle": "0.0.4",
        "nifi_opa_authorizer_plugin": "0.1.0",
    },
]
