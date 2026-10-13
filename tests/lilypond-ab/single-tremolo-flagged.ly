\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\score {
  \new StaffGroup <<
    \new Staff \with { instrumentName = "c8:16" } {
      \time 3/16
      c''8:16 s16
    }
    \new Staff \with { instrumentName = "c16:32" } {
      \time 3/16
      c''16:32 s8
    }
  >>
  \layout { }
}
