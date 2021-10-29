#!/bin/env python
"""
Build and possibly publish product images. It doesn't login to any registry when publishing
but it assumes a `docker login` has been performed before.

Usage: build_product_images.py --help

Example:

    build_product_images.py --product kafka -image_version 0.1 --push

This whill build an image for each Apache Kafka version configured in conf.py
"""

import conf
import argparse
import subprocess
import sys

def parse_args():
    parser = argparse.ArgumentParser(description="Build and publish product images. See conf.py for details regarding product versions.")
    parser.add_argument("-r", "--registry", help="Image registry to publish to.", default='docker.stackable.tech')
    parser.add_argument("-p", "--product", help="Product name")
    parser.add_argument("-i", "--image_version", help="Image version")
    parser.add_argument("-u", "--push", help="Push images", action='store_true')
    parser.add_argument("-d", "--dry", help="Dry run.", action='store_true')
    return parser.parse_args()

def build_image_args(version) -> list[str]:
    """
    Returns a list of --build-arg command line arguments that are used by the
    docker build command.

    Arguments:
    - version: Can be a str, in which case it's considered the PRODUCT_VERSION
                or a dict.
    """
    result = []

    if isinstance(version, dict):
        for k, v in version.items():
            result.extend(['--build-arg', f'{k.upper()}={v}'])
    elif isinstance(version, str):
        result=['--build-arg', f'PRODUCT_VERSION={version}']
    else:
        raise ValueError(f'Unsupported version object: {version}')

    return result

def build_image_tags(image_name: str, image_version: str, product_version) -> list[str]:
    """
    Returns a list of --tag command line arguments that are used by the
    docker build command.
    """
    result = []

    if isinstance(product_version, dict):
        all_versions = "-".join([v for v in product_version.values()])
        result.extend(['-t', f'{image_name}:{all_versions}-{image_version}'])
    elif isinstance(product_version, str):
        result=['-t', f'{image_name}:{product_version}-{image_version}']
    else:
        raise ValueError(f'Unsupported version object: {product_version}')

    return result

def build_and_publish_image(args, product: dict) -> list[list[str]]:
    """
    Returns a list of commands that need to be run in order to build and
    publish product images.
    """
    commands = []
    for v in product['versions']:
        image_name=f'{args.registry}/stackable/{product["name"]}'
        tags = build_image_tags(image_name, args.image_version, v)
        build_args = build_image_args(v)

        commands.append(['docker', 'build', *build_args, *tags, product["name"]])
        if args.push:
            commands.append(['docker', 'push', '--all-tags', image_name])
        
    return commands

def run_commands(dry: bool, commands: list):
    """
    Runs the commands to build and publish images. In dry-run mode it only
    lists the command on stdout.
    """
    for cmd in commands:
        if dry:
            subprocess.run(['echo', *cmd])
        else:
            ret = subprocess.run(cmd)
            if ret.returncode != 0:
                sys.exit(1)

def main():
    args = parse_args()

    product = list(filter(lambda p: p['name'] == args.product, conf.products))

    if len(product) != 1:
        print(f"Product {args.product} not found in conf.py")
        sys.exit(1)

    commands = build_and_publish_image(args, product[0])

    run_commands(args.dry, commands)

if __name__ == "__main__":
    main()
