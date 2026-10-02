// Source score: folk-duet.abc.
#import "../../src/lib.typ": score

#align(center, text(size: 1.6em, weight: "bold", "Hill Song"))

#score(
  staves: (
    flute: (clef: "treble", label: "Flute", short-label: "Fl."),
    cello: (clef: "bass", label: "Cello", short-label: "Vc."),
  ),
  key: "G",
  time: "6/8",
  tempo: "Allegretto",
  composer: "Importer Fixture",
  beams: true,
  scale: 0.7,
  bars: (
    // m. 1
    (
      partial: "1/8",
      flute: "D5:e",
      cello: ("r:e", "s:e"),
      lyrics: (flute: "Oh",),
    ),
    // m. 2
    (
      barline: (left: "repeat-start"),
      harmony: "G:h.",
      flute: "B4:q[dyn=p s1(] A4:e[s1)] G4:q B4:e",
      cello: ("G3:q. D3", "s:h."),
      lyrics: (flute: "the sun -- ny morn --",),
    ),
    // m. 3
    (
      harmony: "D7:h.",
      flute: "A4:q[h1<] C#5:e D5:q C5:e[h1!]",
      cello: ("D3:h.", "r:q. F#3"),
      lyrics: (flute: "ing bright __ and",),
    ),
    // m. 4
    (
      harmony: "G:h.",
      flute: "acciaccatura { C5:e } B4:q. ~ B4:q D5:e",
      cello: ("_", "s:h."),
      lyrics: (flute: "clear _ a --",),
    ),
    // m. 5
    (
      harmony: "C:h.",
      flute: "tuplet 3:2 { E5:e F#5 G5 } / E5:e[stacc] D5:q.[trill]",
      cello: ("_", "s:h."),
      lyrics: (flute: "bove the hill so fine",),
    ),
    // m. 6
    (
      barline: (right: "repeat-end"),
      ending: (label: "1.", start: true, stop: true),
      harmony: "G:h.",
      flute: "G4:q. G4:q[fermata] D4:e",
      cello: ("G3:q. D3", "s:h."),
      lyrics: (flute: "and still my",),
    ),
    // m. 7
    (
      key: "D",
      barline: (right: "final"),
      ending: (label: "2.", start: true, stop: true),
      harmony: "D:h.",
      flute: "D5:h.",
      cello: ("D3:h.", "s:h."),
      lyrics: (flute: "home.",),
    ),
  ),
)
