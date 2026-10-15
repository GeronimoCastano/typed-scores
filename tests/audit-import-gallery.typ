#import "../src/lib.typ": score, read-score

#let panel(id, title, caption, imported) = block(
  width: 100%,
  inset: 8pt,
  radius: 3pt,
  stroke: 0.7pt + luma(205),
  [
    #text(size: 11pt, weight: "bold")[#id · #title]
    #v(2pt)
    #text(size: 8pt, fill: luma(80))[#caption]
    #v(5pt)
    #score(..imported.arguments)
    #if imported.warnings.len() > 0 [
      #v(3pt)
      #text(size: 7pt, fill: rgb("#8a3b00"))[
        Import warning: #imported.warnings.join("; ")
      ]
    ]
  ],
)

#let render_c13() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Flute</part-name></score-part><score-part id=\"P2\"><part-name>Cello</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type></note></measure><measure number=\"2\"><note><pitch><step>D</step><octave>5</octave></pitch><duration>4</duration><type>whole</type></note></measure></part><part id=\"P2\"><measure number=\"1\"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>4</duration><type>whole</type></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C13",
    "Parts with unequal lengths",
    "The flute has two measures; the cello ends after measure 1. The importer fills the absent cello slot with a whole-rest placeholder and keeps the score valid.",
    imported,
  )
}

#let render_c29() = {
  let source = "X:1\nM:4/4\nL:1/4\nK:C\n{c}{[de]}f g a b |]\n"
  let imported = read-score(source, format: "abc")
  panel(
    "C29",
    "Consecutive grace groups and a grace chord",
    "Both groups are kept before the principal note, including the chord inside the second group.",
    imported,
  )
}

#let render_c31() = {
  let source = "X:1\nM:4/4\nL:1/4\nK:C\n!f!Z2 | \"Am\"Z2 | c d e f|]\n"
  let imported = read-score(source, format: "abc")
  panel(
    "C31",
    "Multi-measure rests keep their marks and timing",
    "The dynamic stays with the first rest and Am lands after the two-bar rest span.",
    imported,
  )
}

#let render_c32() = {
  let tuplet-source = "X:1\nM:4/4\nL:3/4\nK:C\n(3C D | E |]\n"
  let tuplet-import = read-score(tuplet-source, format: "abc")
  let musicxml-import = read-score(read("import/cross-measure-tuplet.musicxml"), format: "musicxml")
  let broken-source = "X:1\nM:4/4\nL:1/8\nK:C\nC3>D E4 |]\n"
  let broken-import = read-score(broken-source, format: "abc")
  block(
    width: 100%,
    inset: 8pt,
    radius: 3pt,
    stroke: 0.7pt + luma(205),
    [
      #text(size: 11pt, weight: "bold")[C32 · Tuplets across bars and unsupported broken rhythm]
      #v(2pt)
      #text(size: 8pt, fill: luma(80))[
        ABC and MusicXML keep one bracket and numeral across the barline. The unwritable broken-rhythm pair remains unchanged and exposes its import warning.
      ]
      #v(5pt)
      #score(..tuplet-import.arguments)
      #v(3pt)
      #score(..musicxml-import.arguments)
      #v(3pt)
      #score(..broken-import.arguments)
      #v(3pt)
      #text(size: 7pt, fill: rgb("#8a3b00"))[
        Import warning: #broken-import.warnings.join("; ")
      ]
    ],
  )
}

#let render_c36() = {
  let octave-source = "X:1\nM:4/4\nL:1/4\nK:C clef=treble-8\nC D E F|]\n"
  let octave-import = read-score(octave-source, format: "abc")
  let percussion-source = "X:1\nM:4/4\nL:1/4\nK:C clef=percussion\nC D E F|]\n"
  let percussion-import = read-score(percussion-source, format: "abc")
  block(
    width: 100%,
    inset: 8pt,
    radius: 3pt,
    stroke: 0.7pt + luma(205),
    [
      #text(size: 11pt, weight: "bold")[C36 · Octave and percussion clefs]
      #v(2pt)
      #text(size: 8pt, fill: luma(80))[
        Treble-8 is retained. ABC percussion clef currently falls back to treble, with the conversion warning shown below.
      ]
      #v(5pt)
      #grid(
        columns: (1fr, 1fr),
        gutter: 8pt,
        panel("Treble-8", "Octave clef retained", "The imported notes use the octave clef.", octave-import),
        panel("Percussion", "Clef fallback", "This rendering uses treble after the importer warning.", percussion-import),
      )
    ],
  )
}

#let render_c37() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Voice</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type><lyric number=\"2\"><text>second verse</text></lyric><lyric number=\"1\"><text>first verse</text></lyric></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C37",
    "Numeric lyric verse order",
    "MusicXML lists verse 2 before verse 1 in the note. The importer orders the rendered lyric lines numerically.",
    imported,
  )
}

#let render_c38() = {
  let cross-staff-source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Cross staff</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions><staves>2</staves><time><beats>4</beats><beat-type>4</beat-type></time><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>G</step><octave>4</octave></pitch><duration>2</duration><type>half</type><voice>1</voice><staff>1</staff></note><note><chord/><pitch><step>C</step><octave>3</octave></pitch><duration>2</duration><type>half</type><voice>1</voice><staff>2</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>2</duration><type>half</type><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>"
  let cross-staff-import = read-score(cross-staff-source, format: "musicxml")
  let hidden-tuplet-source = "<score-partwise><part-list><score-part id=\"P1\"/></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>3</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type><time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification><notations><tuplet type=\"start\"/></notations></note><note print-object=\"no\"><pitch><step>D</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type><time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification></note><note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type><time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification><notations><tuplet type=\"stop\"/></notations></note><note><pitch><step>D</step><octave>5</octave></pitch><duration>3</duration><type>quarter</type></note><note><pitch><step>F</step><octave>5</octave></pitch><duration>3</duration><type>quarter</type></note></measure></part></score-partwise>"
  let hidden-tuplet-import = read-score(hidden-tuplet-source, format: "musicxml")
  block(
    width: 100%,
    inset: 8pt,
    radius: 3pt,
    stroke: 0.7pt + luma(205),
    [
      #text(size: 11pt, weight: "bold")[C38 · Cross-staff destination and hidden tuplet note]
      #v(2pt)
      #text(size: 8pt, fill: luma(80))[
      The cross-staff chord tone fills the destination staff instead of a false whole-bar rest. A hidden note in the 3:2 group now emits as a spacer inside the same tuplet, so the imported score compiles without applying the ratio twice.
      ]
      #v(5pt)
      #score(..cross-staff-import.arguments)
      #v(3pt)
      #score(..hidden-tuplet-import.arguments)
    ],
  )
}

#let render_c48() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Set</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C48",
    "Reserved staff name",
    "The MusicXML part is named Set. The generated score keeps the name as a safe staff identifier and still compiles.",
    imported,
  )
}

#let render_c14() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Clarinet in B-flat</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions><key><fifths>2</fifths></key><transpose><diatonic>-1</diatonic><chromatic>-2</chromatic></transpose></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note><note><pitch><step>D</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note><note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note><note><pitch><step>F</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C14",
    "Written-pitch clarinet import",
    "MusicXML pitches remain written as C5–F5 under the notated D key. The clarinet's B-flat sounding-transpose metadata is reported as unsupported; no concert-pitch conversion is implied.",
    imported,
  )
}

#let render_c15() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Chord</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>2</duration><type>half</type><tie type=\"start\"/></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>2</duration><type>half</type></note><note><pitch><step>C</step><octave>5</octave></pitch><duration>2</duration><type>half</type><tie type=\"stop\"/></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>2</duration><type>half</type></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C15",
    "Partial chord tie warning",
    "Only C is tied in the MusicXML chord. The importer drops this partial chord tie and displays the warning; the rendered chord has no tie arc.",
    imported,
  )
}

#let render_c33() = block(
  width: 100%,
  inset: 8pt,
  radius: 3pt,
  stroke: 0.7pt + luma(205),
  [
    #text(size: 11pt, weight: "bold")[C33 · Malformed ABC chord diagnostic]
    #v(2pt)
    #text(size: 8pt, fill: luma(80))[
      The importer rejects an unclosed chord that consumes a barline. This is the captured editor diagnostic from compiling the exact read-score call below.
    ]
    #v(5pt)
    #raw("X:1\nK:C\nL:1/4\nM:4/4\nc d e f |\n[CEG c d e |]", block: true)
    #v(4pt)
    #text(size: 7pt, fill: rgb("#8a3b00"))[
      typed-scores error in read-score: an ABC chord contains a barline before its closing bracket; fix: check the file in its notation program, or report the file if it opens correctly there
    ]
  ],
)

#let render_c34() = {
  let source = "X:1\nM:4/4\nL:1/4\nK:C\nC D E F|]\n\nG A B c|]\n"
  let imported = read-score(source, format: "abc")
  panel(
    "C34",
    "Blank line ends ABC tune body",
    "The first empty body line terminates the selected tune, so notes after it are not included in the imported score.",
    imported,
  )
}

#let render_c35() = {
  let source = "X:1\nM:4/4\nL:1/4\nK:C\nC D [1 E F :| [2 G A |]\n"
  let imported = read-score(source, format: "abc")
  panel(
    "C35",
    "Mid-measure volta warning",
    "The first ending begins mid-measure and is dropped with a warning. The later valid second ending remains in the rendered score.",
    imported,
  )
}

#let render_c49() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Hidden chord head</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions><time><beats>2+3</beats><beat-type>8</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note><attributes><key><fifths>1</fifths></key></attributes><direction placement=\"above\"><direction-type><words>rit.</words></direction-type></direction><note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note><note print-object=\"no\"><chord/><pitch><step>G</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C49",
    "Hidden chord head and unsupported metadata",
    "The hidden G chord member, mid-measure key change, above-staff direction, and additive meter grouping each produce an import warning. The visible notes show the retained score content.",
    imported,
  )
}

#let render_c50() = {
  let source = "<score-partwise><part-list><score-part id=\"P1\"><part-name>Mixed chord</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type></note><note><chord/><unpitched><display-step>G</display-step><display-octave>5</display-octave></unpitched><duration>4</duration><type>whole</type></note></measure></part></score-partwise>"
  let imported = read-score(source, format: "musicxml")
  panel(
    "C50",
    "Unpitched chord head uses display pitch",
    "The unpitched member is rendered at its MusicXML display pitch and the warning states this conversion explicitly.",
    imported,
  )
}

#let render_c60() = {
  let source = "X:1\nM:4/4\nL:1/4\nK:C\n!invertedfermata!c d e f|]\n"
  let imported = read-score(source, format: "abc")
  panel(
    "C60",
    "Inverted fermata warning",
    "The importer renders an upright fermata because below-staff fermatas are unsupported, and exposes that limitation in its warning.",
    imported,
  )
}
