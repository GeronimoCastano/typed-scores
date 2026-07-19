\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\relative {
  \numericTimeSignature
  \time 4/4
  \autoBeamOff
  c''4
  \tuplet 5/4 { f8 e f \tuplet 3/2 { e[ f g] } }
  f4
  \bar "|."
}
