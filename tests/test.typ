#import "../src/lib.typ": *
#import "../examples/chopin-opening.typ": chopin-opening

#set page(margin: 1.8cm)
#set text(font: "New Computer Modern", size: 11pt)

= typed-scores Visual Regression Suite

== WASM Parsed Notes

#grid(
  columns: (1fr, 1fr),
  gutter: 1.5em,
  row-gutter: 1.4em,

  [*Treble C4 quarter* \ #note("C4:q", clef: "treble")],
  [*Treble Bb4 dotted eighth* \ #note("Bb4:e.", clef: "treble")],
  [*Treble C major half chord* \ #note("(C4 E4 G4):h", clef: "treble")],
  [*Bass Eb2 quarter* \ #note("Eb2:q", clef: "bass")],
  [*Quarter rest* \ #note("r:q", clef: "treble")],
  [*Double-dotted open note* \ #note("A5:h..", clef: "treble")],
)

#pagebreak()

== Accent and Articulation Matrix

Canonical marks, stem down (notehead side above):

#bar("C6:q[stacc] C6:q[staccatissimo] C6:q[tenuto] C6:q[accent]", clef: "treble", time: "4/4")
#bar("C6:q[marcato] r:h.", clef: "treble", time: "4/4")

All eight requested combinations, in reading order from the reference image:

#bar("C6:q[marcato stacc] C6:q[marcato tenuto] C6:q[marcato staccatissimo] C6:q[tenuto stacc]", clef: "treble", time: "4/4")
#bar("C6:q[tenuto staccatissimo] C6:q[accent stacc] C6:q[accent tenuto] C6:q[accent staccatissimo]", clef: "treble", time: "4/4")

Stem up (the notehead is below the stem): marks and fingering move below.

#bar("C4:q[marcato stacc f=1] C4:q[tenuto stacc f=2] C4:q[accent staccatissimo f=3] C4:q[accent tenuto f=4]", clef: "treble", time: "4/4")

#pagebreak()

== String Bars

#bar("r:w", clef: "treble")

#v(1em)

#bar("C4:q D4:q E4:q F4:q", clef: "treble")

#v(1em)

#bar("G4:e A4:e B4:e C5:e D5:e E5:e F5:e G5:e", clef: "treble")

#v(1em)

#bar("(C4 E4 G4):h r:q Bb4:e. B4:s", clef: "treble")

#v(1em)

#bar("(C4 E4 G4):h G4:h", clef: "treble")

#v(1em)

#bar("Eb2:q F2:e G2:e r:q (Bb2 D3 F3):q", clef: "bass")

== Compact Notes And Auto Rests

Compact note duration syntax:

#bar("Eb4q F4q G4q Ab4q", clef: "treble", key: "Eb", time: "4/4")

#v(1em)

Auto rests fill the remaining bar duration:

#bar("_ Eb4:h _", clef: "treble", key: "Eb", time: "4/4")

#v(1em)

Ambiguous auto rests use representable equal durations when possible:

#bar("_ E4:q _", clef: "treble", time: "4/4")

== Beaming Syntax

Individual flagged eighths:

#bar("G4:e A4:e B4:e C5:e D5:e E5:e F5:e G5:e", clef: "treble", beams: false)

#v(1em)

Connected eighths:

#bar("G4:e A4:e B4:e C5:e D5:e E5:e F5:e G5:e", clef: "treble", beams: true)

#v(1em)

Mixed durations keep the beam group local:

#bar("C4:q D4:e E4:e F4:q G4:e A4:e", clef: "treble", beams: true)

#v(1em)

Mixed stem directions still beam together:

#bar("B4:e A4:e G4:q F4:e E4:e D4:q", clef: "treble", beams: true)

#v(1em)

Slash breaks a beam group:

#bar("E4:e / F4:e G4:e A4:e", clef: "treble", beams: true)

#v(1em)

Connected sixteenths:

#bar("C5:s D5:s E5:s F5:s G5:s A5:s B5:s C6:s", clef: "treble", beams: true)

== Ties

A tie within a bar (drawn opposite the stem side):

#bar("G4:e ~ G4:q. C5:q ~ C5:q", clef: "treble", time: "4/4", beams: true)

#v(1em)

Tied chords share the curve per pitch:

#bar("(C4 E4 G4):h ~ (C4 E4 G4):h", clef: "treble", time: "4/4")

#v(1em)

A tie crossing a barline in a score:

#score(
  treble: (
    "C5:q D5:q E5:q G5:q ~",
    "G5:h F5:h",
  ),
  bass: (
    "C3:h G2:h",
    "C3:h G2:h",
  ),
  time: "4/4",
)

== Thirty-Second Notes And Rests

#bar("C5:t D5:t E5:t F5:t G5:t A5:t B5:t C6:t C5:s D5:s E5:s F5:s r:e. r:s r:q", clef: "treble", time: "4/4", beams: true)

== Key And Time Signatures

E-flat major, 12/8:

#bar("Eb4:e F4:e G4:e Ab4:e Bb4:e C5:e D5:e Eb5:e F5:e G5:e Ab5:e Bb5:e", clef: "treble", key: "Eb", time: "12/8", beams: true)

#v(1em)

G major, 3/4:

#bar("G4:q A4:q B4:q", clef: "treble", key: "G", time: "3/4")

#v(1em)

B-flat major in bass, 4/4:

#bar("Bb2:q C3:q D3:q Eb3:q", clef: "bass", key: "Bb", time: "4/4")

== Multiple Bars

#bar("C4:q E4:q G4:q C5:q", clef: "treble")

#v(0.7em)

#bar("B4:e A4:e G4:q F4:e E4:e D4:q", clef: "treble", beams: true)

#v(0.7em)

#bar("C4:h (E4 G4 C5):h", clef: "treble")

== Multi-Bar Score

Parallel treble/bass arrays with a slur crossing the barline:

#score(
  treble: (
    "C5:q[s1(] D5:q E5:q F5:q",
    "G5:q A5:q B5:q C6:q[s1)]",
  ),
  bass: (
    "C3:h G2:h",
    "C3:h G2:h",
  ),
  time: "4/4",
)

#v(1em)

Parallel arrays can use dictionaries for key/time changes:

#score(
  treble: (
    "C5:q D5:q E5:q F5:q",
    (key: "Eb", time: "12/8", notes: "Eb4:e F4:e G4:e Ab4:e Bb4:e C5:e D5:e Eb5:e F5:e G5:e Ab5:e Bb5:e"),
    (key: "Ab", time: "3/4", notes: "Ab4:q Bb4:q C5:q"),
  ),
  bass: (
    "C3:h G2:h",
    (notes: "Eb2:q Bb2:q Eb3:q Bb2:q Eb3:q Bb2:q"),
    (notes: "Ab2:q Eb3:q Ab3:q"),
  ),
  key: "C",
  time: "4/4",
  scale: 0.7,
  beams: true,
)

#v(1em)

Measure dictionaries with key and time changes:

#score(
  bars: (
    (
      key: "C",
      time: "4/4",
      treble: "C5:q D5:q E5:q F5:q",
      bass: "C3:h G2:h",
    ),
    (
      key: "Eb",
      time: "12/8",
      treble: "Eb4:e F4:e G4:e Ab4:e Bb4:e C5:e D5:e Eb5:e F5:e G5:e Ab5:e Bb5:e",
      bass: "Eb2:q Bb2:q Eb3:q Bb2:q Eb3:q Bb2:q",
    ),
    (
      key: "Ab",
      time: "3/4",
      treble: "Ab4:q Bb4:q C5:q",
      bass: "Ab2:q Eb3:q Ab3:q",
    ),
  ),
  beams: true,
)

== Sections And Voices

Sections let long regions carry their own key/time defaults:

#score(
  sections: (
    (
      key: "C",
      time: "4/4",
      tempo: "Moderato",
      voices: (
        (name: "Violin I", clef: "treble", notes: ("E5:q F5:q G5:q A5:q", "G5:h E5:h")),
        (name: "Violin II", clef: "treble", notes: ("C5:q D5:q E5:q F5:q", "E5:h C5:h")),
        (name: "Viola", clef: "alto", notes: ("G3:q A3:q B3:q C4:q", "C4:h G3:h")),
        (name: "Cello", clef: "bass", notes: ("C3:h G2:h", "C3:w")),
      ),
    ),
    (
      key: "Cm",
      tempo: "Meno mosso",
      voices: (
        (name: "Violin I", clef: "treble", notes: ("Eb5:q D5:q C5:q Bb4:q", "C5:w")),
        (name: "Violin II", clef: "treble", notes: ("G4:q F4:q Eb4:q D4:q", "Eb4:w")),
        (name: "Viola", clef: "alto", notes: ("C4:q Bb3:q G3:q Eb3:q", "G3:w")),
        (name: "Cello", clef: "bass", notes: ("C3:h G2:h", "C3:w")),
      ),
    ),
  ),
  scale: 0.55,
  beams: true,
)

== Piano Sketch

#score(
  treble: "C5:e D5:e E5:e G5:e E5:e D5:e C5:e D5:e E5:e G5:e E5:e D5:e",
  bass: "C3:q G2:q C3:q G2:q C3:q G2:q",
  beams: true,
  key: "Eb",
  time: "12/8",
)

#v(1em)

Small scale stress: time signatures and stems should remain visible.

#score(
  bars: (
    (
      key: "C",
      time: "4/4",
      treble: "C5:q D5:q E5:q F5:q",
      bass: "C3:h G2:h",
    ),
    (
      key: "Eb",
      time: "12/8",
      treble: "Eb4:e F4:e G4:e Ab4:e Bb4:e C5:e D5:e Eb5:e F5:e G5:e Ab5:e Bb5:e",
      bass: "Eb2:q Bb2:q Eb3:q Bb2:q Eb3:q Bb2:q",
    ),
    (
      key: "Ab",
      time: "3/4",
      treble: "Ab4:q Bb4:q C5:q",
      bass: "Ab2:q Eb3:q Ab3:q",
    ),
  ),
  beams: true,
  scale: 0.5,
)

#v(1em)

#score(
  treble: "(C4 E4 G4):h G4:h",
  bass: "C2:h G2:h",
)

#v(1em)

Wide piano gap should keep the brace intact:

#score(
  treble: "C6:w",
  bass: "C2:w",
  staff-gap: 10,
)

== Nocturne Sketch

Chopin Op. 9 No. 2 opening texture target:

#score(
  bars: (
    (
      key: "Eb",
      time: "12/8",
      treble: "Bb4:e[s1(] G5:e F5:e Eb5:e D5:e C5:e Bb4:e G4:e F4:e G4:e Bb4:e C5:e",
      bass: "Eb2:e Bb2:e G3:e Eb3:e Bb2:e G3:e Eb3:e Bb2:e G3:e Eb3:e Bb2:e G3:e",
    ),
    (
      treble: "D5:e Eb5:e F5:e G5:e Ab5:e G5:e F5:e Eb5:e D5:e C5:e Bb4:e[s1)] G4:e",
      bass: "Eb2:e Bb2:e G3:e Eb3:e Bb2:e G3:e Eb3:e Bb2:e G3:e Eb3:e Bb2:e G3:e",
    ),
  ),
  key: "Eb",
  time: "12/8",
  beams: true,
)

== Expected Compile Errors

These are intentionally shown as source examples rather than compiled calls:

```typ
#score(treble: ("C5:w", "D5:w"), bass: ("C3:w",))
#score(treble: "C5:q", bass: "C3:w", time: "4/4")
#score(treble: "C5:q[s1(] D5:q E5:q F5:q", bass: "C3:w", time: "4/4")
#score(treble: "C5:q[s1)] D5:q E5:q F5:q", bass: "C3:w", time: "4/4")
```

== Extreme Register Stress

Low treble and high bass should not collide:

#score(
  treble: "C3:q E3:q G3:q C4:q",
  bass: "C5:q B4:q A4:q G4:q",
)

#v(1em)

Ledger-heavy single staves:

#bar("C3:q D3:q E3:q F3:q", clef: "treble")

#v(0.7em)

#bar("C5:q B4:q A4:q G4:q", clef: "bass")

#pagebreak()

== Release Gate: Chopin Op. 9 No. 2

Opening pickup and measures 1–2, transcribed note-for-note from the Mutopia
edition based on the 1881 G. Schirmer score:

#chopin-opening()
