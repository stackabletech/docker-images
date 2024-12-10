# Please also update versions.toml. It will eventually replace versions.py
# For more information, see: https://github.com/stackabletech/docker-images/issues/770

versions = [
    {
        "product": "1.27.0",
        "java-base": "11",
        "java-devel": "11",  # There is an error when trying to use the jdk 21 (since nifi 1.26.0)
    },
    {
        "product": "2.0.0",
        "java-base": "21",
        "java-devel": "21",
    },
]
