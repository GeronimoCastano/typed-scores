\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\new DrumStaff <<
  \new DrumVoice \drummode {
    \voiceOne
    \numericTimeSignature
    \time 4/4
    hh8 hh hh hh hh hh hh hh | cymc4 hh8 hh <hh sn>4 hh
    \bar "|."
  }
  \new DrumVoice \drummode {
    \voiceTwo
    bd4 sn bd8 bd sn4 | bd2 bd4 r
  }
>>
