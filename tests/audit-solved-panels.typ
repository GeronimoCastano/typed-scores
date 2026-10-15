#import "../src/lib.typ": score

#let panel-heading(id, title) = [
  #text(size: 13pt, weight: "bold")[#id — #title]
  #v(0.45em)
]

#let export-c22() = [
  #panel-heading("C22", "Incoming tie on the top chord pitch")
  #score(
    width: 40,
    time: "4/4",
    bars: (
      (notes: "(c4 g4):w ~"),
      (notes: "(c4 g4):w"),
    ),
  )
]

#let export-c25() = [
  #panel-heading("C25", "Mixed-register chord stem direction")
  #score(
    time: "4/4",
    scale: 3,
    bars: ((notes: "(e4 f4 g4 a4 a5):h r:h"),),
  )
]

#let export-c28() = [
  #panel-heading("C28", "Attacked unison fret during a tie-in")
  #score(
    staves: (tab: (clef: "tab", tuning: "guitar")),
    time: "4/4",
    bars: (
      (tab: ("c4:w~", "e4:w")),
      (tab: ("c4:w", "c4:w")),
      (tab: ("c4:w", "e4:w")),
    ),
  )
]

#let export-c41() = [
  #panel-heading("C41", "Melisma and grace-tab spacing")
  #score(
    width: 26,
    time: "4/4",
    bars: (
      (notes: "c5:q d e f", lyrics: "Glo __ __ __"),
      (notes: "g5:q a b c6", lyrics: "__ __ __ __"),
    ),
  )
  #v(0.8em)
  #score(
    staves: (tab: (clef: "tab", tuning: "guitar")),
    time: "4/4",
    bars: ((tab: "grace { e5:s f5:s f#5:s g5:s } b5:w"),),
  )
  #text(size: 8pt)[Open strings are excluded from fretted barre construction by design.]
]

#let export-c44() = [
  #panel-heading("C44", "Dashed barline across a brace group")
  #score(
    staves: (
      upper: (clef: "treble"),
      lower: (clef: "bass"),
    ),
    group: "brace",
    time: "4/4",
    bars: ((
      upper: "c5:w",
      lower: "c3:w",
      barline: (right: "dashed"),
    ),),
  )
]

#let export-c54() = [
  #panel-heading("C54", "Final barline clears stem-up seconds")
  #score(
    clef: "treble",
    time: "4/4",
    scale: 0.85,
    wrap: false,
    bars: ((notes: "(C4 E4):h (D4 E4 F4):h"),),
  )
]

#let export-c55() = [
  #panel-heading("C55", "Adjacent ending hooks at the repeat barline")
  #score(
    time: "2/4",
    width: 45,
    ragged-right: false,
    bars: (
      (notes: "c5:q d5:q", barline: (left: "repeat-start")),
      (ending: (label: "1.", start: true), notes: "e5:q f5:q"),
      (barline: (right: "repeat-end"), ending: (label: "1.", stop: true), notes: "b5:q b5:q"),
      (ending: (label: "Final", start: true), notes: "f6:q a5:q"),
      (ending: (label: "Final", stop: true), notes: "g5:h"),
    ),
    scale: 1.2,
    wrap: false,
  )
]

#let panel-exporters = (
  "C22": export-c22,
  "C25": export-c25,
  "C28": export-c28,
  "C41": export-c41,
  "C44": export-c44,
  "C54": export-c54,
  "C55": export-c55,
)

#let export-panel(id) = {
  if id not in panel-exporters {
    panic("unknown engraving audit panel " + id)
  }
  panel-exporters.at(id)()
}
