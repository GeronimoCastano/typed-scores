#import "../src/lib.typ": score

// Four-bar opening phrase of Beethoven's D-major "Ode to Joy" theme, typed at
// concert pitch and transposed a major sixth upward to written pitch for
// E-flat alto saxophone.
#let ode-to-joy-alto-sax(
  scale: 0.82,
  note-spacing: 3.8,
  composer: [L. van Beethoven],
  wrap: false,
  width: none,
) = score(
  clef: "treble",
  bars: (
    (notes: "F#4:q[dyn=p] F# G A"),
    (notes: "A G F# E"),
    (notes: "D D E F#"),
    (notes: "F#:q. E:e E:h"),
  ),
  key: "D",
  time: "4/4",
  transpose: "M6",
  tempo: [Allegro assai],
  composer: composer,
  scale: scale,
  note-spacing: note-spacing,
  beams: true,
  wrap: wrap,
  width: width,
)
