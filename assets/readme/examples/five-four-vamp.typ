// Five-Four Vamp: an original study in 5/4 over E-flat minor changes, Ebm7 and Bbm7.
// The right-hand melody, the chord stabs, and the walking left hand are new
// compositions for this README. They quote no published tune or bass figure.
//
// typst compile --root . --ppi 300 assets/readme/examples/five-four-vamp.typ assets/readme/examples/five-four-vamp.png

#import "../../../src/lib.typ": *

#set page(width: 15cm, height: auto, margin: 12pt)
#set text(font: "New Computer Modern", size: 11pt)

#let plate-header(title, subtitle, instrumentation) = {
  align(center)[
    #text(size: 18pt, weight: "semibold")[#title]
    #v(0.12em)
    #text(size: 10.5pt, style: "italic")[#subtitle]
    #v(0.18em)
    #text(size: 8pt, fill: rgb("555555"))[#instrumentation]
  ]
  v(1em)
}

#block(width: 100%, fill: white, stroke: 0.6pt + luma(215), radius: 8pt, inset: 18pt)[
  #plate-header(
    [Five-Four Vamp],
    [Original study in 5/4 · E-flat minor],
    [Piano],
  )
  #score(
    staves: (
      upper: (clef: "treble"),
      lower: (clef: "bass"),
    ),
    key: "Ebm",
    time: "5/4",
    tempo: [Swing],
    composer: [typed-scores example],
    theme: "light",
    beams: true,
    scale: 0.5,
    wrap: true,
    bars: (
      (
        upper: "bb4:e[dyn=mp] db5 eb5 gb5:q. f5:e ab5 f5 db5",
        lower: "eb2:q gb2 bb2 bb2:e db3 f3 ab3",
        harmony: "Ebm7:h. Bbm7:h",
      ),
      (
        upper: "db5:e eb5 gb5 bb5:q. tuplet 3:2 { f5:e g5 ab5 } (bb4 db5 f5 ab5):q[accent]",
        lower: "gb2:q bb2 db3 f2:e bb2 db3 f3",
        harmony: "Ebm7:h. Bbm7:h",
      ),
      (
        upper: "(eb4 gb4 bb4 db5):q.[accent] db5:e eb5 gb5 f5:q ab5:e db5:e",
        lower: "eb2:e gb2 bb2:q db3:q bb2:q ab2:e db3:e",
        harmony: "Ebm7:h. Bbm7:h",
      ),
      (
        upper: "tuplet 3:2 { eb5:e db5 bb4 } gb4:q (eb4 gb4 bb4 db5):q[accent] db5:e f5 ab5 bb5:e",
        lower: "eb2:q. gb2:e bb2:q f2:q db3:q",
        harmony: "Ebm7:h. Bbm7:h",
      ),
    ),
  )
]
