// typst compile --root . --ppi 260 assets/readme/note-language.typ assets/readme/note-language.png
#import "../../src/lib.typ": *

#set page(width: 18cm, height: auto, margin: 0.7cm, fill: white)
#set text(font: "New Computer Modern", size: 10pt)

#stack(
  spacing: 0.65cm,
  align(center, stack(
    spacing: 0.2cm,
    strong[Pitches, durations, accidentals, rests, and chords],
    score(
      bars: ((notes: "c4:q d4:e e4:e f#4:q (g4 bb4 d5):q"),),
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
    score(
      bars: ((notes: "c5:q ~ c5:e d5:e e5:e f5:e / g5:e a5:e"),),
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
    [#score(time: none, bars: ((notes: "c5:w"),), scale: 0.58)],
    [#score(time: none, bars: ((notes: "d5:h."),), scale: 0.58)],
    [#score(time: none, bars: ((notes: "e5:s"),), scale: 0.58)],
    [#score(time: none, bars: ((notes: "f5:q.."),), scale: 0.58)],
  ),
)
