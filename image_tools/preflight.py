"""OpenShift image validation

Run RedHat's preflight command on images for a product.

Requirements:
* openshift-preflight. See: https://github.com/redhat-openshift-ecosystem/openshift-preflight

Usage:

    python -m image_tools.preflight -p opa -i 23.1.0
"""

from image_tools.lib import Command
from image_tools.bake import generate_bakefile
from image_tools.args import parse

from typing import List, Dict, Any
from subprocess import CalledProcessError, run
import json
import sys
import logging


def get_images_for_target(product: str, bakefile: Dict[str, Any]) -> List[str]:
    tags = []

    for target in bakefile.get("group", {}).get(product, {}).get("targets", []):
        tags.extend(get_images_for_target(target, bakefile))

    tags.extend(bakefile["target"].get(product, {}).get("tags", []))

    return tags


def main():
    """Generate a Docker bake file from conf.py and build the given args.product images."""
    logging.basicConfig(encoding="utf-8", level=logging.INFO, format='%(asctime)s %(levelname)s %(message)s')

    args = parse()
    bakefile = generate_bakefile(args)

    images = get_images_for_target(args.product, bakefile)
    if not images:
        logging.error("No images found for product [%s]", args.product)
        return 1

    preflight_cmds = {image: Command(args=["preflight", "check", "container", image]) for image in images}

    failures = {}
    for image, cmd in preflight_cmds.items():
        if args.dry:
            print(str(cmd))
        else:
            try:
                preflight_result = run(cmd.args, input=cmd.input, check=True, capture_output=True)
                preflight_json = json.loads(preflight_result.stdout)
                failures[image] = preflight_json.get("results", {}).get("failed", [])
            except CalledProcessError as error:
                failures[image] = [error.stderr.decode('utf-8')]

    for image, ifails in failures.items():
        if len(ifails) == 0:
            logging.info("Image [%s] preflight check successful.", image)
        else:
            logging.error("Image [%s] preflight check failures:\n%s", image, "\n".join(ifails))

    fail_count = sum(map(lambda f: len(f), failures.values()))
    return fail_count


if __name__ == '__main__':
    sys.exit(main())
