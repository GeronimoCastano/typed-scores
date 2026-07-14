// typst compile --root . --ppi 260 assets/readme/expressive.typ assets/readme/expressive.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#stack(
  spacing: 0.65cm,
  align(center, stack(
    spacing: 0.2cm,
    strong[Combined accents and articulations],
    bar(
      "C6:q[marcato stacc f=1] Bb5:q[accent tenuto f=2] A5:q[tenuto staccatissimo f=3] G5:q[accent stacc f=4]",
      time: "4/4",
      scale: 0.68,
    ),
    bar(
      "C4:q[marcato stacc f=1] D4:q[accent tenuto f=2] E4:q[tenuto staccatissimo f=3] F4:q[accent stacc f=4]",
      time: "4/4",
      scale: 0.68,
    ),
  )),
  align(center, stack(
    spacing: 0.2cm,
    strong[Slur, turn, fingering, hairpin, and pedal],
    score(
      treble: "C5:q[f=1 s1(] D5:q[f=2 h1<] E5:q[f=3 turn] G5:q[f=5 s1) h1!]",
      bass: "C3:q[p1(] G2:q C3:q G2:q[p1)]",
      time: "4/4",
      scale: 0.62,
      staff-gap: 11,
      wrap: false,
    ),
  )),
)
