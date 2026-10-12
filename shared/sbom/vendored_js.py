#!/usr/bin/env python3
"""Generates a CycloneDX SBOM for third-party JavaScript that is checked into a product's source
tree as pre-built (usually minified) files.

Such files carry no package manifest and no lockfile, so cdxgen, syft and trivy are all blind to
them: the libraries are shipped in our images but appear in no SBOM. We found no tool that
identifies a minified bundle reliably. retire.js recognises only about half of the libraries we
ship and gets some versions wrong. So the components have to be recorded by hand, in a manifest
per product version:

    <product>/stackable/vendored-js/<version>.json

An entry may also declare the libraries that a bundle inlines, which is how a library that is
shipped without a file of its own still ends up in the SBOM. Trino's vendored vis bundle for
example inlines a copy of moment that nothing else would report.

A hand-written manifest goes stale the moment a product version is bumped, so every entry pins the
SHA-256 of the file it describes. Both commands fail on a changed file, on a file that is listed
nowhere and on an entry whose file has disappeared. Recording a version is therefore a one-time
cost per file, and the build tells us when to revisit it.

shared/sbom/identify_js.py helps with writing and updating a manifest.
"""

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from urllib.parse import unquote

# A purl always carries the name and the version, so they are not repeated in the manifest.
PURL = re.compile(r"^pkg:[^/]+/(?P<name>.+)@(?P<version>[^@?#]+)$")


def scan(manifest, source_root):
    """Every JavaScript file below the scanned directories, keyed by its path relative to the
    source root. The manifest uses those relative paths because they stay unambiguous even when a
    product has several scanned directories."""
    return {
        str(path.relative_to(source_root)): path
        for directory in manifest["scan-dirs"]
        for path in sorted((source_root / directory).rglob("*.js"))
    }


def verify(manifest, source_root):
    """Every disagreement between the manifest and the source tree that would make the generated SBOM wrong."""
    own = set(manifest.get("own", []))
    listed = {}
    problems = []

    for library in manifest["libraries"]:
        if library["file"] in listed or library["file"] in own:
            problems.append(
                f"DUPLICATE {library['file']}\n  Listed more than once in the manifest."
            )
        listed[library["file"]] = library

    found = scan(manifest, source_root)
    for file, path in found.items():
        library = listed.get(file)
        if library is None:
            if file not in own:
                problems.append(
                    f'UNLISTED  {file}\n  Add it to "libraries" with a purl if it is third-party,'
                    ' or to "own" if the product wrote it.'
                )
            continue

        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if library["sha256"] != actual:
            problems.append(
                f"CHANGED   {file}\n  manifest {library['sha256']}\n  actual   {actual}\n"
                "  The file was updated upstream, so re-check the version it records."
            )

    for file in listed:
        if file not in found:
            problems.append(
                f"GONE      {file}\n  Listed in the manifest but no longer in the source tree."
            )
    for file in sorted(own - found.keys()):
        problems.append(
            f'GONE      {file}\n  Listed in "own" but no longer in the source tree.'
        )

    return found, problems


def identity(entry):
    """A component's name and version, plus its purl if it has one. A purl is the preferred
    identity because that is what vulnerability scanners match on, but libraries that were never
    published to a package registry cannot have one and are recorded by name only. The "note"
    field of such an entry says why."""
    purl = entry.get("purl")
    if not purl:
        if not entry.get("name"):
            raise SystemExit(f"Entry without a purl and without a name: {entry}")
        return None, entry["name"], entry.get("version")

    match = PURL.match(purl)
    if not match:
        raise SystemExit(f"Cannot parse the purl {purl}")
    return purl, unquote(match["name"]), unquote(match["version"])


def build_bom(manifest, component_version, spec_version):
    components = {}

    def add(entry, location, sha256):
        purl, name, version = identity(entry)
        # Some libraries carry no version anywhere, so the name alone identifies them.
        key = purl or (f"{name}@{version}" if version else name)
        if key not in components:
            component = {"type": "library", "name": name}
            if version:
                component["version"] = version
            component["bom-ref"] = key
            if purl:
                component["purl"] = purl
            if entry.get("license"):
                component["licenses"] = [{"expression": entry["license"]}]
            component["evidence"] = {"occurrences": []}
            components[key] = component

        component = components[key]
        if sha256:
            component.setdefault("hashes", []).append(
                {"alg": "SHA-256", "content": sha256}
            )
        component["evidence"]["occurrences"].append({"location": location})

    for library in manifest["libraries"]:
        # A library can be shipped as several files, for example a minified and a plain build, so
        # the files are collapsed into one component that records each of them as evidence.
        add(library, library["file"], library["sha256"])
        # Bundles inline their own dependencies, which are shipped without a file of their own.
        # Their hash would be the hash of the bundle, so it is deliberately not recorded.
        for bundled in library.get("bundles", []):
            add(bundled, library["file"], None)

    # No timestamp and no serial number, so that repeated runs produce the same file.
    return {
        "bomFormat": "CycloneDX",
        "specVersion": spec_version,
        "version": 1,
        "metadata": {
            "component": {
                "type": "application",
                "bom-ref": f"{manifest['name']}@{component_version}",
                "name": manifest["name"],
                "version": component_version,
            },
            "tools": {
                "components": [
                    {
                        "type": "application",
                        "name": "vendored_js.py",
                        "group": "tech.stackable",
                    }
                ]
            },
        },
        "components": list(components.values()),
    }


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    commands = parser.add_subparsers(dest="command", required=True)

    check_command = commands.add_parser(
        "check", help="report every mismatch between the manifest and the source tree"
    )
    bom_command = commands.add_parser(
        "bom", help="write the SBOM the manifest describes"
    )
    for command in (check_command, bom_command):
        command.add_argument("manifest", type=Path)
        command.add_argument("source_root", type=Path)
    bom_command.add_argument("output", type=Path)
    bom_command.add_argument("component_version")
    bom_command.add_argument("spec_version")

    arguments = parser.parse_args()
    manifest = json.loads(arguments.manifest.read_text())

    # Never generate an SBOM that we know to be wrong, so this also gates "bom".
    found, problems = verify(manifest, arguments.source_root)
    if problems:
        print(
            f"{arguments.manifest} does not match the source tree ({len(problems)} problem(s)):\n",
            file=sys.stderr,
        )
        print("\n".join(problems), file=sys.stderr)
        raise SystemExit(1)

    if arguments.command == "check":
        print(f"{arguments.manifest}: {len(found)} JavaScript files, all accounted for")
        return

    bom = build_bom(manifest, arguments.component_version, arguments.spec_version)
    arguments.output.write_text(json.dumps(bom, indent=2) + "\n")
    print(f"Wrote {arguments.output} with {len(bom['components'])} components")


if __name__ == "__main__":
    main()
