---
name: Early Pre-Release Container Image Updates
about: This template can be used to track the container image updates leading up to the next Stackable release
title: "chore(tracking): Update major/minor|patch versions for YY.M.X"
labels: ['epic']
assignees: ''
---

<!--
    DO NOT REMOVE THIS COMMENT. It is intended for people who might copy/paste from the previous release issue.
    This was created by an issue template: https://github.com/stackabletech/docker-images/issues/new/choose.
-->

<!-- Update this with the parent tracking issue for the release -->
Part of <https://github.com/stackabletech/issues/issues/xxx>.

> [!NOTE]
> Update the product versions based on what has been decided upon in the _Product Spreadsheet[^1]_.
> Follow these rules when creating the tracking issues for individual images:
>
> - Major/minor changes:
>   - Removal and deprecation of entire major or minor version branches
>   - Adding a new major/minor version (optionally marking it as LTS)
> - Patch-level changes:
>   - Removal and deprecation if individual patch-level versions
>   - Marking a (new) patch-level version as LTS
>   - Adding a new patch-level version

[^1]: Currently this is a private spreadsheet

> [!IMPORTANT]
> Replace the items in the task lists below with the subsequent tracking issue.
> Remove this and the above admonitions afterwards to de-clutter the tracking issue.

## Product Container Images

<!--
    Find templates for bases/products:

    find .github/ISSUE_TEMPLATE/update-*.md -printf "%f\n" \
    | sort \
    | xargs -I {} echo "- [ ] [Create issue from template: {}](https://github.com/stackabletech/docker-images/issues/new?template={})"
-->

<!-- todo: consider removing the ubi*-rust-builder from the release process. -->

- [ ] [Create issue from template: update-base-java.md](https://github.com/stackabletech/docker-images/issues/new?template=update-base-java.md)
- [ ] [Create issue from template: update-base-stackable.md](https://github.com/stackabletech/docker-images/issues/new?template=update-base-stackable.md)
- [ ] [Create issue from template: update-base-vector.md](https://github.com/stackabletech/docker-images/issues/new?template=update-base-vector.md)
- [ ] [Create issue from template: update-product-airflow.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-airflow.md)
- [ ] [Create issue from template: update-product-druid.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-druid.md)
- [ ] [Create issue from template: update-product-hbase-phoenix-omid.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-hbase-phoenix-omid.md)
- [ ] [Create issue from template: update-product-hdfs.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-hdfs.md)
- [ ] [Create issue from template: update-product-hive.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-hive.md)
- [ ] [Create issue from template: update-product-kafka.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-kafka.md)
- [ ] [Create issue from template: update-product-nifi.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-nifi.md)
- [ ] [Create issue from template: update-product-opa.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-opa.md)
- [ ] [Create issue from template: update-product-opensearch.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-opensearch.md)
- [ ] [Create issue from template: update-product-opensearch-dashboards.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-opensearch-dashboards.md)
- [ ] [Create issue from template: update-product-spark.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-spark.md)
- [ ] [Create issue from template: update-product-superset.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-superset.md)
- [ ] [Create issue from template: update-product-trino.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-trino.md)
- [ ] [Create issue from template: update-product-zookeeper.md](https://github.com/stackabletech/docker-images/issues/new?template=update-product-zookeeper.md)

## Additional items which don't have a tracking issue

- [ ] jmx_exporter (validate via hdfs-operator smoke tests)
- [ ] krb5
- [ ] tools (update the versions in version.py)
- [ ] testing-tools (update the base, maybe keycloak)
- [ ] statsd_exporter
- [ ] csi-provisioner for secret-operator
- [ ] csi-provisioner for listener-operator
