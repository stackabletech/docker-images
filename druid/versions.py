# Please also update versions.toml. It will eventually replace versions.py
# For more information, see: https://github.com/stackabletech/docker-images/issues/770

versions = [
    {
        "product": "26.0.0",
        "java-base": "11",
        "java-devel": "11",
        "authorizer": "0.5.0",
    },
    {
        "product": "30.0.0",
        # https://druid.apache.org/docs/30.0.0/operations/java/
        "java-base": "17",
        "java-devel": "17",
        "authorizer": "0.5.0",
    },
]
