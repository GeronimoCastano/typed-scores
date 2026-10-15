#import "../src/lib.typ": score

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
