// typed-scores documentation
//
// Compile with:
//   typst compile --root . docs/documentation.typ docs/documentation.pdf

#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#import "../src/lib.typ": score, bar
#import "../examples/chopin-opening.typ": chopin-opening
#import "../examples/mozart-eine-kleine-nachtmusik.typ": mozart-k525-opening
#import "../examples/beethoven-ode-to-joy-alto-sax.typ": ode-to-joy-alto-sax

#let version = "0.1.0"
#let accent = rgb("#7A2141")
#let accent-soft = rgb("#F7E8EE")

// ── Theme shared with the typed-* package family ────────────────────────────

#set document(title: "typed-scores User Guide", author: "Geronimo Castaño")
#set text(font: "New Computer Modern", size: 10.5pt, lang: "en")
#set par(justify: true, leading: 0.62em, spacing: 1.1em)
#set heading(numbering: "1.1")
#show link: set text(fill: accent)
#show ref: set text(fill: accent)
#show raw.where(block: true): set block(breakable: false)

#show heading.where(level: 1): it => {
  pagebreak(weak: true)
  block(width: 100%, breakable: false, sticky: true, {
    set text(fill: accent, size: 18pt, weight: "bold")
    if it.numbering != none [#counter(heading).display() #h(0.5em)]
    it.body
    v(-0.2em)
    line(length: 100%, stroke: 0.6pt + accent)
  })
}
#show heading.where(level: 2): it => block(above: 1.3em, below: 0.7em, sticky: true, {
  set text(fill: accent.darken(8%), size: 13pt, weight: "bold")
  if it.numbering != none [#counter(heading).display() #h(0.4em)]
  it.body
})
#show heading.where(level: 3): it => block(above: 1.1em, below: 0.6em, sticky: true, {
  set text(fill: accent.darken(18%), size: 11pt, weight: "bold")
  it.body
})

#show: codly-init.with()
#codly(
  languages: codly-languages,
  zebra-fill: none,
  inset: (x: 0.5em, y: 0.32em),
  radius: 0pt,
  stroke: none,
)

#let pkg-scope = (score: score, bar: bar)

#let example(body, side: true) = block(
  width: 100%,
  stroke: 0.6pt + luma(210),
  radius: 5pt,
  clip: true,
  breakable: false,
  {
    let rendered = block(
      width: 100%,
      fill: white,
      inset: (x: 12pt, y: 14pt),
      align(center, eval(body.text, mode: "markup", scope: pkg-scope)),
    )
    if side {
      grid(
        columns: (1.05fr, 0.95fr),
        column-gutter: 0pt,
        block(width: 100%, fill: luma(248), inset: (y: 4pt), body),
        block(width: 100%, stroke: (left: 0.6pt + luma(220)), rendered),
      )
    } else {
      block(width: 100%, fill: luma(248), inset: (y: 4pt), body)
      block(width: 100%, stroke: (top: 0.6pt + luma(220)), rendered)
    }
  },
)

#let demo(body) = block(breakable: false, width: 100%, body)

#let callout(clr, label, body) = block(
  width: 100%,
  fill: clr.lighten(90%),
  stroke: (left: 2.5pt + clr),
  radius: (right: 4pt),
  inset: (x: 12pt, y: 9pt),
  above: 1.1em,
  below: 1.1em,
  { text(fill: clr, weight: "bold")[#label.#h(0.5em)]; body },
)
#let note-box(body) = callout(accent, "Note", body)
#let warn(body) = callout(rgb("#b54708"), "Note", body)

#let c(it) = {
  let code-breaks = (".", "[", "]", "-", "_", "/", ":", ",", "(", ")", "+", " ")
  text(font: "DejaVu Sans Mono", size: 0.7em, fill: accent.darken(10%))[
    #for ch in it.clusters() {
      ch
      if ch in code-breaks { sym.zws }
    }
  ]
}

#let argtable(..rows) = table(
  columns: (22%, 14%, 24%, 40%),
  inset: 5.4pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft },
  stroke: 0.5pt + luma(210),
  [*Argument*], [*Type*], [*Default*], [*Description*],
  ..rows,
)

#let reference-table(columns, ..cells) = table(
  columns: columns,
  inset: 6.2pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft },
  stroke: 0.5pt + luma(210),
  ..cells,
)

// ── Cover ───────────────────────────────────────────────────────────────────

#set page(
  paper: "a4",
  margin: (x: 2.2cm, top: 2.6cm, bottom: 2.4cm),
  header: none,
  footer: none,
)

#v(1.1fr)
#align(center)[
  #text(size: 40pt, weight: "bold", fill: accent)[typed-scores]
  #v(0.2em)
  #text(size: 15pt, fill: luma(90))[Western music notation, natively in Typst]
  #v(0.4em)
  #text(size: 12pt, weight: "bold")[User Guide]
  #v(1.6em)
  #block(
    width: 100%,
    fill: white,
    stroke: 0.6pt + luma(215),
    radius: 8pt,
    inset: 12pt,
    image("../assets/readme/chopin-opening.png", width: 100%),
  )
  #v(1.4em)
  #text(size: 11pt)[Version #version]
]
#v(1.5fr)

// ── Header and footer ───────────────────────────────────────────────────────

#set page(
  header: context {
    set text(size: 8.5pt, fill: luma(130))
    grid(
      columns: (1fr, auto),
      align(left)[typed-scores · User Guide],
      align(right)[v#version],
    )
    v(-0.6em)
    line(length: 100%, stroke: 0.4pt + luma(210))
  },
  footer: context {
    set text(size: 8.5pt, fill: luma(130))
    align(center, counter(page).display("1"))
  },
)
#counter(page).update(1)

#pagebreak()
#outline(title: [Contents], indent: 1.2em, depth: 2)

= Introduction

`typed-scores` turns compact musical text into engraved western notation
inside Typst. A Rust/WASM plugin parses pitches, durations, exact onsets, and
beam groups; Typst and CeTZ lay out the score with bundled Bravura glyphs.

The package is designed for lecture notes, theory worksheets, analytical
examples, short compositions, and reusable excerpts whose musical content
should remain readable and versionable as text.

#note-box[The public API intentionally has only two entry points:
#c("score") for complete scores and #c("bar") for a quick one-staff measure.]

#demo[
  #example(```typ
#score(
  time: "4/4",
  bars: ((notes: "c4:q d e f"),),
)
```)
]

= Getting started

== Installation and import

Import the package from the Typst preview namespace:

```typ
#import "@preview/typed-scores:0.1.0": *
```

The examples in this guide use lowercase pitch letters for fast entry.
Uppercase remains accepted with identical pitch and octave semantics.

== Quick measure with #raw("bar()")

`bar` is a convenience wrapper over the same score pipeline used by `score`.
It accepts a note string plus its clef, key, and optional meter.

#demo[
  #example(```typ
#bar(
  "g4:q a b c5",
  clef: "treble",
  key: "G",
  time: "4/4",
)
```)
]

With `time` omitted, the bar remains useful for a short fragment and does not
perform full-measure duration validation.

#argtable(
  [#c("notes")], [`str`], [required], [One event string.],
  [#c("clef")], [`str`], [#c("\"treble\"")], [Treble, bass, alto, or tenor clef.],
  [#c("key")], [`str`], [#c("\"C\"")], [Major or minor key signature.],
  [#c("time")], [`str | none`], [`none`], [Meter and expected duration, such as #c("\"4/4\"").],
)

= Scores and systems <scores>

Every score is an ordered `bars:` list. Each bar is a dictionary: it contains
staff event strings and optional metadata that belongs to that musical bar.
Never place structural bar delimiters inside an event string.

== One staff

For one staff, omit `staves:` and use the reserved `notes:` field in every
bar. There is no artificial staff name to repeat. A string is one rhythmic
voice; an array of two to four strings creates independent simultaneous voices
on the same staff.

#demo[
  #example(```typ
#score(
  clef: "treble",
  time: "4/4",
  bars: (
    (notes: "c5:q d5:q e5:q f5:q"),
    (notes: "g5:h e5:h"),
  ),
)
```, side: false)
]

== Multiple staves

For two or more staves, declare a map of permanent staff IDs and clefs. Every
bar must then contain exactly one string or voice-string array for each declared
ID.

#demo[
  #example(```typ
#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  key: "Eb",
  time: "3/8",
  bars: (
    (
      upper: "bb4:e c5:e d5:e",
      lower: "r:e r:e r:e",
    ),
    (
      upper: "eb5:e f5:e g5:e",
      lower: "eb2:q eb2:e",
    ),
  ),
  beams: true,
)
```, side: false)
]

Staff IDs are Typst dictionary keys, so keep them short and machine-oriented.
They cannot be `notes`, `key`, `time`, `partial`, or `tempo`.

== Independent voices

Use an array when notes on one staff have independent rhythms. The voice count
must stay constant for that staff throughout the score. Following LilyPond's
standard polyphonic convention, the first voice uses upward stems, the second
downward stems, and later voices alternate. Rests move into separate vertical
lanes. Coincident identical noteheads merge; unisons and seconds that cannot
merge are shifted horizontally while retaining their shared onset.

#demo[
  #example(```typ
#score(
  time: "4/4",
  bars: (
    (notes: (
      "e5:h f5:h",
      "c5:q d5:q e5:q f5:q",
    )),
    (notes: (
      "(e5 g5):q (f5 a5):q g5:h",
      "e5:q f5:q (d5 f5):h",
    )),
  ),
)
```, side: false)
]

Each staff may also define `label` for its first-system name and `short-label`
for the optional name on later systems. This follows LilyPond's
`instrumentName` / `shortInstrumentName` behavior: names live in a shared
column to the left of the group, center horizontally within that column, and
center vertically on their individual staff. The compact example below shows
the full first-system names; wrapped systems substitute the short names.

#demo[
  #example(```typ
#score(
  staves: (
    violin-one: (clef: "treble", label: "Violin I", short-label: "Vln. I"),
    violin-two: (clef: "treble", label: "Violin II", short-label: "Vln. II"),
    viola: (clef: "alto", label: "Viola", short-label: "Vla."),
    cello: (clef: "bass", label: "Violoncello", short-label: "Vc."),
  ),
  group: "bracket",
  time: "2/4",
  scale: 0.58,
  ragged-right: false,
  bars: (
    (violin-one: "g5:q a5:q", violin-two: "d5:q e5:q", viola: "a3:q b3:q", cello: "g2:q a2:q"),
    (violin-one: "b5:q c6:q", violin-two: "f5:q g5:q", viola: "c4:q d4:q", cello: "b2:q c3:q"),
    (violin-one: "d6:q c6:q", violin-two: "a5:q g5:q", viola: "e4:q d4:q", cello: "d3:q g2:q"),
  ),
)
```, side: false)
]

The `group` argument controls the symbol at the left of a multi-staff system:
`"brace"`, `"bracket"`, `"line"`, or `"none"`. Its default, `auto`, uses no
symbol for one staff, a brace for two staves, and a bracket for larger
ensembles. Brace groups connect barlines across their staves; the other styles
keep each staff's barline separate.

== Bar metadata

`clef`, `key`, `time`, and `tempo` may be set on any bar and persist until changed.
`partial` changes duration validation only: it does not change the displayed
meter. When a key change removes old alterations, cancellation naturals are
engraved before the replacement key signature. This replaces separate section
or parallel-array forms.

For a single staff, a clef change is `clef: "bass"`. For multiple staves, use
a dictionary containing only the staves that change, such as
`clef: (left: "bass")`. As in LilyPond, a mid-system change is reduced in size;
the active clef is printed full-size when a new system begins.

#demo[
  #example(```typ
#score(
  clef: "treble",
  key: "C",
  time: "4/4",
  bars: (
    (
      notes: "c5:q d5:q e5:q f5:q",
      tempo: (text: [Moderato], beat: "quarter", bpm: 108),
    ),
    (
      key: "Eb",
      time: "3/4",
      notes: "eb5:q f5:q g5:q",
    ),
  ),
)
```, side: false)
]

For a metronome mark, use a tempo dictionary rather than inserting a Unicode
note character. `beat` accepts `whole`, `half`, `quarter`, `eighth`,
`sixteenth`, or `thirty-second`, and the matching note glyph is drawn
automatically. `text` is optional, so `(beat: "eighth", bpm: 132)` produces
just the note-and-number mark.

== Harmony symbols

Use a bar's #c("harmony") field for chord symbols above the top staff. It is
an independent duration-bearing sequence: each token is author-controlled text
followed by #c(":") and a duration, and the sequence must fill the active bar.
Harmony changes use the same onset positions as the notes, including changes
that fall between melody note onsets. Each symbol is centered on the onset
where the harmony becomes active, independent of its duration. Symbols are text,
so jazz spellings such as #c("Cmaj7"), #c("F#7(b9)"), #c("Bb/D"), and
#c("N.C.") remain under the author's control.

#demo[
  #example(```typ
#score(
  time: "4/4",
  bars: (
    (
      notes: "c5:q d5:q e5:q f5:q",
      harmony: "Cmaj7:h A7:h",
    ),
    (
      notes: "g5:h e5:h",
      harmony: "Dm7:q G7:q Cmaj7:h",
    ),
  ),
)
```, side: false)
]

== Barlines, navigation, and labeled endings

Repeat signs belong to a bar edge. Use `repeat-start` on a left edge and
`repeat-end` on a right edge. A labeled ending is a span: mark its first bar
with `start: true` and its last bar with `stop: true`. `label` is rendered
literally, so punctuation and text such as `"1."` or `"Final"` are both valid.

#demo[
  #example(```typ
#score(
  time: "2/4",
  bars: (
    (barline: (left: "repeat-start"), notes: "c5:q d5:q"),
    (
      barline: (right: "repeat-end"),
      ending: (label: "1.", start: true, stop: true),
      notes: "e5:q f5:q",
    ),
    (ending: (label: "Final", start: true), notes: "g5:q a5:q"),
    (ending: (label: "Final", stop: true), notes: "g5:h"),
  ),
)
```, side: false)
]

A repeat end followed immediately by a repeat start becomes one double-sided
repeat barline. Volta brackets continue across system wraps. Endings are
currently sequential: one bracket must stop before another one starts.

A right bar edge may also be `double`, `final`, or `dashed`. Use a bar's
`rehearsal` field for a boxed rehearsal label and `navigation` for `"segno"`,
`"coda"`, arbitrary text, or Typst content. Marks attach to the beginning of
their bar. When a bar number and mark coincide, the number stays closest to the
staff and the mark stacks above it. Long boundary text reserves enough
horizontal space to clear the following bar. `bar-numbers: "systems"` prints the first bar number of later systems,
matching LilyPond's usual sparse numbering; `"all"` prints every bar number.

#demo[
  #example(```typ
#score(
  time: "4/4",
  bar-numbers: "all",
  bars: (
    (rehearsal: "A", notes: "c5:w"),
    (clef: "bass", navigation: "segno", barline: (right: "double"), notes: "c3:w"),
    (navigation: "D.S. al Coda", barline: (right: "dashed"), notes: "g2:w"),
    (clef: "treble", navigation: "coda", barline: (right: "final"), notes: "c5:w"),
  ),
)
```, side: false)
]

#pagebreak()

== Score arguments

#argtable(
  [#c("clef")], [`str`], [#c("\"treble\"")], [Clef for the implicit single staff only.],
  [#c("staves")], [`dictionary | none`], [`none`], [Permanent staff-ID map. Each spec has `clef`, plus optional first-system `label` and later-system `short-label`.],
  [#c("bars")], [`array`], [`()`], [Required non-empty array of bar dictionaries.],
  [#c("key")], [`str`], [#c("\"C\"")], [Initial key signature.],
  [#c("time")], [`str`], [#c("\"4/4\"")], [Initial meter and full-bar duration.],
  [#c("tempo")], [`str | content | dictionary | none`], [`none`], [Tempo text, or `(text:, beat:, bpm:)` for an engraved metronome mark.],
  [#c("composer")], [`str | content | none`], [`none`], [Composer credit above the first system.],
  [#c("width")], [`number | none`], [`none`], [Packing width in staff spaces.],
  [#c("scale")], [`number`], [`1.0`], [Uniform staff-space scale.],
  [#c("note-spacing")], [`number`], [`3.1`], [Horizontal density: the width in staff spaces of a quarter note in a bar of quarters; other durations scale logarithmically from the bar's shortest note.],
  [#c("beams")], [`bool`], [`false`], [Connect automatic beam groups.],
  [#c("staff-gap")], [`number | none`], [`none`], [Fixed distance between adjacent staff lines.],
  [#c("group")], [`auto | str`], [`auto`], [Auto, brace, bracket, line, or none.],
  [#c("wrap")], [`bool`], [`true`], [Wrap complete bars into systems.],
  [#c("indent")], [`number`], [`0`], [First-system indentation in staff spaces. The system still ends at the normal right edge.],
  [#c("short-indent")], [`number`], [`0`], [Indentation for every system after the first.],
  [#c("ragged-right")], [`auto | bool`], [`auto`], [Leave every system at natural width. Auto is ragged only for a one-system score.],
  [#c("ragged-last")], [`bool`], [`false`], [Leave only the final system at its natural width.],
  [#c("system-gap")], [`length`], [`1.2em`], [Vertical gap between systems.],
  [#c("bar-numbers")], [`false | str`], [`false`], [Use `systems` for later system starts or `all` for every bar.],
  [#c("first-bar-number")], [`int`], [`1`], [Positive number assigned to the first bar.],
)

== Indentation and justification

Multi-system scores fill #c("width") by default: note spacing stretches or
compresses so each system's bars fill the line, and adjacent systems keep a
similar density rather than alternating cramped and loose lines. There is no
fixed bar count per line. A one-system score stays at natural width with the
default #c("ragged-right: auto").

Use #c("indent") for the first system and #c("short-indent") for every later
one; both reduce the usable line width while retaining a shared right edge.
Set #c("ragged-last: true") for a natural-width final line.

#demo[
  #example(```typ
#score(
  time: "4/4",
  width: 36,
  indent: 3.5,
  ragged-last: true,
  bars: (
    (notes: "c5:q d e f"),
    (notes: "g5:q a b c6"),
    (notes: "c6:h g5:h"),
    (notes: "f5:q e d c"),
    (notes: "b4:h c5:h"),
    (notes: "d5:q e f g"),
  ),
)
```, side: false)
]

= Event language

#reference-table(
  (35%, 65%),
  [*Syntax*], [*Meaning*],
  [#c("c4:q") or #c("c4q")], [Quarter note.],
  [#c("ce") or #c("c:e")], [Relative C with an eighth-note duration.],
  [#c("c4:q d e f")], [Four quarter notes using inherited register and duration.],
  [#c("bb4:e.")], [Dotted eighth note.],
  [#c("c##5:q") / #c("dbb5:q")], [Double-sharp / double-flat quarter note.],
  [#c("(a4 c e):h (f a c)")], [Relative chord pitches and inherited duration.],
  [#c("r:q")], [Quarter rest.],
  [#c("_")], [Automatic rest placeholder.],
  [#c("~")], [Tie the previous event to the next event.],
  [#c("/")], [Break the automatic beam before the next event.],
  [#c("-")], [Join adjacent flagged events into one beam group.],
  [#c("tuplet 3:2 { c:e d e }")], [Inline time-scaled music group.],
  [#c("acciaccatura { d:e } f:q")], [Slashed single grace note before F.],
  [#c("tremolo 16 { c:h g:h }")], [Two-note alternating sixteenth tremolo.],
)

== Duration inheritance

Durations are `w`, `h`, `q`, `e`, `s`, and `t`; append `.` or `..` for dots.
An explicit duration becomes the current duration for that staff. Subsequent
notes, chords, and ordinary rests may omit it and inherit the complete value,
including dots, across barlines. With no preceding value, the default is `q`.
The computed-rest placeholder `_` does not change this state, and a written
colon must always be followed by a duration. An omitted duration never expands
to fill the measure; normal bar-duration validation still reports an incorrect
total.

== Pitch spelling and relative octaves

Pitch letters are ASCII case-insensitive. `c4`, `C4`, and mixed-case input have
identical pitch and octave semantics; this guide uses lowercase as the compact
style, while uppercase remains valid. Compact `ce` and explicit `c:e` both
mean an eighth-note C. Whitespace distinguishes that single event from `c e`,
which means two notes using the inherited duration. Capitalization never
selects an octave.

Pitches accept `#`, `b`, `##`, or `bb`. In lowercase, `bb4` is B-flat 4 and
`bbb4` is B-double-flat 4. A single note with an explicit octave anchors later
octave-less single notes. Each omitted octave selects the nearest diatonic
pitch to the preceding resolved note, so `g4:e a b c` ends on C5 and all four
notes are eighths. Accidentals and the active key signature do not affect that
register choice. The anchor continues across bars independently for each
staff, and any explicit octave resets it. At the beginning of an unanchored
staff, treble, alto, and tenor start in octave 4; bass starts in octave 3.

== Relative chords

Chords follow LilyPond's relative-entry model. The first written chord pitch is
resolved from the current staff anchor. Each later chord pitch is resolved from
the pitch immediately before it inside the chord. Afterward, the first written
chord pitch becomes the external anchor for the next event. For example,
`(a4 c e):h (f a c)` resolves to `(A4 C5 E5):h (F4 A4 C5):h`.
Explicit octaves remain exact overrides for open or otherwise non-nearest
voicings.

== Beam boundaries

Standalone `/` and `-` are local beam-boundary controls. `/` splits before the
next event; `-` joins the flagged event on each side even across an automatic metric
boundary. A join at an edge, beside a rest, or beside an unflagged duration is
an error. For example, this 3/8 measure is grouped 3+3 rather than the automatic
2+2+2 sixteenths, and the next bar demonstrates relative chord voicing. No
`key` argument is needed here: `score` defaults to C major.

#demo[
  #example(```typ
#score(
  time: "3/8",
  beams: true,
  bars: (
    (notes: "g4:s a - b / c - d e"),
    (notes: "(c5 e g):e (d f a) (e g b)"),
  ),
)
```, side: false)
]

== Tuplets

Tuplets are inline music groups: `tuplet 3:2 { c:e d e }` writes three
eighths in the time normally occupied by two. Any positive ratio is supported,
and groups may nest. The group preserves pitch and duration inheritance, so
only its first event needs an explicit octave or duration when the surrounding
context already supplies one.

By default the numerator is centered on the group, and a bracket is omitted
when one visible beam spans every event in the tuplet; otherwise a bracket is
drawn. Written rests keep the bracket. Use optional group controls when the
engraving needs an explicit choice: `bracket=always`, `bracket=never`,
`side=above`, or `side=below`.

#demo[
  #example(```typ
#score(
  time: "3/4",
  beams: true,
  bars: (
    (
      notes: "tuplet 3:2 { c5:e d e } tuplet 3:2 { f5:e r:e g5:e } c6:q",
    ),
  ),
)
```, side: false)
]

== Grace notes

Grace groups stay inline and consume no bar time. `grace { ... }` writes plain
small grace notes. `acciaccatura { ... }` adds a slur to the following
principal note, as does `appoggiatura { ... }`. For a multi-note group, the
slur starts at the first grace event. A single flagged acciaccatura also
receives the conventional slash; a multi-note beamed acciaccatura does not.
Consecutive flagged grace notes beam as an independent group. Their written
durations control flags and beams but do not contribute to the measure total.

#demo[
  #example(```typ
#score(
  time: "4/4",
  beams: true,
  bars: (
    (notes: "acciaccatura { b4:e } c5:q acciaccatura { d5:s e } f5:q appoggiatura { g5:e } a5:q grace { b5:s c6 } d6:q"),
  ),
)
```, side: false)
]

A grace group must contain notes or chords and must precede a principal event.
Rests, automatic rests, tuplets, alternating tremolos, and nested grace groups
are rejected inside it so timing remains unambiguous.

== Tremolos and arpeggios

For a repeated single event, append `tremolo=8`, `16`, `32`, or `64` in its
annotation block. These values draw one, two, three, or four strokes
respectively, independent of the event's written duration. For an
alternation, place exactly two equal-duration notes or chords in
`tremolo subdivision { ... }`; the connecting strokes replace ordinary flags
or beams.

Append `arpeggio`, `arpeggio=up`, or `arpeggio=down` to a chord. The arpeggio
wave spans the full chord immediately to its left; the directional forms add
an arrow at the appropriate end.

#demo[
  #example(```typ
#score(
  time: "4/4",
  bars: (
    (notes: "c5:h[tremolo=16] (e5 g5 c6):h[arpeggio]"),
    (notes: "tremolo 16 { c5:h g5:h }"),
    (notes: "(c5 e5 g5 c6):h[arpeggio=up] (d5 f5 a5 d6):h[arpeggio=down]"),
  ),
)
```, side: false)
]

== Expressive annotations

Annotations follow an event in square brackets. Supported annotations include
fingering (`f=4`), staccato, staccatissimo, tenuto, accent, marcato, text
directions, turns, fermatas (`fermata`), breath marks (`breath`), named slurs,
pedal spans, hairpins, and dynamics (`dyn=p`, `dyn=pp`, `dyn=mf`, `dyn=sfz`,
and the other standard SMuFL combinations).

Dynamics, hairpins, slurs, articulations, and fingerings avoid collisions with
each other automatically: a hairpin shortens around an adjacent dynamic such
as a closing `dyn=ff`, and fingerings rise above stacked articulations rather
than overlapping them.
Named slurs over beamed passages follow the beam group's stem direction, so a
slur goes below when every note it spans stems upward and above when any note
stems downward.

#reference-table(
  (34%, 66%),
  [*Syntax*], [*Mark*],
  [#c("f=4")], [Fingering.],
  [#c("stacc") / #c("staccatissimo")], [Staccato dot / wedge.],
  [#c("tenuto") / #c("accent") / #c("marcato")], [Articulation marks.],
  [#c("turn") / #c("chromatic-turn")], [Turn ornament.],
  [#c("dyn=pp") / #c("dyn=sfz")], [Dynamic marking.],
  [#c("fermata") / #c("breath")], [Fermata / comma breath mark.],
  [#c("s1(") … #c("s1)")], [Named slur.],
  [#c("p1(") … #c("p1)")], [Pedal span.],
  [#c("h1<") or #c("h1>") … #c("h1!")], [Crescendo / diminuendo hairpin.],
)

#demo[
  #example(```typ
#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  bars: (
    (notes: "c5:q[f=1 s1( dyn=p] d5:q[h1<] e5:q[turn breath] f5:q[s1) h1! dyn=ff fermata]"),
  ),
)
```, side: false)
]

= Validation and layout

The package validates every full bar against its active `time` signature and
every pickup against `partial`. It also reports unknown staff IDs, missing
staff content, malformed or undocumented annotations, impossible automatic
rests, invalid slur/pedal/hairpin lifecycles, and tie errors with staff and bar
context. A tie must connect immediately adjacent events with the same written
pitch or chord; it is split cleanly when the score wraps and suppresses a
redundant continuation accidental. Multi-staff onsets share horizontal
positions, and bars remain atomic when systems wrap.

#reference-table(
  (38%, 62%),
  [*Condition*], [*Diagnostic*],
  [Malformed pitch or duration], [The parser identifies the invalid component.],
  [Wrong full-bar duration], [Actual reduced duration and expected meter.],
  [Wrong pickup duration], [Actual duration and declared #c("partial") value.],
  [Wrong harmony duration], [Actual reduced duration and expected meter.],
  [Unknown or missing staff ID], [The one-based bar and offending field.],
  [Invalid span lifecycle], [The slur, pedal, hairpin, or ending that cannot be paired.],
  [Invalid tie or beam join], [The adjacent events that cannot legally connect.],
)

== Worked example: Chopin

The canonical release fixture uses the multi-staff form and exercises a pickup,
compound beaming, ties, slurs, dynamics, turns, pedal spans, and chromatic
accidentals:

#align(center)[#chopin-opening(
  scale: 0.4,
  note-spacing: 3.35,
  wrap: true,
)]

= Release showcase

The repository's `examples/` directory contains five reusable, source-checked
fixtures and a compiled `showcase.pdf`:

- Chopin, Nocturne Op. 9 No. 2 — piano, pickup and measures 1–4;
- Mozart, Eine kleine Nachtmusik K. 525/I — four-staff strings, measures 1–4;
- Bach, Cello Suite No. 1 BWV 1007/I — solo cello, measures 1–4;
- Beethoven, the four-bar Ode to Joy theme — E-flat alto saxophone at written
  pitch; and
- Beethoven, Für Elise WoO 59 — piano, pickup and measures 1–4.

The saxophone fixture is an explicit transposition: its B-major notation sounds
in Beethoven's D major. The other four retain their reference score's written
pitches. This compact quartet excerpt demonstrates a bracketed ensemble:

#align(center)[#mozart-k525-opening(
  scale: 0.42,
  note-spacing: 2.7,
  wrap: true,
)]

The single-staff case needs no staff declaration, even for a transposing
instrument whose written pitch has been prepared by the author:

#align(center)[#ode-to-joy-alto-sax(scale: 0.66, note-spacing: 3.2)]

= Limitations

== Current scope

- One to four independent rhythmic voices per staff are supported; the count
  for a staff is fixed across its bars.
- Lyrics and cross-staff notation are not yet in the public DSL.
- Grace groups currently exclude rests, tuplets, and nested ornamental groups.
- Arpeggio signs span one chord on one staff; cross-staff piano arpeggios are
  not yet supported.
- Pedals and hairpins are event-anchored and do not split automatically at a
  system break.
- Dense editorial markings may require `staff-gap`, `note-spacing`, or
  `scale` adjustments.

== Grammar at a glance

#reference-table(
  (38%, 62%),
  [*Text*], [*Meaning*],
  [#c("c4:q") / #c("c4q")], [Note with an explicit octave and duration.],
  [#c("c4:q d e f")], [Relative pitches inheriting the quarter duration.],
  [#c("(a4 c e):h")], [Relative chord.],
  [#c("r:q") / #c("_")], [Written rest / computed remainder rest.],
  [#c("~")], [Tie the preceding event.],
  [#c("/") / #c("-")], [Break / force a local beam connection.],
  [#c("tuplet 3:2 { c:e d e }")], [Three written eighths in the time of two.],
  [#c("acciaccatura { d:e } f:q")], [Slashed single grace note resolving to F.],
  [#c("tremolo 16 { c:h g:h }")], [Two-note alternating tremolo.],
  [#c("(c e g):h[arpeggio=up]")], [Upward arpeggio over a chord.],
  [#c("[stacc dyn=pp]")], [One annotation block with multiple marks.],
  [#c("barline: (left: \"repeat-start\")")], [Repeat sign on a bar edge.],
  [#c("barline: (right: \"final\")")], [Final barline on the right edge.],
  [#c("clef: \"bass\"")], [Persistent single-staff clef change at a bar boundary.],
  [#c("rehearsal: \"A\"") / #c("navigation: \"segno\"")], [Boundary rehearsal / navigation mark.],
  [#c("ending: (label: \"Final\", start: true)")], [Literal volta label.],
)

== License and credits

Package code is released under the MIT License. Bundled Bravura glyphs are
released under the SIL Open Font License 1.1. `typed-scores` uses CeTZ for
drawing and Bravura's SMuFL metadata for music-glyph geometry.

The Chopin, Mozart, Bach, and Beethoven compositions used by the release
fixtures are public domain. Their reusable Typst sources live in the
#c("examples/") directory and are shared by the guide, showcase, and visual
regression suite.
