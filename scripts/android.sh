#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
. ./scripts/env.sh
# Keep a separate target tree so desktop and Android builds do not block each other.
export CARGO_TARGET_DIR="$PWD/target/android"
# Use the local debug key only for the test build. A real release key should be
# supplied through these same environment variables by the release pipeline.
export CARGO_APK_RELEASE_KEYSTORE="$HOME/.android/debug.keystore"
export CARGO_APK_RELEASE_KEYSTORE_PASSWORD="android"
cargo apk build --lib --release
# Strip a copy: a second cargo build would restore the unstripped library.
package_tmp=$(mktemp -d)
trap 'rm -rf "$package_tmp"' EXIT
native_lib="$CARGO_TARGET_DIR/aarch64-linux-android/release/libmagnet_island.so"
"$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-strip" --strip-unneeded \
  -o "$package_tmp/libmagnet_island.so" "$native_lib"
python3 - "$CARGO_TARGET_DIR/release/apk/magnet-island-unaligned.apk" "$package_tmp" <<'PYCODE'
import sys, zipfile
from pathlib import Path
original, staging = Path(sys.argv[1]), Path(sys.argv[2])
with zipfile.ZipFile(original) as src, zipfile.ZipFile(staging / "unsigned.apk", "w") as dst:
    for entry in src.infolist():
        if entry.filename.startswith("META-INF/"):
            continue
        # Keep the old concept image in the workspace, but not in the APK:
        # the game loads honey-hunt-storybook-v2.png instead.
        if entry.filename == "assets/honey/honey-hunt-storybook-v1.png":
            continue
        content = ((staging / "libmagnet_island.so").read_bytes()
                   if entry.filename == "lib/arm64-v8a/libmagnet_island.so"
                   else src.read(entry.filename))
        dst.writestr(entry, content)
PYCODE
build_tools=$(python3 - "$ANDROID_HOME/build-tools" <<'PYCODE'
import sys
from pathlib import Path
paths = [p for p in Path(sys.argv[1]).iterdir() if (p / "apksigner").exists() and (p / "zipalign").exists()]
print(max(paths, key=lambda p: tuple(int(x) if x.isdigit() else 0 for x in p.name.split("."))))
PYCODE
)
"$build_tools/zipalign" -P 16 -f 4 "$package_tmp/unsigned.apk" "$package_tmp/aligned.apk"
mkdir -p output
"$build_tools/apksigner" sign --ks "$HOME/.android/debug.keystore" \
  --ks-key-alias androiddebugkey --ks-pass pass:android \
  --out "$package_tmp/signed.apk" "$package_tmp/aligned.apk"
"$build_tools/apksigner" verify "$package_tmp/signed.apk"
"$build_tools/zipalign" -c -P 16 4 "$package_tmp/signed.apk"
cp "$package_tmp/signed.apk" output/magnet-island.apk
echo "APK: $PWD/output/magnet-island.apk"
