#import "../../src/lib.typ": import-score
#let first-measure = read("../import/cross-measure-tuplet.musicxml").split("<measure number=\"2\">").first()
#import-score(first-measure + "</part></score-partwise>", format: "musicxml")
