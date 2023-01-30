#!/usr/bin/env python
"""
Build and possibly publish product images.

Requirements:

- Python 3
- Docker with buildx. Installation details here: https://github.com/docker/buildx

Usage: build_product_images.py --help

Example:

    build_product_images.py --product zookeeper --image-version 23.1.1 --architecture linux/amd64

This will build all images parsed from conf.py (e.g. `docker.stackable.tech/stackable/zookeeper:3.8.0-stackable23.1.1`) for the linux/amd64 architecture.
To also push the image to a remote registry, add the the `--push` argument.

NOTE: Pushing images to a remote registry assumes you have performed a `docker login` beforehand.

Some images build on top of others. These images are used as base images and might need to be built first:
    1. java-base
    2. ubi8-rust-builder
    3. tools
"""
from os.path import isdir
from typing import List, Dict
from argparse import Namespace, ArgumentParser
import subprocess
import conf
import re


def parse_args() -> Namespace:
    parser = ArgumentParser(
        description="Build and publish product images. Requires docker and buildx (https://github.com/docker/buildx)."
    )
    parser.add_argument("-i", "--image-version", help="Image version", required=True)
    parser.add_argument("-p", "--product", help="Product to build images for")
    parser.add_argument("-u", "--push", help="Push images", action="store_true")
    parser.add_argument("-d", "--dry", help="Dry run.", action="store_true")
    parser.add_argument(
        "-a",
        "--architecture",
        help="Target platform for image. Default: linux/amd64.",
        nargs="+",
        default=["linux/amd64"],
        type=check_architecture_input,
    )
    parser.add_argument(
        "-o",
        "--organization",
        help="Organization name within the given registry. Default: stackable",
        default="stackable",
    )
    parser.add_argument(
        "-r",
        "--registry",
        help="Image registry to publish to. Default: docker.stackable.tech",
        default="docker.stackable.tech",
    )
    return parser.parse_args()


def build_image_args(version, release_version):
    """
    Returns a list of --build-arg command line arguments that are used by the
    docker build command.

    Arguments:
    - version: Can be a str, in which case it's considered the PRODUCT
                or a dict.
    """
    result = []

    if isinstance(version, dict):
        for k, v in version.items():
            result.extend(["--build-arg", f"{k.upper()}={v}"])
        result.extend(["--build-arg", f"RELEASE={release_version}"])
    elif isinstance(version, str) and isinstance(release_version, str):
        result = [
            "--build-arg",
            f"PRODUCT={version}",
            "--build-arg",
            f"RELEASE={release_version}",
        ]
    else:
        raise ValueError(f"Unsupported version object: {version}")

    return result


def build_image_tags(image_name: str, image_version: str, product_version: str) -> List[str]:
    """
    Returns the --tag command line arguments that are used by the docker build command.
    """
    return [
        "-t",
        f"{image_name}:{product_version}-stackable{image_version}",
    ]


def build_and_publish_image(args: Namespace, product_name: str, versions: Dict[str, str]) -> List[List[str]]:
    """
    Returns a list of commands that need to be run in order to build and
    publish product images.

    For local building, builder instances are supported.
    """
    image_name = f'{args.registry}/{args.organization}/{product_name}'
    tags = build_image_tags(
        image_name, args.image_version, versions["product"]
    )
    build_args = build_image_args(versions, args.image_version)

    commands = [
        "docker",
        "buildx",
        "build",
        *build_args,
        *tags,
        "-f",
        f"{ product_name }/Dockerfile",
        "--platform",
        ",".join(args.architecture),
    ]

    if args.push:
        commands.append(
            "--push",
        )
    if not args.push and len(args.architecture) == 1:
        commands.append(
            "--load",
        )

    commands.append(".")

    return [commands]


def run_commands(dry, commands):
    """
    Runs the commands to build and publish images. In dry-run mode it only
    lists the command on stdout.
    """
    for cmd in commands:
        if dry:
            subprocess.run(["echo", *cmd], check=True)
        else:
            subprocess.run(cmd, check=True)


def create_virtual_environment(args):
    commands = [["docker", "buildx", "create", "--name", "builder", "--use"]]
    run_commands(args.dry, commands)


def remove_virtual_environment(args):
    commands = [["docker", "buildx", "rm", "builder"]]
    run_commands(args.dry, commands)


def check_architecture_input(architecture):
    supported_arch = ["linux/amd64", "linux/arm64"]

    if architecture not in supported_arch:
        raise ValueError(
            f"Architecture {architecture} not supported. Supported: {supported_arch}"
        )

    return architecture


def main():
    args = parse_args()

    if len(args.architecture) > 1:
        create_virtual_environment(args)

    try:
        for product in conf.products:
            product_name = product.get("name")
            if args.product is not None and (product_name != args.product):
                continue

            for version_dict in product.get("versions"):
                if isdir(product_name):
                    commands = build_and_publish_image(args, product_name, version_dict)
                    run_commands(args.dry, commands)
    finally:
        if len(args.architecture) > 1:
            remove_virtual_environment(args)


if __name__ == "__main__":
    main()
