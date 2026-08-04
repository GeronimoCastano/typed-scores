#!/usr/bin/env sh
set -eu

usage() {
  echo "Usage: scripts/package-preview.sh <version> <typst-packages-repo>" >&2
  echo "Example: scripts/package-preview.sh 0.1.0 /path/to/typst/packages" >&2
}

if [ "$#" -ne 2 ]; then
  usage
  exit 2
fi

version="$1"
packages_repo="${2%/}"
package_name="typed-scores"
runtime_typst_files="
lib.typ
score.typ
diagnostics.typ
parser.typ
score-input.typ
meter.typ
signatures.typ
event-geometry.typ
spacing.typ
lyrics.typ
event-engraving.typ
markings.typ
ties-slurs.typ
systems.typ
render.typ
"

validate_package_exclusions() {
  exclusion_check_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-exclusions.XXXXXX")
  trap 'rm -rf "$exclusion_check_dir"' EXIT HUP INT TERM

  git -C "$exclusion_check_dir" init --quiet
  sed -n '/^exclude = \[/,/^\]/p' "$repo_root/typst.toml" \
    | sed -E -n 's/^[[:space:]]*"([^"]+)",?$/\1/p' \
    >"$exclusion_check_dir/.git/info/exclude"

  required_package_paths="
typst.toml
README.md
LICENSE
src/plugin.wasm
"
  for runtime_typst_file in $runtime_typst_files; do
    required_package_paths="$required_package_paths
src/$runtime_typst_file"
  done
  for glyph_file in "$repo_root"/src/assets/glyphs/*; do
    required_package_paths="$required_package_paths
src/assets/glyphs/$(basename "$glyph_file")"
  done

  for required_package_path in $required_package_paths; do
    mkdir -p "$exclusion_check_dir/$(dirname "$required_package_path")"
    touch "$exclusion_check_dir/$required_package_path"
    if git -C "$exclusion_check_dir" check-ignore --quiet --no-index -- "$required_package_path"; then
      echo "error: typst.toml excludes required runtime file: $required_package_path" >&2
      exit 1
    fi
  done

  rm -rf "$exclusion_check_dir"
  trap - EXIT HUP INT TERM
}

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
package_target="$packages_repo/packages/preview/$package_name/$version"

if [ ! -d "$packages_repo/packages/preview" ]; then
  echo "error: not a typst/packages checkout: $packages_repo" >&2
  exit 1
fi

manifest_version=$(awk -F '"' '/^version = / { print $2; exit }' "$repo_root/typst.toml")
if [ "$manifest_version" != "$version" ]; then
  echo "error: typst.toml version is $manifest_version, not $version" >&2
  exit 1
fi

for required_file in \
  "$repo_root/typst.toml" \
  "$repo_root/README.md" \
  "$repo_root/LICENSE" \
  "$repo_root/src/plugin.wasm" \
  "$repo_root/src/assets/glyphs/BRAVURA-OFL.txt"
do
  if [ ! -f "$required_file" ]; then
    echo "error: missing required file: $required_file" >&2
    exit 1
  fi
done

for runtime_typst_file in $runtime_typst_files; do
  if [ ! -f "$repo_root/src/$runtime_typst_file" ]; then
    echo "error: missing required file: $repo_root/src/$runtime_typst_file" >&2
    exit 1
  fi
done

validate_package_exclusions

if [ -e "$package_target" ]; then
  echo "error: target already exists: $package_target" >&2
  echo "Remove it manually if you intentionally want to recreate it." >&2
  exit 1
fi

mkdir -p "$package_target/assets/readme" "$package_target/src/assets"
cp "$repo_root/typst.toml" "$package_target/typst.toml"
cp "$repo_root/README.md" "$package_target/README.md"
cp "$repo_root/LICENSE" "$package_target/LICENSE"
for runtime_typst_file in $runtime_typst_files; do
  cp "$repo_root/src/$runtime_typst_file" "$package_target/src/$runtime_typst_file"
done
cp "$repo_root/src/plugin.wasm" "$package_target/src/plugin.wasm"
cp -R "$repo_root/src/assets/glyphs" "$package_target/src/assets/glyphs"

found_readme_image=0
for readme_image in "$repo_root"/assets/readme/*.png; do
  if [ -f "$readme_image" ]; then
    cp "$readme_image" "$package_target/assets/readme/"
    found_readme_image=1
  fi
done

if [ "$found_readme_image" -eq 0 ]; then
  echo "error: no README PNG assets found in assets/readme" >&2
  exit 1
fi

echo "Prepared $package_name $version at:"
echo "$package_target"
echo
echo "Next:"
echo "  cd $packages_repo/packages"
echo "  typst-package-check check @preview/$package_name:$version"
