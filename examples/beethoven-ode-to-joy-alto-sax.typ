#import "../src/lib.typ": score

// Four-bar opening phrase of Beethoven's D-major "Ode to Joy" theme,
// transposed a major sixth upward to written pitch for E-flat alto saxophone.
#let ode-to-joy-alto-sax(
  scale: 0.82,
  note-spacing: 3.8,
  composer: [L. van Beethoven],
  wrap: false,
  width: none,
) = score(
  clef: "treble",
  bars: (
    (notes: "D#5:q[dyn=p] D#5:q E5:q F#5:q"),
    (notes: "F#5:q E5:q D#5:q C#5:q"),
    (notes: "B4:q B4:q C#5:q D#5:q"),
    (notes: "D#5:q. C#5:e C#5:h"),
  ),
  key: "B",
  time: "4/4",
  tempo: [Allegro assai],
  composer: composer,
  scale: scale,
  note-spacing: note-spacing,
  beams: true,
  wrap: wrap,
  width: width,
)
