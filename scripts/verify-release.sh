#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
version=$(awk -F '"' '/^version = / { print $2; exit }' "$repo_root/typst.toml")
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-release.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

cd "$repo_root"
./build.sh
cargo test --manifest-path plugin/Cargo.toml
scripts/test-errors.sh
scripts/test-import.sh
typst compile --root . tests/test.typ tests/test.pdf
typst compile --root . docs/documentation.typ docs/documentation.pdf
typst compile --root . examples/showcase.typ examples/showcase.pdf

mkdir -p "$tmp_dir/packages-repo/packages/preview"
scripts/package-preview.sh "$version" "$tmp_dir/packages-repo"

smoke="$tmp_dir/smoke.typ"
printf '%s\n' \
  '#import "packages-repo/packages/preview/typed-scores/'"$version"'/src/lib.typ": score, bar, part, chord-diagram, guitar-chords, import-score' \
  '#bar("C5:q[dyn=mf fermata]", time: "1/4")' \
  '#score(time: "2/4", bars: ((notes: "F#5:h ~"), (notes: "F#5:h")))' \
  '#score(time: "4/4", bar-numbers: "all", bars: ((rehearsal: "A", notes: ("acciaccatura { D5:s } C5:q F5:q G5:h", "C4:w"), barline: (right: "final")),))' \
  '#score(time: "4/4", bars: ((notes: "C5:h[tremolo=16] (E5 G5 C6):h[arpeggio=up]"), (clef: "bass", notes: "tremolo 16 { C3:h G3:h }")))' \
  '#import-score("X:1\nT:Smoke\nM:2/4\nL:1/8\nK:G\n|: \"G\"B2 AG | \"D\"A4 :|")' \
  '#score(transpose: "M2", time: "2/4", bars: ((notes: "C5:h", harmony: "C:h"),))' \
  '#score(clef: "percussion", heads: (g5: "x"), time: "2/4", bars: ((notes: "g5:q c5"),))' \
  '#score(clef: "bass", time: "2/4", bars: ((notes: "c2:h[8vb( 8vb)]", figures: "(6 4):h"),))' \
  '#score(staves: (guitar: (clef: "treble-8"), tab: (clef: "tab", source: "guitar")), time: "2/4", bars: ((guitar: "e3:q g3"),))' \
  '#chord-diagram(guitar-chords.at("C").frets, name: "C")' \
  '#score(..part((staves: (melody: (clef: "treble"), bass: (clef: "bass")), time: "2/4", bars: ((melody: "c5:h", bass: "c3:h"),)), "melody"))' \
  >"$smoke"
typst compile --root "$tmp_dir" "$smoke" "$tmp_dir/smoke.pdf"

echo "Release verification passed for typed-scores $version."
