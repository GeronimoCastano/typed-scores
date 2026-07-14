// typst compile --root . --ppi 260 assets/readme/ensemble.typ assets/readme/ensemble.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#align(center, stack(
  spacing: 0.25cm,
  strong[Treble, alto, and bass voices across two measures],
  score(
    sections: (
      (
        key: "C",
        time: "4/4",
        tempo: "Moderato",
        voices: (
          (name: "Violin", clef: "treble", notes: (
            "E5:q F5:q G5:q A5:q",
            "G5:h E5:h",
          )),
          (name: "Viola", clef: "alto", notes: (
            "C4:q D4:q E4:q F4:q",
            "E4:h C4:h",
          )),
          (name: "Cello", clef: "bass", notes: (
            "C3:h G2:h",
            "C3:w",
          )),
        ),
      ),
    ),
    beams: true,
    scale: 0.56,
    wrap: false,
  ),
))
