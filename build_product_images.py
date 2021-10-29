#!/bin/env python
"""
Kafka 2.5
Kafka 2.6
Kafka 2.7
Kafka 2.8
Kafka 3.0

Plattform:
2022-Q1: Kafka Operator 7.4 -> Kafka 2.5-2.6, Image Major Version 1
2022-Q2: Kafka Operator 7.5 -> Kafka 2.5-2.7, Image Version 1
2022-Q3: Kafka Operator 8.0 -> Kafka 2.5-2.8, Image Version 2
2024-Q4: Kafka Operator 9.1 -> Kafka 2.5-3.3, Image Version 3
         Trino 392-412, Image Version 7

Docker Image
kafka:2.5-1.0
kafka:2.5-1.1
kafka:2.5-1.2

kafka:2.5-2.0
kafka:2.5-2.1 <- kafka:2.5-2
kafka:2.6-2.0
kafka:2.7-2.0
kafka:2.8-2.0 <- kafka:2.8-2

kafka:2.5-3.0

kafka:3.3-3.0

CRD:
Kafka Operator 7.5
kind: KafkaCluster
...
version: 2.5   -> kafka:2.5-1


Kafka Operator 8.0
kind: KafkaCluster
...
version: 2.5   -> kafka:2.5-2


Kafka Operator 7.5
kind: KafkaCluster
...
image: tyrell/kafka-evil:2.5-foobar

"""

import conf
import argparse
import subprocess
import sys

def parse_args():
    parser = argparse.ArgumentParser(description="Build and publish product images.")
    parser.add_argument("-p", "--product", help="Product name")
    parser.add_argument("-i", "--image_version", help="Image version")
    parser.add_argument("-d", "--dry", help="Dry run.", action='store_true')
    return parser.parse_args()

def build_and_publish_image(image_version: str, product: dict) -> list:
    commands = []
    for v in product['versions']:
        image_name=f'docker.stackable.tech/stackable/{product["name"]}'
        tags = ['-t', f'{image_name}:latest', '-t', f'{image_name}:{v}-{image_version}']
        build_args=['--build-arg', f'PRODUCT_VERSION={v}']

        commands.append(['docker', 'build', '--force-rm', *build_args, *tags, product["name"]])
        commands.append(['docker', 'push', '--all-tags', image_name])
        
    return commands

def run_commands(dry: bool, commands: list):
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

    commands = build_and_publish_image(args.image_version, product[0])

    run_commands(args.dry, commands)

if __name__ == "__main__":
    main()