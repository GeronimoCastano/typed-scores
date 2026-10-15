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
  case-heading("C42", "Four-voice density and ornament clearance", [Two current stress cases: tightly layered sixteenths and a flagged note with an acciaccatura immediately before it.])
  score(
    time: "4/4",
    beams: true,
    scale: 0.62,
    bars: ((
      notes: (
        "c5:s d5:s e5:s f5:s g5:s a5:s b5:s c6:s d6:s c6:s b5:s a5:s g5:s f5:s e5:s d5:s",
        "g4:s a4:s b4:s c5:s d5:s e5:s f5:s g5:s a5:s g5:s f5:s e5:s d5:s c5:s b4:s a4:s",
        "e4:s f4:s g4:s a4:s b4:s c5:s d5:s e5:s f5:s e5:s d5:s c5:s b4:s a4:s g4:s f4:s",
        "c4:s d4:s e4:s f4:s g4:s a4:s b4:s c5:s d5:s c5:s b4:s a4:s g4:s f4:s e4:s d4:s",
      ),
    ),),
  )
  v(1em)
  score(time: "4/4", scale: 1.0, bars: ((notes: "c4:q acciaccatura { d5:e } e5:q r:h", barline: (right: "final")),))
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
  case-heading("C57", "Cross-staff slur/beam and accent/dynamic spacing", [The third panel checks the reported voice-two natural; it is visible in the current rendering, so that subclaim is not reproduced here.])
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
  case-heading("C59", "Rest dynamic, turn/slur, and cue-beam spacing", [Current examples for the three remaining cosmetic spacing concerns.])
  score(time: "4/4", scale: 0.9, bars: ((notes: "r:q[dyn=pp] C2:q[fermata] G4:q[breath] C3:q[dyn=ff]", barline: (right: "final")),))
  v(0.8em)
  score(time: "4/4", scale: 1.2, bars: ((notes: "G4:q[s1(] A4:q[chromatic-turn] B4:q C5:q[s1)]", barline: (right: "final")),))
  v(0.8em)
  score(
    staves: (upper: (clef: "treble"), lower: (clef: "bass")),
    key: "Cm",
    time: "2/4",
    beams: true,
    scale: 0.8,
    bars: (
      (partial: "1/16", upper: "cue { tuplet 3:2[number=never] { g4:t d5 d } }", lower: "r:s"),
      (upper: "eb5:s f#4 c5 eb d f4 b d", lower: "(g2 c3 g3):e r (g2 c3 g3):e r"),
    ),
  )
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
