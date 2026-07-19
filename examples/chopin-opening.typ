#import "../src/lib.typ": score

// Opening pickup and measures 1–8 of Chopin's Nocturne in E-flat major,
// Op. 9 No. 2, translated from the 1881 Schirmer LilyPond score distributed
// by Mutopia.
#let _chopin-opening-bars = (
  (
    partial: "1/8",
    upper: "Bb4:e[f=1]",
    lower: "r:e",
  ),
  (
    upper: "G5:q.[f=54] ~ G5:e[text=espress._dolce] F5:e[f=3 s1(] G5:e[f=4] F5:q.[f=3] Eb5:q[f=2 s1)] Bb4:e[f=1 s2(]",
    lower: "Eb2:e[stacc p1(] (G3 Eb4):e[sb1(] (Bb3 Eb4 G4):e[sb1) p1)] Eb2:e[stacc p2(] (Ab3 D4):e[sb2(] (Cb4 D4 Ab4):e[sb2) p2)] Eb2:e[stacc p3(] (G3 Eb4):e[sb3(] (Bb3 Eb4 G4):e[sb3) p3)] D2:e[stacc p4(] (G3 Eb4):e[sb4(] (Bb3 Eb4 G4):e[sb4) p4)]",
  ),
  (
    upper: "G5:q[f=5] C5:e[chromatic-turn turn-f=12121 h1<] C6:q[f=5 h1!] G5:e[f=2] Bb5:q.[f=4 h2>] Ab5:q[f=3] G5:e[f=2 s2) h2!]",
    lower: "C2:e[stacc p5(] (G3 E4):e[sb5(] (Bb3 E4 G4):e[sb5) p5)] C3:e[stacc text-below=Ped._simile] (G3 E4):e[sb6(] (C4 E4 Bb4):e[sb6)] F2:e[stacc] (F3 Db4):e[sb7(] (Bb3 Db4 E4):e[sb7)] F2:e[stacc] (F3 C4):e[sb8(] (Ab3 C4 F4):e[sb8)]",
  ),
  (
    upper: "F5:q.[f=1] G5:q[f=3 s3(] D5:e[f=1 s3)] Eb5:q.[f=2] C5:q.[f=1]",
    lower: "Bb1:e[stacc p9(] (F3 D4):e[sb9(] (Bb3 D4 Ab4):e[sb9) p9)] B1:e[stacc p10(] (G3 F4):e[sb10(] (D4 F4 G4):e[sb10) p10)] C2:e[stacc p11(] (G3 Eb4):e[sb11(] (C4 Eb4 G4):e[sb11) p11)] A1:e[stacc p12(] (Gb3 Eb4):e[sb12(] (C4 Eb4 Gb4):e[sb12) p12)]",
  ),
  (
    upper: "Bb4:e[f=1 dyn=f s4(] D6:e[f=5 s4)] C6:e[f=4 s5(] Bb5:s[f=3] Ab5:s G5:s Ab5:s[f=4 s5)] C5:s[s6(] D5:s Eb5:q.[s6)] r:q Bb4:e[f=1]",
    lower: "Bb1:e[stacc p13(] (F3 Eb4):e[sb13(] (Bb3 Eb4 Ab4):e[sb13) p13)] Bb1:e[stacc p14(] (F3 D4):e[sb14(] (Bb3 Ab4):e[sb14) p14)] Eb2:e[stacc p15(] (G3 Eb4):e[sb15(] (Bb3 Eb4 G4):e[sb15) p15)] Eb2:e[stacc] (G3 Eb4):e[sb16(] (Bb3 Eb4 G4):e[sb16)]",
  ),
  (
    upper: "G5:q.[dyn=p f=54 s17(] F5:s[f=3 s17)] G5:s[s18(] F5:s E5:s F5:s G5:s F5:e[stacc f=3 s18)] Eb5:q[accent f=2] ~ Eb5:s[s19(] F5:s Eb5:s D5:s Eb5:s F5:s",
    lower: "Eb2:e[stacc p17(] (G3 Eb4):e[sb17(] (Bb3 Eb4 G4):e[sb17) p17)] Eb2:e[stacc p18(] (Ab3 D4):e[sb18(] (Cb4 D4 Ab4):e[sb18) p18)] Eb2:e[stacc p19(] (G3 Eb4):e[sb19(] (Bb3 Eb4 G4):e[sb19) p19)] D2:e[stacc p20(] (G3 Eb4):e[sb20(] (Bb3 Eb4 G4):e[sb20) p20)]",
  ),
  (
    upper: "G5:s[f=4 s19) h3<] B4:s[s20(] C5:s[s20)] Db5:s[accent f=3] C5:s[f=1] F5:s[accent f=2] E5:s[f=1] Ab5:s[accent f=3] G5:s[f=1] Db6:s[f=4] C6:s G5:s[h3!] Bb5:q.[f=3 h4>] Ab5:q[f=2 h4!] G5:e[f=1]",
    lower: "C2:e[stacc p21(] (G3 E4):e[sb21(] (Bb3 E4 G4):e[sb21) p21)] C3:e[stacc] (G3 E4):e[sb22(] (C4 E4 Bb4):e[sb22)] F2:e[stacc p22(] (F3 Db4):e[sb23(] (Bb3 Db4 E4):e[sb23) p22)] F2:e[stacc] (F3 C4):e[sb24(] (Ab3 C4 F4):e[sb24)]",
  ),
  (
    upper: "F5:q.[f=23 turn] appoggiatura { E5:s F5:s } G5:e[stacc f=3] G5:e[f=4 s21(] D5:e[f=1 s21)] Eb5:q.[f=2] C5:q.[f=1]",
    lower: "Bb1:e[stacc p23(] (F3 D4):e[sb25(] (Bb3 D4 Ab4):e[sb25) p23)] B1:e[stacc p24(] (G3 F4):e[sb26(] (D4 F4 G4):e[sb26) p24)] C2:e[stacc p25(] (G3 Eb4):e[sb27(] (C4 Eb4 G4):e[sb27) p25)] A1:e[stacc p26(] (Gb3 Eb4):e[sb28(] (C4 Eb4 Gb4):e[sb28) p26)]",
  ),
  (
    upper: "Bb4:e[dyn=f f=1 s22(] D6:e[stacc f=5 s22)] C6:e[stacc f=4 h5< s23(] Bb5:s[stacc f=3] Ab5:s[stacc] G5:s[stacc f=1] Ab5:s[stacc] appoggiatura { Ab5:e } C5:s[f=1] D5:s[h5! s23)] Eb5:q.[f=3] ~ Eb5:e D5:e[f=2] Eb5:e",
    lower: "Bb1:e[stacc p27(] (F3 Eb4):e[sb29(] (Bb3 Eb4 Ab4):e[sb29) p27)] Bb1:e[stacc p28(] (F3 D4):e[sb30(] (Bb3 Ab4):e[sb30) p28)] Eb2:e[stacc p29(] (G3 Eb4):e[sb31(] (Bb3 Eb4 G4):e[sb31) p29)] Eb2:e[stacc] (G3 Eb4):e[sb32(] (Bb3 Eb4 G4):e[sb32)]",
    barline: (right: "final"),
  ),
)

#let chopin-opening(
  scale: 0.42,
  note-spacing: 4.0,
  // staff-gap: none computes each gap from the actual ink, keeping the
  // treble hairpins clear of the bass-staff slur arches.
  staff-gap: none,
  composer: [F. Chopin],
  bar-count: 8,
  wrap: false,
  width: none,
) = score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  bars: _chopin-opening-bars.slice(0, bar-count + 1),
  key: "Eb",
  time: "12/8",
  tempo: (text: [Andante], beat: "eighth", bpm: 132),
  composer: composer,
  scale: scale,
  note-spacing: note-spacing,
  staff-gap: staff-gap,
  beams: true,
  wrap: wrap,
  width: width,
  indent: 2.5,
  short-indent: 0,
  ragged-right: false,
  ragged-last: false,
)
