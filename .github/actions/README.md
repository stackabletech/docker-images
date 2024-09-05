# Actions

This repository contains various reusable actions which encapsulate series of
commands to run a particular step in a workflow.

## Definitions

| Name                               | Example                                                                |
| ---------------------------------- | ---------------------------------------------------------------------- |
| Image Registry                     | `docker.stackable.tech`                                                |
| Image Repository                   | `stackable/kafka`                                                      |
| Image Index Manifest Tag           | `3.4.1-stackable0.0.0-dev`                                             |
| Image Manifest Tag                 | `3.4.1-stackable0.0.0-dev-amd64`                                       |
| Image Repository URI               | `docker.stackable.tech/stackable/kafka`                                |
| Image Index URI (if multi-arch)    | `docker.stackable.tech/stackable/kafka:3.4.1-stackable0.0.0-dev`       |
| Image Manifest URI (if multi-arch) | `docker.stackable.tech/stackable/kafka:3.4.1-stackable0.0.0-dev-amd64` |
| Image Repo Digest                  | `docker.stackable.tech/stackable/kafka@sha256:917f800259ef4915f976...` |
| Digest                             | `sha256:917f800259ef4915f976e93987b752fd64debf347568610d7f685d2022...` |

## `build-product-image`

Manifest: [build-product-image/action.yml][build-product-image]

> [!NOTE]
> The build step is not concerned with registries, ports, paths to repositories,
> but still requires a name. If the name does not contain a registry,
> `hub.docker.com` (?) is implied. Therefore, `localhost` will be used as the
> registry so as to avoid accidental interactions with an unintended registry.
>
> Ideally, bake should be refactored to use `localhost` as the registry for the
> previously mentioned reason (whether or not that is behind some option).

This action builds a *single* container image using `bake`. It does the
following work:

1. Free disk space to avoid running out of disk space during larger builds.
2. Build the image using `bake` which internally uses `docker buildx`.
3. Temporarily retag the image to use `localhost` instead of
   `docker.stackable.tech/stackable`.
4. Produce output values to be used in later steps.

This action is considered to be the **single** source of truth regarding image
index tag and image manifest tag. All subsequent tasks must use these values to
ensure consistency.

Currently, bake provides the following ouput in the `bake-target-tags` file:

```plain
docker.stackable.tech/stackable/kafka:3.4.1-stackable0.0.0-dev-amd64
```

Until bake supports the ability to specify the registry, this action will retag
the image as:

```plain
localhost/kafka:3.4.1-stackable0.0.0-dev-amd64
```

### Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the [build-product-image]
> workflow.

#### Inputs

- `product-name`
- `product-version`
- `image-tools-version`
- `build-cache-username`
- `build-cache-password`

#### Outputs

- `image-manifest-tag`

[build-product-image]: ./build-product-image/action.yml

## `publish-image`

Manifest: [publish-image/action.yml][publish-image]

This action signs and publishes a *single* container image to the given
registry. It does the following work:

1. Tag the `source-image-uri` with the specified `image-registry-uti`,
   `image-repository`, and `image-repository`.
2. Push the container image to the specified registry.
3. Sign the container image (which pushes the signature to the specified
   registry).
4. Generate an SBOM via a syft scan.
5. Attest an image with the SBOM as a predicate (which pushes the attestation
   to the specified registry).

### Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the [publish-image] workflow.

<!-- markdownlint-disable-next-line MD028 -->
> [!IMPORTANT]
> For multi-arch images, the `image-manifest-tag` should have the `-$ARCH`
> suffix, as the tag without it should be reserved for the image index manifest
> which will refer to container images for each architecture we will push images
> for.

#### Inputs

- `image-registry-uri`
- `image-registry-username`
- `image-registry-password`
- `image-repository`
- `image-manifest-tag`
- `source-image-uri`

#### Outputs

None

[publish-image]: ./publish-image/action.yml

## `publish-index-manifest`

Manifest: [publish-index-manifest/action.yml][publish-index-manifest]

This action creates an image index manifest, publishes it, and signs it. It does
the following work:

1. Create an image index manifest and link to each architecture in
   `image-architectures`.
2. Push the image index manifest.
3. Sign the image index manifest (which pushes the signature to the specified
   registry).

### Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the [publish-index-manifest]
> workflow.

#### Inputs

- `image-registry-uri`
- `image-registry-username`
- `image-registry-password`
- `image-repository`
- `image-index-manifest-tag`
- `image-architectures`

#### Outputs

None

[publish-index-manifest]: ./publish-index-manifest/action.yml

## `shard`

Manifest: [shard/action.yml][shard]

This action produces a list of versions for a product. This is to be used as a
matrix dimension to parallelize builds. It does the following work:

1. Reads the `conf.py`, filtering versions for the product
2. Write the JSON array of version to `$GITHUB_OUTPUT` for use in a matrix.

Example usage:

```yaml
jobs:
  generate_matrix:
    name: Generate Version List
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@692973e3d937129bcbf40652eb9f2f61becf3332 # v4.1.7
      - id: shard
        uses: ./.github/actions/shard
        with:
          product-name: ${{ env.PRODUCT_NAME }}
    outputs:
      versions: ${{ steps.shard.outputs.versions }}

  actual_matrix:
    needs: [generate_matrix]
    strategy:
      matrix:
        versions: ${{ fromJson(needs.generate_matrix.outputs.versions) }}
    # ...
```

### Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the [shard] workflow.

#### Inputs

- `product-name`

#### Outputs

- `versions`

[shard]: ./publish-index-manifest/action.yml
