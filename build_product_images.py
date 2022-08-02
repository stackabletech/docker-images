#!/usr/bin/env python
"""
Build and possibly publish product images. It doesn't login to any registry when publishing
but it assumes a `docker login` has been performed before.

Usage: build_product_images.py --help

Example:

    build_product_images.py --product zookeeper,kafka --image_version 0.1.0 --push

This will build an image for each Apache ZooKeeper and Apache Kafka version configured in conf.py

Multiarchitecture builds:
This assumes that the following images are available for target architecture:
    1. Java-Base 11, 1.8.0
    2. ubi8-rust-builder
    3. Tools 0.2.0

For native nodes, it assumes that native node exists in docker context ls
"""

import conf
import argparse
import subprocess
import sys
import platform
import docker
import copy


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
    parser.add_argument("-p", "--product", help="Product to build", type=str, required=True)
    parser.add_argument("-i", "--image_version", help="Image version", required=True)
    parser.add_argument(
        "-v",
        "--product_version",
        help="Product version to build an image for",
        required=True,
    )
    parser.add_argument("-u", "--push", help="Push images", action="store_true")
    parser.add_argument("-d", "--dry", help="Dry run.", action="store_true")
    parser.add_argument("-a", "--architecture", help="Target platform for image")
    parser.add_argument("-c", "--check", help="Setting the flag will enable dependency checks and building layers", action="store_true")
    parser.add_argument("-m", "--multiarch", help="Build and publish multi-architecture images", action="store_true")
    parser.add_argument("-n", "--node", help="Create nodes to a builder. First is Master.", type=str)
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


def build_and_publish_image(args, product):
    """
    Returns a list of commands that need to be run in order to build and
    publish product images.

    For local building, builder instances are supported.
    """
    commands = []
    push = ""
    image_name = f'{args.registry}/stackable/{product["name"]}'
    tags = build_image_tags(image_name, args.image_version, args.product_version)
    build_args = build_image_args(product["versions"][0])

    if args.push:
        push = "--push"

    # Multiarch builds
    if args.multiarch:
        commands.append(
            [
                "docker",
                "buildx",
                "build",
                *build_args,
                *tags,
                "-f",
                product["name"] + "/Dockerfile",
                "--platform",
                "linux/amd64,linux/arm64",
                push,
                ".",
            ]
        )
    # local builds / single architecture
    else:
        commands.append(
            [
                "docker",
                "buildx",
                "build",
                *build_args,
                *tags,
                "-f",
                product["name"] + "/Dockerfile",
                "--platform",
                "linux/" + check_platform(args.architecture),
                "--load",
                ".",
            ]
        )

        if args.push:
            commands.append(["docker", "push", "--all-tags", image_name])

    return commands


def run_commands(dry, commands):
    """
    Runs the commands to build and publish images. In dry-run mode it only
    lists the command on stdout.
    """
    for cmd in commands:
        if dry:
            subprocess.run(["echo", *cmd])
        else:
            ret = subprocess.run(cmd)
            if ret.returncode != 0:
                sys.exit(1)


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


"""
In generall, the following checks weather or not dependencies are available on your local machine. This is only useful if a local repository via Docker-Desktop or
something simular is used. However, this is not checking if a dependency of certain architecutre is existent in a repository!
"""


def check_or_build_dependencies(args, architecture, products):
    """
    Checks if dependencies are currently build on local system, if not they get build
    """

    client = docker.from_env()
    tools = False
    java = False
    rust_builder = False

    images = client.images.list(filters={"label": "architecture=" + architecture})
    for image in images:
        for tags in image.tags:
            if 'java-base' in tags:
                java = True
                print("Found java-base image")
            if 'ubi8-rust-builder' in tags:
                rust_builder = True
                print("Found rust builder image")
            if 'tools' in tags:
                tools = True
                print("Found tools image")

    build_dependencies(java, tools, rust_builder, args, products)


def build_dependencies(java, tools, rust_builder, args, products):
    """
    Builds neccessary dependencies for images if not available on system
    """

    args_dummy = copy.deepcopy(args)

    if not rust_builder:
        print("Building rust builder")
        subprocess.run("make")

    if not java:
        args_dummy.image_version = '0'
        args_dummy.product_version = '11'
        print('Building dependency Java-Base', args_dummy.product_version)

        run_commands(args_dummy.dry, build_and_publish_image(args_dummy, product_to_build('java-base', args_dummy.product_version, products)))

        args_dummy.image_version = '0'
        args_dummy.product_version = '1.8.0'
        print('Building dependency Java-Base', args_dummy.product_version)

        run_commands(args_dummy.dry, build_and_publish_image(args_dummy, product_to_build('java-base', args_dummy.product_version, products)))

    if not tools:
        args_dummy.image_version = '0'
        args_dummy.product_version = '0.2.0'
        print("Building dependency Tools", args_dummy.product_version)

        run_commands(args_dummy.dry, build_and_publish_image(args_dummy, product_to_build('tools', args_dummy.product_version, products)))


def check_platform(architecture):
    """
    Checks if a desired platform is given, gives current platform if not
    """
    if architecture is None:
        architecture = platform.machine()

    return architecture


def create_virtual_enviroment(args):

    commands = []

    commands.append(
        [
            "docker",
            "buildx",
            "create",
            "--name",
            "builder",
            "--use"
        ]
    )
    run_commands(args.dry, commands)


"""
def use_native_node(args):

    commands = []
    nodes = args.node.split(',')

    assert len(nodes) != 0

    commands.append(
        [
            "docker",
            "buildx",
            "create",
            "--use",
            "--name",
            nodes[0]
        ]
    )

    run_commands(args.dry, commands)


def create_nodes(args):

    i = False
    commands = []

    for nodes in args.node.split(','):
        commands = []
        if not i:
            use_native_node(args)
            i = True
        else:
            commands.append(
                [
                    "docker",
                    "buildx",
                    "create",
                    "--name",
                    nodes
                ]
            )
            run_commands(args.dry, commands)
"""


def join_nodes(args):

    nodes = args.node.split(',')

    for node in nodes:
        commands = []
        if node != nodes[0]:
            commands.append(
                [
                    "docker",
                    "buildx",
                    "create",
                    "--name",
                    nodes[0],
                    "--append",
                    node
                ]
            )
            run_commands(args.dry, commands)
            use_builder(args)

        elif len(nodes) == 1:
            use_builder(args)


def use_builder(args):

    nodes = args.node.split(',')
    commands = []

    commands.append(
        [
            "docker",
            "buildx",
            "use", 
            nodes[0]
        ]
    )

    run_commands(args.dry, commands)


def remove_virtual_enviroment(args):

    commands = []
    commands.append(
        [
            "docker",
            "buildx",
            "rm",
            "builder"
        ]
    )
    run_commands(args.dry, commands)


def main():
    args = parse_args()
    print("Current Platform: ", platform.machine())

    if args.check:
        check_or_build_dependencies(args, check_platform(args.architecture), conf.products)

    if args.multiarch:
        if args.node is None:
            create_virtual_enviroment(args)
            
    if args.node is not None:
        #create_nodes(args)
        join_nodes(args) 

    product = product_to_build(args.product, args.product_version, conf.products)

    if product is None:
        raise ValueError(
            f"No products configured for product {args.product} and version {args.product_version}"
        )

    print(product)
    commands = build_and_publish_image(args, product)

    run_commands(args.dry, commands)

    if args.multiarch:
        if args.node is None:
            remove_virtual_enviroment(args)


if __name__ == "__main__":
    main()
