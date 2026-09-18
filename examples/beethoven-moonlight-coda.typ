#import "../src/lib.typ": score

// Measures 178–185 of the Presto agitato from Beethoven's Piano Sonata No. 14
// in C-sharp minor, Op. 27 No. 2, translated from the Mutopia LilyPond
// edition (Chris Sawer and Stewart Holmes, after Berners 1908, CC BY-SA 2.5).
// The right hand's arpeggios fall onto the bass staff and climb back, so the
// passage exercises staff switches, cross-staff slurs, and kneed beams.
#let _moonlight-coda-bars = (
  (
    upper: "tuplet 3:2 { A5:e[dyn=f s1(] F#5 C#5 } tuplet 3:2[number=never] { A4 F#4 C#4[s1)] } tuplet 3:2[number=never] { A4[s2(] F#4 C#4 } tuplet 3:2[number=never] { @lower A3 F#3 C#3[s2)] }",
    lower: "(F#1 C#2 F#2):w ~",
  ),
  (
    upper: "@lower A3:s[s3(] F#3 C#3 A2 C#3 F#3 A3 C#4 @upper F#4 A4 C#5 F#5 A5 F#5 C#5 A4[s3)]",
    lower: "(F#1 C#2 F#2):w",
  ),
  (
    upper: "tuplet 3:2[number=never] { A5:e[dyn=f s4(] F#5 D5 } tuplet 3:2[number=never] { A4 F#4 D4[s4)] } tuplet 3:2[number=never] { A4[s5(] F#4 D4 } tuplet 3:2[number=never] { @lower A3 F#3 D3[s5)] }",
    lower: "(F#1 D2 F#2):w ~",
  ),
  (
    upper: "@lower A3:s[s6(] F#3 D3 A2 D3 F#3 A3 D4 @upper F#4 A4 D5 F#5 A5 F#5 D5 A4[s6)]",
    lower: "(F#1 D2 F#2):w",
  ),
  (
    upper: "tuplet 3:2[number=never] { C#6:e[dyn=f s7(] A#5 F##5 } tuplet 3:2[number=never] { E5 C#5 A#4[s7)] } tuplet 3:2[number=never] { E5[s8(] C#5 A#4 } F##4:s E4 C#4 A#3[s8)]",
    lower: "(F##1 C#2 F##2):w ~",
  ),
  (
    upper: "E4:s[s9(] C#4 @lower A#3 F##3 E3 C#3 E3 F##3 tuplet 6:4 { A#3 @upper C#4 E4 F##4 A#4 C#5 } tuplet 6:4 { E5 F##5 A#5 C#6 A#5 E5[s9)] }",
    lower: "(F##1 C#2 F##2):w",
  ),
  (
    upper: "tuplet 3:2[number=never] { E6:e[s10(] C#6 G#5 } tuplet 3:2[number=never] { E5 C#5 G#4 } tuplet 3:2[number=never] { E5 C#5 G#4 } E4:s C#5 G#4 E4",
    lower: "(G#1 C#2 G#2):w ~",
  ),
  (
    upper: "C#4:s G#4 E4 C#4 @lower G#3 E4 C#4 G#3 E3 C#4 G#3 E3 C#3 G#3 E3 C#3[s10)]",
    lower: "(G#1 C#2 G#2):w",
  ),
)

#let moonlight-coda(
  scale: 0.5,
  note-spacing: 3.1,
  theme: "auto",
  composer: [L. van Beethoven],
  wrap: true,
  width: none,
) = score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  bars: _moonlight-coda-bars,
  key: "C#m",
  time: "4/4",
  composer: composer,
  theme: theme,
  scale: scale,
  note-spacing: note-spacing,
  beams: true,
  wrap: wrap,
  width: width,
  first-bar-number: 178,
  bar-numbers: "systems",
)
