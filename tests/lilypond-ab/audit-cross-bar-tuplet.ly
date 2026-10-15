\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\score {
  \new Staff {
    \numericTimeSignature
    \time 4/4
    \autoBeamOff
    \tuplet 3/2 {
      c'2. d'2. |
      \set Timing.measureLength = #(/ 1 2)
      e'2.
    }
    \bar "|."
  }
}
