#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
fixture_dir="$repo_root/tests/lilypond-ab"
reference_dir="$fixture_dir/reference"
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-lilypond-ab.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

mode=${1:-check}
if [ "$mode" != "check" ] && [ "$mode" != "--update" ]; then
  echo "usage: scripts/test-lilypond-ab.sh [--update]" >&2
  exit 2
fi

if ! command -v lilypond >/dev/null 2>&1; then
  echo "LilyPond is required to regenerate A/B references." >&2
  exit 1
fi

lilypond_version=$(lilypond --version | sed -n '1s/^GNU LilyPond \([^ ]*\).*/\1/p')
if [ "$lilypond_version" != "2.26.0" ]; then
  echo "LilyPond 2.26.0 is required; found $lilypond_version." >&2
  exit 1
fi

mkdir -p "$reference_dir"
has_difference=0
for fixture_source in "$fixture_dir"/*.ly; do
  fixture_name=$(basename "$fixture_source" .ly)
  lilypond --svg -dcrop -dno-point-and-click \
    -o "$tmp_dir/$fixture_name" "$fixture_source" >/dev/null 2>&1
  generated_svg="$tmp_dir/$fixture_name.cropped.svg"
  reference_svg="$reference_dir/$fixture_name.svg"
  if [ "$mode" = "--update" ]; then
    cp "$generated_svg" "$reference_svg"
  elif ! cmp -s "$generated_svg" "$reference_svg"; then
    echo "$fixture_name: LilyPond reference differs; run scripts/test-lilypond-ab.sh --update" >&2
    has_difference=1
  fi
done

if [ "$has_difference" -ne 0 ]; then
  exit 1
fi

typst compile --root "$repo_root" "$repo_root/tests/test.typ" "$tmp_dir/test.pdf"
echo "LilyPond A/B references and Typst comparison suite passed."
