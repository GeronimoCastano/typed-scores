#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
fixture_dir="$repo_root/examples/lilypond-reference"
reference_dir="$fixture_dir/reference"
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-showcase-lilypond.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

mode=${1:-check}
if [ "$mode" != "check" ] && [ "$mode" != "--update" ]; then
  echo "usage: scripts/render-showcase-lilypond.sh [--update]" >&2
  exit 2
fi

if ! command -v lilypond >/dev/null 2>&1; then
  echo "LilyPond is required to regenerate showcase references." >&2
  exit 1
fi

version=$(lilypond --version | sed -n '1s/^GNU LilyPond \([^ ]*\).*/\1/p')
if [ "$version" != "2.26.0" ]; then
  echo "LilyPond 2.26.0 is required; found $version." >&2
  exit 1
fi

mkdir -p "$reference_dir"
failed=0
for source in "$fixture_dir"/*.ly; do
  name=$(basename "$source" .ly)
  lilypond --svg -dcrop -dno-point-and-click \
    -o "$tmp_dir/$name" "$source" >/dev/null 2>&1
  generated="$tmp_dir/$name.cropped.svg"
  reference="$reference_dir/$name.svg"
  if [ "$mode" = "--update" ]; then
    cp "$generated" "$reference"
  elif ! cmp -s "$generated" "$reference"; then
    echo "$name: reference differs; run scripts/render-showcase-lilypond.sh --update" >&2
    failed=1
  fi
done

if [ "$failed" -ne 0 ]; then
  exit 1
fi

echo "LilyPond showcase references passed."
