#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/typed-scores-errors.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

check_error() {
  source_file="$1"
  expected_message="$2"
  compiler_log="$tmp_dir/$(basename "$source_file").log"
  if typst compile --root "$repo_root" "$repo_root/$source_file" "$tmp_dir/out.pdf" >"$compiler_log" 2>&1; then
    echo "error: $source_file compiled successfully; expected an error" >&2
    exit 1
  fi
  if ! grep -F "$expected_message" "$compiler_log" >/dev/null; then
    echo "error: $source_file did not report: $expected_message" >&2
    sed -n '1,100p' "$compiler_log" >&2
    exit 1
  fi
  echo "ok: $source_file"
}

check_error tests/errors/unknown-annotation.typ "unknown annotation"
check_error tests/errors/legacy-bar-separator.typ "bar separator '|' is not allowed inside notes"
check_error tests/errors/tie-mismatch.typ "must connect the same written pitch or chord"
check_error tests/errors/tie-at-end.typ "has no following event"
check_error tests/errors/unopened-pedal.typ "closes without opening"
check_error tests/errors/invalid-group.typ "unsupported grouping style"
check_error tests/errors/legacy-ending-number.typ "ending has unknown field"
check_error tests/errors/invalid-scale.typ "value must be a positive number"
check_error tests/errors/invalid-beam-join.typ "requires a flagged note or chord"
check_error tests/errors/missing-explicit-duration.typ "omit the colon to inherit the previous duration"
check_error tests/errors/inherited-duration-does-not-fill.typ "durations sum to 3/4, expected 1"
check_error tests/errors/invalid-harmony-duration.typ "harmony bar 1: durations sum to 3/4"
check_error tests/errors/invalid-staff-label.typ "staff violin label: label must be a non-empty string"
check_error tests/errors/invalid-tempo-beat.typ "unsupported metronome beat"
check_error tests/errors/invalid-tuplet-option.typ "tuplet bracket must be auto, always, or never"
check_error tests/errors/invalid-arpeggio.typ "arpeggio annotation requires a chord"
check_error tests/errors/invalid-alternating-tremolo.typ "alternating tremolo must contain exactly two notes or chords"
check_error tests/errors/invalid-grace-ending.typ "grace group must be followed by a main note, chord, or rest"
check_error tests/errors/inconsistent-voice-count.typ "voice count changed from earlier bars"
check_error tests/errors/invalid-multistaff-clef-change.typ "single clef string cannot target a multi-staff score"

check_error tests/errors/invalid-clef-type.typ "unknown clef"
check_error tests/errors/unsupported-key.typ "unsupported key signature"
check_error tests/errors/malformed-time.typ "meter has invalid syntax"
check_error tests/errors/invalid-partial-type.typ "partial: meter must be a string"
check_error tests/errors/partial-longer-than-meter.typ "pickup duration is longer than the active meter"
check_error tests/errors/empty-voice.typ "string must contain at least one musical token"
check_error tests/errors/harmony-without-meter.typ "timed harmony requires an active meter"
check_error tests/errors/invalid-tempo-type.typ "score tempo: value must be text or content"
check_error tests/errors/invalid-tempo-text.typ "score tempo text: value must be text or content"
check_error tests/errors/invalid-composer.typ "score composer: value must be text or content"
check_error tests/errors/invalid-system-gap.typ "value must be a finite non-negative length"
check_error tests/errors/empty-barline.typ "barline dictionary does not select a boundary style"
check_error tests/errors/unknown-staff-clef.typ "staff violin clef: unknown clef"
check_error tests/errors/missing-staff-content.typ "bar is missing content for staff lower"
check_error tests/errors/unknown-clef-staff.typ "clef change references an unknown staff"
check_error tests/errors/top-clef-with-staves.typ "cannot be combined with staves"
check_error tests/errors/staff-gap-single-staff.typ "staff-gap requires at least two staves"
check_error tests/errors/group-single-staff.typ "visible staff group requires at least two staves"
check_error tests/errors/short-indent-without-wrap.typ "short-indent has no later system"
check_error tests/errors/system-numbers-without-wrap.typ "systems mode cannot produce a later system"
check_error tests/errors/invalid-pitch-token.typ "invalid event"
check_error tests/errors/unrenderable-octave.typ "supported range -1 through 9"
check_error tests/errors/too-many-dots.typ "at most two dots are supported"
check_error tests/errors/unterminated-chord.typ "unterminated chord"
check_error tests/errors/trailing-event-input.typ "unexpected text after annotations"
check_error tests/errors/empty-annotation.typ "empty annotation block"
check_error tests/errors/duplicate-dynamic.typ "more than one dynamic annotation"
check_error tests/errors/note-annotation-on-rest.typ "cannot be attached to a rest"
check_error tests/errors/turn-fingering-without-turn.typ "requires turn or chromatic-turn"
check_error tests/errors/duplicate-tuplet-option.typ "side option is repeated"
check_error tests/errors/zero-tuplet-ratio.typ "tuplet ratio values must be positive"
check_error tests/errors/unclosed-slur.typ "was never closed"
check_error tests/errors/unopened-hairpin.typ "hairpin h1 closes without opening"
check_error tests/errors/impossible-layout-width.typ "no legal system partition fits the requested width"
check_error tests/errors/empty-bars.typ "bars must be a non-empty array"
check_error tests/errors/empty-staves.typ "staves must be a non-empty dictionary"
check_error tests/errors/reserved-staff-id.typ "staff ID is reserved for bar metadata"
check_error tests/errors/unknown-staff-field.typ "staff configuration has unknown field"
check_error tests/errors/invalid-voice-member.typ "voice 2: value must be a string"
check_error tests/errors/too-many-voices.typ "voice array has an unsupported size"
check_error tests/errors/invalid-width.typ "width: value must be a positive number"
check_error tests/errors/invalid-note-spacing.typ "note-spacing: value must be a positive number"
check_error tests/errors/invalid-indent.typ "indent: value must be a non-negative number"
check_error tests/errors/invalid-wrap.typ "score wrap: value must be a boolean"
check_error tests/errors/invalid-ragged-right.typ "score ragged-right: unsupported value"
check_error tests/errors/invalid-first-bar-number.typ "score first-bar-number: value must be a positive integer"
check_error tests/errors/unknown-tempo-field.typ "tempo dictionary has unknown field"
check_error tests/errors/invalid-barline-value.typ "unsupported right barline"
check_error tests/errors/invalid-ending-flag.typ "start and stop must be booleans"
check_error tests/errors/duplicate-slur-open.typ "slur s1 opens twice"
check_error tests/errors/untimed-auto-rest.typ "needs a time signature context"
check_error tests/errors/trailing-beam-marker.typ "cannot end a sequence"
check_error tests/errors/tie-after-rest.typ "cannot follow a rest"
check_error tests/errors/ignored-unwrapped-width.typ "width is ignored by an unwrapped ragged score"
check_error tests/errors/duplicate-chord-pitch.typ "repeats the same written pitch"
check_error tests/errors/nonfinite-width.typ "width: value must be a positive number"
