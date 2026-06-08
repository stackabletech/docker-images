#!/usr/bin/env bash
# Determine the upstream-tested tool/dependency versions for a given Apache Superset
# version, to fill in the `[versions."X.Y.Z".build-arguments]` section of
# superset/boil-config.toml.
#
# Usage: ./upstream-versions.sh <superset-version>
# Example: ./upstream-versions.sh 6.1.0
#
# Sources used:
#   nodejs-version  -> superset-frontend/.nvmrc at the Superset tag
#   flask-appbuilder -> requirements/base.txt at the Superset tag (informational; must
#                       match the pin in superset/stackable/constraints/<version>/)
#   authlib-version -> requirements/extra.txt at the matching Flask-AppBuilder release
#   python-version  -> `ARG PY_VER` in the upstream Dockerfile (their default image),
#                      plus the CI-tested versions from .github/actions/setup-backend.
#                      Picking among the CI-tested versions is a human decision.
#   uv/nvm-version  -> independent of Superset, latest GitHub release

set -euo pipefail

VERSION=${1:?Usage: $0 <superset-version> (e.g. 6.1.0)}
RAW="https://raw.githubusercontent.com"

fetch() {
    curl --fail --silent --show-error --location "$1"
}

latest_release() {
    fetch "https://api.github.com/repos/$1/releases/latest" \
        | sed -nE 's/.*"tag_name": *"([^"]+)".*/\1/p' | head -1
}

# Node.js: pinned by upstream in superset-frontend/.nvmrc (strip the leading `v`)
nodejs=$(fetch "$RAW/apache/superset/$VERSION/superset-frontend/.nvmrc" | tr -d 'v[:space:]')

# Flask-AppBuilder: pinned in requirements/base.txt
fab=$(fetch "$RAW/apache/superset/$VERSION/requirements/base.txt" \
    | sed -nE 's/^flask-appbuilder==([0-9.]+).*/\1/p' | head -1)

# Authlib: pinned by Flask-AppBuilder in requirements/extra.txt
authlib=$(fetch "$RAW/dpgaspar/Flask-AppBuilder/release/$fab/requirements/extra.txt" \
    | sed -nE 's/^authlib==([0-9.]+).*/\1/p' | head -1)

# Python: the upstream image default (ARG PY_VER in the Dockerfile) ...
py_default=$(fetch "$RAW/apache/superset/$VERSION/Dockerfile" \
    | sed -nE 's/^ARG PY_VER=([0-9]+\.[0-9]+).*/\1/p' | head -1)
# ... and all CI-tested versions (the previous/current/next aliases)
py_tested=$(fetch "$RAW/apache/superset/$VERSION/.github/actions/setup-backend/action.yml" \
    | sed -nE 's/.*PYTHON_VERSION=([0-9]+\.[0-9]+).*/\1/p' | sort -uV | xargs | sed 's/ /, /g')

# uv and nvm are independent of Superset, use the latest release
uv=$(latest_release astral-sh/uv)
nvm=$(latest_release nvm-sh/nvm)

cat <<EOF
Superset $VERSION
flask-appbuilder: $fab (informational, must match stackable/constraints/$VERSION/constraints.txt)
authlib-version:  $authlib
python-version:   $py_default (upstream image default; CI-tested: $py_tested)
nodejs-version:   $nodejs
uv-version:       $uv (latest release)
nvm-version:      $nvm (latest release)
EOF

# Sanity check: the vendored constraints must pin the same Flask-AppBuilder version,
# otherwise the authlib version derived above is based on the wrong FAB release.
constraints="$(dirname "$0")/stackable/constraints/$VERSION/constraints.txt"
if [ -f "$constraints" ]; then
    local_fab=$(sed -nE 's/^flask-appbuilder==([0-9.]+).*/\1/p' "$constraints" | head -1)
    if [ "$local_fab" != "$fab" ]; then
        echo >&2
        echo "WARNING: $constraints pins flask-appbuilder==$local_fab but upstream pins $fab" >&2
        exit 1
    fi
else
    echo >&2
    echo "NOTE: no constraints file at $constraints yet - create it before building" >&2
fi
