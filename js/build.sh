#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

set_pkg_name() {
	local dir="$1" name="$2"
	node -e "const fs=require('fs');const p='${dir}/package.json';const j=JSON.parse(fs.readFileSync(p,'utf8'));j.name='${name}';fs.writeFileSync(p,JSON.stringify(j,null,2)+'\\n')"
}

tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

rm -rf pkg pkg-solar

wasm-pack build --release --target web
cp -a pkg "$tmp_dir/pkg-default"

wasm-pack build --release --target web --features solar
cp -a pkg pkg-solar

rm -rf pkg
mv "$tmp_dir/pkg-default" pkg

set_pkg_name pkg "calendrier-mt"
set_pkg_name pkg-solar "calendrier"

ln -sfn ../js/pkg/calendrier_web.js ../web/calendrier_web.js
ln -sfn ../js/pkg/calendrier_web_bg.wasm ../web/calendrier_web_bg.wasm
ln -sfn ../js/pkg-solar/calendrier_web.js ../web/calendrier_web_solar.js
ln -sfn ../js/pkg-solar/calendrier_web_bg.wasm ../web/calendrier_web_solar_bg.wasm
