// typst compile --root . --ppi 260 assets/readme/note-language.typ assets/readme/note-language.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#stack(
  spacing: 0.65cm,
  align(center, stack(
    spacing: 0.2cm,
    strong[Pitches, durations, accidentals, rests, and chords],
    bar(
      "C4:q D4:e E4:e F#4:q (G4 Bb4 D5):q",
      clef: "treble",
      key: "C",
      time: "4/4",
      beams: true,
      scale: 0.72,
    ),
  )),
  align(center, stack(
    spacing: 0.2cm,
    strong[Ties and manual beam breaks],
    bar(
      "C5:q ~ C5:e D5:e E5:e F5:e / G5:e A5:e",
      clef: "treble",
      time: "4/4",
      beams: true,
      scale: 0.72,
    ),
  )),
  table(
    columns: (1fr, 1fr, 1fr, 1fr),
    gutter: 0.2cm,
    align: center + horizon,
    stroke: 0.4pt + rgb("#dedede"),
    [*Whole*], [*Dotted half*], [*Sixteenth*], [*Double-dotted*],
    [#note("C5:w", scale: 0.58)],
    [#note("D5:h.", scale: 0.58)],
    [#note("E5:s", scale: 0.58)],
    [#note("F5:q..", scale: 0.58)],
  ),
)
