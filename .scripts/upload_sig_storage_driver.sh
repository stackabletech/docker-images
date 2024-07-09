# shellcheck disable=all
#
# Don't do this on your machine, as we need to sign the mirrored image, which should be done by a GitHub CI.
# Use the action "Mirror container image" in the docker-images repo for that.
# You can e.g. pull the image from
# registry.k8s.io/sig-storage/csi-provisioner:v5.0.1
# and put it in
# csi-provisioner:v5.0.1
# as well as
# registry.k8s.io/sig-storage/csi-node-driver-registrar:v2.11.0
# and put it in
# csi-node-driver-registrar:v2.11.0

# On 2024-07-03 we manually copied the image from Harbor to Nexus, so that it's correctly signed there as well.
