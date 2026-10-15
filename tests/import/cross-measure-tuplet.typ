#import "../../src/lib.typ": score, read-score

#let imported = read-score(read("cross-measure-tuplet.musicxml"), format: "musicxml")
#score(..imported.arguments)
