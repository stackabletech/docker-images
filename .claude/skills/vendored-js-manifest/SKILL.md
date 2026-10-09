---
name: vendored-js-manifest
description: Create or update a vendored JavaScript SBOM manifest (<product>/stackable/vendored-js/<version>.json). Use when adding a new product version that has such a directory, or when a build fails with UNLISTED / CHANGED / GONE from vendored_js.py.
---

# Vendored JavaScript manifests

Full reference: `shared/sbom/README.md`. Read it before deviating from the steps below.

Products with a `vendored-js/` directory: trino, hadoop, spark-k8s. hbase generates its list from
webjars instead (`hbase/hbase/stackable/hbase_webapps_deps.py`) and is not covered here.

## Adding a version

1. Check out the patched source and note the worktree path:

   ```bash
   cargo run --bin patchable -- checkout <product>/<image> <version>
   ```

2. Copy the manifest of the previous version to `<version>.json`.

3. Run `check` and resolve everything it reports:

   ```bash
   python3 shared/sbom/vendored_js.py check \
     <product>/<image>/stackable/vendored-js/<version>.json <worktree>
   ```

   - `GONE` + `UNLISTED` for the same file at a different path -> upstream moved the directory.
     Remap `scan-dirs` and every `file`.
   - `UNLISTED` for a new file -> identify it, then add an entry, or add it to `own` if Trino,
     Hadoop or Spark wrote it themselves.
   - `GONE` alone -> drop the entry.
   - `CHANGED` -> re-identify the file. Update `sha256`, `purl`/`version` and `note`.

4. Re-run until clean. Clean output is `<manifest>: N JavaScript files, all accounted for`.

## Do not

- Do not bump a `sha256` without re-identifying the file. The pin exists precisely so a changed
  file gets looked at; silently re-pinning it defeats the whole mechanism.
- Do not guess a version from the file name. Hadoop ships d3 4.1.0 as `d3-v4.1.1.min.js`.
- Do not invent a purl for a library that was never published to npm. Use `purl: null` plus
  `name`, `version` and a `note` saying why.

## Identifying a file

```bash
python3 shared/sbom/identify_js.py inspect <worktree> <scan-dir>...
python3 shared/sbom/identify_js.py identify <worktree>/<file> <npm-package> [--prefix 4.]
```

`identify` needs network access to registry.npmjs.org and caches tarballs. It compares four ways
(identical / whitespace / banner stripped / whitespace collapsed); all four count as a match and
the `note` should say which it was. Several matching releases -> record the lowest and note the
ambiguity. No match -> record the version the file states and say it is unconfirmed.

## Also check

When the product version moved the web UI directories, the Dockerfile's cdxgen section for the
npm-based frontends usually needs the same treatment. The manifest is only one of the web UI
SBOMs.
