// typst compile --root . --ppi 260 assets/readme/system-layout.typ assets/readme/system-layout.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#align(center, score(
  time: "4/4",
  width: 36,
  indent: 3.5,
  ragged-last: true,
  scale: 0.72,
  bars: (
    (notes: "c5:q d e f"),
    (notes: "g5:q a b c6"),
    (notes: "c6:h g5:h"),
    (notes: "f5:q e d c"),
    (notes: "b4:h c5:h"),
    (notes: "d5:q e f g"),
  ),
))
