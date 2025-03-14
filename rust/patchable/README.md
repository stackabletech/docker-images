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

Patchable stores a per-version file in `docker-images/<PRODUCT>/stackable/patches/<VERSION>/patchable.toml`.
It currently recognizes the following keys:

- `upstream` - the URL of the upstream repository (such as `https://github.com/apache/druid.git`)
- `base` - the commit hash of the upstream base commit (such as `7cffb81a8e124d5f218f9af16ad685acf5e9c67c`)

### Template

Instead of creating this manually, run `patchable init`:

```toml
cargo patchable init druid 28.0.0 --upstream=https://github.com/apache/druid.git --base=druid-28.0.0
```

## Glossary

- Images repo/directory - The checkout of stackabletech/docker-images
