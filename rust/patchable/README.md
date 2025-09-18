# Patchable

Patchable is a tool for managing patches for the third-party products distributed by Stackable as part of the Stackable Data Platform.

Patchable works by keeping a series of .patch files (in `docker-images/<PRODUCT>/stackable/patches/<VERSION>`)
as its source of truth, but using temporary Git repositories as an interface to modify those patches.
This lets us track the history of patches over time, but reuse the existing familiarity and tooling around Git.

Patchable designates a commit as the upstream _base_ for each version, and considers each commit made on top of that to be an individual patch.

## Basic Usage

```sh
pushd $(cargo patchable checkout druid 26.0.0)
# do stuff
git commit
popd
cargo patchable export druid 26.0.0
git status
```

For more details, run `cargo patchable --help`.

## Patches

Patchable doesn't _edit_ anything by itself. Instead, it's a uniform way to apply a set of patches
to an upstream Git repository, and then export your local changes back into patch files.

It doesn't care about how you make your local changes - you can edit the branch created by
patchable using any Git frontend, such as the git CLI or [jj](https://jj-vcs.github.io/jj/latest/).

This way, the patch files are the global source of truth and track the history of our patch series,
while you can still use the same familiar Git tools to manipulate them.

### Check out patched sources locally

> [!NOTE]
> This is not required for building images, but is used for when hacking on or debugging patch series.

```sh
# Fetches the upstream repository (if required), and creates a git worktree to work with it
# It also creates two branches:
# - patchable/{version} (HEAD, has all patches applied)
# - patchable/base/{version} (the upstream)
pushd $(cargo patchable checkout druid 26.0.0)

# Commit to add new patches
# NOTE: the commit message will be used to construct the patch filename. Spaces
# will be converted to hyphens automatically.
git commit

# Rebase against the base commit to edit or remove patches
git rebase --interactive patchable/base/26.0.0
# jj edit also works, but make sure to go back to the tip before exporting

# When done, export your patches and commit them (to docker-images)
popd
cargo patchable export druid 26.0.0
git status
```

> ![CAUTION]
> `cargo patchable export` exports whatever is currently checked out (`HEAD`) in the worktree.
> If you use `jj edit` (or `git switch`) then you _must_ go back to the tip before exporting, or
> any patches after that point will be omitted from the export.

### Initialize a new product

If you're adding a completely new product, you need to initialize the product-level config once
using patchable:

```sh
cargo patchable init product druid \
  --upstream https://github.com/apache/druid.git \
  --default-mirror https://github.com/stackabletech/druid.git
```

This will create the product-level configuration in `docker-images/druid/stackable/patches/patchable.toml`
containing the following fields:

- `upstream` - the URL of the upstream repository (such as `https://github.com/apache/druid.git`)
- `default_mirror` - optional: default URL of a mirror repository (such as `https://github.com/stackabletech/druid.git`)

### Initialize a new patch series

Patchable stores metadata about each patch series in its version-level config, and will not be able
to check out a patch series that lacks one. It can be generated using the following command:

```sh
cargo patchable init version druid 28.0.0 --base druid-28.0.0 --mirror
cargo patchable checkout druid 28.0.0
```

This will initialize the version-level config in `docker-images/druid/stackable/patches/28.0.0/patchable.toml`
with the base commit hash and the default mirror URL from the product-level config:

- `base` - the commit hash of the upstream base commit
- `mirror` - optional: URL of the mirror repository for this version, if mirroring is enabled

You can optionally provide the `--ssh` flag to use SSH instead of HTTPS for Git operations.

### Importing patch series into Patchable

Patchable is stricter about applying invalid patches (both metadata and patches themselves) than Git is.

If an initial `cargo patchable checkout` fails then `git am` can be useful for the initial migration:

```sh
# Create Patchable configuration for the new version, if it doesn't already exist
cargo patchable init druid 30.0.0 --upstream https://github.com/apache/druid.git --base druid-30.0.0
# Check out the upstream base commit, without trying to apply the patches
pushd $(cargo patchable checkout druid 30.0.0 --base-only)

# Apply the patch series
git am ../../../stackable/patches/30.0.0/*.patch
# Resolve any conflicts that arise, and `git am --continue` until done

# Leave and export the new patch series!
popd
cargo patchable export druid 30.0.0
```

### Porting patch series to a new version

Patchable doesn't support restoring a patch series that doesn't apply cleanly. Instead, use `git cherry-pick` to rebase the patch series.

For example, let's try rebasing our patch series from Druid 26.0.0 to Druid 28.0.0 (which is not packaged by SDP):

```sh
# Restore the old version
# In addition to creating the version worktree, this also creates the branches
# patchable/26.0.0 (26.0.0 with our patches applied) and
# patchable/base/26.0.0 (upstream 26.0.0 with no patches).
cargo patchable checkout druid 26.0.0

# Tell Patchable about the new version 28.0.0, which can be fetched from
# https://github.com/apache/druid.git, and has the tag druid-28.0.0
cargo patchable init druid 28.0.0 --upstream https://github.com/apache/druid.git --base druid-28.0.0

# Create and go to the worktree for the new version
pushd $(cargo patchable checkout druid 28.0.0)

# Cherry pick the old patch series
git cherry-pick patchable/base/26.0.0..patchable/26.0.0

# Solve conflicts and `git cherry-pick --continue` until done
# You can also use `git cherry-pick --skip` to skip resolving conflicts for
# patches that are no longer required

# If some patches are no longer required, use an interactive rebase to remove
# them (or do other cleanup)
git rebase --interactive patchable/base/28.0.0

# Leave and export the new patch series!
popd
cargo patchable export druid 28.0.0
git status
```

### Porting patches between versions

Individual patches can also be cherry-picked across versions.

For example, assuming we are in the Druid 28.0.0 workspace and want to port the last patch of the
Druid 26.0.0 series:

```sh
# git cherry-pick <hash> is also fine for grabbing arbitrary patches
git cherry-pick patchable/26.0.0
```

## Notes

- patchable only supports linear patch series (no merges beyond the base commit)
- patchable doesn't support merging "materialized" trees, merge the .patch files instead, and `checkout`/`export` to update the hashes
- `patchable checkout` doesn't support resolving patch conflicts, use `git am` instead (and then `patchable export` the resolved patches)
- Always run patchable via `cargo patchable` (rather than `cargo install`ing it), to ensure that you use the correct version for a given checkout of docker-images

## Glossary

- Images repo/directory - The checkout of stackabletech/docker-images
