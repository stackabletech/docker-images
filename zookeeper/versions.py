# Please also update versions.toml. It will eventually replace versions.py
# For more information, see: https://github.com/stackabletech/docker-images/issues/770

versions = [
    {
        "product": "3.9.2",
        "java-base": "17",
        # NOTE (@NickLarsenNZ): Builds fail on Java 17, with the output:
        # [ERROR] Failed to execute goal com.github.spotbugs:spotbugs-maven-plugin:4.0.0:spotbugs (spotbugs) on project
        # zookeeper: Execution spotbugs of goal com.github.spotbugs:spotbugs-maven-plugin:4.0.0:spotbugs failed: Java
        # returned: 1 -> [Help 1]
        "java-devel": "11",
        "jmx_exporter": "1.0.1-stackable",
    },
]
