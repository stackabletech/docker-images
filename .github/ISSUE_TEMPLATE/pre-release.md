---
name: Pre-Release Product Updates
about: This template can be used to track the product leading up to the next Stackable release
title: Release XX.(X)X
labels: ['epic']
assignees: ''
---

<!--
    DO NOT REMOVE THIS COMMENT. It is intended for people who might copy/paste from the previous release issue.
    This was created by an issue template: https://github.com/stackabletech/issues/issues/new/choose.
-->

## Product Updates for Stackable Release XX.(X)X

> [!NOTE]
> Update the product versions based on what has been decided upon in the _Product Spreadsheet[^1]_.

[^1]: Currently this is a private spreadsheet

Replace the items in the task lists below with the applicable Pull Requests

<!--
    Find any other image directories not covered by the list above:

    find . -name "versions.py" \
    | cut -d/ -f2 \
    | sort \
    | xargs -I {} echo "- [ ] https://github.com/stackabletech/docker-images/tree/main/{}/versions.py"
-->

```[tasklist]
### Product Container Images
- [ ] https://github.com/stackabletech/docker-images/tree/main/airflow/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/druid/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/hadoop/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/hbase/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/hello-world/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/hive/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/java-base/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/java-devel/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/kafka/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/kafka-testing-tools/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/kcat/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/krb5/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/nifi/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/omid/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/opa/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/spark-k8s/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/stackable-base/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/superset/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/testing-tools/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/tools/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/trino/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/trino-cli/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/vector/versions.py
- [ ] https://github.com/stackabletech/docker-images/tree/main/zookeeper/versions.py
```

> [!NOTE]
> Generally you will only need to update the rust-toolchain version (`RUST_DEFAULT_TOOLCHAIN_VERSION`).

<!--
    Find any other image directories not covered by the list above

    comm -3 \
    <(find . -name "Dockerfile" | cut -d/ -f2 | sort) \
    <(find . -name "versions.py" | cut -d/ -f2 | sort) \
    | xargs -I {} echo "- [ ] https://github.com/stackabletech/docker-images/tree/main/{}/Dockerfile"
-->

```[tasklist]
### Other Container Images
- [ ] https://github.com/stackabletech/docker-images/tree/main/ubi8-rust-builder/Dockerfile
- [ ] https://github.com/stackabletech/docker-images/tree/main/ubi9-rust-builder/Dockerfile
```
