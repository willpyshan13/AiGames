#!/bin/zsh
cd "${0:A:h}"
source scripts/env.sh
cargo build --offline --quiet || exit 1
exec env MAGNET_START_GAME=snake ./target/debug/magnet-island
