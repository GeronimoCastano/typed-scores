#!/usr/bin/env python3
"""Print the Bravura engraving values used by typed-scores.

Reads bravura_metadata.json (pass a path, or the script downloads the
current copy from the Bravura repository) and prints the engravingDefaults
subset plus the glyph anchors that src/render.typ hard-codes, so the
constants can be checked or refreshed when the font is updated.

Bravura and its metadata are published by Steinberg under the SIL Open
Font License; see src/assets/glyphs/BRAVURA-OFL.txt.
"""

import json
import sys
import urllib.request

METADATA_URL = (
    "https://raw.githubusercontent.com/steinbergmedia/bravura/"
    "master/redist/bravura_metadata.json"
)

ENGRAVING_KEYS = [
    "stemThickness",
    "beamThickness",
    "beamSpacing",
    "legerLineThickness",
    "legerLineExtension",
    "staffLineThickness",
    "thinBarlineThickness",
    "thickBarlineThickness",
    "barlineSeparation",
    "repeatBarlineDotSeparation",
    "repeatEndingLineThickness",
    "hairpinThickness",
    "slurEndpointThickness",
    "slurMidpointThickness",
    "tieEndpointThickness",
    "tieMidpointThickness",
    "bracketThickness",
    "pedalLineThickness",
]

ANCHOR_GLYPHS = {
    "noteheadBlack": ["stemUpSE", "stemDownNW"],
    "noteheadHalf": ["stemUpSE", "stemDownNW"],
    "flag8thUp": ["stemUpNW"],
    "flag8thDown": ["stemDownSW"],
    "flag16thUp": ["stemUpNW"],
    "flag16thDown": ["stemDownSW"],
    "flag32ndUp": ["stemUpNW"],
    "flag32ndDown": ["stemDownSW"],
}


def main() -> None:
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as handle:
            metadata = json.load(handle)
    else:
        with urllib.request.urlopen(METADATA_URL) as response:
            metadata = json.load(response)

    defaults = metadata["engravingDefaults"]
    print("engravingDefaults (staff spaces):")
    for key in ENGRAVING_KEYS:
        print(f"  {key} = {defaults[key]}")

    anchors = metadata["glyphsWithAnchors"]
    print("\nglyph anchors (staff spaces, relative to glyph origin):")
    for glyph, names in ANCHOR_GLYPHS.items():
        entry = anchors.get(glyph, {})
        for name in names:
            print(f"  {glyph}.{name} = {entry.get(name)}")


if __name__ == "__main__":
    main()
