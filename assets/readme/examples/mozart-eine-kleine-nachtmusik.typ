// Eine kleine Nachtmusik, Serenade in G major, K. 525, movement I, measures 1-4:
// the four-staff string opening, imported from the public-domain fixture.
//
// Translated from Mutopia's public-domain LilyPond full score
// (https://www.ibiblio.org/pub/multimedia/mutopia/MozartWA/KV525/eine-kleine-nachtmusik-mvt1/).
// The source is the same transcription used by examples/mozart-eine-kleine-nachtmusik.typ.
//
// typst compile --root . --ppi 300 assets/readme/examples/mozart-eine-kleine-nachtmusik.typ assets/readme/examples/mozart-eine-kleine-nachtmusik.png

#import "../../../examples/mozart-eine-kleine-nachtmusik.typ": mozart-k525-opening

#set page(width: 15cm, height: auto, margin: 12pt)
#set text(font: "New Computer Modern", size: 11pt)

#let plate-header(title, subtitle, instrumentation) = {
  align(center)[
    #text(size: 18pt, weight: "semibold")[#title]
    #v(0.12em)
    #text(size: 10.5pt, style: "italic")[#subtitle]
    #v(0.18em)
    #text(size: 8pt, fill: rgb("555555"))[#instrumentation]
  ]
  v(1em)
}

#block(width: 100%, fill: white, stroke: 0.6pt + luma(215), radius: 8pt, inset: 18pt)[
  #plate-header(
    [Eine kleine Nachtmusik],
    [Serenade in G major, K. 525 · movement I, measures 1–4],
    [Violin I · Violin II · Viola · Violoncello],
  )
  #mozart-k525-opening(
    bar-count: 4,
    scale: 0.42,
    note-spacing: 3.45,
    composer: [W. A. Mozart],
    wrap: true,
  )
]
