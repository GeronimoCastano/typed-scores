#import "chopin-opening.typ": chopin-opening
#import "mozart-eine-kleine-nachtmusik.typ": mozart-k525-opening
#import "bach-cello-suite-prelude.typ": bach-bwv1007-opening

#set page(
  paper: "a4",
  margin: (x: 12mm, top: 10mm, bottom: 10mm),
  numbering: "1",
)
#set text(font: "New Computer Modern", size: 10pt)

#let plate(title, subtitle, instrumentation, source-note, body) = {
  align(center)[
    #text(size: 18pt, weight: "semibold")[#title]
    #v(0.12em)
    #text(size: 10.5pt, style: "italic")[#subtitle]
    #v(0.18em)
    #text(size: 8pt, fill: rgb("555555"))[#instrumentation]
  ]
  v(1.25em)
  align(center, body)
  v(1fr)
  line(length: 100%, stroke: 0.35pt + rgb("bbbbbb"))
  v(0.35em)
  text(size: 7.6pt, fill: rgb("666666"))[#source-note]
}

#plate(
  [Nocturne in E-flat major],
  [Op. 9, No. 2 · opening pickup and measures 1–8],
  [Piano],
  [Translated from Mutopia's LilyPond transcription of G. Schirmer, 1881 (CC BY-SA 3.0).],
)[
  #chopin-opening(
    scale: 0.47,
    note-spacing: 2.45,
    wrap: true,
    width: 129,
  )
]

#pagebreak()

#plate(
  [Eine kleine Nachtmusik],
  [Serenade in G major, K. 525 · movement I, measures 1–10],
  [Violin I · Violin II · Viola · Violoncello],
  [Translated from Mutopia's public-domain LilyPond full score.],
)[
  #mozart-k525-opening(
    scale: 0.51,
    note-spacing: 3.15,
    wrap: true,
    width: 118,
  )
]

#pagebreak()

#plate(
  [Cello Suite No. 1 in G major],
  [BWV 1007 · Prelude, measures 1–16],
  [Solo cello],
  [Translated from Mutopia's public-domain LilyPond cello part, after Schirmer, 1916.],
)[
  #bach-bwv1007-opening(
    scale: 0.82,
    note-spacing: 2.30,
    wrap: true,
    width: 82,
  )
]
