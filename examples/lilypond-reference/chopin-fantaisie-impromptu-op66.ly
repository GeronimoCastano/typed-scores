\version "2.26.0"

\header {
  title = "Fantaisie-Impromptu"
  subtitle = "Bars 5-10 (Part A, Allegro)"
  composer = "Frédéric Chopin"
  opus = "Op. 66"
  source = "Herrmann Scholtz (1845-1918)"
  style = "Classical"
  copyright = "Public Domain"
  maintainer = "Guy D. Lederfein"
  mutopiacomposer = "ChopinFF"
  mutopiainstrument = "Piano"
  footer = "Mutopia-2009/09/05-1693"
  tagline = ##f
}

% Bars 5-10 of Chopin's Fantaisie-Impromptu, Op. 66, translated from the
% Mutopia Project LilyPond source (piece 1693, edition by Herrmann Scholtz,
% public domain): https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1693
% The typed-scores translation is assets/readme/examples/chopin-fantaisie-impromptu.typ.
% The excerpt keeps the Mutopia pitches, rhythms, slurs, pedal spans, hairpins,
% dynamics, and the 8va passage; the layout is one system per two bars.

pd = \sustainOn
pu = \sustainOff

\score {
  \new PianoStaff <<
    \new Staff = "upper" {
      \clef treble
      \key cis \minor
      \time 4/4
      \relative c'' {
        %%% upper staff, bars 5-10 %%%
        r16 gis16\p( a gis fisis gis cis e dis cis dis cis bis cis e gis) |
  r16 gis,( a gis fisis gis cis e dis cis dis cis bis cis e gis) |
  r16 a,(\< cis dis fis a cis dis\! \ottava #1 b'\> a gis fis e dis fis cis)\! \ottava #0 |
  bis( dis a gis fis a e dis) fis( cis bis dis a gis b a~) |
  a16\p gis( a gis fisis gis cis e dis cis dis cis bis cis e gis) |
  r16\< gis,( ais gis fisis gis cis e dis cis dis cis bis cis e gis\!)
      }
    }
    \new Staff = "lower" {
      \clef bass
      \key cis \minor
      \time 4/4
      \relative c {
        %%% lower staff, bars 5-10 %%%
        \times 4/6 { cis8(\pd gis' cis e cis gis) } \times 4/6 { e( gis cis e cis gis)\pu } |
  \times 4/6 { cis,(\pd gis' cis e cis gis) } \times 4/6 { e( gis cis e cis gis)\pu } |
  \times 4/6 { dis(\pd a' cis fis cis a) } \times 4/6 { fis( cis' dis a' dis, cis)\pu } |
  \times 4/6 { gis,(\pd dis' fis bis fis dis } \times 4/6 { gis, dis' fis bis fis dis)\pu } |
  \times 4/6 { cis(\pd gis' cis e cis gis) } \times 4/6 { e( gis cis e cis gis)\pu } |
  \times 4/6 { cis,(\pd gis' ais e' ais, gis)\pu } \times 4/6 { e(\pd gis ais cis ais gis)\pu }
      }
    }
  >>
  \layout {
    \context { \Score \override SpacingSpanner.common-shortest-duration = #(ly:make-moment 1/16) }
  }
}
