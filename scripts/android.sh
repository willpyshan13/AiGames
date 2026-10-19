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
import io, re, sys, zipfile
from pathlib import Path
try:
    from PIL import Image
except ImportError as error:
    raise SystemExit("Pillow is required to create the approved WebP assets") from error

original, staging = Path(sys.argv[1]), Path(sys.argv[2])
asset_root = Path("assets")
referenced = set()
for source in Path("src").rglob("*.rs"):
    text = source.read_text(encoding="utf-8")
    for match in re.findall(r"[\w./-]+\.(?:png|webp|wav|ogg|mp3|otf|ttf|json)", text):
        path = asset_root / match
        if path.is_file():
            relative = path.as_posix().removeprefix("assets/")
            referenced.add(relative)
            if path.suffix == ".png":
                referenced.add(path.with_suffix(".webp").as_posix().removeprefix("assets/"))
        elif path.suffix == ".webp" and path.with_suffix(".png").is_file():
            referenced.add(path.as_posix().removeprefix("assets/"))

def encode_webp(content):
    image = Image.open(io.BytesIO(content))
    original_alpha = image.convert("RGBA").getchannel("A").tobytes()
    output = io.BytesIO()
    image.save(output, format="WEBP", quality=90, method=0)
    encoded = output.getvalue()
    check = Image.open(io.BytesIO(encoded)).convert("RGBA")
    if check.size != image.size or check.getchannel("A").tobytes() != original_alpha:
        raise RuntimeError("WebP encoding changed image size or transparency")
    return encoded

removed = []
converted = 0
with zipfile.ZipFile(original) as src, zipfile.ZipFile(staging / "unsigned.apk", "w") as dst:
    for entry in src.infolist():
        if entry.filename.startswith("META-INF/"):
            continue
        if entry.filename.startswith("assets/"):
            asset_path = entry.filename.removeprefix("assets/")
            webp_path = Path(asset_path).with_suffix(".webp").as_posix()
            if asset_path.endswith(".png") and webp_path in referenced:
                dst.writestr(f"assets/{webp_path}", encode_webp(src.read(entry.filename)),
                             compress_type=zipfile.ZIP_STORED)
                converted += 1
                continue
            if asset_path not in referenced:
                removed.append(asset_path)
                continue
            content = src.read(entry.filename)
        elif entry.filename == "lib/arm64-v8a/libmagnet_island.so":
            content = (staging / "libmagnet_island.so").read_bytes()
        else:
            content = src.read(entry.filename)
        dst.writestr(entry, content)
print(f"Converted {converted} referenced PNG files to WebP")
print(f"Excluded {len(removed)} unreferenced asset files")
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
