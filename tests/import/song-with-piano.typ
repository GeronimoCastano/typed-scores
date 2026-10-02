// Imported from song-with-piano.musicxml by typed-scores.
#import "../../src/lib.typ": score

#align(center, text(size: 1.6em, weight: "bold", "Morning Song"))

#score(
  staves: (
    voice: (clef: "treble", label: "Voice", short-label: "V."),
    piano-upper: (clef: "treble", label: "Piano", short-label: "Pno."),
    piano-lower: (clef: "bass"),
  ),
  key: "F",
  time: "3/4",
  tempo: (text: "Andante", beat: "quarter", bpm: 72),
  composer: "Importer Fixture",
  beams: true,
  scale: 0.7,
  bars: (
    // m. 0
    (
      partial: "1/4",
      voice: "C5:q[dyn=mp]",
      piano-upper: ("r:q", "s:q"),
      piano-lower: "F3:q",
      lyrics: (voice: "The",),
    ),
    // m. 1
    (
      harmony: "F:h C7:q",
      voice: "F5:e[s1(] G5[s1)] tuplet 3:2[bracket=never] { A5:e G5 F5 } E5:q[tenuto]",
      piano-upper: ("(A4 C5 F5):h. ~", "s:h."),
      piano-lower: "F2:e[p1(] C3 - A3 @piano-upper C4 F4:q[p1)]",
      lyrics: (voice: "morn -- ing light __ __ breaks",),
    ),
    // m. 2
    (
      barline: (left: "repeat-start"),
      voice: "acciaccatura { D5:e } C5:q[h1<] D5 E5[fermata h1! dyn=f]",
      piano-upper: ("(A4 C5 F5):q r (Bb4 D5)[stacc arpeggio]", "F4:h r:q"),
      piano-lower: "Bb2:h.",
      lyrics: (voice: "sing a song",),
    ),
    // m. 3
    (
      clef: (piano-lower: "treble",),
      barline: (right: "repeat-end"),
      ending: (label: "1.", start: true, stop: true),
      voice: "F5:h s:q",
      piano-upper: ("(A4 C5):h r:q", "s:h."),
      piano-lower: "C4:h.",
      lyrics: (voice: "now",),
    ),
    // m. 4
    (
      clef: (piano-lower: "bass",),
      barline: (right: "final"),
      ending: (label: "2.", start: true, stop: true),
      voice: "F5:h.",
      piano-upper: ("_", "s:h."),
      piano-lower: "F2:h.",
      lyrics: (voice: "home.",),
    ),
  ),
)
