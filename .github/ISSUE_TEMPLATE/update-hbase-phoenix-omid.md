---
name: Update HBase, Phoenix, Omid
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(hbase-phoenix-omid): Update container images ahead of Stackable Release XX.(X)X
labels: []
projects: ['stackabletech/10']
assignees: ''
---

Part of #xxx.

> [!TIP]
> Please add the `scheduled-for/20XX-XX` label.

```[tasklist]
### Update tasks (HBase, Phoenix)
- [ ] Update `versions.py` to reflect the agreed upon versions in the spreadsheet (including the removal of old versions).
- [ ] Upload new versions (see the `hbase/*.sh` scripts).
- [ ] Update `versions.py` to the latest supported version of JVM (base and devel).
- [ ] Update other dependencies if applicable (eg: phoenix, opa_authorizer, etc).
- [ ] Check other operators (getting_started / kuttl) for usage of the versions. Add the PR to the list below.
```

```[tasklist]
### Update tasks (Omid)
- [ ] Update `omid/versions.py`to reflect the agreed upon versions in the spreadsheet (including the removal of old versions).
- [ ] Upload new version (see `omid/upload_new_omid_version.sh`).
- [ ] Update `versions.py` to the latest supported version of JVM (base and devel).
- [ ] Update other dependencies if applicable (eg: jmx_exporter, etc).
```

```[tasklist]
### Related Pull Requests
- [ ] _Link to the docker-images PR (product update)_
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

bake --product hbase=x.y.z # where x.y.z is the new version added in this PR
bake --product omid=x.y.z # where x.y.z is the new version added in this PR

kind load docker-image docker.stackable.tech/stackable/hbase:x.y.z-stackable0.0.0-dev
kind load docker-image docker.stackable.tech/stackable/omid:x.y.z-stackable0.0.0-dev

# Change directory into the hbase-operator repository and update the
# product version in tests/test-definition.yaml
./scripts/run-tests --test-suite smoke-latest
./scripts/run-tests --test omid
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
