// Prelude in C from Preludes in Progressive Keys (Carcassi, Methode complete pour
// Guitare, Op. 59, 1836), No. 1, bars 1-4: the upper voice of the opening arpeggio,
// in C major, A minor, D minor, and G7.
//
// Translated from the Mutopia Project LilyPond source
// (https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=2209), edition of the
// 1906 Oliver Ditson printing edited by G. C. Santisteban, typeset by Jeffrey
// Olson. Licence: Public Domain, as stated by Mutopia. The source applies
// \transpose c c' to its guitar voices, so the printed pitches sit one octave
// above sounding pitch; the notes here are sounding pitches for the treble-8
// clef. Each note has an explicit string. The bass voice of the source doubles
// the arpeggio's lowest notes, so only the upper voice is set. Chord symbols
// follow the bass changes.
//
// typst compile --root . --ppi 300 assets/readme/examples/carcassi-prelude-c.typ assets/readme/examples/carcassi-prelude-c.png

#import "../../../src/lib.typ": *

#set page(width: 17cm, height: auto, margin: 12pt)
#set text(font: "New Computer Modern", size: 11pt)

#block(width: 100%, fill: white, stroke: 0.6pt + luma(215), radius: 8pt, inset: 18pt)[
  #text(size: 9pt, style: "italic")[Prelude in C, Op. 59 No. 1 --- M. Carcassi]
  #v(0.4em)
  #score(
    staves: (
      guitar: (clef: "treble-8"),
      tab: (clef: "tab", source: "guitar"),
    ),
    key: "C",
    time: "4/4",
    theme: "light",
    beams: true,
    chord-diagrams: guitar-chords,
    scale: 0.6,
    width: 85,
    system-gap: 0.5em,
    ragged-right: false,
    bars: (
    (
      guitar: "c3:e[string=5] g3:e[string=3] c4:e[string=2] e4:e[string=1] e3:e[string=4] g3:e[string=3] c4:e[string=2] e4:e[string=1]",
      harmony: "C:w",
    ),
    (
      guitar: "a2:e[string=5] a3:e[string=3] c4:e[string=2] e4:e[string=1] c3:e[string=5] a3:e[string=3] c4:e[string=2] e4:e[string=1]",
      harmony: "Am:w",
    ),
    (
      guitar: "d3:e[string=4] a3:e[string=3] d4:e[string=2] f4:e[string=1] f3:e[string=4] a3:e[string=3] d4:e[string=2] f4:e[string=1]",
      harmony: "Dm:w",
    ),
    (
      guitar: "g2:e[string=6] g3:e[string=3] b3:e[string=2] f4:e[string=1] b2:e[string=5] g3:e[string=3] d4:e[string=2] f4:e[string=1]",
      harmony: "G7:w",
    ),
    ),
  )
]
