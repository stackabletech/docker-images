.PHONY: build-ubi8-rust-builder, push-ubi8-rust-builder, login

REPO   := docker.stackable.tech/stackable
TAG    := $(shell git rev-parse --short HEAD)
ARCH   := $(shell arch)
NAME   := ubi8-rust-builder

define push
	docker push --all-tags "${REPO}/$(1)"
endef

define build
	@docker build --force-rm -t "${REPO}/${1}:${TAG}-${ARCH}" -f $(1)/Dockerfile .
endef

define manifest
	@docker manifest create "${REPO}/${1}:latest" "${REPO}/${1}:${TAG}-aarch64" "${REPO}/${1}:${TAG}-x86_64"
endef

build-ubi8-rust-builder:
build-ubi8-rust-builder:
	$(call build,${NAME})

push-ubi8-rust-builder:
push-ubi8-rust-builder : build-ubi8-rust-builder login
	$(call push,${NAME})

build-manifest-list:
build-manifest-list: login
	$(call manifest,${NAME})

login:
ifndef DOCKER_PASSWORD
		$(error DOCKER_PASSWORD is undefined)
endif
ifndef DOCKER_USER
		$(error DOCKER_USER is undefined)
endif
	echo "${DOCKER_PASSWORD}" | docker login --username ${DOCKER_USER} --password-stdin docker.stackable.tech

