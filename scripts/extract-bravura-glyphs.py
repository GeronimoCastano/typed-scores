#!/usr/bin/env python3
"""Extract selected Bravura SMuFL glyphs as package-local SVG assets."""

from __future__ import annotations

import argparse
import html
import json
from pathlib import Path

from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.ttLib import TTFont

UNITS_PER_STAFF_SPACE = 250.0

GLYPHS = [
    ("brace", "brace.svg", 0xE000),
    ("noteheadBlack", "notehead-black.svg", 0xE0A4),
    ("noteheadHalf", "notehead-half.svg", 0xE0A3),
    ("noteheadWhole", "notehead-whole.svg", 0xE0A2),
    ("augmentationDot", "augmentation-dot.svg", 0xE1E7),
    ("gClef", "treble-clef.svg", 0xE050),
    ("fClef", "bass-clef.svg", 0xE062),
    ("cClef", "alto-clef.svg", 0xE05C),
    ("cClef", "tenor-clef.svg", 0xE05C),
    ("accidentalSharp", "sharp.svg", 0xE262),
    ("accidentalFlat", "flat.svg", 0xE260),
    ("accidentalNatural", "natural.svg", 0xE261),
    ("restWhole", "rest-whole.svg", 0xE4E3),
    ("restHalf", "rest-half.svg", 0xE4E4),
    ("restQuarter", "rest-quarter.svg", 0xE4E5),
    ("rest8th", "rest-eighth.svg", 0xE4E6),
    ("rest16th", "rest-sixteenth.svg", 0xE4E7),
    ("rest32nd", "rest-thirty-second.svg", 0xE4E8),
    ("timeSig0", "time-sig-0.svg", 0xE080),
    ("timeSig1", "time-sig-1.svg", 0xE081),
    ("timeSig2", "time-sig-2.svg", 0xE082),
    ("timeSig3", "time-sig-3.svg", 0xE083),
    ("timeSig4", "time-sig-4.svg", 0xE084),
    ("timeSig5", "time-sig-5.svg", 0xE085),
    ("timeSig6", "time-sig-6.svg", 0xE086),
    ("timeSig7", "time-sig-7.svg", 0xE087),
    ("timeSig8", "time-sig-8.svg", 0xE088),
    ("timeSig9", "time-sig-9.svg", 0xE089),
    ("flag8thUp", "flag-eighth-up.svg", 0xE240),
    ("flag8thDown", "flag-eighth-down.svg", 0xE241),
    ("flag16thUp", "flag-sixteenth-up.svg", 0xE242),
    ("flag16thDown", "flag-sixteenth-down.svg", 0xE243),
    ("flag32ndUp", "flag-thirty-second-up.svg", 0xE244),
    ("flag32ndDown", "flag-thirty-second-down.svg", 0xE245),
    ("articStaccatoAbove", "staccato-above.svg", 0xE4A2),
    ("articStaccatoBelow", "staccato-below.svg", 0xE4A3),
    ("articTenutoAbove", "tenuto-above.svg", 0xE4A4),
    ("articTenutoBelow", "tenuto-below.svg", 0xE4A5),
    ("articStaccatissimoWedgeAbove", "staccatissimo-above.svg", 0xE4A8),
    ("articStaccatissimoWedgeBelow", "staccatissimo-below.svg", 0xE4A9),
    ("articMarcatoAbove", "marcato-above.svg", 0xE4AC),
    ("articMarcatoBelow", "marcato-below.svg", 0xE4AD),
    ("articAccentAbove", "accent-above.svg", 0xE4A0),
    ("articAccentBelow", "accent-below.svg", 0xE4A1),
    ("ornamentTurn", "ornament-turn.svg", 0xE567),
    ("keyboardPedalPed", "pedal-ped.svg", 0xE650),
    ("keyboardPedalUp", "pedal-up.svg", 0xE655),
]


def metadata_bbox(metadata: dict, smufl_name: str) -> tuple[float, float, float, float]:
    bbox = metadata["glyphBBoxes"][smufl_name]
    sw_x, sw_y = bbox["bBoxSW"]
    ne_x, ne_y = bbox["bBoxNE"]
    return (
        sw_x * UNITS_PER_STAFF_SPACE,
        sw_y * UNITS_PER_STAFF_SPACE,
        ne_x * UNITS_PER_STAFF_SPACE,
        ne_y * UNITS_PER_STAFF_SPACE,
    )


def svg_for_glyph(font: TTFont, metadata: dict, smufl_name: str, codepoint: int) -> str:
    cmap = font.getBestCmap()
    glyph_name = cmap[codepoint]
    glyph_set = font.getGlyphSet()
    pen = SVGPathPen(glyph_set)
    glyph_set[glyph_name].draw(pen)
    path = pen.getCommands()
    min_x, min_y, max_x, max_y = metadata_bbox(metadata, smufl_name)
    width = max_x - min_x
    height = max_y - min_y
    view_box = f"{min_x:g} {-max_y:g} {width:g} {height:g}"
    escaped_path = html.escape(path, quote=True)
    return "\n".join(
        [
            '<svg xmlns="http://www.w3.org/2000/svg" '
            f'viewBox="{view_box}">',
            f'  <path d="{escaped_path}" transform="scale(1,-1)" fill="#000"/>',
            "</svg>",
            "",
        ]
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--bravura-root",
        type=Path,
        default=Path("/private/tmp/bravura"),
        help="Path to a clone of https://github.com/steinbergmedia/bravura",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=Path("src/assets/glyphs"),
        help="Directory where package SVG glyph assets should be written",
    )
    args = parser.parse_args()

    font_path = args.bravura_root / "redist" / "otf" / "Bravura.otf"
    metadata_path = args.bravura_root / "redist" / "bravura_metadata.json"
    font = TTFont(font_path)
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))

    args.out_dir.mkdir(parents=True, exist_ok=True)
    for smufl_name, file_name, codepoint in GLYPHS:
        svg = svg_for_glyph(font, metadata, smufl_name, codepoint)
        (args.out_dir / file_name).write_text(svg, encoding="utf-8")


if __name__ == "__main__":
    main()
