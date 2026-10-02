\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\new Staff {
  \numericTimeSignature
  \key f \major
  \time 3/4
  << { <a' c'' f''>4 r <bes' d''> } \\ { f'2 r4 } >> |
  << { a'4 r c'' } \\ { r4 c'2 } >> |
  << { e''4 f'' g'' } \\ { r4 r r } >> |
  << { r4 r r } \\ { a'4 c'' e'' } >> |
  << { c''4 r2 } \\ { a'4 r2 } >>
  \bar "|."
}
