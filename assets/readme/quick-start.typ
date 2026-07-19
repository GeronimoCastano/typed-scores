// typst compile --root . --ppi 260 assets/readme/quick-start.typ assets/readme/quick-start.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#stack(
  spacing: 0.6cm,
  table(
    columns: (1fr, 1fr, 1fr),
    gutter: 0.25cm,
    align: center + horizon,
    stroke: 0.4pt + rgb("#dedede"),
    [*Single note*], [*Chord*], [*Rest*],
    [#score(time: none, bars: ((notes: "c4:q"),), scale: 0.72)],
    [#score(time: none, bars: ((notes: "(c4 e4 g4):h"),), scale: 0.72)],
    [#score(clef: "bass", time: none, bars: ((notes: "r:q"),), scale: 0.72)],
  ),
  align(center, stack(
    spacing: 0.2cm,
    strong[Validated and automatically beamed measure],
    score(
      bars: ((notes: "g4:e a4:e b4:e c5:e d5:e e5:e f#5:e g5:e"),),
      clef: "treble",
      key: "G",
      time: "4/4",
      beams: true,
      scale: 0.68,
    ),
  )),
  align(center, stack(
    spacing: 0.2cm,
    strong[Parallel grand staff],
    score(
      staves: (upper: (clef: "treble"), lower: (clef: "bass")),
      bars: ((
        upper: "c5:q d5:q e5:q g5:q",
        lower: "c3:h g2:h",
      ),),
      time: "4/4",
      scale: 0.62,
      wrap: false,
    ),
  )),
)
