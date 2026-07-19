// typst compile --root . --ppi 260 assets/readme/measures.typ assets/readme/measures.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#align(center, stack(
  spacing: 0.25cm,
  strong[Pickup validation with persistent and changing signatures],
  score(
    staves: (
      upper: (clef: "treble"),
      lower: (clef: "bass"),
    ),
    bars: (
      (
        key: "Eb",
        time: "12/8",
        partial: "1/8",
        upper: "Bb4:e",
        lower: "r:e",
      ),
      (
        upper: "Eb4:e F4:e G4:e Ab4:e Bb4:e C5:e D5:e Eb5:e F5:e G5:e Ab5:e Bb5:e",
        lower: "Eb2:q Bb2:q Eb3:q Bb2:q Eb3:q Bb2:q",
      ),
      (
        key: "G",
        time: "3/4",
        upper: "G4:q B4:q D5:q",
        lower: "G2:q D3:q G3:q",
      ),
    ),
    beams: true,
    scale: 0.56,
    wrap: false,
  ),
))
