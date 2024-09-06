---
name: Update Kafka
about: >-
  This template contains instructions specific to updating this product and/or
  container image(s).
title: >-
  chore(kafka): Update container images ahead of Stackable Release XX.(X)X
labels: []
projects: ['stackabletech/10']
assignees: ''
---

Part of #xxx.

> [!TIP]
> Please add the `scheduled-for/20XX-XX` label.

```[tasklist]
### Update tasks (kafka)
- [ ] Update `versions.py` to reflect the agreed upon versions in the spreadsheet (including the removal of old versions).
- [ ] Upload new version (see `kafka/upload_new_kafka_version.sh`).
- [ ] Update `versions.py` to the latest supported version of JVM (base and devel).
- [ ] Update other dependencies if applicable (eg: jmx_exporter, kcat, scala, etc).
- [ ] Check other operators (getting_started / kuttl) for usage of the versions. Add the PR(s) to the list below.
- [ ] Update the version in demos. Add the PR(s) to the list below.
```

<!-- todo: ensure this is the correct procedure -->
```[tasklist]
### Update tasks (kcat and kafka-testing-tools)
- [ ] Update `kcat/versions.py`.
- [ ] Update `kafka-testing-tools/versions.py`.
- [ ] Upload new version (see `.scripts/upload_new_kcat_version.sh`).
- [ ] Update `versions.py` to the latest supported version of JVM (base and devel).
- [ ] Check other operators (getting_started / kuttl) for usage of the versions. Add the PR(s) to the list below.
- [ ] Update the version in demos. Add the PR(s) to the list below.
```

```[tasklist]
### Related Pull Requests
- [ ] _Link to the docker-images PR (product update)_
- [ ] _Link to the operator PR (getting_started / kuttl)_
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

bake --product kafka=x.y.z # where x.y.z is the new version added in this PR
bake --product kafka-testing-tools=1.0.0 # This version doesn't change

kind load docker-image docker.stackable.tech/stackable/kafka:x.y.z-stackable0.0.0-dev
kind load docker-image docker.stackable.tech/stackable/kafka-testing-tools:1.0.0-stackable0.0.0-dev

# Change directory into the kafka-operator repository and update the
# product version in tests/test-definition.yaml
./scripts/run-tests --test-suite smoke-latest # or similar
```

</details>

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
