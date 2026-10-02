#!/usr/bin/env sh
set -eu

# Every committed importer expectation must still compile against the package.
# The plugin's unit tests (cargo test) check that each fixture imports to its
# expectation; regenerate one with
#   cargo run --manifest-path plugin/Cargo.toml --release --example import -- tests/import/NAME.musicxml > tests/import/NAME.typ

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-import.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

cd "$repo_root"
for expected in tests/import/*.typ; do
  typst compile --root . "$expected" "$tmp_dir/out.pdf"
  echo "ok: $expected"
done
