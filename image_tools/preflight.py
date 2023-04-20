"""OpenShift image validation

Run RedHat's preflight command on images for a product.

Requirements:
* openshift-preflight. See: https://github.com/redhat-openshift-ecosystem/openshift-preflight

Usage:

    python -m image_tools.preflight -p opa -i 23.1.0
"""

from argparse import Namespace, ArgumentParser
from typing import List, Dict, Any
import subprocess
import json
import sys
import logging

from image_tools.lib import Command
from image_tools.bake import generate_bakefile
from image_tools.args import check_architecture_input, check_image_version_format
from image_tools.conf import open_shift_projects


def get_images_for_target(product: str, bakefile: Dict[str, Any]) -> List[str]:
    tags = []

    for target in bakefile.get("group", {}).get(product, {}).get("targets", []):
        tags.extend(get_images_for_target(target, bakefile))

    tags.extend(bakefile["target"].get(product, {}).get("tags", []))

    return tags


def get_preflight_failures(image_commands: Dict[str, Command]) -> Dict[str, List[Any]]:
    """Run preflight commands for each image and return the failure field of the response."""
    failures = {}
    for image, cmd in image_commands.items():
        try:
            preflight_result = subprocess.run(
                cmd.args, input=cmd.input, check=True, capture_output=True
            )
            preflight_json = json.loads(preflight_result.stdout)
            failures[image] = preflight_json.get("results", {}).get("failed", [])
        except subprocess.CalledProcessError as error:
            failures[image] = [error.stderr.decode("utf-8")]
        except FileNotFoundError:
            failures[image] = [
                "preflight: command not found. Install from https://github.com/redhat-openshift-ecosystem/openshift-preflight"
            ]
        except json.JSONDecodeError as error:
            failures[image] = [error.msg]
    return failures


def parse_args() -> Namespace:
    parser = ArgumentParser(
        description="Run OpenShift certification checks and submit results to RedHat Partner Connect portal"
    )
    parser.add_argument(
        "-i",
        "--image-version",
        help="Image version",
        required=True,
        type=check_image_version_format,
    )
    parser.add_argument(
        "-p", "--product", help="Product to build images for", required=True
    )
    parser.add_argument("-s", "--submit", help="Submit results", action="store_true")
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
        "-t",
        "--token",
        help="RedHat portal API token",
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

    result = parser.parse_args()

    if result.submit and not result.token:
        raise ValueError("Missing API token for submitting results.")

    return result


def preflight_commands(images: List[str], args: Namespace) -> Dict[str, Command]:
    """A mapping of image name to preflight command"""
    result = {}
    for img in images:
        cmd_args = ["preflight", "check", "container", img]
        if args.submit:
            cmd_args.extend(
                [
                    "--submit",
                    "--pyxis-api-token",
                    args.token,
                    "--certification-project-id",
                    f"ospid-{open_shift_projects[args.product]['id']}",
                ]
            )
        result[img] = Command(args=cmd_args)
    return result


def main() -> int:
    """Run OpenShift verification checks against the product images."""
    logging.basicConfig(
        encoding="utf-8",
        level=logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s",
    )

    args = parse_args()
    bakefile = generate_bakefile(args)

    # List of images (with tags) to apply preflight checks to.
    # Filter out images with platform release tags such as "23.4" and
    # only check images with patch versions such as "23.4.0".
    images = list(
        filter(
            lambda i: i.endswith(args.image_version),
            get_images_for_target(args.product, bakefile),
        )
    )
    if not images:
        logging.error("No images found for product [%s]", args.product)
        return 1

    # A mapping of image name to preflight command
    image_commands = preflight_commands(images, args)

    if args.dry:
        for _, cmd in image_commands.items():
            logging.info(str(cmd))
        return 0

    # Run preflight and return failures
    failures = get_preflight_failures(image_commands)

    for image, img_fails in failures.items():
        if len(img_fails) == 0:
            logging.info("Image [%s] preflight check successful.", image)
        else:
            logging.error(
                "Image [%s] preflight check failures: %s", image, ",".join(img_fails)
            )

    fail_count = sum(map(len, failures.values()))
    return fail_count


if __name__ == "__main__":
    sys.exit(main())
