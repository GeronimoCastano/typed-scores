// typed-scores documentation
//
// Compile with:
//   typst compile --root . docs/documentation.typ docs/documentation.pdf

#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#import "../src/lib.typ": *
#import "../examples/chopin-opening.typ": chopin-opening

#let version = "0.1.0"
#let accent = rgb("#7A2141")
#let accent-soft = rgb("#F7E8EE")

// ── Theme ────────────────────────────────────────────────────────────────────

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

// ── codly and examples ───────────────────────────────────────────────────────

#show: codly-init.with()
#codly(
  languages: codly-languages,
  zebra-fill: none,
  inset: (x: 0.5em, y: 0.32em),
  radius: 0pt,
  stroke: none,
)

#let pkg-scope = (
  note: note,
  bar: bar,
  staff: staff,
  score: score,
  grand-staff: grand-staff,
)

#let example(body, side: true) = block(
  width: 100%,
  stroke: 0.6pt + luma(210),
  radius: 5pt,
  clip: true,
  breakable: false,
  {
    let rendered = block(
      width: 100%, fill: white, inset: (x: 12pt, y: 14pt),
      align(center + horizon, eval(body.text, mode: "markup", scope: pkg-scope)),
    )
    if side {
      grid(
        columns: (1.05fr, 0.95fr), column-gutter: 0pt,
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
  width: 100%, fill: clr.lighten(90%), stroke: (left: 2.5pt + clr),
  radius: (right: 4pt), inset: (x: 12pt, y: 9pt), above: 1.1em, below: 1.1em,
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
  columns: (22%, 14%, 24%, 40%), inset: 5.4pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Argument*], [*Type*], [*Default*], [*Description*],
  ..rows,
)

#let subargtable(title, ..rows) = block(width: 100%, above: 0.7em, below: 1.0em, {
  text(fill: accent.darken(18%), weight: "bold")[#title]
  v(0.35em)
  table(
    columns: (28%, 16%, 18%, 38%), inset: 5.2pt,
    align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
    fill: (x, y) => if y == 0 { accent-soft } else if x == 0 { luma(248) } else { none },
    stroke: 0.5pt + luma(215),
    [*Key*], [*Type*], [*Default*], [*Description*],
    ..rows,
  )
})

// ── Cover ────────────────────────────────────────────────────────────────────

#set page(paper: "a4", margin: (x: 2.2cm, top: 2.6cm, bottom: 2.4cm), header: none, footer: none)

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

// ── Header / footer ──────────────────────────────────────────────────────────

#set page(
  header: context {
    set text(size: 8.5pt, fill: luma(130))
    grid(columns: (1fr, auto),
      align(left)[typed-scores · User Guide],
      align(right)[v#version])
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

// ═════════════════════════════════════════════════════════════════════════════
= Introduction
// ═════════════════════════════════════════════════════════════════════════════

`typed-scores` turns compact musical text into engraved notation inside Typst.
The Rust/WASM layer parses pitches and exact durations; the Typst layer aligns
voices, packs systems, and draws bundled Bravura glyphs through CeTZ. No music
font installation or external engraving program is required.

Use the package for theory worksheets, analytical examples, lecture notes,
short compositions, and piano or ensemble excerpts where musical content
should remain readable and versionable as text.

#demo[
  #example(```typ
  #bar(
    "C4:q D4:e E4:e F4:q G4:e A4:e",
    clef: "treble",
    time: "4/4",
    beams: true,
  )
  ```)
]

The current release supports notes through thirty-seconds, two dots, chords,
rests, ties, automatic rests, beat-aware beams, accidentals, four clefs, key
and time signatures, named slurs, fingering, combined articulations, text directions, turns,
hairpins, pedal spans, pickup measures, grand staves, ensemble staves, and
automatic system wrapping.

// ═════════════════════════════════════════════════════════════════════════════
= Getting started
// ═════════════════════════════════════════════════════════════════════════════

== Installation and import

Import every public symbol from the Typst preview namespace:

```typ
#import "@preview/typed-scores:0.1.0": *
```

Or import only the builders you use:

```typ
#import "@preview/typed-scores:0.1.0": note, bar, score
```

The public rendering API is:

#table(
  columns: (auto, 1fr), inset: 7pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Symbol*], [*Purpose*],
  [#c("note")], [Render one note, chord, or rest on a staff.],
  [#c("bar")], [Render and optionally validate one measure.],
  [#c("score")], [Render one or more aligned staves across measures and systems.],
  [#c("grand-staff")], [Piano-oriented wrapper around #c("score").],
  [#c("staff")], [Compatibility wrapper for a single rendered event.],
)

#note-box[The low-level drawing functions from #c("render.typ") are also
exported. They are useful for custom extensions, but #c("note"), #c("bar"),
and #c("score") are the stable user-facing entry points.]

== Your first score

#demo[
  #example(```typ
  #score(
    treble: "C5:q D5:q E5:q F5:q",
    bass: "C3:h G2:h",
    key: "C",
    time: "4/4",
  )
  ```, side: false)
]

The strings contain musical events. The score shares every onset across its
staves, so notes beginning together receive the same horizontal coordinate.

// ═════════════════════════════════════════════════════════════════════════════
= Note language <note-language>
// ═════════════════════════════════════════════════════════════════════════════

== Pitches

Pitches use an uppercase letter, optional accidental, and scientific octave:

#table(
  columns: (auto, 1fr), inset: 6.5pt,
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Text*], [*Pitch*],
  [#c("C4")], [Middle C.],
  [#c("F#5")], [F-sharp in octave 5.],
  [#c("Bb2")], [B-flat in octave 2.],
  [#c("E4")], [E-natural. In a flat key this prints a natural sign when needed.],
)

The supported clefs are #c("treble"), #c("bass"), #c("alto"), and
#c("tenor"). Ledger lines are generated from the diatonic staff position.

== Durations and dots

#table(
  columns: (18%, 28%, 18%, 36%), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Code*], [*Value*], [*Code*], [*Value*],
  [#c("w")], [Whole], [#c("w.") / #c("w..")], [Dotted / double-dotted whole],
  [#c("h")], [Half], [#c("h.") / #c("h..")], [Dotted / double-dotted half],
  [#c("q")], [Quarter], [#c("q.") / #c("q..")], [Dotted / double-dotted quarter],
  [#c("e")], [Eighth], [#c("e.") / #c("e..")], [Dotted / double-dotted eighth],
  [#c("s")], [Sixteenth], [#c("s.") / #c("s..")], [Dotted / double-dotted sixteenth],
  [#c("t")], [Thirty-second], [#c("t.") / #c("t..")], [Dotted / double-dotted thirty-second],
)

Join pitch and duration with a colon (#c("Eb4:q")) or use the compact form
(#c("Eb4q")). The two spellings are identical.

== Notes, chords, and rests

#demo[
  #example(```typ
  #note("Bb4:e.", clef: "treble")
  #note("(C4 E4 G4):h", clef: "treble")
  #note("r:q", clef: "bass")
  ```)
]

Chord pitches are space-separated inside parentheses and share one duration.
Rests begin with #c("r:") and use the same duration codes.

== Ties

Put #c("~") after the source event. A chord tie produces one curve per pitch.

#demo[
  #example(```typ
  #bar(
    "G4:e ~ G4:q. C5:q ~ C5:q",
    time: "4/4",
  )
  ```)
]

Within a system, the tie goes to the next event. At a system edge it continues
to the margin and resumes on the next system.

== Automatic rests

An underscore is a rest placeholder. When a time signature is known, the
remaining duration is divided across all placeholders using supported note
values.

#demo[
  #example(```typ
  #bar(
    "_ Eb4:h _",
    key: "Eb",
    time: "4/4",
  )
  ```)
]

If the remaining value cannot be represented across the requested number of
placeholders, compilation fails with the remaining rational duration.

== Beams and manual breaks

Set #c("beams: true") to connect flagged notes. Groups break at rests, manual
#c("/") markers, and beat boundaries. Compound meters such as 6/8, 9/8, and
12/8 use dotted beats.

#demo[
  #example(```typ
  #bar(
    "E4:e F4:e G4:e / A4:e B4:e C5:e",
    time: "6/8",
    beams: true,
  )
  ```)
]

// ═════════════════════════════════════════════════════════════════════════════
= Expressive notation <annotations>
// ═════════════════════════════════════════════════════════════════════════════

Annotations live in square brackets after an event. They do not change the
event duration. Put spaces between multiple annotations.

```typ
G5:q.[f=4 s1(]
```

== Annotation reference

#table(
  columns: (31%, 1fr), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Annotation*], [*Meaning*],
  [#c("f=4")], [Fingering outside the articulation stack on its placement side.],
  [#c("stacc")], [Staccato dot.],
  [#c("staccatissimo")], [Wedge staccatissimo.],
  [#c("tenuto") / #c("legato")], [Tenuto line.],
  [#c("accent")], [Accent.],
  [#c("marcato") / #c("strong")], [Marcato.],
  [#c("text=espress._dolce")], [Italic direction near the staff; underscores become spaces.],
  [#c("text-below=Ped._simile")], [Italic direction below low beams and pedal marks.],
  [#c("turn")], [Turn ornament.],
  [#c("chromatic-turn")], [Turn with flat-above and natural-below auxiliaries.],
  [#c("turn-f=12121")], [Fingering shown above a turn.],
  [#c("s1(") / #c("s1)")], [Open / close named slur #c("s1").],
  [#c("p1(") / #c("p1)")], [Open / close pedal span #c("p1").],
  [#c("h1<") / #c("h1!")], [Open crescendo / stop hairpin #c("h1").],
  [#c("h1>") / #c("h1!")], [Open diminuendo / stop hairpin #c("h1").],
)

IDs pair the ends of a spanner. They may contain letters and digits. A slur ID
must begin with #c("s"), a pedal ID with #c("p"), and a hairpin ID with
#c("h"). Use distinct IDs for overlapping spans.

Articulations combine by listing multiple names, such as #c("[marcato stacc]"),
#c("[tenuto staccatissimo]"), or #c("[accent tenuto]"). Duration marks sit
nearest the notehead and force accents outside them. The entire stack follows
the notehead side of the effective stem: above stem-down notes and below
stem-up notes, including beamed groups.

== Slurs

Named slurs may stay within a measure, cross barlines, nest, or overlap. Slurs
are placed above their anchor notes and raised to clear interior note anchors.

#demo[
  #example(```typ
  #score(
    treble: (
      "C5:q[s1(] D5:q E5:q F5:q",
      "G5:q A5:q B5:q C6:q[s1)]",
    ),
    bass: ("C3:w", "C3:w"),
    time: "4/4",
  )
  ```, side: false)
]

== Fingering, text, and ornaments

#demo[
  #example(```typ
  #bar(
    "C5:q[f=1 text=espress.] D5:q[f=2] E5:q[turn turn-f=321] F5:q[stacc f=4]",
    time: "4/4",
  )
  ```, side: false)
]

== Hairpins and pedal spans

Hairpins attach to event onsets. Pedal spans print a Bravura Ped. mark and a
release bracket.

#demo[
  #example(```typ
  #score(
    treble: "C5:q[h1<] D5:q E5:q F5:q[h1!]",
    bass: "C3:e[p1(] G3:e E4:e[p1)] C3:e[p2(] G3:e E4:e[p2)] r:q",
    time: "4/4",
    beams: true,
    staff-gap: 11,
  )
  ```, side: false)
]

// ═════════════════════════════════════════════════════════════════════════════
= Single-staff builders
// ═════════════════════════════════════════════════════════════════════════════

== #raw("note()")

#c("note") draws one event with a clef and enough staff width for its ink.

#argtable(
  [#c("note-str")], [`str`], [required], [One note, chord, or rest event.],
  [#c("clef")], [`str`], [#c("\"treble\"")], [#c("\"treble\""), #c("\"bass\""), #c("\"alto\""), or #c("\"tenor\"").],
  [#c("width")], [`number | none`], [`none`], [Staff width in staff spaces; automatic when omitted.],
  [#c("scale")], [`number`], [`1.0`], [Uniform score scale. One staff space is #c("8pt * scale").],
)

== #raw("bar()")

#c("bar") parses a sequence, validates its duration when #c("time") is set,
and draws a single staff measure.

#argtable(
  [#c("sequence-str")], [`str`], [required], [Space-separated event sequence.],
  [#c("clef")], [`str`], [#c("\"treble\"")], [Clef name.],
  [#c("width")], [`number | none`], [`none`], [Explicit measure width in staff spaces.],
  [#c("scale")], [`number`], [`1.0`], [Uniform score scale.],
  [#c("note-spacing")], [`number`], [`3.1`], [Base rhythmic spacing in staff spaces.],
  [#c("beams")], [`bool`], [`false`], [Draw automatic beam groups.],
  [#c("key")], [`str`], [#c("\"C\"")], [Major or minor key name.],
  [#c("time")], [`str | none`], [`none`], [Time signature and expected duration, such as #c("\"4/4\"").],
)

== #raw("staff()")

#c("staff") is a compatibility wrapper around #c("note"). Pass its event in
#c("note-str:"). The #c("body") form is reserved for a future structured DSL.

// ═════════════════════════════════════════════════════════════════════════════
= Scores and systems <scores>
// ═════════════════════════════════════════════════════════════════════════════

== Parallel treble and bass

For a piano score, pass a string for one measure or arrays for multiple
measures. Parallel arrays must have equal lengths.

#demo[
  #example(```typ
  #score(
    treble: (
      "C5:q D5:q E5:q F5:q",
      "G5:h E5:h",
    ),
    bass: (
      "C3:h G2:h",
      "C3:w",
    ),
    time: "4/4",
  )
  ```, side: false)
]

An array item may be a dictionary with #c("notes"), #c("key"),
#c("time"), or #c("partial") fields. Key and meter changes found on one
voice apply to the whole measure; conflicting changes are rejected.

== Measure dictionaries

The #c("bars:") form keeps all measure-wide properties together.

```typ
#score(
  bars: (
    (
      key: "Eb",
      time: "12/8",
      partial: "1/8",
      treble: "Bb4:e",
      bass: "r:e",
    ),
    (
      treble: "G5:q. F5:e G5:e Bb5:e Ab5:q. G5:q F5:e",
      bass: "Eb2:e (G3 Eb4):e (Bb3 Eb4 G4):e Eb2:e (Ab3 D4):e (Cb4 D4 Ab4):e Eb2:e (G3 Eb4):e (Bb3 Eb4 G4):e D2:e (G3 Eb4):e (Bb3 Eb4 G4):e",
    ),
  ),
  beams: true,
)
```

#subargtable([Measure dictionary keys],
  [#c("key")], [`str`], [inherits], [Key signature from this measure onward.],
  [#c("time")], [`str`], [inherits], [Time signature from this measure onward.],
  [#c("partial")], [`str | none`], [`none`], [Exact duration for an incomplete measure, such as #c("\"1/8\"").],
  [#c("treble")], [`str`], [—], [Treble sequence; pair with #c("bass").],
  [#c("bass")], [`str`], [—], [Bass sequence; pair with #c("treble").],
  [#c("voices")], [`array`], [—], [Explicit voice dictionaries instead of treble/bass.],
)

== Pickup measures

#c("partial") changes validation, not the displayed meter. In the example
above, both staves must equal 1/8 while the prologue still prints 12/8. The
next measure returns to the inherited full 12/8 duration.

== Voice arrays

Use #c("voices:") for ensembles or non-piano clef combinations.

```typ
#score(
  voices: (
    (name: "Violin", clef: "treble", notes: (
      "E5:q F5:q G5:q A5:q",
      "G5:h E5:h",
    )),
    (name: "Viola", clef: "alto", notes: (
      "C4:q D4:q E4:q F4:q",
      "E4:h C4:h",
    )),
    (name: "Cello", clef: "bass", notes: (
      "C3:h G2:h",
      "C3:w",
    )),
  ),
  time: "4/4",
)
```

#subargtable([Voice dictionary keys],
  [#c("name")], [`str`], [generated], [Stable identity used in diagnostics and slur validation.],
  [#c("clef")], [`str`], [#c("\"treble\"")], [Voice clef.],
  [#c("notes")], [`str | array`], [required], [One measure or an array of measure strings/dictionaries.],
)

Every measure must keep the same voice count, names, and clefs. Start a new
#c("score") call when the ensemble itself changes.

== Sections

Sections carry new defaults across a run of measures. A section accepts
#c("key"), #c("time"), #c("tempo"), and one of #c("voices"), #c("bars"), or
parallel #c("treble") / #c("bass") arrays.

```typ
#score(
  sections: (
    (
      key: "C",
      time: "4/4",
      tempo: "Moderato",
      treble: ("C5:q D5:q E5:q F5:q", "G5:w"),
      bass: ("C3:h G2:h", "C3:w"),
    ),
    (
      key: "Cm",
      tempo: "Meno mosso",
      treble: ("Eb5:q D5:q C5:q Bb4:q", "C5:w"),
      bass: ("C3:h G2:h", "C3:w"),
    ),
  ),
)
```

== #raw("score()") argument reference

#argtable(
  [#c("body")], [`content | none`], [`none`], [Reserved structured-score body.],
  [#c("note-str")], [`str`], [#c("\"C4:q\"")], [Fallback event when no multi-staff input is supplied.],
  [#c("clef")], [`str`], [#c("\"treble\"")], [Clef for the fallback event.],
  [#c("voices")], [`array | none`], [`none`], [Explicit stable voice definitions.],
  [#c("sections")], [`array | none`], [`none`], [Section dictionaries with persistent defaults.],
  [#c("treble")], [`str | array | none`], [`none`], [Treble measure(s).],
  [#c("bass")], [`str | array | none`], [`none`], [Bass measure(s).],
  [#c("bars")], [`array | none`], [`none`], [Measure dictionaries.],
  [#c("key")], [`str`], [#c("\"C\"")], [Initial key. Minor names use #c("m"), such as #c("\"Cm\"").],
  [#c("time")], [`str`], [#c("\"4/4\"")], [Initial meter and default measure duration.],
  [#c("tempo")], [`str | content | none`], [`none`], [Tempo text above the first system or section.],
  [#c("bpm")], [`number | none`], [`none`], [Numeric suffix combined with #c("tempo") when both are set.],
  [#c("composer")], [`str | content | none`], [`none`], [Composer credit above the first system, right aligned.],
  [#c("width")], [`number | none`], [`none`], [System packing width in staff spaces; available layout width when omitted.],
  [#c("scale")], [`number`], [`1.0`], [Uniform staff-space scale.],
  [#c("note-spacing")], [`number`], [`3.1`], [Base horizontal rhythmic spacing.],
  [#c("beams")], [`bool`], [`false`], [Draw automatic beam groups.],
  [#c("staff-gap")], [`number | none`], [`none`], [Fixed bottom-line distance between adjacent staves.],
  [#c("wrap")], [`bool`], [`true`], [Pack measures into multiple systems.],
  [#c("system-gap")], [`length`], [`1.2em`], [Vertical Typst space between wrapped systems.],
)

== #raw("grand-staff()")

#c("grand-staff") forwards piano-oriented arguments to #c("score"). It
accepts #c("treble"), #c("bass"), or #c("bars") plus the same key, meter,
spacing, scale, wrapping, staff-gap, and composer options. Use #c("score") when
you need #c("voices"), #c("sections"), #c("tempo"), or #c("bpm").

== System packing

With #c("wrap: true"), each measure is packed atomically into the available
width. Every new system redraws clefs and the active key and time signatures.
Set #c("width") for predictable package-level packing independent of the
surrounding column width. Set #c("wrap: false") for an intentionally unbroken
line such as the Chopin release fixture.

// ═════════════════════════════════════════════════════════════════════════════
= Engraving behavior
// ═════════════════════════════════════════════════════════════════════════════

== Exact durations and onset alignment

Durations are stored as reduced rational fractions of a whole note. The WASM
plugin computes every event onset exactly; the Typst layout merges those
onsets across voices before assigning x coordinates. A half note and two
quarters therefore align without floating-point drift.

== Key signatures and accidental state

Major keys from C-flat through C-sharp are supported. Minor key names map to
their relative-major signatures. A matching accidental is suppressed by the
key signature. A conflicting accidental prints a sharp, flat, or natural and
persists for that letter and octave until the barline.

== Rhythmic spacing

#c("note-spacing") is the base quarter-note advance. Longer values receive
more room sub-linearly; shorter values are clamped so noteheads, flags, dots,
and accidentals remain clear. All staves share the strictest spacing demand at
each onset.

== Stems and beams

Single-event stems are chosen from the average staff position. Beam groups use
all pitches in the group, cap their slope, and enforce minimum stem lengths.
Mixed sixteenth/eighth groups receive full beam segments and local beam stubs.

== Curves

Tie curves go opposite the stem side. Slurs sit above their note anchors and
grow to clear interior events. Each edge is one exact quadratic parabola,
#c("y(t) = (1-t)sy + t*ey + 4h*t*(1-t)"), converted exactly to CeTZ's cubic
path representation. There is no midpoint join or piecewise change in
curvature.

== Bravura assets

Clefs, noteheads, rests, flags, time-signature digits, accidentals,
articulations, turns, pedal marks, braces, and brackets are SVGs extracted from
Steinberg’s Bravura SMuFL font. Geometry follows the bundled metadata and one
canvas unit always equals one staff space.

// ═════════════════════════════════════════════════════════════════════════════
= Worked example: Chopin <chopin>
// ═════════════════════════════════════════════════════════════════════════════

The release-gate fixture is the opening pickup and measures 1–2 of Frédéric
Chopin’s Nocturne in E-flat major, Op. 9 No. 2. It exercises the complete
first-release path: 12/8 compound beaming, an exact 1/8 pickup, key-signature
accidentals and E-natural cancellation, tied melody, local and long slurs,
fingerings, a chromatic turn, hairpins, low piano accompaniment, and pedals.

#block(width: 100%, breakable: false, inset: (y: 1em), [
  #align(center)[#chopin-opening(scale: 0.48, note-spacing: 3.6, staff-gap: 12.5)]
])

The canonical Typst source is #c("examples/chopin-opening.typ"). It is imported
by the visual test and by the README image generator, so all three artifacts
render the same musical data.

#raw(read("../examples/chopin-opening.typ"), lang: "typ", block: true)

The pitches, rhythms, and historical score text were checked against the
#link("https://www.loc.gov/item/2023842215/")[public-domain 1881 G. Schirmer score]
from the Library of Congress and the
#link("https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1590")[Mutopia transcription]
made from that scan.

// ═════════════════════════════════════════════════════════════════════════════
= Diagnostics and validation
// ═════════════════════════════════════════════════════════════════════════════

Errors begin with #c("typed-scores error") when they come from score-level
validation. Diagnostics identify the voice and one-based bar number whenever
possible.

#table(
  columns: (37%, 1fr), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Condition*], [*Result*],
  [Malformed pitch or duration], [Parser reports the invalid character or missing component.],
  [Wrong full-measure duration], [Actual reduced duration and expected time signature.],
  [Wrong pickup duration], [Actual reduced duration and declared #c("partial") value.],
  [Unequal parallel arrays], [Voice name, actual bar count, and expected bar count.],
  [Voice structure changes], [Bar where names or clefs stop matching the first measure.],
  [Invalid slur lifecycle], [Named slur and the bar where it opens or closes incorrectly.],
  [Impossible automatic rests], [Remaining rational duration and placeholder count.],
)

Typical failures can be tested as source snippets without compiling them into
the guide:

```typ
#score(treble: ("C5:w", "D5:w"), bass: ("C3:w",))
#score(treble: "C5:q", bass: "C3:w", time: "4/4")
#score(treble: "C5:q[s1(] D5:q E5:q F5:q", bass: "C3:w")
```

// ═════════════════════════════════════════════════════════════════════════════
= Limitations <limitations>
// ═════════════════════════════════════════════════════════════════════════════

- One rhythmic voice per staff is supported. Simultaneous independent voices
  on one staff are not yet modeled; use separate staves when that is musically
  acceptable.
- Grace notes, tuplets, tremolos, arpeggio signs, lyrics, dynamics glyphs,
  repeat structures, volta brackets, and specialized final barlines are not
  yet in the public DSL.
- Enharmonic spellings support one sharp or flat. Double accidentals are not
  parsed.
- Pedals and hairpins are event-anchored. They do not split automatically at a
  system break yet.
- Dense editorial markings may require manual #c("staff-gap"),
  #c("note-spacing"), or #c("scale") adjustments.
- The exported #c("voice") and #c("pedal") function names are reserved for a
  future structured body DSL. Current multi-staff voices and pedal spans use
  score dictionaries and annotations.

// ═════════════════════════════════════════════════════════════════════════════
= Quick reference
// ═════════════════════════════════════════════════════════════════════════════

== Builders

#table(
  columns: (48%, 52%), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Call*], [*Result*],
  [#c("note(note-str, clef:, width:, scale:)")], [One event on a clefed staff.],
  [#c("bar(sequence-str, clef:, width:, scale:, note-spacing:, beams:, key:, time:)")], [One validated measure.],
  [#c("score(voices: | sections: | treble:/bass: | bars:, ...)")], [Aligned multi-measure score with wrapping.],
  [#c("grand-staff(treble:, bass:, bars:, ...)")], [Piano wrapper around #c("score").],
  [#c("staff(note-str:, clef:, width:, scale:)")], [Compatibility wrapper around #c("note").],
)

== Event grammar

#table(
  columns: (34%, 1fr), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Syntax*], [*Meaning*],
  [#c("C4:q") / #c("C4q")], [Single note.],
  [#c("(C4 E4 G4):h")], [Chord.],
  [#c("r:q")], [Rest.],
  [#c("_")], [Automatic rest placeholder.],
  [#c("~")], [Tie the preceding event.],
  [#c("/")], [Break beam before the next event.],
  [#c("[annotation ...]")], [One or more expressive annotations.],
)

== Score dictionary keys

#table(
  columns: (28%, 1fr), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Context*], [*Keys*],
  [Measure], [#c("key, time, partial, treble, bass, voices")],
  [Voice], [#c("name, clef, notes")],
  [Section], [#c("key, time, tempo, voices, bars, treble, bass")],
)

== Annotation summary

#table(
  columns: (38%, 1fr), inset: 6.5pt,
  align: (x, y) => if y == 0 { center + horizon } else { left + horizon },
  fill: (_, y) => if y == 0 { accent-soft }, stroke: 0.5pt + luma(210),
  [*Syntax*], [*Mark*],
  [#c("f=text")], [Fingering],
  [#c("stacc") / #c("staccatissimo")], [Staccato / wedge staccatissimo],
  [#c("tenuto") / #c("legato")], [Tenuto],
  [#c("accent")], [Accent],
  [#c("marcato") / #c("strong")], [Marcato],
  [#c("text=...") / #c("text-below=...")], [Italic directions],
  [#c("turn") / #c("chromatic-turn")], [Turn ornament],
  [#c("turn-f=text")], [Turn fingering],
  [#c("sID(") … #c("sID)")], [Slur],
  [#c("pID(") … #c("pID)")], [Pedal span],
  [#c("hID<") or #c("hID>") … #c("hID!")], [Hairpin],
)

// ═════════════════════════════════════════════════════════════════════════════
= License and credits
// ═════════════════════════════════════════════════════════════════════════════

Package code is released under the MIT License. Bundled Bravura glyphs are
released under the SIL Open Font License 1.1. The Chopin composition and the
1881 Schirmer score used for the worked example are public domain.

`typed-scores` uses CeTZ for drawing and Bravura’s SMuFL metadata for music-glyph
geometry.
