\version "2.26.0"

\header { tagline = ##f }
\paper { indent = 0 }

\fixed c' {
  \numericTimeSignature
  \time 4/4
  c''4
  \acciaccatura { d''16 e'' } f''4
  \appoggiatura { g''8 } a''4
  \grace { b''16 c''' } d'''4
  \bar "|."
}
