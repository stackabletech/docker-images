---
name: Update UBI Rust Builders
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(ubi-rust-builders): Update container images ahead of Stackable Release YY.M.X
labels: []
# Currently, projects cannot be assigned via front-matter.
projects: ['stackabletech/10']
assignees: ''
---

<!--
I don't think we should tie rust bumps to releases - they should be done as
developers need newer versions , which could be multiple times in a release.
If there are no bumps in a release, we can still rely on SecObserve and Renovate
to alert us to security vulnerabilities.
-->
Part of #xxx.

<!--
This gives hints to the person doing the work.
Add/Change/Remove anything that isn't applicable anymore
-->
- Add: `x.x.x`
- Remove: `y.y.y`

> [!TIP]
> Please add the `scheduled-for/20XX-XX` label, and add to the [Stackable Engineering][1] project.
>
> [1]: https://github.com/orgs/stackabletech/projects/10

## Update tasks

- [ ] Update UBI version hash in the Dockerfile (`FROM`)
- [ ] Update `RUST_DEFAULT_TOOLCHAIN_VERSION`
- [ ] Update `CARGO_CYCLONEDX_CRATE_VERSION`
- [ ] Update `CARGO_AUDITABLE_CRATE_VERSION`
- [ ] Update `PROTOC_VERSION`

## Related Pull Requests

- [ ] _Link to the docker-images PR (product update)_
- [ ] _Bump rust toolchain in operator-rs_
- [ ] _Bump rust toolchain in operator-templating_

## Acceptance

> [!TIP]
> This list should be completed by the assignee(s), once respective PRs have been merged. Once all items have been
> checked, the issue can be moved into _Development: Done_.

- Done for [ubi8-rust-builder/Dockerfile](https://github.com/stackabletech/docker-images/blob/main/ubi8-rust-builder/Dockerfile)
- Done for [ubi9-rust-builder/Dockerfile](https://github.com/stackabletech/docker-images/blob/main/ubi9-rust-builder/Dockerfile)
- [ ] Can build the image locally
- [ ] Can build an operator image

<details>
<summary>Testing instructions</summary>

```shell
docker build -t oci.stackable.tech/sdp/ubi9-rust-builder . -f ubi9-rust-builder/Dockerfile

# Change directory into the an operator repository and ensure the image can build
docker build . -f docker/Dockerfile
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
