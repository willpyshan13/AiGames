#!/usr/bin/env python3
"""Build the Android font asset from only the glyphs this offline game uses."""

from pathlib import Path

from fontTools import subset


ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "font_sources" / "NotoSansCJKsc-Medium.full.otf"
TARGET = ROOT / "assets" / "fonts" / "NotoSansCJKsc-Medium.otf"


def main() -> None:
    if not SOURCE.is_file():
        raise SystemExit(f"Full font source is missing: {SOURCE}")

    characters = {chr(codepoint) for codepoint in range(0x20, 0x7F)}
    for source in (ROOT / "src").rglob("*.rs"):
        characters.update(source.read_text(encoding="utf-8"))

    options = subset.Options()
    options.desubroutinize = True
    options.hinting = False
    options.layout_features = ["*"]
    options.name_IDs = [1, 2, 3, 4, 6]
    options.name_legacy = True

    font = subset.load_font(str(SOURCE), options)
    glyphs = subset.Subsetter(options=options)
    glyphs.populate(unicodes={ord(character) for character in characters})
    glyphs.subset(font)
    subset.save_font(font, str(TARGET), options)
    print(f"Subset {len(characters)} source characters into {TARGET}")


if __name__ == "__main__":
    main()
