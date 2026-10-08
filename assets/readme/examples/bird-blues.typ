// Bird blues in F: an original 12-bar bebop lead sheet over Parker-style changes.
// The melody, walking bass, and harmony are new compositions, not transcriptions.
// Concert pitch. Key of F; the pickup is one triplet beat that leads into bar 1,
// and the bar 12 turnaround uses a triplet beat as well.
//
// typst compile --root . --ppi 300 assets/readme/examples/bird-blues.typ assets/readme/examples/bird-blues.png

#import "../../../src/lib.typ": *

#set page(width: 17cm, height: auto, margin: 12pt)
#set text(font: "New Computer Modern", size: 11pt)

#block(width: 100%, fill: white, stroke: 0.6pt + luma(215), radius: 8pt, inset: 18pt)[
  #text(size: 9pt, style: "italic")[Bird blues in F --- original lead sheet]
  #v(0.4em)
  #score(
    staves: (
      upper: (clef: "treble"),
      lower: (clef: "bass"),
    ),
    key: "F",
    time: "4/4",
    theme: "light",
    beams: true,
    scale: 0.46,
    width: 117,
    staff-gap: none,
    note-spacing: 2.2,
    system-gap: 0.5em,
    bars: (
    (
      partial: "1/4",
      tempo: text(size: 10pt, style: "italic")[Swing],
      upper: "tuplet 3:2 { e5:e f5 g5 }",
      lower: "r:q",
    ),
    (
      upper: "a5:e c6 e5 f5 a5 c6 a5 f5",
      lower: "f2:q a2 c3 e2",
      harmony: "Fmaj7:w",
    ),
    (
      upper: "g5:e a5 bb5 a5 e5 g5 c#6 bb5",
      lower: "e2:q g2 a2 c#3",
      harmony: "Em7b5:h A7(b9):h",
    ),
    (
      upper: "d5:e f5 a5 c6 g5 f5 d5 b4",
      lower: "d2:q f2 g2 b1",
      harmony: "Dm7:h G7:h",
    ),
    (
      upper: "bb4:e eb5 g5 c6 a5 c6 f5 eb5",
      lower: "c2:q eb2 f2 a2",
      harmony: "Cm7:h F7:h",
    ),
    (
      upper: "d5:e f5 ab5[accent] f5 d5 bb4 d5 f5",
      lower: "bb2:q d3 f3 a2",
      harmony: "Bb7:w",
    ),
    (
      upper: "db5:e f5 ab5 f5 g5 bb5 eb5 db5",
      lower: "bb2:q db3 eb2 g#2",
      harmony: "Bbm7:h Eb7:h",
    ),
    (
      upper: "e5:e g5 c6 a5 f#5 a5 c6[tenuto] a5",
      lower: "a2:q c3 d2 g2",
      harmony: "Am7:h D7:h",
    ),
    (
      upper: "eb5:e gb5 cb5 eb5 f5 db5 ab4 f5",
      lower: "ab2:q cb3 db2 f#2",
      harmony: "Abm7:h Db7:h",
    ),
    (
      upper: "g5:e bb5 d6 bb5 g5 f5[stacc] d5 bb4",
      lower: "g2:q bb2 d3 b1",
      harmony: "Gm7:w",
    ),
    (
      upper: "e5:e g5 bb5 g5 e5 c5 bb4 a4",
      lower: "c2:q g2 bb2 e2",
      harmony: "C7:w",
    ),
    (
      upper: "a4:e c5 e5 c5 d5 f#5 a5 f#5",
      lower: "f2:q a2 d2 f#2",
      harmony: "Fmaj7:h D7:h",
    ),
    (
      upper: "d5:e g5 bb5 g5 e5 g5 tuplet 3:2 { e5:e d5 c5 }",
      lower: "g2:q bb2 c2 e2",
      harmony: "Gm7:h C7:h",
    ),
    ),
  )
]
