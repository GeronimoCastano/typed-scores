\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

<<
  \new Staff {
    \clef bass
    \numericTimeSignature
    \time 4/4
    c2 b, | a,4 g, f,2 | g,1 | c1
    \bar "|."
  }
  \figures { <_>2 <6 4>4 <5 3> | <7 _+>4 <6+ 5> <_!>2 | <4>2 <3> | <9 7 5>1 }
>>
