// Ein' feste Burg (hymntune), by Martin Luther (1529): the first four lines of
// the stanza, with the German text under the soprano.
//
// Translated from the Mutopia Project LilyPond source
// (https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1237), typeset by
// Steve Dunlop. Licence: Public Domain, as stated by Mutopia. The soprano, alto,
// tenor, and bass are the four voices of that source, transcribed as written and
// checked against LilyPond's absolute pitch output. Mutopia's source is in C major.
// The text is Martin Luther's 1529 stanza (public domain). The melisma marks (__)
// extend syllables over the tune's extra notes. Fermatas mark the line ends.
//
// typst compile --root . --ppi 300 assets/readme/examples/ein-feste-burg.typ assets/readme/examples/ein-feste-burg.png

#import "../../../src/lib.typ": *

#set page(width: 17cm, height: auto, margin: 12pt)
#set text(font: "New Computer Modern", size: 11pt)

#block(width: 100%, fill: white, stroke: 0.6pt + luma(215), radius: 8pt, inset: 18pt)[
  #text(size: 9pt, style: "italic")[Ein' feste Burg --- M. Luther, 1529]
  #v(0.4em)
  #score(
    staves: (
      upper: (clef: "treble"),
      lower: (clef: "bass"),
    ),
    key: "C",
    time: "4/4",
    theme: "light",
    scale: 0.5,
    width: 100,
    staff-gap: none,
    system-gap: 0.6em,
    lyric-size: 1.3,
    verse-gap: 1.5,
    bars: (
    (
      partial: "1/4",
      upper: ("c5:q", "e4:q"),
      lower: ("g3:q", "c3:q"),
      lyrics: (upper: "Ein"),
    ),
    (
      upper: ("c5:q c5:q g4:e a4:e b4:q", "c4:q e4:q d4:q g4:q"),
      lower: ("g3:q g3:q b3:q b3:q", "e3:q c3:q g3:q d3:q"),
      lyrics: (upper: "fes -- te Burg __ ist"),
    ),
    (
      upper: ("c5:e b4:e a4:q g4:q c5:q[fermata]", "e4:q f#4:q g4:q e4:q"),
      lower: ("a3:q d4:e c4:e b3:q a3:q", "a3:q d3:q g3:q a3:q"),
      lyrics: (upper: "un -- ser Gott __ __"),
    ),
    (
      upper: ("b4:q a4:q g4:q a4:q", "e4:q c4:q c4:q c4:q"),
      lower: ("g3:q f3:q e3:q e3:q", "e3:q f3:q c3:q a2:q"),
      lyrics: (upper: "ein gu -- te Wehr"),
    ),
    (
      upper: ("f4:e e4:e d4:q c4:q c5:q[fermata]", "a3:q b3:q c4:q e4:q"),
      lower: ("f3:q g3:q e3:q g3:q", "d3:q g2:q c3:q c3:q"),
      lyrics: (upper: "und Waf -- fen __ __"),
    ),
    (
      upper: ("c5:q c5:q g4:e a4:e b4:q", "c4:q e4:q d4:q e4:q"),
      lower: ("g3:q g3:q b3:q g3:q", "e3:q c3:q g3:q e3:q"),
      lyrics: (upper: "er hilft uns frei aus"),
    ),
    (
      upper: ("c5:e b4:e a4:q g4:q c5:q[fermata]", "c4:q d4:e c4:e b3:q c4:q"),
      lower: ("e3:q f#3:q g3:q e3:q", "a2:q d3:q g2:q a2:q"),
      lyrics: (upper: "al -- ler Not __ __"),
    ),
    (
      upper: ("b4:q a4:q g4:q a4:q", "e4:q c4:q c4:q c4:q"),
      lower: ("g3:q f3:q e3:q e3:q", "e3:q f3:q c3:q a2:q"),
      lyrics: (upper: "die uns jetzt hat"),
    ),
    (
      upper: ("f4:e e4:e d4:q c4:q c4:q[fermata]", "a3:q b3:q c4:q c4:q"),
      lower: ("f3:q g3:e f3:e e3:q e3:q", "d3:q g2:q c3:q c3:q"),
      lyrics: (upper: "be -- trof -- fen __ __"),
    ),
    ),
  )
]
