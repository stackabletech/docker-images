#!/bin/env python
"""
Build and possibly publish product images. It doesn't login to any registry when publishing
but it assumes a `docker login` has been performed before.

Usage: build_product_images.py --help

Example:

    build_product_images.py --product zookeeper,kafka -image_version 0.1 --push

This whill build an image for each Apache Zookeeper and APache Kafka version configured in conf.py
"""

import conf
import argparse
import subprocess
import sys
import re

def parse_args():
    parser = argparse.ArgumentParser(description="Build and publish product images. See conf.py for details regarding product versions.")
    parser.add_argument("-r", "--registry", help="Image registry to publish to.", default='docker.stackable.tech')
    parser.add_argument("-p", "--product", help="Product names to build as a comma separated list", type=str)
    parser.add_argument("-i", "--image_version", help="Image version", required=True)
    parser.add_argument("-u", "--push", help="Push images", action='store_true')
    parser.add_argument("-d", "--dry", help="Dry run.", action='store_true')
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
            result.extend(['--build-arg', f'{k.upper()}={v}'])
    elif isinstance(version, str):
        result=['--build-arg', f'PRODUCT={version}']
    else:
        raise ValueError(f'Unsupported version object: {version}')

    return result

def build_image_tags(image_name, image_version, product_version):
    """
    Returns a list of --tag command line arguments that are used by the
    docker build command.
    """
    result = []

    latest_image_version = re.search(r'^\d+', image_version)[0]

    if isinstance(product_version, dict):
        dep_versions = "-".join([f'{key}{value}' for key, value in product_version.items() if key != "product"])
        full_versions = "-".join([product_version['product'], dep_versions, f'stackable{image_version}'])

        result.extend(['-t', f'{image_name}:{full_versions}'])

        if latest_image_version != image_version:
            latest_versions = "-".join([product_version['product'], dep_versions, f'stackable{latest_image_version}'])
            result.extend(['-t', f'{image_name}:{latest_versions}'])

    elif isinstance(product_version, str):
        result=['-t', f'{image_name}:{product_version}-stackable{image_version}']
        if latest_image_version != image_version:
            result.extend(['-t', f'{image_name}:{product_version}-stackable{latest_image_version}'])
    else:
        raise ValueError(f'Unsupported version object: {product_version}')

    return result

def build_and_publish_image(args, products):
    """
    Returns a list of commands that need to be run in order to build and
    publish product images.
    """
    commands = []

    for p in products:
        for v in p['versions']:
            image_name=f'{args.registry}/stackable/{p["name"]}'
            tags = build_image_tags(image_name, args.image_version, v)
            build_args = build_image_args(v)

            commands.append(['docker', 'build', *build_args, *tags, p["name"]])
            if args.push:
                commands.append(['docker', 'push', '--all-tags', image_name])
        
    return commands

def run_commands(dry, commands):
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

def products_to_build(product_names, products):
    if not product_names:
        return products
    else:
        pnd = {n: 1 for n in product_names.split(',')}
        return list(filter(lambda p: p['name'] in pnd, products))

def main():
    args = parse_args()

    products = products_to_build(args.product, conf.products)

    commands = build_and_publish_image(args, products)

    run_commands(args.dry, commands)

if __name__ == "__main__":
    main()
