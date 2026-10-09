// Fantaisie-Impromptu in C-sharp minor, Op. 66, by F. Chopin: bars 5-8,
// the opening of the Allegro section after the four-bar introduction.
// The tie from bar 8 into bar 9 is omitted because bar 9 is outside the excerpt.
//
// Translated from the Mutopia Project LilyPond source
// (https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1693), edition by
// Herrmann Scholtz (1845-1918). Licence: Public Domain, as stated by Mutopia.
// The LilyPond reference is examples/lilypond-reference/chopin-fantaisie-impromptu-op66.ly.
//
// typst compile --root . --ppi 300 assets/readme/examples/chopin-fantaisie-impromptu.typ assets/readme/examples/chopin-fantaisie-impromptu.png

#import "../../../src/lib.typ": *

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
    [Fantaisie-Impromptu],
    [C-sharp minor, Op. 66 · measures 5–8],
    [Piano],
  )
  #score(
    staves: (
      upper: (clef: "treble"),
      lower: (clef: "bass"),
    ),
    key: "C#m",
    time: "4/4",
    tempo: [Allegro agitato],
    composer: [F. Chopin],
    theme: "light",
    beams: true,
    scale: 0.36,
    wrap: true,
    bars: (
    (
      upper: "r:s g#4:s[dyn=p s1(] a4:s g#4:s f##4:s g#4:s c#5:s e5:s d#5:s c#5:s d#5:s c#5:s b#4:s c#5:s e5:s g#5:s[s1)]",
      lower: "tuplet 6:4 { c#3:e[s8( p1(] g#3:e c#4:e e4:e c#4:e g#3:e[s8)] } tuplet 6:4 { e3:e[s9(] g#3:e c#4:e e4:e c#4:e g#3:e[s9) p1)] }",
    ),
    (
      upper: "r:s g#4:s[s2(] a4:s g#4:s f##4:s g#4:s c#5:s e5:s d#5:s c#5:s d#5:s c#5:s b#4:s c#5:s e5:s g#5:s[s2)]",
      lower: "tuplet 6:4 { c#3:e[s10( p2(] g#3:e c#4:e e4:e c#4:e g#3:e[s10)] } tuplet 6:4 { e3:e[s11(] g#3:e c#4:e e4:e c#4:e g#3:e[s11) p2)] }",
    ),
    (
      upper: "r:s a4:s[s3( h1<] c#5:s d#5:s f#5:s a5:s c#6:s d#6:s[h1!] b6:s[8va( h2>] a6:s g#6:s f#6:s e6:s d#6:s f#6:s c#6:s[s3) h2! 8va)]",
      lower: "tuplet 6:4 { d#3:e[s12( p3(] a3:e c#4:e f#4:e c#4:e a3:e[s12)] } tuplet 6:4 { f#3:e[s13(] c#4:e d#4:e a4:e d#4:e c#4:e[s13) p3)] }",
    ),
    (
      upper: "b#5:s[s4(] d#6:s a5:s g#5:s f#5:s a5:s e5:s d#5:s[s4)] f#5:s[s5(] c#5:s b#4:s d#5:s a4:s g#4:s b4:s a4:s[s5)]",
      lower: "tuplet 6:4 { g#2:e[s14( p4(] d#3:e f#3:e b#3:e f#3:e d#3:e } tuplet 6:4 { g#2:e d#3:e f#3:e b#3:e f#3:e d#3:e[s14) p4)] }",
    ),
    ),
  )
]
