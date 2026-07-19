// typst compile --root . --ppi 260 assets/readme/quick-start.typ assets/readme/quick-start.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#stack(
  spacing: 0.6cm,
  align(center, stack(
    spacing: 0.2cm,
    strong[#raw("bar(...)", lang: "typst")],
    bar(
      "g4:e a4:e b4:e c5:e d5:e e5:e f#5:e g5:e",
      clef: "treble",
      key: "G",
      time: "4/4",
    ),
  )),
  align(center, stack(
    spacing: 0.2cm,
    strong[#raw("score(...)", lang: "typst")],
    score(
      clef: "treble",
      time: "4/4",
      bars: (
        (notes: "c5:q d5:q e5:q f5:q"),
        (notes: "g5:h e5:h"),
      ),
    ),
  )),
)
