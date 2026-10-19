#!/usr/bin/env python3
"""Run the opt-in debug UI checks with a separate save for every game."""
import json
import os
from pathlib import Path
import subprocess
import struct
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / ("output/remaining-games-check-wide" if os.environ.get("MAGNET_QA_SIZE") else "output/remaining-games-check")
GAMES = ["island", "cloudbridge", "freebuild", "ball", "tetris", "fire",
         "detective", "farm", "tank", "sky", "bubble", "bomber", "fruit",
         "fortune", "slots", "arcade", "fishing", "honey", "snake", "mines",
         "sokoban", "lightpath", "garden", "courier"]


def run(game):
    folder = OUT / game
    folder.mkdir(parents=True, exist_ok=True)
    # A fresh profile avoids silently changing any real player's progress.
    saved = folder / ("save-" + str(time.time_ns()))
    saved.mkdir()
    env = os.environ.copy()
    env.update(MAGNET_START_GAME=game, MAGNET_PHONE_PREVIEW="1", MAGNET_QA="1",
               MAGNET_CAPTURE=str(folder / "capture.png"), MAGNET_SAVE_DIR=str(saved))
    began = time.monotonic()
    with (folder / "run.log").open("w") as log:
        process = subprocess.Popen([str(ROOT / "target/debug/magnet-island")],
                                   cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT)
        try:
            code = process.wait(timeout=50)
        except subprocess.TimeoutExpired:
            process.terminate()
            try:
                process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            code = "timeout"
    report = folder / "result.json"
    data = json.loads(report.read_text()) if report.exists() else {}
    png = folder / "initial.png"
    dimensions = struct.unpack(">II", png.read_bytes()[16:24]) if png.exists() else None
    aspect_ok = True
    if os.environ.get("MAGNET_QA_SIZE"):
        width, height = map(int, os.environ["MAGNET_QA_SIZE"].split("x"))
        aspect_ok = dimensions is not None and abs(dimensions[0] / dimensions[1] - width / height) < 0.01
    result = {"game": game, "title": data.get("game"), "exit": code,
              "passed": code == 0 and not data.get("failed", True) and data.get("completed", False) and aspect_ok,
              "dimensions": dimensions, "aspect_ok": aspect_ok,
              "seconds": round(time.monotonic() - began, 1), "save": str(saved.relative_to(ROOT)),
              "checks": sum("check" in r for r in data.get("records", []))}
    print(json.dumps(result, ensure_ascii=False), flush=True)
    return result


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    selected = sys.argv[1:] or GAMES
    assert all(g in GAMES for g in selected)
    summary_path = OUT / "summary.json"
    results = json.loads(summary_path.read_text()) if summary_path.exists() else []
    for game in selected:
        result = run(game)
        results = [r for r in results if r["game"] != game] + [result]
        summary_path.write_text(json.dumps(results, ensure_ascii=False, indent=2))
    sys.exit(0 if all(r["passed"] for r in results if r["game"] in selected) else 1)
