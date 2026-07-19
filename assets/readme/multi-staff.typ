// typst compile --root . --ppi 260 assets/readme/multi-staff.typ assets/readme/multi-staff.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#align(center, score(
  staves: (
    upper: (clef: "treble", label: "Violin I", short-label: "Vln. I"),
    lower: (clef: "bass", label: "Violoncello", short-label: "Vc."),
  ),
  key: "Eb",
  time: "12/8",
  bars: (
    (
      partial: "1/8",
      upper: "bb4:e",
      lower: "r:e",
    ),
    (
      upper: "g5:q. f5:e g5:e bb5:e ab5:q. g5:q f5:e",
      lower: "eb2:e (g3 eb4):e (bb3 eb4 g4):e eb2:e (ab3 d4):e (cb4 d4 ab4):e eb2:e (g3 eb4):e (bb3 eb4 g4):e d2:e (g3 eb4):e (bb3 eb4 g4):e",
    ),
  ),
  beams: true,
  scale: 0.72,
))
