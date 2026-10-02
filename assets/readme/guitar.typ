// typst compile --root . --ppi 260 assets/readme/guitar.typ assets/readme/guitar.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#align(center, score(
  staves: (
    guitar: (clef: "treble-8"),
    tab: (clef: "tab", source: "guitar"),
  ),
  time: "4/4",
  chord-diagrams: guitar-chords,
  beams: true,
  bars: (
    (guitar: "a2:e e3 a3 c4 e4 c4 a3 e3", harmony: "Am:w"),
    (guitar: "c3:e e3 g3 c4 e4 c4 g3 e3", harmony: "C:w"),
    (guitar: "d3:e a3 d4 f#4 (d3 a3 d4 f#4):h", harmony: "D:w"),
    (guitar: "(f2 c3 f3 a3 c4 f4):w", harmony: "F:w", barline: (right: "final")),
  ),
  scale: 0.72,
  width: 78,
))
