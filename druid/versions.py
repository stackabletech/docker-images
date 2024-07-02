versions = [
    {
        "product": "26.0.0",
        "java-base": "11",
        "java-devel": "11",
        "jackson_dataformat_xml": "2.10.5",
        "stax2_api": "4.2.1",
        "woodstox_core": "6.2.1",
        "authorizer": "0.5.0",
    },
    {
        "product": "28.0.1",
        # Java 17 should be fully supported as of 27.0.0 https://github.com/apache/druid/releases#27.0.0-highlights-java-17-support
        # Did not work in a quick test due to reflection error:
        # Caused by: java.lang.reflect.InaccessibleObjectException: Unable to make protected final java.lang.Class
        # java.lang.ClassLoader.defineClass(java.lang.String,byte[],int,int,java.security.ProtectionDomain) throws java.lang.ClassFormatError
        "java-base": "11",
        "java-devel": "11",
        "jackson_dataformat_xml": "2.12.7",  # from https://github.com/apache/druid/blob/b8201e31aa6b124049a61764309145baaad78db7/pom.xml#L100
        "stax2_api": "4.2.2",
        "woodstox_core": "6.6.0",
        "authorizer": "0.5.0",
    },
]
