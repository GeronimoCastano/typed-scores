\version "2.26.0"

#(set-global-staff-size 15)

\header {
  composer = "J. S. Bach"
  tagline = ##f
}

\paper {
  paper-width = 190\mm
  indent = 5\mm
  short-indent = 0\mm
  line-width = 172\mm
  ragged-last = ##f
  page-breaking = #ly:one-page-breaking
}

\layout {
  \context {
    \Score
    \override SpacingSpanner.uniform-stretching = ##t
  }
}

music = \absolute {
  \clef bass
  \key g \major
  \numericTimeSignature
  \time 4/4
  \tempo \markup \italic "Prélude"

  g,16( d b a) b( d b d) g,( d b a) b( d b d) |
  g,( e c' b) c'( e c' e) g,( e c' b) c'( e c' e) | \break
  g,( fis c' b) c'( fis c' fis) g,( fis c' b) c'( fis c' fis) |
  g,( g b a) b( g b g) g,( g b a) b( g b fis) | \break
  g,( e b a) b( g fis g) e( g fis g) b,( d cis b,) |
  cis16( g a g) a( g a g) cis( g a g) a( g a g) | \break
  fis16( a d' cis') d'( a g a) fis( a g a) d( fis e d) |
  e,16( b, g fis) g( b, g b,) e,( b, g fis) g( b, g b,) | \break
  e,16( cis d e) d( cis b, a,) g( fis e d') cis'( b a g) |
  fis16( e d d') a( d' fis a) d( e fis a) g( fis e d) | \break
  gis16( d f e) f( d gis d) a( d f e) f( d gis d) |
  c16( e a b) c'( a e d) c( e a b) c'( a fis e) | \break
  dis16( fis dis fis) a( fis a fis) dis( fis dis fis) a( fis a fis) |
  g16( fis e g) fis( g a fis) g( fis e d) c( b, a, g,) | \break
  fis16( c' d' c') d'( c' d' c') fis( c' d' c') d'( c' d' c') |
  g16( b f' e') f'( b f' b) g( b f' e') f'( b f' b) \bar "|."
}

\score {
  \new Staff \music
}
