# Stackable nginx Docker image

## Description

This image is based on the official (currently 1.21.6) version of the nginx image.

It adds the possibility to upload and download files to `/upload/opa/bundle`. As the name suggests, this is used by the Stackable OPA Operator to upload bundles and by OPA-Services to download them again.

This image stores the files under `/tmp` by default (and for testing purposes), but the OPA operator uses a persistent volume.

## Testing with Docker

  docker run --rm --name nginx -p 8080:80 docker.stackable.tech/stackable/nginx:1.21.6-stackable0
  echo Upload test.tar.gz ...
  curl -T test.tar.gz http://localhost:8080/upload/opa/bundle/test.tar.gz
  echo Download downloaded-test.tar.gz ...
  curl -o downloaded-test.tar.gz http://localhost:8080/upload/opa/bundle/test.tar.gz

