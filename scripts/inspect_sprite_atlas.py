#!/usr/bin/env python3
"""Read-only alpha-component inspection for generated sprite atlases."""
from collections import deque
from pathlib import Path
import sys
from PIL import Image


def inspect(path: str) -> None:
    image = Image.open(path).convert("RGBA")
    width, height = image.size
    alpha = image.getchannel("A").tobytes()
    mask = bytearray(value > 100 for value in alpha)
    parts = []
    for origin in range(len(mask)):
        if not mask[origin]:
            continue
        mask[origin] = 0
        queue = deque([origin])
        count = 0
        min_x = max_x = origin % width
        min_y = max_y = origin // width
        while queue:
            index = queue.popleft()
            x, y = index % width, index // width
            count += 1
            min_x, max_x = min(min_x, x), max(max_x, x)
            min_y, max_y = min(min_y, y), max(max_y, y)
            for neighbor in ((index - 1 if x else -1),
                             (index + 1 if x + 1 < width else -1),
                             (index - width if y else -1),
                             (index + width if y + 1 < height else -1)):
                if neighbor >= 0 and mask[neighbor]:
                    mask[neighbor] = 0
                    queue.append(neighbor)
        if count > 500:
            parts.append((min_x, min_y, max_x + 1, max_y + 1, count))
    print(Path(path).name, image.size)
    for part in sorted(parts, key=lambda p: (round(p[1] / (height / 4)), p[0])):
        print(part)


if __name__ == "__main__":
    for filename in sys.argv[1:]:
        inspect(filename)
