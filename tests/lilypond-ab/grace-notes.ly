\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\relative {
  \numericTimeSignature
  \time 4/4
  \acciaccatura d''8 c4
  \appoggiatura e8 d4
  \acciaccatura { g16 f } e2
  c4( d) r2
  \bar "|."
}
