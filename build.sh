#!/usr/bin/env bash

set -eou pipefail

#wasm-pack build

wasm-pack build -t nodejs --release --out-name plugin --no-pack

#cp -v pkg/plugin_bg.wasm plugin.wasm
