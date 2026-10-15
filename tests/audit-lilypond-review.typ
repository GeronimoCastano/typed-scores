#import "../src/lib.typ": score, read-score

#set page(width: 190mm, height: auto, margin: 12mm)
#set text(size: 9pt)

= Grace chord and grace-group references

== C07 · Acciaccatura chord before a quarter note

Typed-scores:
#score(time: "4/4", scale: 1.8, bars: ((notes: "c4:q acciaccatura { (d5 e5):e } f5:q r:h"),))

LilyPond 2.26, with the same pitches and durations:
#image("lilypond-ab/reference/audit-grace-chord.svg", width: 100%)

== C29 · A grace note followed by a grace chord

C5 precedes the simultaneous D5/E5 grace chord. F5, G5, A5, and B5
are quarter notes. These are plain grace notes, without an acciaccatura slash.

#let grace-import = read-score("X:1\nM:4/4\nL:1/4\nK:C\n{c}{[de]}f g a b |]\n", format: "abc")
#score(..grace-import.arguments)

LilyPond 2.26:
#image("lilypond-ab/reference/audit-grace-groups.svg", width: 100%)

#pagebreak()
= C02 · Hairpin endpoint and system break

The exact case ends the crescendo on the first note of the second system.
The pedal releases on that measure's fourth note.

#score(clef: "bass", time: "4/4", width: 40, bars: (
  (notes: "c3:q[p1( h1<] d3:q e3:q f3:q"),
  (notes: "g3:q a3:q b3:q c4:q"),
  (notes: "c3:q[h1!] d3:q e3:q f3:q[p1)]"),
  (notes: "g3:q a3:q b3:q c4:q"),
  (notes: "c3:w"),
))

LilyPond 2.26, with the system break fixed after measure 2:
#image("lilypond-ab/reference/audit-spanning-hairpin.svg", width: 100%)

Typed-scores control, with the crescendo endpoint moved to beat 4:
#score(clef: "bass", time: "4/4", width: 40, bars: (
  (notes: "c3:q[p1( h1<] d3:q e3:q f3:q"),
  (notes: "g3:q a3:q b3:q c4:q"),
  (notes: "c3:q d3:q e3:q f3:q[h1! p1)]"),
  (notes: "g3:q a3:q b3:q c4:q"),
  (notes: "c3:w"),
))

#pagebreak()
= C32 · Tuplet spanning a barline

Each written dotted half note occupies a half note under the 3:2 ratio.
The last measure is a half-measure fragment. Timing is retained in the
imported score; its per-measure bracket fragments differ from LilyPond's
single bracket across the barline.

#let tuplet-import = read-score("X:1\nM:4/4\nL:3/4\nK:C\n(3C D | E |]\n", format: "abc")
#score(..tuplet-import.arguments, scale: 1.5)

LilyPond 2.26:
#image("lilypond-ab/reference/audit-cross-bar-tuplet.svg", width: 100%)
