#import "../src/lib.typ": score

#set page(margin: 1.4cm)
#set text(font: "New Computer Modern", size: 10pt)

= Engraving audit examples

== Stems, tremolos, dots, and grace spacing

#score(time: "4/4", scale: 2.5, bars: ((notes: "g3:q a3:q c3:q e3:q"),))
#score(time: "4/4", scale: 2, bars: ((notes: "(e4 f4 g4 a4 a5):h r:h"),))

#score(time: "4/4", scale: 1.6, bars: ((notes: "tremolo 16 { c5:h g5:h }"),))
#score(time: "4/4", scale: 1.6, bars: ((notes: "tremolo 8 { c5:h g5:h }"),))
#score(time: "4/4", scale: 1.6, bars: ((notes: "tremolo 16 { c5:e g5:e } r:h."),))

#score(time: "4/4", scale: 3, bars: ((notes: "(e4 f4):q. (b4 c5):q. r:q"),))

#score(time: "4/4", scale: 5, bars: ((notes: "c4:q acciaccatura { (d5 e5):e } f5:q r:h"),))

== Spans, slurs, ties, and tuplets

#score(time: "4/4", scale: 2.5, bars: ((notes: "e5:q[stacc s1(] d5:q c5:q b4:q[stacc s1)]"),))

#score(time: "4/4", scale: 2.5, bars: ((notes: "d5:q[stacc fermata] e5:q[marcato fermata] e5:q[accent fermata] r:q"),))

#score(time: "4/4", scale: 2, bars: ((notes: "c5:w[s1(]"), (notes: "d5:w[s1)]")))

#score(width: 40, time: "4/4", bars: ((notes: "(c4 g4):w ~"), (notes: "(c4 g4):w")))

#score(time: "4/4", scale: 2, bars: ((notes: "tuplet 3:2 { r:e r:e r:e } c4:q r:h"),))

== Cross-staff marks, harmony, and barline groups

#score(staves: (upper: (clef: "treble"), lower: (clef: "bass")), time: "4/4", bars: ((upper: "s:w", lower: "C2:q @upper E4:q[dyn=f] G4:h"),))

#score(time: "4/4", bars: ((notes: "a6:h g6:h", harmony: "Am:h G:h"),))

#score(staves: (s: (clef: "treble"), a: (clef: "treble"), t: (clef: "bass"), b: (clef: "bass")), time: "4/4", bars: ((s: "c5:w", a: "g4:w", t: "e3:w", b: "c3:w", barline: (right: "repeat-end")),))

#score(staves: (upper: (clef: "treble"), lower: (clef: "bass")), group: "brace", time: "4/4", bars: ((upper: "c5:w", lower: "c3:w", barline: (right: "dashed")),))

== Tablature unison across a tie

#score(staves: (tab: (clef: "tab", tuning: "guitar")), time: "4/4", bars: ((tab: ("c4:w~", "e4:w")), (tab: ("c4:w", "c4:w")), (tab: ("c4:w", "e4:w"))))

#score(staves: (tab: (clef: "tab", tuning: "guitar")), time: "4/4", bars: ((tab: "grace { e5:s f5:s f#5:s g5:s } b5:w"),))

== Endings, seconds at a system edge, and wrapped direction spans

#score(width: 26, time: "4/4", bars: ((notes: "c5:q d5:q e5:q f5:q", lyrics: "Glo __ __ __"), (notes: "g5:q a5:q b5:q c6:q", lyrics: "__ __ __ __")))

#score(time: "2/4", width: 45, ragged-right: false, bars: ((notes: "c5:q d5:q", barline: (left: "repeat-start")), (ending: (label: "1.", start: true), notes: "e5:q f5:q"), (barline: (right: "repeat-end"), ending: (label: "1.", stop: true), notes: "b5:q b5:q"), (ending: (label: "Final", start: true), notes: "f6:q a5:q"), (ending: (label: "Final", stop: true), notes: "g5:h")), scale: 1.2, wrap: false)

#score(clef: "treble", time: "4/4", scale: 0.85, wrap: false, bars: ((notes: "(C4 E4):h (D4 E4 F4):h"),))

#score(clef: "bass", time: "4/4", width: 24, bars: ((notes: "c3:q[p1( h1<] d3:q e3:q f3:q"), (notes: "g3:q a3:q b3:q c4:q"), (notes: "c3:q[h1!] d3:q e3:q f3:q[p1)]"), (notes: "g3:q a3:q b3:q c4:q"), (notes: "c3:w")))

== Rehearsal marks above high notes and endings

#score(time: "4/4", bars: ((notes: "c6:q[stacc] d6:q[accent] e6:q[marcato] f6:q", rehearsal: "A", navigation: "Fine", ending: (label: "1.", start: true)), (notes: "g6:w", ending: (label: "1.", stop: true))),)

== Cross-staff slurs and cue voices

#score(staves: (upper: (clef: "treble"), lower: (clef: "bass")), time: "2/4", beams: true, width: 23, bars: ((upper: "C5:e[s1(] G4 @lower E3 C3[s1)]", lower: "C2:h"),))

#score(staves: (upper: (clef: "treble"), lower: (clef: "bass")), key: "Cm", time: "2/4", beams: true, scale: 1.2, bars: ((partial: "1/16", upper: ("g4:s", "cue { tuplet 3:2[side=above] { g4:t d5 d } }"), lower: "r:s"),))
