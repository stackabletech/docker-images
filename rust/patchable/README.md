# Patchable

Patchable is a tool for managing patches for the third-party products distributed by Stackable as part of the Stackable Data Platform.

Patchable works by keeping a series of .patch files (in `docker-images/<PRODUCT>/stackable/patches/<VERSION>`)
as its source of truth, but using temporary Git repositories as an interface to modify those patches.
This lets us track the history of patches over time, but reuse the existing familiarity and tooling around Git.

Patchable designates a commit as the upstream _base_ for each version, and considers each commit made on top of that
to be an individual patch.

## Usage

```sh
pushd $(cargo patchable checkout druid 26.0.0)
# do stuff
git commit
popd
cargo patchable export druid 26.0.0
git status
```

For more details, run `cargo patchable --help`.

## Notes

- patchable only supports linear patch series (no merges beyond the base commit)
- patchable doesn't support merging "materialized" trees, merge the .patch files instead, and `checkout`/`export` to update the hashes
- `patchable checkout` doesn't support resolving patch conflicts, use `git am` instead (and then `patchable export` the resolved patches)
- Always run patchable via `cargo patchable` (rather than `cargo install`ing it), to ensure that you use the correct version for a given checkout of docker-images

## Configuration

Patchable uses a two-level configuration system:

1. A product-level config file at `docker-images/<PRODUCT>/stackable/patches/patchable.toml`
2. A version-level config file at `docker-images/<PRODUCT>/stackable/patches/<VERSION>/patchable.toml`

The product-level config contains:

- `upstream` - the URL of the upstream repository (such as `https://github.com/apache/druid.git`)
- `default_mirror` - optional: default URL of a mirror repository (such as `https://github.com/stackabletech/druid.git`)

The version-level config contains:

- `base` - the commit hash of the upstream base commit
- `mirror` - optional: URL of the mirror repository for this version, if mirroring is enabled

### Template

If you're adding a completely new product, you need to initialize the product-level config once using patchable:

```sh
cargo patchable init product druid --upstream https://github.com/apache/druid.git --default-mirror https://github.com/stackabletech/druid.git
```

This will create the product-level configuration in `docker-images/druid/stackable/patches/patchable.toml`.

If you just want to add a new version, initialize the version-level config with patchable:

```sh
cargo patchable init version druid 28.0.0 --base druid-28.0.0 --mirror
```

This will initialize the version-level config in `docker-images/druid/stackable/patches/28.0.0/patchable.toml` with the base commit hash and the default mirror URL from the product-level config.
You can optionally provide the `--ssh` flag to use SSH instead of HTTPS for Git operations.

## Glossary

- Images repo/directory - The checkout of stackabletech/docker-images
