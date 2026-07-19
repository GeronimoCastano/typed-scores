#import "../src/lib.typ": score, bar
#import "../examples/chopin-opening.typ": chopin-opening
#import "../examples/mozart-eine-kleine-nachtmusik.typ": mozart-k525-opening
#import "../examples/bach-cello-suite-prelude.typ": bach-bwv1007-opening
#import "../examples/beethoven-ode-to-joy-alto-sax.typ": ode-to-joy-alto-sax
#import "../examples/beethoven-fur-elise.typ": fur-elise-opening

#set page(margin: 1.8cm)
#set text(font: "New Computer Modern", size: 11pt)

= typed-scores Visual Regression Suite

== `bar()` convenience wrapper

The public quick-bar helper uses the same layout and validation pipeline as a
one-bar single-staff score.

#bar("C4:q D4:q E4:q F4:q", clef: "treble", time: "4/4")

#v(1em)

#bar("(C4 E4 G4):h r:q Bb4:e. B4:s", clef: "treble", time: "4/4")

#v(1em)

#bar("Eb2:q F2:e G2:e r:q (Bb2 D3 F3):q", clef: "bass", key: "Eb")

== Single-staff score bars

The implicit single staff is expressed only with `notes:`.

#score(
  clef: "bass",
  key: "Eb",
  time: "4/4",
  bars: (
    (notes: "Eb4q F4q G4q Ab4q"),
    (notes: "_ Eb4:h _"),
  ),
)

#v(1em)

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  bars: (
    (notes: "G4:e A4:e B4:e C5:e D5:e E5:e F5:e G5:e"),
    (notes: "E4:e / F4:e G4:e A4:e B4:e C5:e D5:e E5:e"),
  ),
)

== Ties, slurs, and expressive marks

#score(
  clef: "treble",
  key: "Eb",
  time: "4/4",
  beams: true,
  bars: (
    (
      notes: "G4:e ~ G4:q. C5:q ~ C5:q",
    ),
    (
      notes: "C6:q[marcato stacc f=1 s1(] Bb5:q[accent tenuto f=2 h1<] A5:q[tenuto staccatissimo f=3 turn] G5:q[accent stacc f=4 s1) h1!]",
    ),
  ),
)

== Canonical multi-staff bars

Staves are declared once; every bar supplies each staff ID.

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  key: "C",
  time: "4/4",
  bars: (
    (
      upper: "C5:q[s2(] D5:q E5:q G5:q ~",
      lower: "C3:h G2:h",
    ),
    (
      upper: "G5:h F5:h[s2)]",
      lower: "C3:h G2:h",
    ),
  ),
)

#v(1em)

#score(
  staves: (
    violin: (clef: "treble"),
    viola: (clef: "alto"),
    cello: (clef: "bass"),
  ),
  key: "C",
  time: "4/4",
  tempo: "Moderato",
  beams: true,
  scale: 0.65,
  bars: (
    (
      violin: "E5:q F5:q G5:q A5:q",
      viola: "C4:q D4:q E4:q F4:q",
      cello: "C3:h G2:h",
    ),
    (
      key: "Cm",
      tempo: "Meno mosso",
      violin: "Eb5:q D5:q C5:q Bb4:q",
      viola: "C4:q Bb3:q G3:q Eb3:q",
      cello: "C3:h G2:h",
    ),
  ),
)

== Pickups and signature changes

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  key: "Eb",
  time: "12/8",
  beams: true,
  bars: (
    (
      partial: "1/8",
      upper: "Bb4:e",
      lower: "r:e",
    ),
    (
      upper: "Eb4:e F4:e G4:e Ab4:e Bb4:e C5:e D5:e Eb5:e F5:e G5:e Ab5:e Bb5:e",
      lower: "Eb2:q Bb2:q Eb3:q Bb2:q Eb3:q Bb2:q",
    ),
    (
      key: "Ab",
      time: "3/4",
      upper: "Ab4:q Bb4:q C5:q",
      lower: "Ab2:q Eb3:q Ab3:q",
    ),
  ),
)

== Repeat barlines and volta endings

Repeat signs live on the left or right edge of a bar. Labeled endings span
bars and pair their `start` and `stop` markers.

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  time: "2/4",
  bars: (
    (
      barline: (left: "repeat-start"),
      upper: "C5:q D5:q",
      lower: "C3:h",
    ),
    (
      ending: (label: "1.", start: true),
      upper: "E5:q F5:q",
      lower: "G2:h",
    ),
    (
      barline: (right: "repeat-end"),
      ending: (label: "1.", stop: true),
      upper: "B5:q B5:q",
      lower: "C3:h",
    ),
    (
      ending: (label: "Final", start: true),
      upper: "F6:q A5:q",
      lower: "G2:h",
    ),
    (
      ending: (label: "Final", stop: true),
      upper: "G5:h",
      lower: "C3:h",
    ),
  ),
  scale: 0.8,
  wrap: false,
)

#v(1em)

Combined end/start repeats keep their two heavy strokes distinct:

#score(
  clef: "treble",
  time: "2/4",
  scale: 0.7,
  wrap: false,
  bars: (
    (barline: (right: "repeat-end"), notes: "C5:q D5:q"),
    (barline: (left: "repeat-start"), notes: "E5:q F5:q"),
  ),
)

#v(1em)

#block(breakable: false)[
  The bracket clears extreme notes and remains proportionate at small scales:

  #score(
    clef: "treble",
    time: "2/4",
    tempo: "Allegro",
    scale: 0.65,
    wrap: false,
    bars: (
      (ending: (label: "1.", start: true), notes: "C7:q B6:q"),
      (ending: (label: "1.", stop: true), notes: "A6:h"),
    ),
  )
]

#v(1em)

Volta brackets continue when a system wraps:

#score(
  clef: "treble",
  time: "2/4",
  width: 10,
  bars: (
    (notes: "C5:q D5:q"),
    (ending: (label: "1.", start: true), notes: "E5:q F5:q"),
    (notes: "G5:q A5:q"),
    (ending: (label: "1.", stop: true), notes: "B5:q C6:q"),
  ),
)

== Extreme-register and spacing stress

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  time: "4/4",
  bars: (
    (
      upper: "C3:q E3:q G3:q C4:q",
      lower: "C5:q B4:q A4:q G4:q",
    ),
  ),
)

== Expected compile errors

These source snippets are intentionally not compiled:

```typ
#score(bars: ((notes: "C5:q"),), time: "4/4")
#score(
  staves: (upper: (clef: "treble")),
  bars: ((lower: "C5:w"),),
)
#score(
  staves: (upper: (clef: "treble"), lower: (clef: "bass")),
  bars: ((upper: "C5:w"),),
)
#score(
  clef: "treble",
  bars: ((notes: "C5:q[s1(] D5:q E5:q F5:q"),),
)
```

== Release gate: Chopin Op. 9 No. 2

#chopin-opening(scale: 0.48, note-spacing: 3.55, wrap: true)

== Tie semantics and wrapped continuations

The first tie crosses a system boundary. Tied continuation accidentals are not
repeated, while a later re-articulation shows the accidental normally.

#score(
  clef: "treble",
  key: "C",
  time: "2/4",
  width: 9.5,
  bars: (
    (notes: "C5:q F#5:q ~"),
    (notes: "F#5:q E5:q"),
    (notes: "F#5:q G5:q"),
    (notes: "(C#5 E5 G5):q ~ (C#5 E5 G5):q"),
  ),
)

== Key cancellation and double accidentals

Naturals cancel only the alterations removed from the old key, and appear
before the replacement signature.

#score(
  clef: "treble",
  key: "E",
  time: "4/4",
  wrap: false,
  scale: 0.72,
  bars: (
    (notes: "E5:w"),
    (key: "D", notes: "D5:w"),
    (key: "C", notes: "C5:w"),
    (key: "Eb", notes: "Eb5:w"),
  ),
)

#v(0.8em)

#score(
  clef: "treble",
  time: "4/4",
  wrap: false,
  scale: 0.72,
  bars: (
    (notes: "C##5:q Dbb5:q F##4:q Abb4:q"),
  ),
)

== Dynamics, holds, and pauses

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  wrap: false,
  scale: 0.72,
  note-spacing: 4.6,
  bars: (
    (notes: "C5:q[dyn=p] D5:q[dyn=pp] E5:q[dyn=mp] F5:q[dyn=mf]"),
    (notes: "G5:q[dyn=f] A5:q[dyn=ff] B5:q[dyn=sfz breath] C6:q[dyn=fp fermata]"),
  ),
)

#v(0.8em)

#score(
  clef: "treble",
  time: "4/4",
  wrap: false,
  scale: 0.58,
  bars: (
    (notes: "r:q[dyn=pp] C2:q[fermata] G4:q[breath] C3:q[dyn=ff]"),
  ),
)

== Staff grouping control

#let grouping-example(kind) = score(
  staves: (
    violin: (clef: "treble"),
    viola: (clef: "alto"),
    cello: (clef: "bass"),
  ),
  group: kind,
  time: "2/4",
  wrap: false,
  scale: 0.48,
  staff-gap: 6.5,
  bars: ((
    violin: "G5:h",
    viola: "C4:h",
    cello: "C3:h",
  ),),
)

#grid(
  columns: (1fr, 1fr),
  gutter: 0.7em,
  [*Brace* #grouping-example("brace")],
  [*Bracket* #grouping-example("bracket")],
  [*Line* #grouping-example("line")],
  [*None* #grouping-example("none")],
)

#pagebreak()

== Group symbol geometry stress

The Bravura terminals and proportional brace outlines remain complete across
a tall four-staff group. Every vertical stem starts at the lowest staff line
and stops at the highest staff line.

#let tall-grouping-example(kind, scale: 0.48) = score(
  staves: (
    first: (clef: "treble"),
    second: (clef: "treble"),
    third: (clef: "alto"),
    fourth: (clef: "bass"),
  ),
  group: kind,
  time: "2/4",
  wrap: false,
  scale: scale,
  staff-gap: 10,
  bars: ((
    first: "C6:h",
    second: "G4:h",
    third: "C4:h",
    fourth: "C2:h",
  ),),
)

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 0.6em,
  [*Tall brace* #tall-grouping-example("brace")],
  [*Bravura bracket* #tall-grouping-example("bracket")],
  [*Heavy line* #tall-grouping-example("line")],
)

#pagebreak()

== Famous-score fixture gallery

These public-domain excerpts exercise the release examples at the same time as
the notation engine: quartet grouping, dense solo beaming, a transposing
single staff, and a chromatic piano pickup.

=== Mozart · Eine kleine Nachtmusik, K. 525

#mozart-k525-opening(
  scale: 0.39,
  note-spacing: 2.85,
)

=== Bach · Cello Suite No. 1, BWV 1007

#bach-bwv1007-opening(
  scale: 0.58,
  note-spacing: 2.25,
  wrap: true,
)

=== Beethoven · Ode to Joy for E-flat alto saxophone

#ode-to-joy-alto-sax(
  scale: 0.68,
  note-spacing: 3.5,
)

=== Beethoven · Für Elise, WoO 59

#fur-elise-opening(
  scale: 0.58,
  note-spacing: 3.35,
  staff-gap: 10.2,
)

#pagebreak()

== Relative notes and chords, inherited durations, and local 3+3 beams

The key is intentionally omitted, so this uses the C-major default. Each staff
begins in its clef's default register, keeps its own pitch and duration anchors
across barlines, and groups six sixteenths as 3+3 with local `-` joins and `/`
breaks. The last bar resolves every chord from its first written pitch and then
resolves the remaining chord pitches in written order.

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  time: "3/8",
  beams: true,
  wrap: false,
  scale: 0.62,
  note-spacing: 3.5,
  staff-gap: 9.5,
  bars: (
    (
      upper: "G:s A - B / C - D E",
      lower: "C:s D - E / F - G A",
    ),
    (
      upper: "F#:e G A",
      lower: "B:e C D",
    ),
    (
      upper: "B C D",
      lower: "E F G",
    ),
    (
      upper: "(C5 E G) (D F A) (E G B)",
      lower: "(C3 E G) (D F A) (E G B)",
    ),
  ),
)

#pagebreak()

== Lowercase, uppercase, and compact duration equivalence

Pitch-letter capitalization has no musical meaning. The first two bars render
the same scale using compact `ce` and explicit `c:e`; mixed case is accepted.
The last bars exercise lowercase flats and double flats, a compact double
sharp, and mixed-case relative chord entry.

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  wrap: false,
  scale: 0.56,
  note-spacing: 3.1,
  bars: (
    (notes: "ce d E f g A b c"),
    (notes: "c4:e d e f g a b c"),
    (notes: "bb4:q bbb4:q f##5q g#5"),
    (notes: "(c5 e G):h (d F a):h"),
  ),
)

#pagebreak()

== Engraved tie and slur bows

Bows are cubic Beziers whose height saturates with the span and whose
shoulders flatten long arcs. Ties center inside a staff space and keep their
tips and apex clear of staff lines; ties on a line borrow the adjacent space.
Slurs pick endpoint raises by scored candidates: the high middle note lifts
both endpoints instead of ballooning the arch, the rising line tilts the bow
with the melody, and the long phrase goes flat on top with rounded shoulders.

#score(
  clef: "treble",
  time: "4/4",
  bars: (
    (notes: "B4:q ~ B4:q C5:q ~ C5:q"),
    (notes: "E4:h ~ E4:h"),
    (notes: "(C4 E4 G4):h ~ (C4 E4 G4):h"),
    (notes: "G4:w ~"),
    (notes: "G4:w"),
  ),
)

#v(1em)

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  bars: (
    (notes: "C5:q[s1(] D5:q[s1)] E5:q[s2(] G5:q[s2)]"),
    (notes: "C5:q[s3(] E5:q G5:q C6:q[s3)]"),
    (notes: "E4:q[s4(] C6:q E4:q[s4)] r:q"),
    (notes: "C4:e[s5(] D4:e E4:e F4:e G4:e A4:e B4:e C5:e[s5)]"),
  ),
)

#pagebreak()

== Slur, fingering, and dynamic interaction

Ties leaving dotted notes clear the augmentation dots, and ties outside the
staff float past the notehead instead of centering on it. Slur tips sit
above their own note's fingering, while fingerings and turn ornaments inside
a slur move above the finished bow. A slur with nothing inside it keeps its
tips at the notes no matter how steep the interval, and dynamics drop below
stems and beams that reach under the staff.

#score(
  clef: "treble",
  key: "Eb",
  time: "12/8",
  beams: true,
  scale: 0.8,
  bars: (
    (notes: "G5:q.[f=54] ~ G5:e F5:e[f=3 s1(] G5:e[f=4] F5:q.[f=3] Eb5:q[f=2 s1)] Bb4:e[f=1 s2(]"),
    (notes: "G5:q[f=5] C5:e[chromatic-turn turn-f=12121] C6:q[f=5] G5:e[f=2] Bb5:q.[f=4] Ab5:q[f=3] G5:e[f=2 s2)]"),
    (notes: "Bb4:e[f=1 dyn=f s3(] D6:e[f=5 s3)] C6:e[f=4 s4(] Bb5:s[f=3] Ab5:s G5:s Ab5:s[f=4 s4)] C5:s[s5(] D5:s Eb5:q.[s5)] r:q Bb4:e[f=1]"),
  ),
)

#pagebreak()

== Classical spacing, beam quanting, and aligned dynamics

Horizontal space grows with the logarithm of duration from the bar's
shortest note, so mixed rhythms breathe like engraved plate work. Beam
slants follow the outer interval in quarter-space steps, go flat over
concave contours, and their ends sit on, straddle, or hang from staff
lines. Dynamics in one voice share a system baseline, staccato dots keep to
staff spaces, ledger-line stems reach the middle line, and chord ties curve
outward from the chord.

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  bars: (
    (notes: "C5:q D5:e E5:e F5:s G5:s A5:s B5:s C6:q"),
    (notes: "C5:e[dyn=f] D5:e E5:q[dyn=p] F5:q G5:q[stacc]"),
    (notes: "G4:e A4:e B4:e G4:e E5:e C5:e E4:e C4:e"),
    (notes: "(C4 E4 G4):h ~ (C4 E4 G4):h"),
    (notes: "C6:q E6:q A3:q C4:q"),
  ),
)

#pagebreak()

== Accidental columns, systemic barline, and computed staff gaps

Chord accidentals pack into columns from the outside in - topmost nearest
the heads, then bottommost - so clusters never overlap. Multi-staff systems
open with a barline joining the staves, and with no explicit
#raw("staff-gap") the gaps come from the actual ink: the forte under the
upper staff's down-stem chord pushes the staves apart just enough.

#score(
  clef: "treble",
  time: "4/4",
  scale: 0.8,
  bars: (
    (notes: "(C#4 D#4 F#4 A#4):q (Db4 Eb4 Gb4):q (C#5 E5 G#5 B5):q (F#4 G#4 A#4 B#4):q"),
  ),
)

#v(1em)

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  time: "4/4",
  scale: 0.8,
  bars: (
    (
      upper: "(B4 D5 G5):q[dyn=f] r:q D5:e E5:e F#5:e G5:e",
      lower: "G2:h[dyn=f] D3:h",
    ),
  ),
)

#pagebreak()

== Seconds in chords

Notes a second (or unison) apart cannot share a notehead column: the upper
note of each clashing pair moves to the right of the stem while the lower
keeps the left, alternating through longer clusters, with accidentals,
dots, ties, and spacing making room for the widened chord.

#score(
  clef: "treble",
  time: "4/4",
  scale: 0.85,
  bars: (
    (notes: "(C4 D4):q (F5 G5):q (C4 D4 E4):q (Cb4 D4 Ab4):q"),
    (notes: "(C5 D5 E5 F5):h ~ (C5 D5 E5 F5):h"),
    (notes: "(B4 C5):h (D4 E4 F4):h"),
  ),
)

#pagebreak()

== Harmony symbols

Harmony is an independent duration-bearing timeline in each bar. Symbols remain
above the top staff and are centered on the onset where they become active.

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

#pagebreak()

== Justified systems and indentation

Multi-system scores use LilyPond-style justification by default: timed gaps
stretch to the requested width, while the opening system may reserve a distinct
indent. The final example opts into a natural-width final system.

#score(
  time: "4/4",
  width: 36,
  indent: 3.5,
  bars: (
    (notes: "c5:q d e f"),
    (notes: "g5:q a b c6"),
    (notes: "c6:h g5:h"),
    (notes: "f5:q e d c"),
    (notes: "b4:h c5:h"),
    (notes: "d5:q e f g"),
  ),
)

#v(1em)

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

#pagebreak()

== Staff labels

Full staff labels occupy the opening-system name column; optional short labels
occupy the corresponding column in later systems. Each label centers on its own
staff, independently of the group symbol.

#score(
  staves: (
    violin-one: (clef: "treble", label: "Violin I", short-label: "Vln. I"),
    violin-two: (clef: "treble", label: "Violin II", short-label: "Vln. II"),
    viola: (clef: "alto", label: "Viola", short-label: "Vla."),
    cello: (clef: "bass", label: "Violoncello", short-label: "Vc."),
  ),
  group: "bracket",
  time: "2/4",
  width: 24,
  bars: (
    (violin-one: "g5:q a5:q", violin-two: "d5:q e5:q", viola: "a3:q b3:q", cello: "g2:q a2:q"),
    (violin-one: "b5:q c6:q", violin-two: "f5:q g5:q", viola: "c4:q d4:q", cello: "b2:q c3:q"),
    (violin-one: "d6:q c6:q", violin-two: "a5:q g5:q", viola: "e4:q d4:q", cello: "d3:q g2:q"),
    (violin-one: "b5:h", violin-two: "f5:h", viola: "d4:h", cello: "g2:h"),
  ),
)

#pagebreak()

== Global system balancing

Line breaking evaluates all complete-bar partitions instead of taking the first
overflow. This evenly spaced passage chooses two similarly dense systems rather
than a compressed three-bar system followed by one over-stretched bar.

#score(
  clef: "treble",
  time: "4/4",
  width: 36,
  bars: (
    (notes: "c5:q d e f"),
    (notes: "g5:q a b c6"),
    (notes: "c6:q b5 a g"),
    (notes: "f5:q e d c"),
  ),
)

#pagebreak()

== Harmony onset anchoring

Chord-symbol duration determines the next change, not the horizontal anchor.
Every symbol centers on its own onset: a half-note Cmaj7 centers on the first
quarter-note onset, while the following A7 centers on the third.

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

#pagebreak()

== Dynamics and hairpin clearance

Dynamics and hairpins share one baseline. A wedge shortens before its ending
dynamic and begins after a dynamic at its opening event, keeping both labels
clear at normal and compact spacing.

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  bars: (
    (notes: "c5:q[dyn=p h1<] d5:q e5:q f5:q[h1! dyn=ff]"),
    (notes: "g5:q[dyn=ff h2>] f5:q e5:q d5:q[h2! dyn=p]"),
  ),
)

#pagebreak()

== Semantic metronome marks

Tempo dictionaries use named beat values, so users never need to paste an
eighth-note character. The renderer supplies the appropriate Bravura glyph.

#score(
  time: "4/4",
  tempo: (text: [Andante], beat: "eighth", bpm: 132),
  bars: (
    (notes: "c5:q d e f"),
    (tempo: (beat: "quarter", bpm: 96), notes: "g5:q a b c6"),
  ),
)

#pagebreak()

== Metronome glyph set

Every supported tempo beat is tested beside tempo text and parentheses. Each
Bravura glyph maintains clearance from the surrounding mark.

#let tempo-glyph-test(label, beat, bpm) = score(
  time: "2/4",
  width: 16,
  scale: 0.62,
  tempo: (text: label, beat: beat, bpm: bpm),
  bars: ((notes: "c5:q d"),),
)

#grid(
  columns: (1fr, 1fr, 1fr),
  column-gutter: 1.2em,
  row-gutter: 1.2em,
  tempo-glyph-test([Whole], "whole", 60),
  tempo-glyph-test([Half], "half", 72),
  tempo-glyph-test([Quarter], "quarter", 96),
  tempo-glyph-test([Eighth], "eighth", 132),
  tempo-glyph-test([Sixteenth], "sixteenth", 168),
  tempo-glyph-test([Thirty-second], "thirty-second", 216),
)

#pagebreak()

== Tuplets

Tuplets remain inline in the note string. A complete beam suppresses the
bracket, while rests or an explicit option retain it.

#score(
  time: "4/4",
  beams: true,
  bars: (
    (
      notes: "tuplet 3:2 { c5:e d e } tuplet 3:2 { f5:e r:e g5:e } tuplet 3:2[bracket=always side=above] { a5:e b c6 } c6:q",
    ),
  ),
)

#v(1em)

#score(
  time: "2/4",
  bars: (
    (
      notes: "tuplet 5:4[bracket=always] { c5:e d tuplet 3:2[bracket=always] { e5:s f g } a5:e b }",
    ),
  ),
)

#pagebreak()

= Clef changes, barlines, and navigation

#score(
  time: "4/4",
  width: 42,
  ragged-right: false,
  bar-numbers: "all",
  bars: (
    (rehearsal: "A", notes: "c5:q d e f"),
    (clef: "bass", navigation: "segno", barline: (right: "double"), notes: "c3:q d e f"),
    (barline: (right: "dashed"), navigation: "D.S. al Coda", notes: "g2:h c3:h"),
    (clef: "treble", navigation: "coda", barline: (right: "final"), notes: "g4:q a b c5"),
  ),
)

#v(1em)

= Polyphonic voices

#score(
  time: "4/4",
  beams: true,
  width: 42,
  ragged-right: false,
  bars: (
    (
      notes: (
        "e5:h f5:h",
        "c5:q d5:q e5:q f5:q",
      ),
    ),
    (
      notes: (
        "(e5 g5):q (f5 a5):q g5:h",
        "e5:q f5:q (d5 f5):h",
      ),
      barline: (right: "final"),
    ),
  ),
)

#pagebreak()

= Grace notes, tremolos, and arpeggios

#score(
  time: "4/4",
  beams: true,
  width: 52,
  ragged-right: false,
  bars: (
    (notes: "c5:q acciaccatura { d5:s e } f5:q appoggiatura { g5:e } a5:q grace { b5:s c6 } d6:q"),
    (notes: "c5:h[tremolo=16] (e5 g5 c6):h[arpeggio]"),
    (notes: "tremolo 16 { c5:h g5:h }"),
    (notes: "(c5 e5 g5 c6):h[arpeggio=up] (d5 f5 a5 d6):h[arpeggio=down]", barline: (right: "final")),
  ),
)

#v(1em)

Single-event tremolo subdivisions 8 through 64 select one through four strokes
independently of the written note value. Quarter notes below exercise both stem
directions and verify that every cluster remains clear of its notehead.

#score(
  time: "4/4",
  width: 52,
  ragged-right: false,
  bars: (
    (notes: "c5:q[tremolo=8] d5:q[tremolo=16] e5:q[tremolo=32] f5:q[tremolo=64]", barline: (right: "final")),
  ),
)

#pagebreak()

= LilyPond A/B: grace notation

Each committed LilyPond 2.26.0 reference is followed by live `typed-scores`
output using only the public API. The reference SVG is sized so one LilyPond
staff space equals the renderer's 8pt staff space. This compares geometry at a
common engraving scale even though LilyPond uses Emmentaler and `typed-scores`
uses Bravura.

#let lilypond-ab(reference, reference-spaces, rendered) = [
  *LilyPond 2.26.0*
  #v(0.3em)
  #align(center, image(reference, width: reference-spaces * 8pt))
  #v(0.7em)
  *typed-scores*
  #v(0.3em)
  #align(center, rendered)
]

== Documented grace forms

The tapered grace-slur tips retain LilyPond's visible clearance from both the
small grace head and the full-size principal head. The final ordinary slur
confirms that grace slurs retain the normal, unscaled bow weight.

#lilypond-ab(
  "lilypond-ab/reference/grace-notes.svg",
  35.7652,
  score(
    time: "4/4",
    beams: true,
    scale: 1.0,
    width: 40,
    ragged-right: false,
    bars: (
      (notes: "acciaccatura { d5:e } c5:q appoggiatura { e5:e } d5:q acciaccatura { g5:s f5:s } e5:h"),
      (notes: "c5:q[s1(] d5:q[s1)] r:h", barline: (right: "final")),
    ),
  ),
)

== Grace-note ledger clearance

#lilypond-ab(
  "lilypond-ab/reference/grace-ledgers.svg",
  27.2719,
  score(
    time: "4/4",
    beams: true,
    scale: 1.0,
    width: 32,
    ragged-right: false,
    bars: (
      (notes: "c6:q acciaccatura { d6:s e } f6:q appoggiatura { g6:e } a6:q grace { b6:s c7 } d7:q", barline: (right: "final")),
    ),
  ),
)

#pagebreak()

= LilyPond A/B: nested tuplets

This is LilyPond's nested-tuplet example: an unbeamed outer 5:4 group with an
explicitly beamed inner 3:2 group. It compares numeral size and optical
centering on the note-column origins as well as bracket thickness, hook height,
and beam proximity.

#lilypond-ab(
  "lilypond-ab/reference/nested-tuplets.svg",
  36.3090,
  score(
    time: "4/4",
    beams: false,
    scale: 1.0,
    width: 32,
    ragged-right: false,
    bars: (
      (notes: "c5:q tuplet 5:4 { f5:e e f tuplet 3:2 { e:e - f - g } } f5:q", barline: (right: "final")),
    ),
  ),
)

#pagebreak()

= LilyPond A/B: arpeggios and tremolos

== Directional arpeggios

#lilypond-ab(
  "lilypond-ab/reference/arpeggio-directions.svg",
  29.6715,
  score(
    time: "4/4",
    scale: 1.0,
    width: 32,
    ragged-right: false,
    bars: (
      (notes: "(c4 e4 g4 c5):h[arpeggio] (c4 e4 g4 c5):h[arpeggio=up]"),
      (notes: "(c4 e4 g4 c5):h[arpeggio=down] (c4 e4 g4 c5):h[arpeggio]", barline: (right: "final")),
    ),
  ),
)

== Single-note tremolo values

The comparison includes the vertical end edges and sloped faces of each
StemTremolo strip, not only its overall thickness.

#lilypond-ab(
  "lilypond-ab/reference/single-tremolos.svg",
  27.7364,
  score(
    time: "4/4",
    scale: 1.0,
    width: 32,
    ragged-right: false,
    bars: (
      (notes: "c5:h[tremolo=8] c5:h[tremolo=32]"),
      (notes: "c5:h[tremolo=32] c5:h[tremolo=32]", barline: (right: "final")),
    ),
  ),
)

#pagebreak()

= Grace slur weight

Grace-note glyphs shrink, but the independent resolving Slur grob retains the
same midpoint and tapered-tip weights as an ordinary slur. Both bows below use
the renderer's normal 0.22-space midpoint and 0.10-space endpoint thicknesses.

#score(
  time: "4/4",
  width: 24,
  bars: (
    (notes: "acciaccatura { d5:e } c5:q e5:q[s1(] f5:q[s1)] r:q", barline: (right: "final")),
  ),
)

#pagebreak()

= Beam-aware slur endpoints

Slur anchors and collision obstacles use the beam group's rendered stem
direction. The bass phrase begins beside its low first head instead of an
imaginary upward stem tip; the first treble phrase descends to its final
sixteenth without lifting the right endpoint away from the note. The final
treble group has upward stems throughout, so LilyPond's neutral-direction rule
puts its slur below the notes.

#score(
  clef: "bass",
  time: "4/4",
  beams: true,
  width: 24,
  ragged-right: false,
  bars: (
    (notes: "g2:s[s1(] d3:s b3:s a3:s[s1)] r:h r:q", barline: (right: "final")),
  ),
)

#v(1em)

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  width: 24,
  ragged-right: false,
  bars: (
    (notes: "c5:e[s1(] b4:s a4:s[s1)] r:q r:h", barline: (right: "final")),
  ),
)

#v(1em)

#score(
  clef: "treble",
  time: "4/4",
  beams: true,
  width: 24,
  ragged-right: false,
  bars: (
    (notes: "c4:s[s1(] e4:s d#4:s c4:s[s1)] r:h r:q", barline: (right: "final")),
  ),
)

#pagebreak()

= Slur clearance over beamed sixteenth runs

A slur must keep free air over every notehead it spans, measured across the
head's full width rather than only above its center, and beamed boundary
notes let the bow deviate an extra space from the melodic interval. In the
Bach figure the second and fourth phrases arch over the returning high note
and float their right tip above the final low sixteenth instead of diving
onto it.

#score(
  clef: "bass",
  key: "G",
  time: "4/4",
  beams: true,
  note-spacing: 2.35,
  bars: (
    (
      notes: "G2:s[s1(] D3:s B3:s A3:s[s1)] B3:s[s2(] D3:s B3:s D3:s[s2)] G2:s[s3(] D3:s B3:s A3:s[s3)] B3:s[s4(] D3:s B3:s D3:s[s4)]",
      barline: (right: "final"),
    ),
  ),
)

#pagebreak()

= Staccato and script quantization against staff lines

Following LilyPond's script placement, a staccato dot or tenuto bar leaves
its notehead and then quantizes to the half-space grid in its direction,
never resting on a staff line: the dot under the G on the second line hops
below the staff, the A dot centers in the first space, and dots above the
middle-line B and the C skip the fourth line into the space above. Ledger
notes take their dot a whole space from the pitch with no extra shift. The
taller accent family always clears the staff outline by a quarter space.

#score(
  time: "4/4",
  wrap: false,
  bars: (
    (notes: "C4:q[stacc] E4:q[stacc] G4:q[stacc] A4:q[stacc]"),
    (notes: "B4:q[stacc] C5:q[stacc] D5:q[stacc] F5:q[stacc]"),
    (notes: "A5:q[stacc] C6:q[stacc] G4:q[tenuto] B4:q[tenuto]"),
    (notes: "E4:q[accent] G4:q[accent] B4:q[accent] G5:q[accent]", barline: (right: "final")),
  ),
)

#pagebreak()

= Fingering band and script family placement

Fingerings follow LilyPond's defaults: digits sit in a common band just above
the staff — half a space of staff padding — and rise only when the notehead
column or an articulation stacked above reaches higher; stems never push a
digit up. The staccato dot below the C shares its event with a digit above.

#score(
  time: "4/4",
  wrap: false,
  bars: (
    (notes: "C4:q[f=1] E4:q[f=2] G4:q[f=3] A4:q[f=4]"),
    (notes: "B4:q[f=1] D5:q[f=2] F5:q[f=3] A5:q[f=4]"),
    (notes: "C4:q[stacc f=1] G4:q[stacc f=2] B4:q[stacc f=3] D5:q[stacc f=4]", barline: (right: "final")),
  ),
)

The wedge and marcato quantize their near edge to the half-space grid like
the dot quantizes its center, and the accent — LilyPond's one unquantized
script — clears the staff outline by a quarter space:

#score(
  time: "4/4",
  wrap: false,
  bars: (
    (notes: "E4:q[staccatissimo] G4:q[staccatissimo] A4:q[staccatissimo] B4:q[staccatissimo]"),
    (notes: "E4:q[marcato] G4:q[marcato] A4:q[marcato] B4:q[marcato]"),
    (notes: "E4:q[accent] G4:q[accent] A4:q[accent] B4:q[accent]", barline: (right: "final")),
  ),
)

#pagebreak()

= Cross-staff clearance of hairpins over slurred chords

Computed staff gaps reserve room for slur arches, so a treble-voice hairpin
riding the dynamics baseline keeps free air above the bass staff's slurred
chords instead of cutting through their bows.

#score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  time: "6/8",
  beams: true,
  wrap: false,
  bars: (
    (
      upper: "C5:e[dyn=f h1<] D5:e E5:e F5:e[h1!] G5:q",
      lower: "C3:e (G3 E4):e[s1(] (B3 E4 G4):e[s1)] C3:e (A3 F4):e[s2(] (C4 F4 A4):e[s2)]",
    ),
    (
      upper: "G5:q.[h2>] E5:q.[h2!]",
      lower: "C3:e (G3 E4):e[s3(] (B3 E4 G4):e[s3)] C3:e (G3 E4):e[s4(] (B3 E4 G4):e[s4)]",
      barline: (right: "final"),
    ),
  ),
)
