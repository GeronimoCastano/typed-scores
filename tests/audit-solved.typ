#import "../src/lib.typ": score
#import "../examples/chopin-opening.typ": chopin-opening
#import "../examples/mozart-eine-kleine-nachtmusik.typ": mozart-k525-opening

#set page(width: 190mm, height: auto, margin: 12mm)
#set text(size: 9pt)

#let case-heading(id, title, caption) = {
  text(size: 15pt, weight: "bold")[#id · #title]
  v(0.35em)
  text(fill: rgb("444444"))[#caption]
  v(0.8em)
}

#let diagnostic-panel(source, diagnostic) = block(
  width: 100%,
  inset: 8pt,
  fill: rgb("f2f2f2"),
  stroke: 0.5pt + rgb("bbbbbb"),
)[
  *Source*
  #v(0.3em)
  #raw(source, lang: "typ")
  #v(0.45em)
  *Current editor diagnostic*
  #v(0.3em)
  #text(size: 8pt)[#diagnostic]
]

#let c26 = () => {
  case-heading("C26", "Infeasible dense layouts now report errors", [Both formerly problematic width cases now stop with targeted editor-visible diagnostics instead of drawing merged bars or reaching the renderer crash.])
  diagnostic-panel(
    "#score(width: 14, time: \"4/4\", bars: ((notes: \"c5:s \" * 16),))",
    "no legal system partition fits the requested width; got 14; expected enough staff-space width for every system prologue, label, and complete bar",
  )
  v(0.8em)
  diagnostic-panel(
    "#score(width: 14, wrap: false, ragged-right: false, time: \"4/4\", bars: ((notes: \"c5:s \" * 16),))",
    "unwrapped score is wider than the requested width; got 14; expected enough width for every complete measure and the staff prologue",
  )
}

#let c43 = () => {
  case-heading("C43", "Nested tuplets and duration-based spacing", [Sibling inner tuplets share the single outer nesting lane. The lower pair keeps its written half-note spacing rather than treating each event as a quarter note.])
  score(
    time: "4/4",
    scale: 2.2,
    bars: ((notes: "tuplet 3:2 { tuplet 3:2 { c5:e d5:e e5:e } tuplet 3:2 { f5:e g5:e a5:e } } tuplet 6:4 { c5:s d5:s e5:s f5:s } c5:h", barline: (right: "final")),),
  )
  v(0.9em)
  text(size: 8pt, weight: "bold")[Shortest written value is a half note:]
  score(time: "4/4", wrap: false, bars: ((notes: "c5:h d5:h", barline: (right: "final")),))
}

#let c45 = () => {
  case-heading("C45", "Auto-width score uses its natural page width", [With no requested width, this two-measure score renders at its content width without a forced page-sized measure.])
  score(bars: ((notes: "c4:w"), (notes: "c4:w")))
}

#let c46 = () => {
  case-heading("C46", "Ottava can close and reopen between adjacent events", [The first 8va span closes on D and a second span opens on that same event, then closes after F.])
  score(time: "4/4", wrap: false, bars: ((notes: "c6:q[8va(] d6:q[8va) 8va(] e6:q f6:q[8va)]", barline: (right: "final")),))
}

#let c56-chopin = () => {
  case-heading("C56", "Chopin notation matches its LilyPond source", [Current canonical Typst engraving includes the chromatic turn with fingering, the corresponding hairpin placement, and the corrected phrase markings. The matching LilyPond source is examples/lilypond-reference/chopin-nocturne-op9-no2.ly.])
  chopin-opening(
    bar-count: 8,
    scale: 0.31,
    note-spacing: 2.6,
    staff-gap: none,
    wrap: true,
    width: 118,
  )
}

#let c56-mozart = () => {
  case-heading("C56", "Mozart slur endpoints match the source", [Measures 5–8 show the phrase slurs ending before the final staccato notes, matching examples/lilypond-reference/mozart-eine-kleine-nachtmusik.ly.])
  mozart-k525-opening(
    bar-count: 8,
    scale: 0.38,
    note-spacing: 2.8,
    staff-gap: none,
    wrap: true,
    width: 116,
  )
}

= Solved audit findings: current render evidence

#c26()

#pagebreak()

#c43()

#pagebreak()

#c45()

#pagebreak()

#c46()

#pagebreak()

#c56-chopin()

#pagebreak()

#c56-mozart()
