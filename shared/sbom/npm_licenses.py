#!/usr/bin/env python3
"""Fills in the licenses that cdxgen leaves empty for a Yarn 1 project, by reading them from the
installed node_modules tree.

cdxgen builds the component list of a Yarn project from yarn.lock, and the Yarn 1 lockfile format
records no license field. Its only other source is the npm registry, behind FETCH_LICENSE, which
means one network request per package at build time. For OpenSearch Dashboards that is more than
4000 requests, and it makes the build depend on the availability of a registry it otherwise never
talks to.

The licenses are already on disk, though: `yarn install` writes the full package.json of every
dependency into node_modules, and npm requires its `license` field to hold an SPDX expression.
Reading them from there needs no network, and it reports what the package itself declares rather
than what a registry says about it today.

Only components without a license are touched. cdxgen does resolve a few from the workspace
package.json files and those stay as they are.

The SPDX identifier list comes from cdxgen's own data directory, so that this script and cdxgen
agree on what counts as a valid identifier without a second copy of the list going stale here.
"""

import argparse
import json
import os
import re
from pathlib import Path
from urllib.parse import unquote

PURL = re.compile(r"^pkg:npm/(?P<name>.+)@(?P<version>[^@?#]+)$")

# Everything in an SPDX expression that is not a license identifier.
EXPRESSION_OPERATORS = {"AND", "OR", "WITH"}


def declared_licenses(package_json):
    """The license strings a package.json declares.

    npm defines a single `license` holding an SPDX expression. Two forms from before that are
    still in the wild: `license` as an object with a `type`, and `licenses` as a list of such
    objects for a package under more than one license.
    """
    license = package_json.get("license")
    if isinstance(license, str):
        return [license]
    if isinstance(license, dict) and isinstance(license.get("type"), str):
        return [license["type"]]

    licenses = package_json.get("licenses")
    if isinstance(licenses, dict):
        licenses = [licenses]
    if isinstance(licenses, list):
        return [
            entry["type"]
            for entry in licenses
            if isinstance(entry, dict) and isinstance(entry.get("type"), str)
        ]
    return []


def installed_packages(source_root):
    """Every installed package below the source root, keyed by name and version.

    A package directory sits directly in a node_modules directory, or one level deeper below an
    @scope directory. Anything else that contains a package.json is a subdirectory of a package,
    not a package of its own. Symlinks are not followed, which keeps the workspace links Yarn
    creates from being walked twice.
    """
    packages = {}
    for directory, subdirectories, _ in os.walk(source_root):
        if os.path.basename(directory) != "node_modules":
            continue
        candidates = []
        for entry in subdirectories:
            if entry.startswith("@"):
                scope = Path(directory) / entry
                candidates.extend(sorted(scope.iterdir()))
            else:
                candidates.append(Path(directory) / entry)

        for candidate in candidates:
            manifest = candidate / "package.json"
            if not manifest.is_file():
                continue
            try:
                package_json = json.loads(manifest.read_text(encoding="utf-8"))
            except (json.JSONDecodeError, UnicodeDecodeError, OSError):
                continue
            if not isinstance(package_json, dict):
                continue
            name, version = package_json.get("name"), package_json.get("version")
            if not isinstance(name, str) or not isinstance(version, str):
                continue
            licenses = declared_licenses(package_json)
            # The same package and version is installed many times over in a Yarn 1 tree. The
            # copies declare the same license, so the first one that declares anything wins.
            if licenses and (name, version) not in packages:
                packages[(name, version)] = licenses
    return packages


def license_entry(value, spdx_identifiers):
    """One CycloneDX `licenses` entry for a declared license string.

    CycloneDX validates `license.id` against the SPDX identifier list, so a value that is not on
    it must not go there or the document stops validating. A compound expression goes into
    `expression`, and anything else, `UNLICENSED` and `SEE LICENSE IN <file>` above all, is kept
    verbatim as a license name.
    """
    value = value.strip()
    if value in spdx_identifiers:
        return {"license": {"id": value}}

    tokens = [token for token in re.split(r"[()\s]+", value) if token]
    if len(tokens) > 1 and all(
        token in EXPRESSION_OPERATORS or token.rstrip("+") in spdx_identifiers
        for token in tokens
    ):
        return {"expression": value}

    return {"license": {"name": value}}


def identify(component):
    """The name and version a component is known by in node_modules.

    The purl is the authoritative form, because it carries the scope in the name, while `group`
    and `name` are split differently depending on which cdxgen code path produced the component.
    """
    purl = component.get("purl")
    if isinstance(purl, str):
        match = PURL.match(purl)
        if match:
            return unquote(match["name"]), unquote(match["version"])

    name, version = component.get("name"), component.get("version")
    if not isinstance(name, str) or not isinstance(version, str):
        return None, None
    group = component.get("group")
    return (f"{group}/{name}" if group else name), version


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("bom", type=Path, help="the cdxgen SBOM, rewritten in place")
    parser.add_argument(
        "spdx_licenses",
        type=Path,
        help="cdxgen's data/spdx-licenses.json, a JSON array of SPDX identifiers",
    )
    parser.add_argument(
        "source_root", type=Path, help="the directory the node_modules trees live under"
    )
    arguments = parser.parse_args()

    spdx_identifiers = set(json.loads(arguments.spdx_licenses.read_text()))
    packages = installed_packages(arguments.source_root)
    bom = json.loads(arguments.bom.read_text())

    components = bom.get("components", [])
    without_license = 0
    filled = 0
    for component in components:
        if component.get("licenses"):
            continue
        without_license += 1
        name, version = identify(component)
        declared = packages.get((name, version))
        if not declared:
            continue
        component["licenses"] = [
            license_entry(value, spdx_identifiers) for value in declared
        ]
        filled += 1

    arguments.bom.write_text(json.dumps(bom, indent=2) + "\n")
    print(
        f"{arguments.bom}: {len(packages)} packages installed, "
        f"{filled} of {without_license} components without a license filled in, "
        f"{without_license - filled} still without one"
    )


if __name__ == "__main__":
    main()
