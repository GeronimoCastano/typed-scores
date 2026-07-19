\version "2.26.0"

#(set-global-staff-size 13)

\header {
  composer = "F. Chopin"
  tagline = ##f
}

\paper {
  paper-width = 190\mm
  indent = 7\mm
  short-indent = 0\mm
  line-width = 172\mm
  ragged-last = ##f
  page-breaking = #ly:one-page-breaking
}

global = {
  \key ees \major
  \time 12/8
}

upper = \absolute {
  \global
  \tempo \markup \italic "Andante" 8 = 132
  \partial 8 bes'8-1 |

  g''4.-5-4 ~ g''8^\markup \italic "espress. dolce" f''8-3( g''-4
    f''4.-3 ees''4-2) bes'8-1( |
  g''4-5 c''8-1\turn c'''4-5 g''8-2 bes''4.-4\> aes''4-3 g''8-2\! ) | \break

  f''4.-1 g''4-3( d''8-1) ees''4.-2 c''4.-1 |
  bes'8-1\f( d'''8-5) c'''8-4( bes''16-3 aes'' g'' aes''-4)
    c''16( d'' ees''4.) r4 bes'8-1 | \break

  g''4.-5-4\p( f''16-3) g''16( f'' e'' f'' g'' f''8-.-3)
    ees''4->-2 ~ ees''16( f'' ees'' d'' ees'' f'') |
  g''16-4( b' c'') des''16->-3 c''-1 f''->-2 e''-1 aes''->-3 g''-1 des'''-4 c''' g''
    bes''4.-3\> aes''4-2 g''8-1\! | \break

  f''4.-2-3\turn \appoggiatura { e''16 f'' } g''8-.-3 g''8-4( d''8-1)
    ees''4.-2 c''4.-1 |
  bes'8-1\f( d'''8-.-5) c'''8-.-4\<( bes''16-.-3 aes''-. g''-.-1 aes''-.)
    \appoggiatura aes''8 c''16-1( d''16\! ees''4.-3 ~ ees''8 d''8-2 ees''8) \bar "|."
}

lower = \absolute {
  \global
  \clef bass
  \partial 8 r8 |

  ees,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff
    ees,8-.\sustainOn <aes d'>8( [<ces' d' aes'>8])\sustainOff
    ees,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff
    d,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff |
  c,8-.\sustainOn <g e'>8( [<bes e' g'>8])\sustainOff
    c8-. <g e'>8( [<c' e' bes'>8])
    f,8-. <f des'>8( [<bes des' e'>8])
    f,8-. <f c'>8( [<aes c' f'>8]) |

  bes,,8-.\sustainOn <f d'>8( [<bes d' aes'>8])\sustainOff
    b,,8-.\sustainOn <g f'>8( [<d' f' g'>8])\sustainOff
    c,8-.\sustainOn <g ees'>8( [<c' ees' g'>8])\sustainOff
    a,,8-.\sustainOn <ges ees'>8( [<c' ees' ges'>8])\sustainOff |
  bes,,8-.\sustainOn <f ees'>8( [<bes ees' aes'>8])\sustainOff
    bes,,8-.\sustainOn <f d'>8( [<bes aes'>8])\sustainOff
    ees,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff
    ees,8-. <g ees'>8( [<bes ees' g'>8]) |

  ees,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff
    ees,8-.\sustainOn <aes d'>8( [<ces' d' aes'>8])\sustainOff
    ees,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff
    d,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff |
  c,8-.\sustainOn <g e'>8( [<bes e' g'>8])\sustainOff
    c8-. <g e'>8( [<c' e' bes'>8])
    f,8-.\sustainOn <f des'>8( [<bes des' e'>8])\sustainOff
    f,8-. <f c'>8( [<aes c' f'>8]) |

  bes,,8-.\sustainOn <f d'>8( [<bes d' aes'>8])\sustainOff
    b,,8-.\sustainOn <g f'>8( [<d' f' g'>8])\sustainOff
    c,8-.\sustainOn <g ees'>8( [<c' ees' g'>8])\sustainOff
    a,,8-.\sustainOn <ges ees'>8( [<c' ees' ges'>8])\sustainOff |
  bes,,8-.\sustainOn <f ees'>8( [<bes ees' aes'>8])\sustainOff
    bes,,8-.\sustainOn <f d'>8( [<bes aes'>8])\sustainOff
    ees,8-.\sustainOn <g ees'>8( [<bes ees' g'>8])\sustainOff
    ees,8-. <g ees'>8( [<bes ees' g'>8]) |
}

\score {
  \new PianoStaff <<
    \new Staff = "upper" \upper
    \new Staff = "lower" \lower
  >>
}
