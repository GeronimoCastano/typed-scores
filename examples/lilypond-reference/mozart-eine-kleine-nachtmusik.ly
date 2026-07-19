\version "2.26.0"

#(set-global-staff-size 13)

\header {
  composer = "W. A. Mozart"
  tagline = ##f
}

\paper {
  paper-width = 190\mm
  indent = 16\mm
  short-indent = 9\mm
  line-width = 172\mm
  ragged-last = ##f
  page-breaking = #ly:one-page-breaking
}

global = {
  \key g \major
  \numericTimeSignature
  \time 4/4
}

violinOne = \absolute {
  \global
  \tempo "Allegro"
  <b' d'' g''>4\f r8 d'' g''4 r8 d'' |
  g''8 d'' g'' b'' d'''4 r |
  c'''4 r8 a'' c'''4 r8 a'' |
  c'''8 a'' fis'' a'' d''4 r | \break
  <b' d'' g''>8 r g''4. b''8( a'' g''-.) |
  a''32[ g'' a'' g''] fis''8 fis''4. a''8( c''' fis''-.) | \break
  a''8( g'') g''4. b''8( a'' g''-.) |
  a''32[ g'' a'' g''] fis''8 fis''4. a''8( c''' fis''-.) | \break
  g''8-. g''-. \appoggiatura g''16 fis''8( e''16 fis'') g''8-. g''-.
    \appoggiatura b''16 a''8( g''16 a'') |
  b''8-. b''-. \appoggiatura d'''16 c'''8( b''16 c''') d'''4 r \bar "|."
}

violinTwo = \absolute {
  \global
  <b' d'' g''>4\f r8 d'' g''4 r8 d'' |
  g''8 d'' g'' b'' d'''4 r |
  c'''4 r8 a'' c'''4 r8 a'' |
  c'''8 a'' fis'' a'' d''4 r |
  <b d'>2:16 <b d'>2:16 |
  <c' d'>2:16 <c' d'>2:16 |
  <b d'>2:16 <b d'>2:16 |
  <c' d'>2:16 <c' d'>2:16 |
  <b d'>4 c''8 c'' d'' d'' c''( b'16 a') |
  g'8 g' fis' fis' g'4 r |
}

viola = \absolute {
  \global
  \clef alto
  g'4\f r8 d' g'4 r8 d' |
  g'8 d' g' b' d''4 r |
  c''4 r8 a' c''4 r8 a' |
  c''8 a' fis' a' d'4 r |
  g'8 g' g' g' g' g' g' g' |
  a'8 a' a' a' a'( c'') fis'( a') |
  g'8 g' g' g' g' g' g' g' |
  a'8 a' a' a' a'( c'') fis'( a') |
  d'2:16 d'2:16 |
  d'2:16 d'4 r |
}

cello = \absolute {
  \global
  \clef bass
  g4\f r8 d g4 r8 d |
  g8 d g b d'4 r |
  c'4 r8 a c'4 r8 a |
  c'8 a fis a d4 r |
  g8 g g g g g g g |
  g8 g g g g g g g |
  g8 g g g g g g g |
  g8 g g g g g g g |
  g8 g a a b b fis fis |
  g8 g a a b4 r |
}

\score {
  \new StaffGroup <<
    \new Staff \with {
      instrumentName = "Violin I"
      shortInstrumentName = "Vln. I"
    } \violinOne
    \new Staff \with {
      instrumentName = "Violin II"
      shortInstrumentName = "Vln. II"
    } \violinTwo
    \new Staff \with {
      instrumentName = "Viola"
      shortInstrumentName = "Vla."
    } \viola
    \new Staff \with {
      instrumentName = "Violoncello"
      shortInstrumentName = "Vc."
    } \cello
  >>
}
