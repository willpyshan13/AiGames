"""Generate original, deterministic pixel textures. Python standard library only."""
from pathlib import Path
import random
import struct
import zlib

OUT = Path(__file__).resolve().parents[1] / "assets" / "textures"
OUT.mkdir(parents=True, exist_ok=True)

def png(path, pixels, size=32):
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))
    raw = b"".join(b"\0" + bytes(sum(pixels[y], ())) for y in range(size))
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))

for seed, (name, base) in enumerate([
    ("grass", (119, 161, 74)), ("wood", (182, 127, 66)),
    ("stone", (138, 155, 157)), ("crystal", (90, 188, 206)), ("soil", (126, 91, 65)),
]):
    rng = random.Random(seed)
    noise = [[rng.randint(-18, 18) for _ in range(8)] for _ in range(8)]
    pixels = []
    for y in range(32):
        row = []
        for x in range(32):
            delta = noise[y // 4][x // 4]
            if name == "wood":
                delta += -32 if y % 8 == 0 or (x + (y // 8) * 11) % 32 == 0 else 0
                if y % 8 == 2: delta += 12
            if name == "stone" and (y % 11 == 0 or (x + (y // 11) * 9) % 16 == 0): delta -= 35
            if name == "crystal" and (x + y) % 17 < 2: delta += 30
            if x in (0, 31) or y in (0, 31): delta -= 28
            if x == 1 or y == 1: delta += 19
            row.append(tuple(max(0, min(255, c + delta)) for c in base))
        pixels.append(row)
    png(OUT / f"{name}.png", pixels)
print(f"Generated 5 original textures in {OUT}")
