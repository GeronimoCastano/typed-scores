#import "../src/lib.typ": score

// Opening pickup and measures 1–4 of Beethoven's Bagatelle in A minor,
// WoO 59, following the first published version represented by Mutopia.
#let fur-elise-opening(
  scale: 0.68,
  note-spacing: 3.65,
  staff-gap: 10.5,
  composer: [L. van Beethoven],
  wrap: false,
  width: none,
) = score(
  staves: (
    upper: (clef: "treble"),
    lower: (clef: "bass"),
  ),
  bars: (
    (
      partial: "1/8",
      upper: "E5:s[dyn=pp s1(] D#",
      lower: "r:e",
    ),
    (
      upper: "E D# - E / B4 - D C[s1)]",
      lower: "r:q.",
    ),
    (
      upper: "A4:e r:s C4 - E A4",
      lower: "A2:s[p1(] E3 - A[p1)] r r:e",
    ),
    (
      upper: "B4:e r:s E4 - G# B4",
      lower: "E2:s[p2(] E3 - G#[p2)] r r:e",
    ),
    (
      upper: "C5:e r:s E4 - E5 D#",
      lower: "A2:s[p3(] E3 - A[p3)] r r:e",
    ),
  ),
  key: "Am",
  time: "3/8",
  tempo: [Poco moto],
  composer: composer,
  scale: scale,
  note-spacing: note-spacing,
  staff-gap: staff-gap,
  beams: true,
  wrap: wrap,
  width: width,
)
