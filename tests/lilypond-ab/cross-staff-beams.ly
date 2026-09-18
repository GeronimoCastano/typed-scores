\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\new PianoStaff <<
  \new Staff = "up" {
    \numericTimeSignature
    \time 2/4
    s2 |
    c''8 \change Staff = "down" g \change Staff = "up" e'' \change Staff = "down" c
    \bar "|."
  }
  \new Staff = "down" {
    \numericTimeSignature
    \time 2/4
    \clef bass
    c,16 g, \change Staff = "up" e' g' c'' g' \change Staff = "down" e c |
    c,2
  }
>>
