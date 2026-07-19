#import "../src/lib.typ": score

// Measures 1–10 of the first movement of Mozart's Serenade in G major,
// K. 525, translated from the public-domain Mutopia LilyPond full score.
// staff-gap: none lets the engine size the gaps from the actual ink, which
// this excerpt needs: the forte marks hang below down-stem chords.
#let mozart-k525-opening(
  scale: 0.46,
  note-spacing: 3.45,
  staff-gap: none,
  composer: [W. A. Mozart],
  bar-count: 10,
  wrap: false,
  width: none,
) = score(
  staves: (
    violin-one: (clef: "treble", label: "Violin I", short-label: "Vln. I"),
    violin-two: (clef: "treble", label: "Violin II", short-label: "Vln. II"),
    viola: (clef: "alto", label: "Viola", short-label: "Vla."),
    cello: (clef: "bass", label: "Violoncello", short-label: "Vc."),
  ),
  bars: (
    (
      violin-one: "(B4 D5 G5):q[dyn=f] r:e D5:e G5:q r:e D5:e",
      violin-two: "(B4 D5 G5):q[dyn=f] r:e D5:e G5:q r:e D5:e",
      viola: "G4:q[dyn=f] r:e D4:e G4:q r:e D4:e",
      cello: "G3:q[dyn=f] r:e D3:e G3:q r:e D3:e",
    ),
    (
      violin-one: "G5:e D5:e G5:e B5:e D6:q r:q",
      violin-two: "G5:e D5:e G5:e B5:e D6:q r:q",
      viola: "G4:e D4:e G4:e B4:e D5:q r:q",
      cello: "G3:e D3:e G3:e B3:e D4:q r:q",
    ),
    (
      violin-one: "C6:q r:e A5:e C6:q r:e A5:e",
      violin-two: "C6:q r:e A5:e C6:q r:e A5:e",
      viola: "C5:q r:e A4:e C5:q r:e A4:e",
      cello: "C4:q r:e A3:e C4:q r:e A3:e",
    ),
    (
      violin-one: "C6:e A5:e F#5:e A5:e D5:q r:q",
      violin-two: "C6:e A5:e F#5:e A5:e D5:q r:q",
      viola: "C5:e A4:e F#4:e A4:e D4:q r:q",
      cello: "C4:e A3:e F#3:e A3:e D3:q r:q",
    ),
    (
      violin-one: "(B4 D5 G5):e r:e G5:q. B5:e[s1(] A5:e[s1)] G5:e[stacc]",
      violin-two: "(B3 D4):h[tremolo=16] (B3 D4):h[tremolo=16]",
      viola: "G4:e G4 G4 G4 G4 G4 G4 G4",
      cello: "G3:e G3 G3 G3 G3 G3 G3 G3",
    ),
    (
      violin-one: "A5:t - G5 - A5 - G5 - F#5:e F#5:q. A5:e[s2(] C6:e[s2)] F#5:e[stacc]",
      violin-two: "(C4 D4):h[tremolo=16] (C4 D4):h[tremolo=16]",
      viola: "A4:e A4 A4 A4 A4[s1(] C5[s1)] F#4[s2(] A4[s2)]",
      cello: "G3:e G3 G3 G3 G3 G3 G3 G3",
    ),
    (
      violin-one: "A5:e[s3(] G5:e[s3)] G5:q. B5:e[s4(] A5:e[s4)] G5:e[stacc]",
      violin-two: "(B3 D4):h[tremolo=16] (B3 D4):h[tremolo=16]",
      viola: "G4:e G4 G4 G4 G4 G4 G4 G4",
      cello: "G3:e G3 G3 G3 G3 G3 G3 G3",
    ),
    (
      violin-one: "A5:t - G5 - A5 - G5 - F#5:e F#5:q. A5:e[s5(] C6:e[s5)] F#5:e[stacc]",
      violin-two: "(C4 D4):h[tremolo=16] (C4 D4):h[tremolo=16]",
      viola: "A4:e A4 A4 A4 A4[s3(] C5[s3)] F#4[s4(] A4[s4)]",
      cello: "G3:e G3 G3 G3 G3 G3 G3 G3",
    ),
    (
      violin-one: "G5:e[stacc] G5:e[stacc] appoggiatura { G5:s } F#5:e[s6(] E5:s F#5:s[s6)] G5:e[stacc] G5:e[stacc] appoggiatura { B5:s } A5:e[s7(] G5:s A5:s[s7)]",
      violin-two: "(B3 D4):q C5:e C5 D5 D5 C5:e[s8(] B4:s A4:s[s8)]",
      viola: "D4:h[tremolo=16] D4:h[tremolo=16]",
      cello: "G3:e G3 A3 A3 B3 B3 F#3 F#3",
    ),
    (
      violin-one: "B5:e[stacc] B5:e[stacc] appoggiatura { D6:s } C6:e[s9(] B5:s C6:s[s9)] D6:q r:q",
      violin-two: "G4:e G4 F#4 F#4 G4:q r:q",
      viola: "D4:h[tremolo=16] D4:q r:q",
      cello: "G3:e G3 A3 A3 B3:q r:q",
      barline: (right: "final"),
    ),
  ).slice(0, bar-count),
  key: "G",
  time: "4/4",
  tempo: [Allegro],
  composer: composer,
  scale: scale,
  note-spacing: note-spacing,
  staff-gap: staff-gap,
  group: "bracket",
  beams: true,
  wrap: wrap,
  width: width,
  indent: 2.5,
  short-indent: 0,
  ragged-right: false,
  ragged-last: false,
)
