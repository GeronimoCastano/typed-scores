#import "../src/lib.typ": score, read-score

#set page(width: 190mm, height: 275mm, margin: 12mm)
#set text(size: 9pt)

#let case-heading(id, title, caption) = {
  text(size: 15pt, weight: "bold")[#id · #title]
  v(0.35em)
  text(fill: rgb("444444"))[#caption]
  v(0.8em)
}

#let c17 = () => {
  case-heading("C17", "Automatic rest crosses the beat grid", [Current output for `c4:e _` in 4/4: one 7/8 rest spans the half-bar.])
  score(time: "4/4", wrap: false, bars: ((notes: "c4:e _", barline: (right: "final")),))
}

#let c42 = () => {
  case-heading("C42", "Four-voice noteheads overlap", [Four simultaneous whole-note voices on adjacent staff positions remain visibly superimposed in the current renderer.])
  score(time: "4/4", scale: 2, bars: ((notes: ("c5:w", "d5:w", "e5:w", "f5:w")),))
}

#let c53 = () => {
  let imported = read-score(read("import/audit-multiple-rest.musicxml", encoding: none), format: "musicxml")
  case-heading("C53", "MusicXML multiple-measure rest remains unexpanded", [Source measure numbers: 1, 2, 5; measure 2 declares a three-measure rest covering measures 2–4. `read-score` currently returns three rendered bars and an explicit warning.])
  block(
    width: 100%,
    inset: 7pt,
    fill: rgb("f2f2f2"),
    stroke: 0.5pt + rgb("bbbbbb"),
  )[
    *Current conversion warning:* #imported.warnings.join("; ")
  ]
  v(0.8em)
  text(size: 8pt, weight: "bold")[Read-score rendering (3 bars; source rest span covers 3 measures):]
  score(..imported.arguments, width: 58, scale: 0.7, wrap: false, ragged-right: false)
}

#let c57 = () => {
  case-heading("C57", "Cross-staff slur/beam and accent/dynamic spacing", [The cross-staff slur crosses the upper beam near its start; the second panel shows the below-note accent and ff dynamic. The third panel checks the reported voice-two natural, which is visible in the current render.])
  score(
    staves: (upper: (clef: "treble"), lower: (clef: "bass")),
    time: "2/4",
    beams: true,
    width: 38,
    bars: ((upper: "C5:e[s1(] G4 @lower E3 C3[s1)]", lower: "C2:h"),),
  )
  v(0.8em)
  score(
    staves: (soprano: (clef: "treble"), bass: (clef: "bass")),
    group: "brace",
    time: "4/4",
    wrap: false,
    scale: 0.62,
    bars: ((soprano: "(c3 e3 g3 c4):q[accent dyn=ff s1(] b3:q a3:q g3:q[s1)]", bass: "c3:w"),),
  )
  v(0.8em)
  score(
    staves: (upper: (clef: "treble"), lower: (clef: "bass")),
    time: "2/4",
    bars: ((upper: ("F#5:q G5", "D5:q F5"), lower: "F#4:q @upper F4"),),
  )
}

#let c59 = () => {
  case-heading("C59", "Rest dynamic and chromatic-turn/slur spacing", [These two remaining spacing concerns reproduce in the current render. The cited cue example has an upper-staff cue pickup against a lower-staff rest, not lower-staff cue notes beneath upper beams; that third claim is not reproduced.])
  score(time: "4/4", scale: 0.9, bars: ((notes: "r:q[dyn=pp] C2:q[fermata] G4:q[breath] C3:q[dyn=ff]", barline: (right: "final")),))
  v(0.8em)
  score(time: "4/4", scale: 1.2, bars: ((notes: "G4:q[s1(] A4:q[chromatic-turn] B4:q C5:q[s1)]", barline: (right: "final")),))
}

= Partial audit findings: current render examples

#c17()

#pagebreak()

#c42()

#pagebreak()

#c53()

#pagebreak()

#c57()

#pagebreak()

#c59()
