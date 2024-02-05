.PHONY: build-ubi8-rust-builder, push-ubi8-rust-builder, login

REPO   := docker.stackable.tech/stackable
TAG    := $(shell git rev-parse --short HEAD)
ARCH   := $(shell arch)

define push
	docker push --all-tags ${REPO}/$(1)
endef

define build
	@docker build --force-rm -t "${REPO}/${1}-${ARCH}:${TAG}" -t "${REPO}/${1}-${ARCH}:latest" -f $(1)/Dockerfile .
endef

# Tag for arm64 is fixed since this will run on gh ubuntu-latest
define manifest
	@docker manifest create "${REPO}/${1}:latest" ${REPO}/${1}-${ARCH}:latest ${REPO}/${1}-arm64:latest
endef

define manifest_push
	docker manifest push ${REPO}/${1}:latest
endef

# Pulling both images after building them, ugly need a better way
define pull-arm64
	@docker pull ${REPO}/${1}-arm64:latest 
endef

define pull-amd64
	@docker pull ${REPO}/${1}-amd64:latest 
endef

build-ubi8-rust-builder: NAME = ubi8-rust-builder
build-ubi8-rust-builder:
	$(call build,${NAME})

push-ubi8-rust-builder: NAME = ubi8-rust-builder
push-ubi8-rust-builder : build-ubi8-rust-builder login
	$(call push,${NAME})

pull-ubi8-rust-builder: NAME = ubi8-rust-builder
pull-ubi8-rust-builder: login
	$(call pull-arm64,${NAME})
	$(call pull-amd64,${NAME})

build-manifest-list: NAME = ubi8-rust-builder
build-manifest-list:
	$(call manifest,${NAME})

push-manifest-list: NAME = ubi8-rust-builder
push-manifest-list: login pull-ubi8-rust-builder build-manifest-list 
	$(call manifest_push,${NAME})

login:
ifndef DOCKER_PASSWORD
		$(error DOCKER_PASSWORD is undefined)
endif
ifndef DOCKER_USER
		$(error DOCKER_USER is undefined)
endif
	echo "${DOCKER_PASSWORD}" | docker login --username ${DOCKER_USER} --password-stdin docker.stackable.tech

