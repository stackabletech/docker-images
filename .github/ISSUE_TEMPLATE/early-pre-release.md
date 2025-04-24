---
name: Early Pre-Release Container Image Updates
about: This template can be used to track the container image updates leading up to the next Stackable release
title: "chore: Update Container Images for Stackable Release YY.M.X"
labels: ['epic']
assignees: ''
---

<!--
    DO NOT REMOVE THIS COMMENT. It is intended for people who might copy/paste from the previous release issue.
    This was created by an issue template: https://github.com/stackabletech/docker-images/issues/new/choose.
-->

<!-- Update this with the parent tracking issue for the release -->
Part of stackabletech/issues#xxx.

## Container Image Updates for Stackable Release XX.(X)X

> [!NOTE]
> Update the product versions based on what has been decided upon in the _Product Spreadsheet[^1]_.

[^1]: Currently this is a private spreadsheet

Replace the items in the task lists below with the subsequent tracking issue.

<!--
    Find templates for bases/products:

    find .github/ISSUE_TEMPLATE/update-*.md -printf "%f\n" \
    | sort \
    | xargs -I {} echo "- [ ] https://github.com/stackabletech/docker-images/issues/new?template={}"
-->

<!-- todo: consider removing the ubi*-rust-builder from the release process. -->
```[tasklist]
### Product Container Images
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-base-java.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-base-stackable.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-base-ubi-rust-builders.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-base-vector.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-airflow.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-druid.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-hbase-phoenix-omid.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-hdfs.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-hive.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-kafka.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-nifi.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-opa.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-spark.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-superset.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-trino.md
- [ ] https://github.com/stackabletech/docker-images/issues/new?template=update-product-zookeeper.md
```

```[tasklist]
### Additional items which don't have a tracking issue
- [ ] hello-world
- [ ] krb5
- [ ] tools
- [ ] testing-tools
- [ ] statsd_exporter
```
