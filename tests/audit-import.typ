#import "../src/lib.typ": score, read-score

#let imported-safe-name = read-score("X:1\nK:C\nC4|]\n", format: "abc", name: "audit\rname.abc")
#assert(imported-safe-name.source.contains("Source score: audit name.abc."))

= Importer corrections

ABC bar accidentals carry across octaves, and lyrics stay aligned to every
note in a tie. These scores come from the ABC importer.

#let imported-accidentals = read-score("X:1\nM:4/4\nL:1/4\nK:C\n^C c C c |]\n", format: "abc")
#score(..imported-accidentals.arguments)

#let imported-lyrics = read-score("X:1\nM:4/4\nL:1/4\nK:C\nc-c d e |\nw: la- la do re\n", format: "abc")
#score(..imported-lyrics.arguments)

ABC lyric extenders stop at their explicit target before a skipped note.

#let imported-extender = read-score("X:1\nM:4/4\nL:1/4\nK:C\nc c c c |]\nw: Ho _ * ly\n", format: "abc")
#score(..imported-extender.arguments)

Grace decorations do not turn their letters into extra grace notes. The
importer reports that the grace decoration is unsupported while retaining the
single grace pitch.

#let imported-grace = read-score("X:1\nM:4/4\nL:1/4\nK:C\n{!accent!c}d e f g |]\n", format: "abc")
#score(..imported-grace.arguments)

A fermata attached to an ABC rest remains attached to the imported rest.

#let imported-fermata = read-score("X:1\nM:4/4\nL:1/4\nK:C\n!fermata!z C D E |]\n", format: "abc")
#score(..imported-fermata.arguments)
