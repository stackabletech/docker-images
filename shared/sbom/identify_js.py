#!/usr/bin/env python3
"""Authoring aid for the manifests that shared/sbom/vendored_js.py consumes. Not used by any build.

Writing such a manifest means naming a library and a version for a pre-built, usually minified
JavaScript file. The file name is not evidence: Hadoop ships d3 4.1.0 as "d3-v4.1.1.min.js" and
mustache.js as "jquery.mustache.js". Neither is a header comment always present.

This tool provides the two things that turn writing a manifest into reading off facts:

    inspect   lists every .js file under the given directories with its SHA-256 and any version
              string found near the top of the file, which is the starting point for a new
              manifest and for seeing what a version bump changed.

    identify  downloads every published version of the given npm packages and hashes every file in
              them, looking for one that is byte-identical to ours. A match is proof, and it is
              what established that Hadoop's "d3-v4.1.1.min.js" really is d3 4.1.0.

A file is compared in four ways, because vendoring rarely copies a release byte for byte:

    identical   the file as it is. Proof.
    whitespace  the file with its line endings normalised, its trailing spaces removed and its
                surrounding whitespace stripped. Copying through an editor or a shell redirection
                commonly appends a trailing newline or strips trailing spaces. Trino's
                clipboard.min.js is the published clipboard 2.0.11 plus one newline, and Hadoop's
                d3.v3.js is the published d3 3.2.7 with one trailing space gone.
    comments    the same, and additionally without the comment banner at the top of the file and
                without the source-map link at the bottom. Vendoring often drops or rewrites the
                copyright header, or adds one where the release has none, which is the other half
                of what makes d3.v3.js look modified. Dropping the source-map link is just as
                common, because the .map file is not vendored along with it.
    formatting  the same, and with every run of whitespace collapsed, which makes the comparison
                blind to how the file is indented. Trino's jquery-3.7.1.js is the published release
                re-indented with spaces instead of tabs. This is the weakest of the four: a
                difference inside a multi-line string literal would be collapsed away with it, so
                it says the code is the same up to formatting rather than proving it byte for byte.

All four mean the same code, so the same advisories apply, and all four count as a match. The
manifest entry says which kind it was. Only the banner and the source-map link are ignored, never
a comment inside the code, so two releases that differ in the code itself can still be told apart.

When several releases match, the file was shipped unchanged across them and hashing cannot tell
them apart. Record the lowest one: it is the earliest release the code appeared in, and it keeps
the widest set of advisories applicable, which is the safe direction. Note the ambiguity in the
manifest entry. Evidence outside the code beats that rule, because it names one release instead of
a range: Spark's d3-flamegraph.min.js is identical in 4.1.2 and 4.1.3, and its banner is the
jsDelivr URL it was downloaded from, so 4.1.3 is recorded.

When nothing matches, the library either predates its npm releases or the product modified or
rebuilt it. Fall back to the version the file states and say so in the entry.

Examples:
    identify_js.py inspect . hadoop-hdfs-project/hadoop-hdfs/src/main/webapps
    identify_js.py identify webapps/static/d3-v4.1.1.min.js d3 --prefix 4.

Tarballs are cached, override the location with IDENTIFY_JS_CACHE.
"""

import argparse
import hashlib
import json
import os
import re
import tarfile
import tempfile
from pathlib import Path
from urllib.error import URLError
from urllib.request import urlopen

CACHE = Path(
    os.environ.get(
        "IDENTIFY_JS_CACHE", Path(tempfile.gettempdir()) / "stackable-identify-js-cache"
    )
)

# Version strings are written in every conceivable way, so cast a wide net over the top of the file
# and let the caller judge. Matching on bytes keeps minified files with odd encodings readable.
HINTS = [
    re.compile(rb"@version\s+v?([0-9]+\.[0-9][\w.-]*)"),
    re.compile(rb"\bversion\s*[:=]\s*['\"]?v?([0-9]+\.[0-9][\w.-]*)", re.IGNORECASE),
    # Banners such as "// https://d3js.org Version 4.1.0." separate with a space. All three
    # components are required here, otherwise every "Apache License, Version 2.0" header matches.
    re.compile(rb"\bversion\s+v?([0-9]+\.[0-9]+\.[0-9][\w.-]*)", re.IGNORECASE),
    re.compile(
        rb"^/\*!?\s*([A-Za-z][\w.\- ]*?)\s+v?([0-9]+\.[0-9][\w.-]*)", re.MULTILINE
    ),
    re.compile(rb"\bv([0-9]+\.[0-9]+\.[0-9][\w.-]*)"),
]


# The digest of an empty file, which is what a file that holds nothing but a comment strips down to.
EMPTY = hashlib.sha256(b"").hexdigest()


def sha256(contents):
    return hashlib.sha256(contents).hexdigest()


def normalize_whitespace(contents):
    """The file with its line endings normalised, its trailing spaces removed and its surrounding
    whitespace stripped. None of that changes what the code does, and all of it happens to a file
    on the way into a source tree."""
    lines = contents.replace(b"\r\n", b"\n").split(b"\n")
    return b"\n".join(line.rstrip() for line in lines).strip()


def strip_comments(contents):
    """The file without the comment banner at the top of it, without the source-map link at the
    bottom, and without its surrounding whitespace. Only comments outside the code are removed, so
    the code itself is compared in full."""
    rest = contents.strip()
    if rest.rsplit(b"\n", 1)[-1].startswith(
        (b"//# sourceMappingURL=", b"//@ sourceMappingURL=")
    ):
        rest = rest.rsplit(b"\n", 1)[0].strip()
    while True:
        if rest.startswith(b"/*"):
            end = rest.find(b"*/")
            if end < 0:
                break
            rest = rest[end + 2 :].strip()
        elif rest.startswith(b"//"):
            end = rest.find(b"\n")
            if end < 0:
                break
            rest = rest[end + 1 :].strip()
        else:
            break
    return rest


def digests(contents):
    """The SHA-256 of the file and of its three normalised forms, keyed by the kind of match each
    one is evidence for. Comparing only the first would report a copy that merely lost a trailing
    space or a copyright header as a modified library."""
    return {
        "identical": sha256(contents),
        "whitespace": sha256(normalize_whitespace(contents)),
        "comments": sha256(normalize_whitespace(strip_comments(contents))),
        "formatting": sha256(re.sub(rb"\s+", b" ", strip_comments(contents)).strip()),
    }


def version_hints(contents):
    hints = []
    for pattern in HINTS:
        for match in pattern.finditer(contents[:3000]):
            # Rstrip because a version at the end of a sentence swallows the full stop.
            hint = (
                b" ".join(group for group in match.groups() if group)
                .decode("latin1")
                .rstrip(".-")
            )
            if hint not in hints:
                hints.append(hint)
    return hints


def published_versions(package, prefix):
    """Every non-prerelease version of a package, ascending. The registry lists them in publication
    order, which is not always ascending, and the advice to record the lowest match depends on the
    order being right."""
    # The scope separator has to stay encoded, otherwise the registry sees two path segments.
    with urlopen(
        f"https://registry.npmjs.org/{package.replace('/', '%2f')}"
    ) as response:
        metadata = json.load(response)

    releases = [
        (version, release["dist"]["tarball"])
        for version, release in metadata.get("versions", {}).items()
        # A prerelease is never what a product vendored.
        if "-" not in version
        and version.startswith(prefix)
        and release.get("dist", {}).get("tarball")
    ]
    return sorted(
        releases,
        key=lambda release: [
            int(part) if part.isdigit() else 0 for part in release[0].split(".")
        ],
    )


def tarball_hashes(package, version, url):
    """The digests of every file in a release, keyed by its path inside the package."""
    archive_path = CACHE / f"{package.replace('/', '_')}-{version}.tgz"
    if not archive_path.exists():
        CACHE.mkdir(parents=True, exist_ok=True)
        with urlopen(url) as response:
            archive_path.write_bytes(response.read())

    try:
        with tarfile.open(archive_path) as archive:
            # Every member is below a "package/" directory that is of no interest here.
            return {
                member.name.split("/", 1)[-1]: digests(
                    archive.extractfile(member).read()
                )
                for member in archive
                if member.isfile()
            }
    except tarfile.TarError:
        # A handful of very old releases have broken tarballs, skip them.
        return {}


def inspect(source_root, directories):
    for directory in directories:
        for path in sorted((source_root / directory).rglob("*.js")):
            contents = path.read_bytes()
            hints = " | ".join(version_hints(contents)[:3]) or "-"
            print(
                "\t".join([str(path.relative_to(source_root)), sha256(contents), hints])
            )


# What a match of each kind means, and how the manifest entry should describe it. Ordered from the
# strongest evidence to the weakest, and that is also the order they are reported in.
KINDS = {
    "identical": "byte-identical",
    "whitespace": "differs only in whitespace",
    "comments": "differs only in whitespace and in the banner or the source-map link",
    "formatting": "differs only in formatting, the banner or the source-map link",
}


def match_kind(wanted, published):
    """How a published file matches ours, or None if it does not. A file that is nothing but a
    comment is empty once the banner is gone, which would match anything, so that is not a match."""
    for kind in KINDS:
        if wanted[kind] == published[kind] and published[kind] != EMPTY:
            return kind
    return None


def identify(target, packages, prefix):
    wanted = digests(target.read_bytes())
    print(f"{target}\n  sha256 {wanted['identical']}\n")

    matches = []
    for package in packages:
        try:
            releases = published_versions(package, prefix)
        except URLError as error:
            print(f"{package}: {error}")
            continue
        print(f"{package}: checking {len(releases)} version(s)")
        for version, url in releases:
            for name, published in tarball_hashes(package, version, url).items():
                kind = match_kind(wanted, published)
                if kind:
                    matches.append((package, version, kind))
                    print(f"  MATCH {package}@{version}  {name}  ({KINDS[kind]})")

    if not matches:
        print(
            "\nNo match. Fall back to the version the file states and note that in the manifest."
        )
        return

    kinds = {kind for _, _, kind in matches}
    if kinds != {"identical"}:
        print(
            "\nThe code is identical, so the release applies. Record it and say in the manifest"
        )
        print(
            f"what differs: {', '.join(KINDS[kind] for kind in KINDS if kind in kinds)}."
        )

    if len(matches) > 1:
        package, version, _ = matches[0]
        print(
            f"\nThe file is the same in {len(matches)} releases, so record the lowest,"
        )
        print(f"{package}@{version}, and note the ambiguity in the manifest.")


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    commands = parser.add_subparsers(dest="command", required=True)

    inspect_command = commands.add_parser(
        "inspect", help="list the JavaScript files below the given directories"
    )
    inspect_command.add_argument("source_root", type=Path)
    inspect_command.add_argument("directories", nargs="+")

    identify_command = commands.add_parser(
        "identify", help="find the npm release a file came from"
    )
    identify_command.add_argument("file", type=Path)
    identify_command.add_argument(
        "packages", nargs="+", help="npm packages the file might come from"
    )
    identify_command.add_argument(
        "--prefix", default="", help='only check versions starting with this, e.g. "4."'
    )

    arguments = parser.parse_args()
    if arguments.command == "inspect":
        inspect(arguments.source_root, arguments.directories)
    else:
        identify(arguments.file, arguments.packages, arguments.prefix)


if __name__ == "__main__":
    main()
