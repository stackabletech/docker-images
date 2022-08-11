.PHONY: build-ubi8-rust-builder, push-ubi8-rust-builder, login

ORG=stackable

REPO   := docker.stackable.tech/${ORG}
TAG    := $(shell git rev-parse --short HEAD)

define push
	docker push --all-tags ${REPO}/$(1)
endef

define build
	@docker build --force-rm -t "${REPO}/${1}:${TAG}" -t "${REPO}/${1}:latest" -f $(1)/Dockerfile .
endef

define build-multi-arch
	@docker buildx create --name builder --use 
	@docker buildx build --force-rm -t "${REPO}/${1}:${TAG}" -t "${REPO}/${1}:latest" -f $(1)/Dockerfile --platform linux/arm64,linux/amd64 --push .
	@docker buildx rm builder
endef

build-ubi8-rust-builder: NAME = ubi8-rust-builder
build-ubi8-rust-builder:
	$(call build,${NAME})

build-ubi8-rust-builder-multi-arch: NAME = ubi8-rust-builder
build-ubi8-rust-builder-multi-arch:
	$(call build-multi-arch,${NAME})	

push-ubi8-rust-builder: NAME = ubi8-rust-builder
push-ubi8-rust-builder : build-ubi8-rust-builder login
	$(call push,${NAME})

login:
ifndef DOCKER_PASSWORD
		$(error DOCKER_PASSWORD is undefined)
endif
ifndef DOCKER_USER
		$(error DOCKER_USER is undefined)
endif
	echo "${DOCKER_PASSWORD}" | docker login --username ${DOCKER_USER} --password-stdin docker.stackable.tech
