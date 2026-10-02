\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

{
  \numericTimeSignature
  \time 4/4
  \set Staff.ottavationMarkups = #ottavation-ordinals
  \ottava #1 c'''4 d''' e''' f''' | g''' a''' b''' c'''' | \ottava #0
  c''1 | \ottava #2 c''''2 e'''' | \ottava #0 g''1
  \bar "|."
}
