---
name: Update Airflow
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(airflow): Update container images ahead of Stackable Release XX.(X)X
labels: []
projects: ['stackabletech/10']
assignees: ''
---

Part of #xxx.

> [!TIP]
> Please add the `scheduled-for/20XX-XX` label.

```[tasklist]
### Update tasks
- [ ] Update `versions.py` to reflect the agreed upon versions in the spreadsheet (including the removal of old versions).
- [ ] Download new constraints file (see `airflow/download_constraints.sh`).
- [ ] Update other dependencies if applicable (eg: python, statsd_exporter, etc).
- [ ] Check other operators (getting_started / kuttl) for usage of the versions. Add the PR(s) to the list below.
- [ ] Update the version in demos. Add the PR(s) to the list below.
```

```[tasklist]
### Related Pull Requests
- [ ] _Link to operator PR (getting_started / kuttl)_
- [ ] _Link to any other operator PRs (getting_started / kuttl)_
- [ ] _Link to demo PR (raise against the `next` branch)_
```

<!--
Make this a regular list so it isn't easily editable from the rendered
description?
-->
```[tasklist]
### Acceptance
- [ ] Can build image locally
- [ ] Kuttl smoke tests passes locally
- [ ] Release notes written in a comment below
- [ ] Applicable `release-note` label added to this issue
```

<details>
<summary>Testing instructions</summary>

```shell
# See the latest version at https://pypi.org/project/image-tools-stackabletech/
pip install image-tools-stackabletech==0.0.12

bake --product airflow=x.y.z # where x.y.z is the new version added in this PR

kind load docker-image docker.stackable.tech/stackable/airflow:x.y.z-stackable0.0.0-dev

# Change directory into the airflow-operator repository and update the
# product version in tests/test-definition.yaml
./scripts/run-tests --test-suite smoke-latest
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
