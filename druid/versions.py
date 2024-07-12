versions = [
    {"product": "26.0.0", "java-base": "11", "java-devel": "11", "authorizer": "0.5.0"},
    {
        "product": "28.0.1",
        # Java 17 should be fully supported as of 27.0.0 https://github.com/apache/druid/releases#27.0.0-highlights-java-17-support
        # Did not work in a quick test due to reflection error:
        # Caused by: java.lang.reflect.InaccessibleObjectException: Unable to make protected final java.lang.Class
        # java.lang.ClassLoader.defineClass(java.lang.String,byte[],int,int,java.security.ProtectionDomain) throws java.lang.ClassFormatError
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
