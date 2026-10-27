#!/bin/sh
# Compile only the small Android website host; cargo-apk supplies the native app.
set -eu
cd "$(dirname "$0")/.."
. ./scripts/env.sh
webview_output=${1:?Pass an output directory}
mkdir -p "$webview_output/classes" "$webview_output/dex"
webview_output=$(cd "$webview_output" && pwd)
android_jar="$ANDROID_HOME/platforms/android-35/android.jar"
webview_build_tools=$(python3 - "$ANDROID_HOME/build-tools" <<'PY'
import sys
from pathlib import Path
paths = [p for p in Path(sys.argv[1]).iterdir() if (p / "d8").exists()]
print(max(paths, key=lambda p: tuple(int(x) if x.isdigit() else 0 for x in p.name.split("."))))
PY
)
"$JAVA_HOME/bin/javac" --release 8 -classpath "$android_jar" \
  -d "$webview_output/classes" android/java/com/aigames/magnetisland/WebGame.java
"$JAVA_HOME/bin/jar" cf "$webview_output/webview.jar" -C "$webview_output/classes" .
"$webview_build_tools/d8" --release --min-api 26 --lib "$android_jar" \
  --output "$webview_output/dex" "$webview_output/webview.jar"
test -s "$webview_output/dex/classes.dex"
