# Trino vendored JavaScript manifests

One `<version>.json` per Trino version, listing the pre-built JavaScript that Trino checks into
its source tree. `shared/sbom/vendored_js.py` turns it into a CycloneDX SBOM during the build and
fails the build when the manifest and the source tree disagree.

**How to write or update one: [`shared/sbom/README.md`](../../../../shared/sbom/README.md).**

## Trino specifics

The scanned directories are the legacy web UI's `vendor` and `assets`. Trino 483 renamed the web
UI directories: the preview UI became `webapp` and the old UI moved to `webapp-legacy`. So

- up to 481: `core/trino-web-ui/src/main/resources/webapp/{vendor,assets}`
- from 483: `core/trino-web-ui/src/main/resources/webapp-legacy/{vendor,assets}`

The vendored files themselves were unchanged across 477 to 483 - 483.json is 481.json with the
paths remapped.

The new `webapp` is an npm project and is covered by cdxgen in the Dockerfile, not here. Note
that both frontends switched from `package-lock.json` to `bun.lock` in 483, which the cdxgen
section of `trino/trino/Dockerfile` has to account for.
