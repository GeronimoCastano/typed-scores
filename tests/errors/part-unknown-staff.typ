#import "../../src/lib.typ": score, part
#let duet = (
  staves: (flute: (clef: "treble"), cello: (clef: "bass")),
  bars: ((flute: "C5:w", cello: "C3:w"),),
)
#score(..part(duet, "viola"))
