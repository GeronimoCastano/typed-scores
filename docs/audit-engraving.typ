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
