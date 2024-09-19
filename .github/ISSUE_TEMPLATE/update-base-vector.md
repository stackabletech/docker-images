---
name: Update Vector
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(vector): Update container images ahead of Stackable Release XX.(X)X
labels: []
projects: ['stackabletech/10']
assignees: ''
---

Part of #xxx.

> [!TIP]
> Please add the `scheduled-for/20XX-XX` label.

<!-- markdownlint-disable-next-line MD028 -->
> [!IMPORTANT]
>
> - Uses [stackable-base](https://github.com/stackabletech/docker-images/blob/main/stackable-base/Dockerfile).
> - Used as a base for [java-base](https://github.com/stackabletech/docker-images/blob/main/java-base/Dockerfile).

```[tasklist]
### Update tasks
- [ ] Update `versions.py` to reflect the agreed upon versions in the spreadsheet (including the removal of old versions).
- [ ] Upload new version (see `vector/upload_new_vector_version.sh`).
- [ ] Update `versions.py` to the latest supported version of JVM (base and devel).
- [ ] Update other dependencies if applicable (eg: inotify_tools, etc).
- [ ] Check other operators (getting_started / kuttl) for usage of the versions. Add the PR(s) to the list below.
- [ ] Update the version in demos. Add the PR(s) to the list below.
```

```[tasklist]
### Related Pull Requests
- [ ] _Link to the docker-images PR (product update)_
- [ ] _Link to the operator PR (getting_started / kuttl)_
- [ ] _Link to any other operator PRs (getting_started / kuttl)_
- [ ] _Link to demo PR (raise against the `next` branch)_
- [ ] _Link to the Release Notes PR in the documentation repo (if not a comment below)_
```

> [!TIP]
> Delete any items that do not apply so that all applicable items can be checked.
> For example, if you add release notes to the documentation repository, you do not need the latter two criteria.

```[tasklist]
### Acceptance
- [ ] Can build image locally
- [ ] Kuttl smoke tests passes locally
- [ ] Release notes added to documentation and linked as a PR above
- [ ] Release notes written in a comment below
- [ ] Applicable `release-note` label added to this issue
```

<details>
<summary>Testing instructions</summary>

```shell
# See the latest version at https://pypi.org/project/image-tools-stackabletech/
pip install image-tools-stackabletech==0.0.12

bake --product vector=x.y.z # where x.y.z is the new version added in this PR

kind load docker-image docker.stackable.tech/stackable/vector:x.y.z-stackable0.0.0-dev

# Change directory into one of the operator repositories and update the
# product version in tests/test-definition.yaml
./scripts/run-tests --test-suite smoke-latest # or similar
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
