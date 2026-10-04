#import "../../src/lib.typ": score

#score(
  staves: (upper: (clef: "treble"), lower: (clef: "bass")),
  time: "4/4",
  bars: ((upper: "c5:w", lower: "c2:q clef treble @upper c4:h r:q"),),
)
