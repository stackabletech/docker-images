---
name: Update HDFS
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(hdfs): Update container images ahead of Stackable Release XX.(X)X
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

```[tasklist]
### Update tasks
- [ ] Update `versions.py` to reflect the agreed upon versions in the spreadsheet (including the removal of old versions).
- [ ] Upload new version (see `hadoop/upload_new_hadoop_version.sh`).
- [ ] Update `versions.py` to the latest supported version of JVM (base and devel).
- [ ] Update other dependencies if applicable (eg: hdfs_utils, jmx_exporter, protobuf, etc).
- [ ] Check other operators (getting_started / kuttl / supported-versions) for usage of the versions. Add the PR(s) to the list below.
- [ ] Update the version in demos. Add the PR(s) to the list below.
```

```[tasklist]
### Related Pull Requests
- [ ] _Link to the docker-images PR (product update)_
- [ ] _Link to [hdfs-utils](https://github.com/stackabletech/hdfs-utils/) PR (if applicable)_
- [ ] _Link to the operator PR (getting_started / kuttl / supported-versions)_
- [ ] _Link to any other operator PRs (getting_started / kuttl)_
- [ ] _Link to demo PR (raise against the `next` branch)_
- [ ] _Link to the Release Notes PR in the documentation repo (if not a comment below)_
```

> [!TIP]
> Delete any items that do not apply so that all applicable items can be checked.
> For example, if you add release notes to the documentation repository, you do not need the latter two criteria.

This list should be completed by the assignee(s), once respective PRs have been merged. Once all items have been checked, the issue can be moved into _Development: Done_.

```[tasklist]
### Acceptance
- [ ] Can build image (either locally, or in CI)
- [ ] Kuttl smoke tests passes (either locally, or in CI)
- [ ] Release notes added to documentation and linked as a PR above
- [ ] Release notes written in a comment below
- [ ] Applicable `release-note` label added to this issue
```

<details>
<summary>Testing instructions</summary>

```shell
# See the latest version at https://pypi.org/project/image-tools-stackabletech/
pip install image-tools-stackabletech==0.0.13

bake --product hadoop=x.y.z # where x.y.z is the new version added in this PR

kind load docker-image oci.stackable.tech/sdp/hadoop:x.y.z-stackable0.0.0-dev

# Change directory into the hdfs-operator repository and update the
# product version in tests/test-definition.yaml
./scripts/run-tests --test-suite smoke-latest # or similar
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
