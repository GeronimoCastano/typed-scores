#import "../src/lib.typ": score, read-score
#set page(width: 190mm, height: auto, margin: 12mm)
#set text(size: 10pt)

= One tuplet across a measure boundary
#let imported = read-score("X:1\nM:4/4\nL:3/4\nK:C\n(3C D | E |]\n", format: "abc")
#score(..imported.arguments, scale: 2)

= A tuplet continued across systems
#score(time: "4/4", width: 24, ragged-right: false, bars: (
  (notes: "tuplet 3:2[span=continued] { c4:h. d4 }"),
  (notes: "tuplet 3:2[span=continued] { e4:h. f4 }"),
  (notes: "tuplet 3:2[span=continued] { g4:h. a4 }"),
))

= Separate beams under one cross-measure bracket
#score(time: "1/4", beams: true, bars: (
  (notes: "tuplet 3:2[span=beamed] { c5:e d e }"),
  (notes: "tuplet 3:2[span=beamed] { f5:e g a }"),
))

= Independent voices may reuse a span ID
#score(time: "4/4", bars: (
  (notes: ("tuplet 3:2[span=triplet side=above] { c5:h. d5 }", "tuplet 3:2[span=triplet side=below] { c4:h. d4 }")),
  (partial: "1/2", notes: ("tuplet 3:2[span=triplet side=above] { e5:h. }", "tuplet 3:2[span=triplet side=below] { e4:h. }")),
))

= A notation span mirrored on tablature
#score(time: "4/4", staves: (
  guitar: (clef: "treble-8"),
  tab: (clef: "tab", source: "guitar"),
), bars: (
  (guitar: "tuplet 3:2[span=triplet] { c4:h. d4 }"),
  (partial: "1/2", guitar: "tuplet 3:2[span=triplet] { e4:h. }"),
))
