== Import preservation example

#import "../src/lib.typ": score, read-score

ABC bar accidentals carry to every octave of the same pitch through the bar.
Lyrics consume one syllable per note, including notes joined by ties. Import
warnings identify source notation that typed-scores cannot reproduce. Use
`read-score` to inspect those warnings and review the editable score arguments.

#let imported-example = read-score("X:1\nM:4/4\nL:1/4\nK:C\n^C c C c |]\n", format: "abc")
#score(..imported-example.arguments)

Fermatas on rests are retained. Grace decorations that cannot be engraved are
reported as warnings and do not become extra grace pitches.
