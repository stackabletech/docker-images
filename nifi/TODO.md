* [ ] Release nifi-opa-plugin 0.4.0 and use that
* [x] Add patch 0006-replace-process-groups-root-with-root-ID.patch (it's complicated as NiFi changed things)
* [x] Use logback 1.15.24
* [ ] Test out the `0006-Add-hadoop-commons-library-needed-for-nifi-iceberg-h.patch`:

```diff
From 8b12f915ae28e35b3d2a54ab37af07a8f255376f Mon Sep 17 00:00:00 2001
From: Sebastian Bernauer <sebastian.bernauer@stackable.tech>
Date: Tue, 27 Jan 2026 08:28:21 +0100
Subject: Add hadoop-commons library needed for nifi-iceberg-hive-bundle

---
 .../nifi-standard-shared-bom/pom.xml          | 27 +++++++++++++++++++
 1 file changed, 27 insertions(+)

diff --git a/nifi-extension-bundles/nifi-standard-shared-bundle/nifi-standard-shared-bom/pom.xml b/nifi-extension-bundles/nifi-standard-shared-bundle/nifi-standard-shared-bom/pom.xml
index 09a37b4954..a4b6fc382d 100644
--- a/nifi-extension-bundles/nifi-standard-shared-bundle/nifi-standard-shared-bom/pom.xml
+++ b/nifi-extension-bundles/nifi-standard-shared-bundle/nifi-standard-shared-bom/pom.xml
@@ -28,6 +28,14 @@
         copies of these commonly used Jars</description>
     <dependencyManagement>
         <dependencies>
+            <!-- Stackable specific! -->
+            <!-- We need the hadoop-commons, so we can use that in our nifi-iceberg-hive-bundle -->
+            <dependency>
+                <groupId>org.apache.hadoop</groupId>
+                <artifactId>hadoop-common</artifactId>
+                <version>${hadoop.version}</version>
+                <scope>provided</scope>
+            </dependency>
             <!-- Apache Commons -->
             <dependency>
                 <groupId>commons-codec</groupId>
@@ -256,6 +264,24 @@
         </dependencies>
     </dependencyManagement>
     <dependencies>
+        <dependency>
+            <groupId>org.apache.hadoop</groupId>
+            <artifactId>hadoop-common</artifactId>
+            <exclusions>
+                <exclusion>
+                    <groupId>org.slf4j</groupId>
+                    <artifactId>slf4j-reload4j</artifactId>
+                </exclusion>
+                <exclusion>
+                    <groupId>commons-logging</groupId>
+                    <artifactId>commons-logging</artifactId>
+                </exclusion>
+                <exclusion>
+                    <groupId>com.nimbusds</groupId>
+                    <artifactId>nimbus-jose-jwt</artifactId>
+                </exclusion>
+            </exclusions>
+        </dependency>
         <dependency>
             <groupId>commons-codec</groupId>
             <artifactId>commons-codec</artifactId>
@@ -388,6 +414,7 @@
                 <artifactId>maven-dependency-plugin</artifactId>
                 <configuration>
                     <ignoredDependencies combine.children="append">
+                        <dependency>org.apache.hadoop:hadoop-common</dependency>
                         <dependency>commons-codec:commons-codec</dependency>
                         <dependency>org.apache.commons:commons-compress</dependency>
                         <dependency>com.github.luben:zstd-jni</dependency>
```
