# Description

*Please add a description here.*

## Definition of Done Checklist

- Not all of these items are applicable to all PRs, the author should update this template to only leave the boxes in that are relevant
- Please make sure all these things are done and tick the boxes

- [ ] Changes are OpenShift compatible
- [ ] All added packages (via microdnf or otherwise) have a comment on why they are added
- [ ] Things not downloaded from Red Hat repositories should be mirrored in the Stackable repository and downloaded from there
- [ ] All packages should have (if available) signatures/hashes verified
- [ ] Add an entry to the CHANGELOG.md file
- [ ] Integration tests ran successfully

<details>
<summary>TIP: Running integration tests with a new product image</summary>

The image can be built and uploaded to the kind cluster with the following commands:

```shell
bake --product <product> --image-version <stackable-image-version>
kind load docker-image <image-tagged-with-the-major-version> --name=<name-of-your-test-cluster>
```

See the output of `bake` to retrieve the image tag for `<image-tagged-with-the-major-version>`.
</details>
