\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\relative {
  \numericTimeSignature
  \time 4/4
  <c' e g c>2\arpeggio
  \arpeggioArrowUp
  <c e g c>2\arpeggio
  \arpeggioArrowDown
  <c e g c>2\arpeggio
  \arpeggioNormal
  <c e g c>2\arpeggio
  \bar "|."
}
