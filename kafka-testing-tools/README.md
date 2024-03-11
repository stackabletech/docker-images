# KCat image

This image is needed for the Kafka integration tests on OpenShift. The `edenhill/kcat` used initially doesn't work reliably on that platform.

This is not in `testing-tools` because that image is Debian based and we might want to reuse this binary in the Kafka image in the future.
