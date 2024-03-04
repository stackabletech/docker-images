.PHONY: build-ubi8-rust-builder, push-ubi8-rust-builder, login

REPO   := docker.stackable.tech/stackable
TAG    := $(shell git rev-parse --short HEAD)
ARCH   := $(shell arch)
NAME   := ubi8-rust-builder
SHA-amd := $(shell export sha-x86_64)
SHA-arm := $(shell export sha-aarch64)

define push
	docker push --all-tags ${REPO}/$(1)
endef

define build
	@docker build --iidfile imageid --force-rm -t "${REPO}/${1}:${TAG}-${ARCH}" -t "${REPO}/${1}:latest-${ARCH}" -f $(1)/Dockerfile .
endef

define manifest
	@docker manifest create "${REPO}/${1}:latest" ${REPO}/${1}@${SHA-amd} ${REPO}/${1}@${SHA-arm}	
endef

define manifest_push
	docker manifest push ${REPO}/${1}:latest
endef

build-ubi8-rust-builder:
build-ubi8-rust-builder:
	$(call build,${NAME})

push-ubi8-rust-builder:
push-ubi8-rust-builder : build-ubi8-rust-builder login
	$(call push,${NAME})

pull-ubi8-rust-builder:
pull-ubi8-rust-builder: login
	$(call pull-arm64,${NAME})
	$(call pull-amd64,${NAME})

build-manifest-list:
build-manifest-list:
	$(call manifest,${NAME})

push-manifest-list:
push-manifest-list: login build-manifest-list 
	$(call manifest_push,${NAME})

login:
ifndef DOCKER_PASSWORD
		$(error DOCKER_PASSWORD is undefined)
endif
ifndef DOCKER_USER
		$(error DOCKER_USER is undefined)
endif
	echo "${DOCKER_PASSWORD}" | docker login --username ${DOCKER_USER} --password-stdin docker.stackable.tech

