# Vendored JavaScript SBOMs

Some products check pre-built, usually minified JavaScript into their source tree
(`webapp/vendor/...`). Those files carry no package manifest and no lockfile, so cdxgen, syft and
trivy do not see them. The libraries ship in our images and appear in no SBOM.

The fix is a hand-written manifest per product version:

    <product>/stackable/vendored-js/<version>.json

The build turns it into a CycloneDX SBOM and fails if the source tree and the manifest disagree.

## The two scripts

| Script | Role |
| --- | --- |
| `vendored_js.py` | Used by the build. `check` reports every disagreement, `bom` writes the SBOM. |
| `identify_js.py` | Authoring aid, never run by the build. `inspect` lists files with hashes and version hints, `identify` proves which npm release a file came from. |

Both have a long docstring at the top; read it before doing anything unusual.

## Manifest format

```jsonc
{
  "name": "trino-web-ui-vendor",        // root component name of the generated SBOM
  "scan-dirs": [                        // searched recursively for *.js, paths relative to the source root
    "core/trino-web-ui/src/main/resources/webapp-legacy/vendor"
  ],
  "libraries": [
    {
      "file": "...//vendor/jquery/jquery-3.7.1.js",   // relative to the source root
      "purl": "pkg:npm/jquery@3.7.1",                 // null if the library was never published
      "license": "MIT",
      "sha256": "5e6769f1...",                        // pins the exact file this entry describes
      "note": "Why this version, and how the file differs from the release.",
      "bundles": [                                    // libraries inlined in this file, no file of their own
        { "purl": "pkg:npm/moment@2.29.4", "license": "MIT" }
      ]
    }
  ],
  "own": [                              // product-written JavaScript, not third-party
    "...//assets/js/login.js"
  ]
}
```

Every `.js` file under `scan-dirs` must be in `libraries` or in `own`. A library without a purl
needs `name` and `version` instead, and a `note` saying why there is no purl.

## Adding a version (the common case: a version bump)

Most bumps do not touch the vendored files, so start from the previous version and let `check`
tell you what actually changed.

1. Check out the patched source tree:

   ```bash
   cargo run --bin patchable -- checkout <product>/<image> <version>
   ```

   It prints the worktree path, e.g. `trino/trino/patchable-work/worktree/483`.

2. Copy the previous manifest:

   ```bash
   cp <product>/<image>/stackable/vendored-js/{481,483}.json
   ```

3. Run `check` against the new source tree:

   ```bash
   python3 shared/sbom/vendored_js.py check \
     <product>/<image>/stackable/vendored-js/483.json \
     <product>/<image>/patchable-work/worktree/483
   ```

4. Resolve what it reports, then re-run until it is clean:

   - `GONE` plus `UNLISTED` for the same file under a different path: upstream moved the
     directory. Update `scan-dirs` and the `file` paths. Trino 483 moved the whole legacy UI from
     `webapp/` to `webapp-legacy/`, and nothing else changed.
   - `UNLISTED` for a genuinely new file: identify it (see below) and add an entry, or add it to
     `own` if the product wrote it.
   - `GONE` for a file that is really gone: drop the entry.
   - `CHANGED`: the file was updated upstream. Re-identify it, update `sha256`, `purl` and `note`.
     Do not just update the hash - the point of the pin is that this gets looked at.

5. Commit the manifest. The Dockerfile picks it up by `${PRODUCT_VERSION}`, so nothing else needs
   touching unless the scanned paths moved.

## Identifying a file

Never trust the file name. Hadoop ships d3 4.1.0 as `d3-v4.1.1.min.js`.

```bash
# What is in the tree, with hashes and any version string near the top
python3 shared/sbom/identify_js.py inspect <worktree> <scan-dir> [<scan-dir>...]

# Which npm release is this file, really
python3 shared/sbom/identify_js.py identify <worktree>/<path>/foo.min.js <npm-package> [--prefix 4.]
```

`identify` downloads every non-prerelease version of the package and hashes every file in it,
comparing four ways: identical, whitespace-normalised, banner/source-map stripped, and
whitespace-collapsed. All four mean the same code, so the same advisories apply. The entry's
`note` should say which kind of match it was and what differs.

When several releases match, record the lowest one and note the ambiguity - that keeps the widest
set of advisories applicable. Evidence outside the code (a jsDelivr URL in the banner, for
example) beats that rule.

When nothing matches, record the version the file states and say in the `note` that it could not
be confirmed against a release.

## Why there is no generator

The mechanical half of a bump is `cp` plus `check`, and `check` already prints exactly what is
wrong and where. The other half - deciding which release a minified blob is - is evidence work
that a script cannot do for you. A generator would only hide that.
