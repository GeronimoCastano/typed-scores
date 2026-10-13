\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\relative c'' {
  \numericTimeSignature
  \time 4/4
  \repeat tremolo 4 { c16 g }
  \repeat tremolo 8 { d32 a }
  \bar "|."
}
