#import "../src/lib.typ": score

// Opening pickup and measures 1–2 of Chopin's Nocturne in E-flat major,
// Op. 9 No. 2, following the Mutopia transcription of the 1881 Schirmer score.
#let chopin-opening(
  scale: 0.62,
  note-spacing: 4.0,
  staff-gap: 13,
  composer: [F. Chopin],
) = score(
  bars: (
    (
      partial: "1/8",
      treble: "Bb4:e[f=1]",
      bass: "r:e",
    ),
    (
      treble: "G5:q.[f=54] ~ G5:e[text=espress._dolce] F5:e[f=3 s1(] G5:e[f=4] F5:q.[f=3] Eb5:q[f=2 s1)] Bb4:e[f=1 s2(]",
      bass: "Eb2:e[stacc p1(] (G3 Eb4):e[sb1(] (Bb3 Eb4 G4):e[sb1) p1)] Eb2:e[stacc p2(] (Ab3 D4):e[sb2(] (Cb4 D4 Ab4):e[sb2) p2)] Eb2:e[stacc p3(] (G3 Eb4):e[sb3(] (Bb3 Eb4 G4):e[sb3) p3)] D2:e[stacc p4(] (G3 Eb4):e[sb4(] (Bb3 Eb4 G4):e[sb4) p4)]",
    ),
    (
      treble: "G5:q[f=5] C5:e[chromatic-turn turn-f=12121 h1<] C6:q[f=5 h1!] G5:e[f=2] Bb5:q.[f=4 h2>] Ab5:q[f=3] G5:e[f=2 s2) h2!]",
      bass: "C2:e[stacc p5(] (G3 E4):e[sb5(] (Bb3 E4 G4):e[sb5) p5)] C3:e[stacc text-below=Ped._simile] (G3 E4):e[sb6(] (C4 E4 Bb4):e[sb6)] F2:e[stacc] (F3 Db4):e[sb7(] (Bb3 Db4 E4):e[sb7)] F2:e[stacc] (F3 C4):e[sb8(] (Ab3 C4 F4):e[sb8)]",
    ),
  ),
  key: "Eb",
  time: "12/8",
  tempo: [Andante (♪ = 132)],
  composer: composer,
  scale: scale,
  note-spacing: note-spacing,
  staff-gap: staff-gap,
  beams: true,
  wrap: false,
)
