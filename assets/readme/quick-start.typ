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
    [#note("C4:q", clef: "treble", scale: 0.72)],
    [#note("(C4 E4 G4):h", clef: "treble", scale: 0.72)],
    [#note("r:q", clef: "bass", scale: 0.72)],
  ),
  align(center, stack(
    spacing: 0.2cm,
    strong[Validated and automatically beamed measure],
    bar(
      "G4:e A4:e B4:e C5:e D5:e E5:e F#5:e G5:e",
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
      treble: "C5:q D5:q E5:q G5:q",
      bass: "C3:h G2:h",
      time: "4/4",
      scale: 0.62,
      wrap: false,
    ),
  )),
)
