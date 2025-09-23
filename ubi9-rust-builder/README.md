# ubi9-rust-builder

These images are meant to be used in multi-stage builds as a base image for projects building Rust projects.
They are automatically rebuilt and pushed every night and also on every push to the main branch, in addition a build can be triggered using GitHub Actions.

The image will run `cargo build --release` in the current context and copy all binaries to an `/app` directory.

This will bake in the current stable Rust version at the time this image was built, which means it should be rebuilt (and tagged) for every release of Rust.

## Example usage

```dockerfile
FROM oci.stackable.tech/ubi9-rust-builder AS builder

FROM registry.access.redhat.com/ubi9/ubi-minimal AS operator
LABEL maintainer="Stackable GmbH"

# Update image
RUN microdnf update \
  && microdnf install \
    shadow-utils \
  && rm -rf /var/cache/yum

COPY --from=builder /app/stackable-zookeeper-operator /

RUN groupadd -g 1000 stackable && adduser -u 1000 -g stackable -c 'Stackable Operator' stackable

USER 1000:1000

ENTRYPOINT ["/stackable-zookeeper-operator"]
```
