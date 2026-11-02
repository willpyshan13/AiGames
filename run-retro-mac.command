#!/bin/zsh
cd "${0:A:h}"
source scripts/env.sh
cargo build --locked --offline --quiet || exit 1
exec env MAGNET_START_LIBRARY=1 ./target/debug/magnet-island
