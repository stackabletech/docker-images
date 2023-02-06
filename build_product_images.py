#!/usr/bin/env python
"""
Build and possibly publish product images.

Run doc tests with:

    python -m doctest -v build_product_images.py

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
import json

# This is the stackable release version
DEFAULT_IMAGE_VERSION_FROMATS = [re.compile("[2-9][0-9]\.[1-9][0-2]?\.\d+"), re.compile("[2-9][0-9]\.[1-9][0-2]?\.\d+-rc[1-9]\d?")]


def parse_args() -> Namespace:
    parser = ArgumentParser(
        description="Build and publish product images. Requires docker and buildx (https://github.com/docker/buildx)."
    )
    parser.add_argument("-i", "--image-version", help="Image version", required=True, type=check_image_version_format)
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


def check_image_version_format(image_version) -> str:
    """
    Check image version against allowed formats.

    >>> check_image_version_format("23.4.0")
    '23.4.0'
    >>> check_image_version_format("23.4.0-rc1")
    '23.4.0-rc1'
    >>> check_image_version_format("23.04.0")
    Traceback (most recent call last):
    ...
    ValueError: Invalid image version: 23.04.0
    >>> check_image_version_format("23.4.0.prerelease")
    Traceback (most recent call last):
    ...
    ValueError: Invalid image version: 23.4.0.prerelease
    """
    for p in DEFAULT_IMAGE_VERSION_FROMATS:
        if p.fullmatch(image_version):
            return image_version
    raise ValueError(f"Invalid image version: {image_version}")


def build_image_args(version, release_version):
    """
    Returns a list of --build-arg command line arguments that are used by the
    docker build command.

    Arguments:
    - version: Can be a str, in which case it's considered the PRODUCT
                or a dict.
    """
    result = {}

    if isinstance(version, dict):
        for k, v in version.items():
            result[k.upper()] = v
        result["RELEASE"] = release_version
    elif isinstance(version, str) and isinstance(release_version, str):
        {
            "PRODUCT": version,
            "RELEASE": release_version,
        }
    else:
        raise ValueError(f"Unsupported version object: {version}")

    return result


def build_image_tags(image_name: str, image_version: str, product_version: str) -> List[str]:
    """
    Returns the --tag command line arguments that are used by the docker build command.
    """
    return [
        f"{image_name}:{product_version}-stackable{image_version}",
    ]

def generate_bakefile(args: Namespace):
    """
    Generates a Bakefile (https://docs.docker.com/build/bake/file-definition/) describing how to build the whole image graph.

    build_and_publish_images() ensures that only the desired images are actually built.
    """
    targets = {}
    groups = {}
    contexts = {}
    products = {product["name"]: product for product in conf.products}
    product_names = list(products.keys())
    for product_name, product in products.items():
        product_targets = {}
        for version_dict in product.get("versions"):
            product_targets.update(bakefile_product_version_targets(args, product_name, version_dict, contexts, product_names))
        groups[product_name] = {
            "targets": list(product_targets.keys()),
        }
        targets.update(product_targets)
    return {
        "target": targets,
        "group": groups,
    }

def bakefile_target_name_for_product_version(product_name: str, version: str) -> str:
    """
    Creates a normalized Bakefile target name for a given (product, version) combination.
    """
    return f"{ product_name }-{ version.replace('.', '_') }"

def bakefile_product_version_targets(args: Namespace, product_name: str, versions: Dict[str, str], contexts: Dict[str, str], product_names: List[str]):
    """
    Creates Bakefile targets defining how to build a given product version.

    A product is assumed to depend on another if it defines a `versions` field with the same name as the other product.
    """
    image_name = f'{args.registry}/{args.organization}/{product_name}'
    tags = build_image_tags(
        image_name, args.image_version, versions["product"]
    )
    build_args = build_image_args(versions, args.image_version)

    return {
        bakefile_target_name_for_product_version(product_name, versions['product']): {
            "dockerfile": f"{ product_name }/Dockerfile",
            "tags": tags,
            "args": build_args,
            "platforms": args.architecture,
            "context": ".",
            "contexts": {f"stackable/image/{dep_name}": f"target:{bakefile_target_name_for_product_version(dep_name, dep_version)}" for dep_name, dep_version in versions.items() if dep_name in product_names},
        },
    }


def build_and_publish_image(args: Namespace, product_name: str, bakefile) -> List[List[str]]:
    """
    Returns a list of commands that need to be run in order to build and
    publish product images.

    For local building, builder instances are supported.
    """

    if args.push:
        target_mode = ["--push"]
    elif len(args.architecture) == 1:
        target_mode = ["--load"]
    else:
        target_mode = []

    command = {
        "args": [
            "docker",
            "buildx",
            "bake",
            "--file",
            "-",
            product_name,
            *target_mode,
        ],
        "stdin": json.dumps(bakefile),
    }
    return [command]

def run_commands(dry, commands):
    """
    Runs the commands to build and publish images. In dry-run mode it only
    lists the command on stdout.
    """
    for cmd in commands:
        if isinstance(cmd, dict):
            args = cmd['args']
            stdin = cmd.get('stdin')
        else:
            args = cmd
            stdin = None

        if dry:
            if stdin:
                print(f"{' '.join(args)} <<<EOF")
                print(stdin)
                print("EOF;")
            else:
                print(' '.join(args))
        else:
            subprocess.run(args, input=stdin.encode("UTF-8"), check=True)


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
    bakefile = generate_bakefile(args)

    if len(args.architecture) > 1:
        create_virtual_environment(args)

    try:
        for product in conf.products:
            product_name = product.get("name")
            if args.product is not None and (product_name != args.product):
                continue

            commands = build_and_publish_image(args, product_name, bakefile)
            run_commands(args.dry, commands)
    finally:
        if len(args.architecture) > 1:
            remove_virtual_environment(args)


if __name__ == "__main__":
    main()
