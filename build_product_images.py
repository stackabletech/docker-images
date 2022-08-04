#!/usr/bin/env python
"""
Build and possibly publish product images. It doesn't login to any registry when publishing
but it assumes a `docker login` has been performed before.

Usage: build_product_images.py --help

Example:

    build_product_images.py --product zookeeper,kafka --image_version 0.1.0 -a linux/amd64 -u

This will build an image for each Apache ZooKeeper and Apache Kafka version configured in conf.py

Multiarchitecture builds:
This assumes that the following images are available for target architecture:
    1. Java-Base 11, 1.8.0
    2. ubi8-rust-builder
    3. Tools 0.2.0
"""

import conf
import argparse
import subprocess


def parse_args():
    parser = argparse.ArgumentParser(
        description="Build and publish product images. See conf.py for details regarding product versions."
    )
    parser.add_argument(
        "-r",
        "--registry",
        help="Image registry to publish to.",
        default="docker.stackable.tech",
    )
    parser.add_argument(
        "-p", "--product", help="Product to build", type=str, required=True
    )
    parser.add_argument("-i", "--image_version", help="Image version", required=True)
    parser.add_argument(
        "-v",
        "--product_version",
        help="Product version to build an image for",
        required=True,
    )
    parser.add_argument("-u", "--push", help="Push images", action="store_true")
    parser.add_argument("-d", "--dry", help="Dry run.", action="store_true")
    parser.add_argument(
        "-a",
        "--architecture",
        help="Target platform for image, Expecting -a <platform 1> <platform 2> ... At least one argument",
        nargs="+",
        required=True,
        type=check_architecture_input
    )
    return parser.parse_args()


def build_image_args(version):
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
    elif isinstance(version, str):
        result = ["--build-arg", f"PRODUCT={version}"]
    else:
        raise ValueError(f"Unsupported version object: {version}")

    return result


def build_image_tags(image_name, image_version, product_version):
    """
    Returns a list of --tag command line arguments that are used by the
    docker build command.
    Each image is tagged with two tags as follows:
        1. <product>-<image>
    """

    if isinstance(product_version, dict):
        product_version = product_version["product"]

    return [
        "-t",
        f"{image_name}:{product_version}-stackable{image_version}",
    ]


def build_and_publish_image(args, product) -> list[list[str]]:
    """
    Returns a list of commands that need to be run in order to build and
    publish product images.

    For local building, builder instances are supported.
    """
    image_name = f'{args.registry}/stackable/{product["name"]}'
    tags = build_image_tags(image_name, args.image_version, args.product_version)
    build_args = build_image_args(product["versions"][0])

    commands = [
        "docker",
        "buildx",
        "build",
        *build_args,
        *tags,
        "-f",
        product["name"] + "/Dockerfile",
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
            subprocess.run(["echo", *cmd])
        else:
            subprocess.run(cmd, check=True)


def product_to_build(product_name, product_version, products):
    product_to_build = [p for p in products if p["name"] == product_name]

    assert len(product_to_build) == 1

    product_to_build = product_to_build.pop()

    product_to_build_version = []

    for version in product_to_build["versions"]:
        if isinstance(version, dict):
            if version["product"] == product_version:
                product_to_build_version.append(version)
        elif isinstance(version, str):
            if version == product_version:
                product_to_build_version.append(version)

    if len(product_to_build_version) == 1:
        return {
            "name": product_name,
            "versions": product_to_build_version,
        }
    else:
        return None


def create_virtual_enviroment(args):

    commands = []

    commands.append(["docker", "buildx", "create", "--name", "builder", "--use"])
    run_commands(args.dry, commands)


def remove_virtual_enviroment(args):

    commands = []
    commands.append(["docker", "buildx", "rm", "builder"])
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

    product = product_to_build(args.product, args.product_version, conf.products)

    if product is None:
        raise ValueError(
            f"No products configured for product {args.product} and version {args.product_version}"
        )

    if len(args.architecture) > 1:
        create_virtual_enviroment(args)

    try:
        commands = build_and_publish_image(args, product)

        run_commands(args.dry, commands)

    finally:
        if len(args.architecture) > 1:
            remove_virtual_enviroment(args)


if __name__ == "__main__":
    main()
