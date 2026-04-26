#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

DRY_RUN=""
if [[ "${1:-}" == "--dry-run" ]]; then
	DRY_RUN="--dry-run"
fi

./build.sh

cp README-non-solar.md ./pkg/README.md
cp README.md ./pkg-solar/README.md

if ! npm whoami > /dev/null 2>&1; then
	echo "Error: You are not logged in to npm. Please run 'npm login' first."
	exit 1
fi

npm publish --access public $DRY_RUN ./pkg
npm publish --access public $DRY_RUN ./pkg-solar
