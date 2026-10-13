#import "../src/lib.typ": score

== Chord names and directions around high notes

Chord names, rehearsal marks, and navigation directions clear the highest
notes and ledger lines on the top staff. Repeat-ending brackets also clear
the voices and rehearsal marks in their system.

#score(
  time: "4/4",
  bars: (
    (notes: "a6:h g6:h", harmony: "Am:h G:h"),
  ),
)

Tempo marks use the same high-note clearance as other header text, even when
no chord symbols or ending bracket appears in the system.

#score(
  time: "4/4",
  bars: (
    (notes: "a6:h g6:h", tempo: "Allegro"),
    (notes: "a6:h g6:h", harmony: "Am:h G:h", rehearsal: "A", tempo: "Moderato"),
  ),
)

== Alternating tremolo subdivisions

In an alternating tremolo, note durations determine how much score time the
two written events occupy. The subdivision number determines the repeat rate
and stroke count: 8 draws one stroke, 16 draws two, 32 draws three, and 64
draws four. For example, `tremolo 16 { c5:q g5:q }` occupies a half note and
shows two strokes.

For a single-note tremolo, the subdivision strokes replace the note's ordinary
flags. An eighth note with subdivision 16 therefore has two total stem
strokes, and a sixteenth note with subdivision 32 has three.

LilyPond's `\repeat tremolo N` uses `N` as the number of repetitions. The
repetition count and the written note durations together set the total time;
the number is not itself the stroke-count label. A four-repeat pair of
sixteenths and an eight-repeat pair of thirty-seconds each occupy a half note,
but show two and three strokes respectively.

#score(
  time: "4/4",
  bars: (
    (notes: "tremolo 16 { c5:q g5:q } tremolo 32 { d5:q a4:q }"),
  ),
)

== Hairpins and pedal spans across wrapped systems

Pedal lines and hairpins continue through a system break. A continuation
keeps the same hairpin opening at the next system, and pedal marks appear at
the span's original start.

#score(
  clef: "bass",
  time: "4/4",
  width: 40,
  bars: (
    (notes: "c3:q[p1( h1<] d3:q e3:q f3:q"),
    (notes: "g3:q a3:q b3:q c4:q"),
    (notes: "c3:q[h1!] d3:q e3:q f3:q[p1)]"),
    (notes: "g3:q a3:q b3:q c4:q"),
    (notes: "c3:w"),
  ),
)
