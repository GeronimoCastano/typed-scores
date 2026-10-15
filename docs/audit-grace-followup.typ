#import "../src/lib.typ": score

Grace chords keep their seconds displaced while leaving a visible gap before
the principal note. The placement uses the actual displaced-head widths,
stem flags, and principal-event accidentals; rhythmic spacing reserves the
same measured span. This applies to plain and slashed groups, including a
group before the first principal event.

#score(
  time: "4/4",
  scale: 3,
  bars: ((notes: "c4:q acciaccatura { (d5 e5):e } f5:q r:h"),),
)

The tab staff keeps its established grace-number spacing, while notation
staves use head and flag extents to place grace events.
