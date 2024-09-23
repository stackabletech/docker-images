---
name: Update Stackable Base
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(stackable-base): Update container images ahead of Stackable Release XX.(X)X
labels: []
projects: ['stackabletech/10']
assignees: ''
---

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

```[tasklist]
### Update tasks
- [ ] Update UBI version hash in the Dockerfile (`FROM`)
- [ ] Update `RUST_DEFAULT_TOOLCHAIN_VERSION`
- [ ] Update `CARGO_CYCLONEDX_CRATE_VERSION`
- [ ] Update `CARGO_AUDITABLE_CRATE_VERSION`
- [ ] Update `PROTOC_VERSION`
- [ ] Update `CONFIG_UTILS_VERSION`
```

```[tasklist]
### Related Pull Requests
- [ ] _Link to the docker-images PR (product update)_
```

This list should be completed by the assignee(s), once respective PRs have been merged. Once all items have been checked, the issue can be moved into _Development: Done_.

```[tasklist]
### Acceptance
- [ ] Can build the image locally
- [ ] Can build the vector image
```

<details>
<summary>Testing instructions</summary>

```shell
# See the latest version at https://pypi.org/project/image-tools-stackabletech/
pip install image-tools-stackabletech==0.0.12

bake --product vector=x.y.z # where x.y.z is a valid version
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
