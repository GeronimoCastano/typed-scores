#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-errors.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

check_error() {
  source_file="$1"
  expected="$2"
  output="$tmp_dir/$(basename "$source_file").log"
  if typst compile --root "$repo_root" "$repo_root/$source_file" "$tmp_dir/out.pdf" >"$output" 2>&1; then
    echo "error: $source_file compiled successfully; expected an error" >&2
    exit 1
  fi
  if ! grep -F "$expected" "$output" >/dev/null; then
    echo "error: $source_file did not report: $expected" >&2
    sed -n '1,100p' "$output" >&2
    exit 1
  fi
  echo "ok: $source_file"
}

check_error tests/errors/unknown-annotation.typ 'unknown annotation "mystery"'
check_error tests/errors/legacy-bar-separator.typ "bar separator '|' is not allowed inside notes"
check_error tests/errors/tie-mismatch.typ "must connect the same written pitch or chord"
check_error tests/errors/tie-at-end.typ "has no following event"
check_error tests/errors/unopened-pedal.typ "closes without opening"
check_error tests/errors/invalid-group.typ "group must be auto, brace, bracket, line, or none"
check_error tests/errors/legacy-ending-number.typ "ending has unknown field number"
check_error tests/errors/invalid-scale.typ "scale must be a positive number"
check_error tests/errors/invalid-beam-join.typ "requires a flagged note or chord"
check_error tests/errors/missing-explicit-duration.typ "omit the colon to inherit the previous duration"
check_error tests/errors/inherited-duration-does-not-fill.typ "durations sum to 3/4, expected 1"
check_error tests/errors/invalid-harmony-duration.typ "harmony bar 1: durations sum to 3/4, expected 4/4"
check_error tests/errors/invalid-staff-label.typ "staff violin label must be a non-empty string"
check_error tests/errors/invalid-tempo-beat.typ "tempo in bar 1 beat must be whole, half, quarter, eighth, sixteenth, or thirty-second"
check_error tests/errors/invalid-tuplet-option.typ "tuplet bracket must be auto, always, or never"
check_error tests/errors/invalid-arpeggio.typ "arpeggio annotation requires a chord"
check_error tests/errors/invalid-alternating-tremolo.typ "alternating tremolo must contain exactly two notes or chords"
check_error tests/errors/invalid-grace-ending.typ "grace group must be followed by a main note, chord, or rest"
check_error tests/errors/inconsistent-voice-count.typ "bar 2 notes has 1 voices; expected 2"
check_error tests/errors/invalid-multistaff-clef-change.typ "clef must be a staff-id dictionary in a multi-staff score"
