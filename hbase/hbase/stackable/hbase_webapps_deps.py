#!/usr/bin/env python3
"""Derives the JavaScript dependencies of the HBase web UI from the HBase poms and writes them out
as an npm package.json.

The web UI assets are not checked into the HBase source tree. They are unpacked from webjars by the
maven-dependency-plugin into hbase-webapps/static during the build, see the "unpack-ui-resources-js"
and "unpack-ui-resources-css" executions in hbase-server/pom.xml. Because those webjars are declared
as <artifactItem>s of a plugin and not as project dependencies, the CycloneDX Maven plugin does not
pick them up, so they are missing from the HBase SBOM.

The generated package.json is only an intermediate artifact: cdxgen turns it into the actual
CycloneDX SBOM. npm coordinates are used rather than the Maven ones, because vulnerability scanners
match advisories against pkg:npm and largely fail to match pkg:maven/org.webjars purls.
"""

import argparse
import json
import re
from pathlib import Path
from xml.etree import ElementTree

PROPERTY = re.compile(r"\$\{([\w.-]+)\}")


def tag(element):
    """The tag of an element without the Maven POM namespace."""
    # ElementTree keeps the namespace in the tag itself, so the root element of a pom is called
    # "{http://maven.apache.org/POM/4.0.0}project".
    return element.tag.rpartition("}")[2]


def properties(pom):
    """Every <properties> entry of a pom, for example <jquery.version>3.7.1</jquery.version>."""
    # The whole tree is walked because <properties> is not only a top-level element: HBase declares
    # most of its properties inside profiles.
    entries = {}
    for block in pom.iter():
        if tag(block) == "properties":
            for entry in block:
                entries[tag(entry)] = (entry.text or "").strip()
    return entries


def webjars(server_pom, versions):
    """The org.webjars artifacts that the maven-dependency-plugin unpacks, as npm dependencies.
    The webjar artifact IDs match their npm package names, so they can be used verbatim."""
    dependencies = {}
    for item in server_pom.iter():
        if tag(item) != "artifactItem":
            continue

        fields = {tag(field): (field.text or "").strip() for field in item}
        if fields.get("groupId") != "org.webjars":
            continue
        if not fields.get("artifactId") or not fields.get("version"):
            raise SystemExit(f"<artifactItem> without an artifactId or version: {fields}")

        version = PROPERTY.sub(lambda match: versions.get(match[1], match[0]), fields["version"])
        if "${" in version:
            raise SystemExit(f"Cannot resolve the version of {fields['artifactId']} from the root pom: {version}")

        # bootstrap is unpacked twice, once for its JavaScript and once for its CSS.
        dependencies[fields["artifactId"]] = version

    # Guard against upstream restructuring the pom, which would otherwise silently produce an SBOM
    # without any components.
    if not dependencies:
        raise SystemExit("No org.webjars <artifactItem> found in hbase-server/pom.xml, did the pom layout change?")
    return dependencies


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("source_root", type=Path, help="the HBase source tree")
    parser.add_argument("version", help="the HBase version, used as the version of the generated package")
    parser.add_argument("output", type=Path, help="the package.json to write")
    arguments = parser.parse_args()

    dependencies = webjars(
        ElementTree.parse(arguments.source_root / "hbase-server/pom.xml"),
        properties(ElementTree.parse(arguments.source_root / "pom.xml")),
    )
    package = {"name": "hbase-webapps", "version": arguments.version, "private": True, "dependencies": dependencies}
    arguments.output.write_text(json.dumps(package, indent=2) + "\n")

    summary = ", ".join(f"{name}@{version}" for name, version in dependencies.items())
    print(f"Wrote {arguments.output}: {summary}")


if __name__ == "__main__":
    main()
