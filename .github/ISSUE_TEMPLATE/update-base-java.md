---
name: Update Java Bases
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(java-bases): Update container images ahead of Stackable Release YY.M.X
labels: []
# Currently, projects cannot be assigned via front-matter.
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

<!-- markdownlint-disable-next-line MD028 -->
> [!IMPORTANT]
>
> - `java-base` uses [vector](https://github.com/stackabletech/docker-images/blob/main/vector/Dockerfile) and is used as a base for java products.
> - `java-devel` uses [stackable-base](https://github.com/stackabletech/docker-images/blob/main/stackable-base/Dockerfile) and is used to build java products.

Typically product updates will determine which version of Java is required, but
we should also make new versions of Java available for use.

> [!TIP]
> You can search for available java versions at [rpmfind.net], and search the
> term `openjdk-headless`.
> _It isn't perfect, as it will depend on what is available via microdnf._

## Update tasks

- [ ] Add any new versions of java to both `java-base/versions.py` and `java-devel/versions.py`
- [ ] Remove versions when there are no long any references (eg: `grep java- **/versions.py | grep "1.8.0"`)

## Related Pull Requests

- [ ] _Link to the docker-images PR (product update)_

## Acceptance

> [!TIP]
> This list should be completed by the assignee(s), once respective PRs have been merged. Once all items have been
> checked, the issue can be moved into _Development: Done_.

- [ ] Can build a product image that uses the new version(s)
- [ ] Both `java-base` and `java-devel` have the same Java versions in `versions.py`
- [ ] Kuttl smoke test passes locally for a product using the new Java version

<details>
<summary>Testing instructions</summary>

```shell
# See the latest version at https://pypi.org/project/image-tools-stackabletech/
pip install image-tools-stackabletech==0.0.16

# Test a product image can build, eg: ZooKeeper
bake --product zookeeper=x.y.z # where x.y.z is a valid product version using the newly added Java version

kind load docker-image oci.stackable.tech/sdp/zookeeper:x.y.z-stackable0.0.0-dev

# Change directory into one of the operator repositories (eg: zookeeper-operator) and update the
# product version in tests/test-definition.yaml
./scripts/run-tests --test-suite smoke-latest # or similar
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._

[rpmfind.net]: https://rpmfind.net/linux/RPM/Development_Java.html
