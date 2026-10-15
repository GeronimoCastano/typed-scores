#import "../src/lib.typ": read-score, score
#import "../src/engraving/event-geometry.typ": _event-notation-scale, _head-half-width
#import "../src/engraving/primitives.typ": stem-center-offset, stem-thickness
#import "../src/engraving/spacing.typ": _cluster-offsets

#set page(margin: 12pt)
#set text(size: 9pt)

= Grace chord clearance at score start

#score(
  time: "4/4",
  scale: 3,
  bars: ((notes: "acciaccatura { (d5 e5):e } f5:q g5:q r:h"),),
)

= Grace chord clearance between notes

#score(
  time: "4/4",
  scale: 3,
  bars: ((notes: "c4:q acciaccatura { (d5 e5):e } f5:q r:h"),),
)

= Plain grace seconds and a single flagged grace

#score(
  time: "4/4",
  scale: 3,
  bars: ((notes: "grace { (d5 e5):e } f5:q g5:q r:h"),),
)

#score(
  time: "4/4",
  scale: 2.5,
  bars: (
    (notes: "grace { (d5 e5):e } f5:q acciaccatura { d5:e } f5:q r:h", barline: (right: "final")),
  ),
)

= Forced up and down stems around grace chords

#score(
  time: "4/4",
  scale: 2.5,
  bars: ((
    notes: (
      "c4:q acciaccatura { (d5 e5):e } f5:q r:h",
      "r:q acciaccatura { (d3 e3):e } f3:q r:h",
    ),
    barline: (right: "final"),
  ),),
)

= Multiple grace notes and mirrored tablature

#score(
  staves: (
    guitar: (clef: "treble-8"),
    tab: (clef: "tab", source: "guitar"),
  ),
  time: "4/4",
  bars: ((guitar: "c4:q acciaccatura { d5:s e } f5:q r:h"),),
)

= Grace chord stem attachment at reduced size

#let assert-reduced-chord-stem-attachment(direction, is-cue: false) = {
  let layout = (
    notehead: "black",
    stem: true,
    grace: not is-cue,
    cue: is-cue,
    pitches: ((staff_position: 8), (staff_position: 9)),
  )
  let notation-scale = _event-notation-scale(layout)
  let cluster-offsets = _cluster-offsets(layout, direction, notation-scale: notation-scale)
  let attached-head-index = if direction == "up" { 1 } else { 0 }
  let stem-x = if direction == "up" {
    stem-center-offset(scale: notation-scale)
  } else {
    -stem-center-offset(scale: notation-scale)
  }
  let attached-head-center = cluster-offsets.at(attached-head-index)
  let attached-head-left = attached-head-center - _head-half-width(layout) * notation-scale
  let attached-head-right = attached-head-center + _head-half-width(layout) * notation-scale
  assert(attached-head-left < stem-x + stem-thickness / 2)
  assert(attached-head-right > stem-x - stem-thickness / 2)
}

#assert-reduced-chord-stem-attachment("up")
#assert-reduced-chord-stem-attachment("down")
#assert-reduced-chord-stem-attachment("up", is-cue: true)
#assert-reduced-chord-stem-attachment("down", is-cue: true)

#let imported-grace-chord = read-score(
  "X:1\nM:4/4\nL:1/4\nK:C\n{c}{[de]}f g a b |]\n",
  format: "abc",
)
#score(..imported-grace-chord.arguments)

#score(
  time: "4/4",
  beams: true,
  bars: ((notes: "grace { c5:e } grace { (d5 e5):e } f5:q g5:q r:h"),),
)

#score(
  time: "4/4",
  beams: false,
  bars: ((notes: "grace { c5:s } grace { (d5 e5):s } f5:q g5:q r:h"),),
)

#score(
  time: "4/4",
  bars: ((notes: "c4:q grace { (d5 e5 g5):e } f5:q r:h"),),
)

#score(
  time: "4/4",
  bars: ((notes: "cue { (d5 e5):q } f5:q g5:q r:q"),),
)
