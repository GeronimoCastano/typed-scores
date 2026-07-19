// typst compile --root . --ppi 260 assets/readme/barlines.typ assets/readme/barlines.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#align(center, score(
  clef: "treble",
  time: "2/4",
  scale: 0.8,
  bars: (
    (barline: (left: "repeat-start"), notes: "c5:q d5:q"),
    (ending: (label: "1.", start: true), notes: "e5:q f5:q"),
    (
      barline: (right: "repeat-end"),
      ending: (label: "1.", stop: true),
      notes: "g5:q a5:q",
    ),
    (ending: (label: "Final", start: true), notes: "b5:q a5:q"),
    (ending: (label: "Final", stop: true), notes: "g5:h"),
  ),
))
