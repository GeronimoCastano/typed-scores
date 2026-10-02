// Imported from export-quirks.musicxml by typed-scores.
// Import note: parts with different key signatures share the first part's key.
#import "../../src/lib.typ": score

#align(center, text(size: 1.6em, weight: "bold", "Export Quirks"))

#score(
  staves: (
    violin: (clef: "treble", label: "Violín"),
    clarinet-in-b: (clef: "treble", label: "Clarinet in B♭", short-label: "Cl."),
  ),
  key: "D",
  time: "4/4",
  beams: true,
  scale: 0.7,
  bars: (
    // m. 1
    (
      rehearsal: "A",
      navigation: "segno",
      harmony: "D/F#:w",
      violin: ("D5:s E5 F#5:e tuplet 5:4 { G5:s A5 G5 F#5 E5 } D5:q. C#5:t B4 A4:s", "s:h A4:h ~"),
      clarinet-in-b: "_",
      lyrics: (violin: ("One ti -- me _ _ _ _ _ _ _ _ _", "Two by two _ _ _ _ _ _ _ _ _"),),
    ),
    // m. 2
    (
      time: "3/4",
      tempo: (text: "poco più mosso", beat: "quarter", bpm: 96),
      violin: ("grace { (E5 G5):s F#5 } D5:q[accent trill] cue { B4:e C#5 } D5:q[tremolo=32 f=3 text-below=dolce]", "A4:q s:h"),
      clarinet-in-b: "E5:h.[s1(]",
      lyrics: (violin: ("go‿on _ _ _", "_ _ _ _"),),
    ),
    // m. 3
    (
      key: "Cm",
      barline: (right: "double"),
      violin: ("C5:q[dyn=sfz] (Eb5 G5) (Eb5 Ab5)[text=D.C._al_Fine]", "s:h."),
      clarinet-in-b: "D5:q[s1)] r:h",
    ),
  ),
)
