#import "../src/lib.typ": score
#import "../examples/beethoven-fur-elise.typ": fur-elise-opening

#set page(width: 150mm, height: 120mm, margin: 8mm)
#set text(size: 9pt)

= C18 · default groups in short and irregular meters

#score(time: "3/8", beams: true, bars: ((notes: "c5:e d5:e e5:e"),))
#score(time: "4/8", beams: true, bars: ((notes: "c5:e d5:e e5:e f5:e"),))
#score(time: "5/8", beams: true, bars: ((notes: "c5:e d5:e e5:e f5:e g5:e"),))
#score(time: "7/16", beams: true, bars: ((notes: "c5:s d5:s e5:s f5:s g5:s a5:s b5:s"),))

= C43 · nested tuplets and duration-based spacing

#score(time: "4/4", bars: ((notes: "tuplet 3:2 { tuplet 3:2 { c5:e d5:e e5:e } tuplet 3:2 { f5:e g5:e a5:e } } tuplet 6:4 { c5:s d5:s e5:s f5:s } c5:h"),))
#score(time: "4/4", bars: ((notes: "c5:h d5:h"),))

= C46 · ottava continuation on one event

#score(time: "4/4", bars: ((notes: "c6:q[8va(] d e[8va) 8va(] f[8va)]"),))

= C52 · Fur Elise opening in the documented octave

#fur-elise-opening(scale: 0.55, note-spacing: 3.4, staff-gap: 8)
