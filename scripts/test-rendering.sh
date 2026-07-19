#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
render_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-render.XXXXXX")
trap 'rm -rf "$render_dir"' EXIT HUP INT TERM

actual_pdf="$render_dir/actual.pdf"
reference_dir="$render_dir/reference"
actual_dir="$render_dir/actual"
mkdir -p "$reference_dir" "$actual_dir"

cd "$repo_root"
typst compile --root . tests/test.typ "$actual_pdf"
pdftoppm -r 144 tests/test.pdf "$reference_dir/page" >/dev/null 2>&1
pdftoppm -r 144 "$actual_pdf" "$actual_dir/page" >/dev/null 2>&1
python3 scripts/compare-rendering.py "$reference_dir" "$actual_dir"
