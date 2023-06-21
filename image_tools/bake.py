"""Image builder

Requirements: docker and buildx.

Usage:

    python -m image_tools.bake -p opa -i 22.12.0
"""
from typing import List, Dict, Any
from argparse import Namespace
from subprocess import run
import json
import re

from image_tools.lib import Command
from image_tools import conf
from image_tools.args import parse


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


def build_image_tags(
    image_name: str, image_version: str, product_version: str,
) -> List[str]:
    """
    Returns a list of --tag command line arguments that are used by the
    docker build command.
    Each image is tagged with two tags as follows:
        1. <product>-<image>
        2. <product>-<platform>
    """
    arr = re.split("\\.", image_version)
    platform_version = arr[0] + "." + arr[1]

    return [
        f"{image_name}:{product_version}-stackable{image_version}",
        f"{image_name}:{product_version}-stackable{platform_version}",
    ]


def generate_bakefile(args: Namespace) -> Dict[str, Any]:
    """
    Generates a Bakefile (https://docs.docker.com/build/bake/file-definition/) describing how to build the whole image graph.

    build_and_publish_images() ensures that only the desired images are actually built.
    """
    targets = {}
    groups = {}
    product_names = [product["name"] for product in conf.products]
    for product in conf.products:
        product_name: str = product["name"]
        product_targets = {}
        for version_dict in product.get("versions", []):
            product_targets.update(
                bakefile_product_version_targets(
                    args, product_name, version_dict, product_names
                )
            )
        groups[product_name] = {
            "targets": list(product_targets.keys()),
        }
        targets.update(product_targets)
    groups["default"] = {
        "targets": list(groups.keys()),
    }
    return {
        "target": targets,
        "group": groups,
    }


def bakefile_target_name_for_product_version(product_name: str, version: str) -> str:
    """
    Creates a normalized Bakefile target name for a given (product, version) combination.
    """
    return f"{ product_name }-{ version.replace('.', '_') }"


def bakefile_product_version_targets(
    args: Namespace,
    product_name: str,
    versions: Dict[str, str],
    product_names: List[str],
):
    """
    Creates Bakefile targets defining how to build a given product version.

    A product is assumed to depend on another if it defines a `versions` field with the same name as the other product.
    """
    image_name = f"{args.registry}/{args.organization}/{product_name}"
    tags = build_image_tags(image_name, args.image_version, versions["product"])
    build_args = build_image_args(versions, args.image_version)

    if args.multiarch:
        if args.architecture[0] == "linux/amd64":
            tags[0] = tags[0] + "-amd64"
        else:
            tags[0] = tags[0] + "-arm64" 
    return {
        bakefile_target_name_for_product_version(product_name, versions["product"]): {
            "dockerfile": f"{ product_name }/Dockerfile",
            "tags": tags,
            "args": build_args,
            "platforms": args.architecture,
            "context": ".",
            "contexts": {
                f"stackable/image/{dep_name}": f"target:{bakefile_target_name_for_product_version(dep_name, dep_version)}"
                for dep_name, dep_version in versions.items()
                if dep_name in product_names
            },
        },
    }


def bake_command(args: Namespace, product_name: str, bakefile) -> Command:
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

    return Command(
        args=[
            "docker",
            "buildx",
            "bake",
            "--file",
            "-",
            *([] if product_name is None else [product_name]),
            *target_mode,
        ],
        stdin=json.dumps(bakefile),
    )


def main():
    """Generate a Docker bake file from conf.py and build the given args.product images."""
    args = parse()
    bakefile = generate_bakefile(args)

    cmd = bake_command(args, args.product, bakefile)

    if args.dry:
        print(cmd)
    else:
        run(cmd.args, input=cmd.input, check=True)


if __name__ == "__main__":
    main()
