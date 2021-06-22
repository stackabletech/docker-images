.PHONY: build-ubi8-base-rust-builder, push-ubi8-base-rust-builder, login

REPO   := docker.stackable.tech
TAG    := $(shell git rev-parse --short HEAD)

define push
	docker push --all-tags ${REPO}/$(1)
endef

define build
	@docker build --force-rm -t "${REPO}/${1}:${TAG}" -t "${REPO}/${1}:latest" -f $(1)/Dockerfile .
endef

build-ubi8-base-rust-builder: NAME = ubi8-base-rust-builder
build-ubi8-base-rust-builder:
	$(call build,${NAME})

push-ubi8-base-rust-builder : build-ubi8-base-rust-builder, login
	$(call push,${NAME})

login:
ifndef DOCKER_PASSWORD
		$(error DOCKER_PASSWORD is undefined)
endif
ifndef DOCKER_USER
		$(error DOCKER_USER is undefined)
endif
	echo "${DOCKER_PASSWORD}" | docker login --username ${DOCKER_USER} --password-stdin docker.stackable.tech

