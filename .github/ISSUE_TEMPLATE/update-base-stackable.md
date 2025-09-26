---
name: Update Stackable Base
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(stackable-base): Update image for YY.M.X
labels: []
# Currently, projects cannot be assigned via front-matter.
projects: ['stackabletech/10']
assignees: ''
---

Part of <https://github.com/stackabletech/docker-images/issues/xxx>.

> [!TIP]
> Please add the `scheduled-for/YY.M.X` label, and add to the [Stackable Engineering][1] project.
>
> [1]: https://github.com/orgs/stackabletech/projects/10

## Update tasks

> [!NOTE]
> When updating the base image, you will likely get a build failure related to the CA certificates.
> This means you will need to update the `ca-certificates-*` package and try again. The build will
> fail if the blocked certificates are still found.
> The package check exists so that we can remove it once (if ever) the _bad_ CA has been removed.

### `stackable-base`

- [ ] Update `FROM ...ubi-minimal` version hash in the Dockerfile
- [ ] Update `CONFIG_UTILS_VERSION`

### `stackable-devel`

- [ ] Update `FROM ...ubi-minimal` version hash in the Dockerfile
- [ ] Update `RUST_DEFAULT_TOOLCHAIN_VERSION` (if tools need it, eg: patchable, config-utils)
- [ ] Update `CARGO_CYCLONEDX_CRATE_VERSION` (if necessary)
- [ ] Update `CARGO_AUDITABLE_CRATE_VERSION` (if necessary)

## Related Pull Requests

- _Link to the docker-images PR (product update)_

## Acceptance

> [!TIP]
> This list should be completed by the assignee(s), once respective PRs have been merged. Once all items have been
> checked, the issue can be moved into _Development: Done_.

- [ ] Can build the image locally
- [ ] Can build the vector image

<details>
<summary>Testing instructions</summary>

```shell
# Where x.y.z is a valid version
boil build vector=x.y.z --strip-architecture --load
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
