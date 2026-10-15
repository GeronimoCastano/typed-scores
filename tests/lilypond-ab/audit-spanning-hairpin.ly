\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\score {
  <<
    \new Staff \with { instrumentName = "Stop on beat 1" } {
      \clef bass
      \numericTimeSignature
      \time 4/4
      \set Staff.pedalSustainStyle = #'bracket
      \absolute {
        c4\sustainOn_\markup { \italic "Ped." }\< d e f |
        g a b c'
        \break
        c4\! d e f\sustainOff |
        g a b c' |
        c1
      }
    }
    \new Staff \with { instrumentName = "Control: stop on beat 4" } {
      \clef bass
      \numericTimeSignature
      \time 4/4
      \set Staff.pedalSustainStyle = #'bracket
      \absolute {
        c4\sustainOn_\markup { \italic "Ped." }\< d e f |
        g a b c'
        \break
        c4 d e f\!\sustainOff |
        g a b c' |
        c1
      }
    }
  >>
}
