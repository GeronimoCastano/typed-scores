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

== Hidden notes inside MusicXML tuplets

An invisible MusicXML note still occupies rhythmic time. The importer keeps
that slot as a spacer inside its tuplet, so the tuplet ratio applies once to
the full group while the hidden notehead remains absent.

#let hidden-tuplet-xml = (
  "<score-partwise>",
  "  <part-list><score-part id=\"P1\"><part-name>Hidden note</part-name></score-part></part-list>",
  "  <part id=\"P1\">",
  "    <measure number=\"1\">",
  "      <attributes><divisions>3</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes>",
  "      <note>",
  "        <pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type>",
  "        <time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification>",
  "        <notations><tuplet type=\"start\"/></notations>",
  "      </note>",
  "      <note print-object=\"no\">",
  "        <pitch><step>D</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type>",
  "        <time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification>",
  "      </note>",
  "      <note>",
  "        <pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type>",
  "        <time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification>",
  "        <notations><tuplet type=\"stop\"/></notations>",
  "      </note>",
  "      <note><pitch><step>D</step><octave>5</octave></pitch><duration>3</duration><type>quarter</type></note>",
  "      <note><pitch><step>F</step><octave>5</octave></pitch><duration>3</duration><type>quarter</type></note>",
  "    </measure>",
  "  </part>",
  "</score-partwise>",
).join("\n")
#let hidden-tuplet-import = read-score(hidden-tuplet-xml, format: "musicxml")
#score(..hidden-tuplet-import.arguments)
