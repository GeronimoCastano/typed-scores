#import "@preview/cetz:0.5.2"

// All coordinates live in staff-unit space: 1 unit = one staff space
// (the gap between adjacent staff lines). The `unit` parameter converts
// to actual page length. Glyph geometry follows Bravura's SMuFL metadata.

// Bravura engraving defaults (in staff spaces).
#let stem-thickness = 0.12
#let beam-thickness = 0.5
#let beam-spacing = 0.25
#let ledger-thickness = 0.16
#let ledger-extension = 0.4
#let staff-line-thickness = 0.13
#let thin-barline-thickness = 0.16

// Vertical distance between a notehead center and its stem attachment
// point (SMuFL stemUpSE / stemDownNW anchors of noteheadBlack).
#let stem-anchor-dy = 0.168
// Half-width of the regular (black/half) notehead.
#let notehead-half-width = 0.59
// Horizontal center of a stem relative to the notehead center.
#let stem-center-dx = notehead-half-width - stem-thickness / 2

#let staff-y(position, bottom-y: 0, line-gap: 1.0) = {
  bottom-y + (position - 2) * line-gap / 2
}

#let music-canvas(length: 8pt, body) = {
  cetz.canvas(length: length, body)
}

// Glyph bounding boxes from bravura_metadata.json, relative to the glyph
// origin, in staff spaces.
#let _bravura-bbox(kind) = {
  let table = (
    "notehead-black": (sw: (0.0, -0.5), ne: (1.18, 0.5)),
    "notehead-half": (sw: (0.0, -0.5), ne: (1.18, 0.5)),
    "notehead-whole": (sw: (0.0, -0.5), ne: (1.688, 0.5)),
    "augmentation-dot": (sw: (0.0, -0.2), ne: (0.4, 0.2)),
    "treble-clef": (sw: (0.0, -2.632), ne: (2.684, 4.392)),
    "bass-clef": (sw: (-0.02, -2.54), ne: (2.736, 1.048)),
    "alto-clef": (sw: (0.0, -2.024), ne: (2.796, 2.024)),
    "tenor-clef": (sw: (0.0, -2.024), ne: (2.796, 2.024)),
    "sharp": (sw: (0.0, -1.392), ne: (0.996, 1.4)),
    "flat": (sw: (0.0, -0.7), ne: (0.904, 1.756)),
    "natural": (sw: (0.0, -1.34), ne: (0.672, 1.364)),
    "rest-whole": (sw: (0.0, -0.54), ne: (1.128, 0.036)),
    "rest-half": (sw: (0.0, -0.008), ne: (1.128, 0.568)),
    "rest-quarter": (sw: (0.004, -1.5), ne: (1.08, 1.492)),
    "rest-eighth": (sw: (0.0, -1.004), ne: (0.988, 0.696)),
    "rest-sixteenth": (sw: (0.0, -2.0), ne: (1.28, 0.716)),
    "rest-thirty-second": (sw: (0.0, -2.0), ne: (1.452, 1.704)),
    "flag-eighth-up": (sw: (0.0, -3.2408), ne: (1.056, 0.0352)),
    "flag-eighth-down": (sw: (0.0, -0.0576), ne: (1.224, 3.2329)),
    "flag-sixteenth-up": (sw: (0.0, -3.252), ne: (1.116, 0.008)),
    "flag-sixteenth-down": (sw: (0.0, -0.036), ne: (1.1636, 3.248)),
    "flag-thirty-second-up": (sw: (0.0, -3.248), ne: (1.044, 0.596)),
    "flag-thirty-second-down": (sw: (0.0, -0.6875), ne: (1.092, 3.248)),
    "staccato-above": (sw: (0.0, 0.0), ne: (0.336, 0.336)),
    "staccato-below": (sw: (0.0, -0.336), ne: (0.336, 0.0)),
    "tenuto-above": (sw: (-0.004, 0.0), ne: (1.352, 0.192)),
    "tenuto-below": (sw: (-0.004, -0.192), ne: (1.352, 0.0)),
    "staccatissimo-above": (sw: (0.004, 0.0), ne: (0.356, 1.16)),
    "staccatissimo-below": (sw: (0.004, -1.16), ne: (0.356, 0.0)),
    "marcato-above": (sw: (-0.004, -0.004), ne: (0.94, 1.012)),
    "marcato-below": (sw: (-0.004, -1.016), ne: (0.94, 0.0)),
    "accent-above": (sw: (0.0, 0.004), ne: (1.356, 0.98)),
    "accent-below": (sw: (0.0, -0.976), ne: (1.356, 0.0)),
    "ornament-turn": (sw: (0.0, 0.0), ne: (1.84, 0.872)),
    "pedal-ped": (sw: (0.0, -0.032), ne: (4.076, 2.22)),
    "pedal-up": (sw: (0.0, 0.0), ne: (1.8, 1.8)),
    "time-sig-0": (sw: (0.08, -1.0), ne: (1.8, 1.004)),
    "time-sig-1": (sw: (0.08, -1.0), ne: (1.256, 1.004)),
    "time-sig-2": (sw: (0.08, -1.028), ne: (1.704, 1.016)),
    "time-sig-3": (sw: (0.08, -1.004), ne: (1.604, 0.996)),
    "time-sig-4": (sw: (0.08, -1.0), ne: (1.8, 1.004)),
    "time-sig-5": (sw: (0.08, -1.004), ne: (1.532, 0.984)),
    "time-sig-6": (sw: (0.08, -0.996), ne: (1.656, 1.004)),
    "time-sig-7": (sw: (0.08, -1.0), ne: (1.684, 0.996)),
    "time-sig-8": (sw: (0.08, -1.036), ne: (1.664, 1.036)),
    "time-sig-9": (sw: (0.08, -0.996), ne: (1.656, 1.004)),
  )
  if kind not in table {
    panic("unknown Bravura glyph " + kind)
  }
  table.at(kind)
}

#let _bravura-file(kind) = "assets/glyphs/" + kind + ".svg"

#let _bravura-width(kind) = {
  let bbox = _bravura-bbox(kind)
  bbox.ne.at(0) - bbox.sw.at(0)
}

#let _bravura-height(kind) = {
  let bbox = _bravura-bbox(kind)
  bbox.ne.at(1) - bbox.sw.at(1)
}

// Draw a Bravura glyph. With `origin: true`, (x, y) is the glyph's SMuFL
// origin; otherwise (x, y) is the center of its bounding box.
#let _draw-bravura-glyph(kind, x, y, unit: 8pt, origin: false, glyph-scale: 1.0) = {
  let bbox = _bravura-bbox(kind)
  let cx = if origin { x + (bbox.sw.at(0) + bbox.ne.at(0)) * glyph-scale / 2 } else { x }
  let cy = if origin { y + (bbox.sw.at(1) + bbox.ne.at(1)) * glyph-scale / 2 } else { y }
  cetz.draw.content(
    (cx, cy),
    image(_bravura-file(kind), width: _bravura-width(kind) * unit * glyph-scale),
    anchor: "center",
    padding: 0pt,
  )
}

#let draw-staff-lines(
  width,
  x: 0,
  bottom-y: 0,
  line-gap: 1.0,
  unit: 8pt,
) = {
  import cetz.draw: *
  for i in range(5) {
    let y = bottom-y + i * line-gap
    line((x, y), (x + width, y), stroke: staff-line-thickness * unit + black)
  }
}

#let draw-filled-notehead(x, y, unit: 8pt, scale: 1.0) = {
  _draw-bravura-glyph("notehead-black", x, y, unit: unit, glyph-scale: scale)
}

#let draw-open-notehead(x, y, unit: 8pt, scale: 1.0) = {
  _draw-bravura-glyph("notehead-half", x, y, unit: unit, glyph-scale: scale)
}

#let draw-whole-notehead(x, y, unit: 8pt, scale: 1.0) = {
  _draw-bravura-glyph("notehead-whole", x, y, unit: unit, glyph-scale: scale)
}

#let draw-augmentation-dot(x, y, unit: 8pt) = {
  _draw-bravura-glyph("augmentation-dot", x, y, unit: unit)
}

// The page-space point where a stem of the given length ends.
// (x, y) is the notehead center; length is measured from the stem
// attachment point to the tip.
#let stem-tip(x, y, direction: "up", length: 3.5) = {
  if direction == "up" {
    (x + stem-center-dx, y + stem-anchor-dy + length)
  } else {
    (x - stem-center-dx, y - stem-anchor-dy - length)
  }
}

#let draw-stem(x, y, direction: "up", length: 3.5, unit: 8pt) = {
  import cetz.draw: *
  let tip = stem-tip(x, y, direction: direction, length: length)
  let attach-y = if direction == "up" { y + stem-anchor-dy } else { y - stem-anchor-dy }
  line(
    (tip.at(0), attach-y),
    tip,
    stroke: stem-thickness * unit + black,
  )
}

#let draw-ledger-lines(
  x,
  position,
  bottom-y: 0,
  line-gap: 1.0,
  head-half-width: notehead-half-width,
  unit: 8pt,
) = {
  import cetz.draw: *
  let half = head-half-width + ledger-extension
  let stroke-style = ledger-thickness * unit + black
  if position <= 0 {
    for p in range(0, position - 1, step: -2) {
      let y = staff-y(p, bottom-y: bottom-y, line-gap: line-gap)
      line((x - half, y), (x + half, y), stroke: stroke-style)
    }
  }
  if position >= 12 {
    for p in range(12, position + 1, step: 2) {
      let y = staff-y(p, bottom-y: bottom-y, line-gap: line-gap)
      line((x - half, y), (x + half, y), stroke: stroke-style)
    }
  }
}

// Draw 1-3 flags hanging off a stem tip. (x, y) is the stem tip; the
// glyph's SMuFL attachment anchor is aligned with the stem's outer edge.
#let draw-flag(x, y, direction: "up", count: 1, unit: 8pt) = {
  let (kind, anchor-y) = if direction == "up" {
    // stemUpNW anchors of flag8thUp / flag16thUp / flag32ndUp.
    if count == 1 { ("flag-eighth-up", -0.04) }
    else if count == 2 { ("flag-sixteenth-up", -0.088) }
    else { ("flag-thirty-second-up", 0.376) }
  } else {
    // stemDownSW anchors of flag8thDown / flag16thDown / flag32ndDown.
    if count == 1 { ("flag-eighth-down", 0.132) }
    else if count == 2 { ("flag-sixteenth-down", 0.128) }
    else { ("flag-thirty-second-down", -0.448) }
  }
  let stem-left = x - stem-thickness / 2
  _draw-bravura-glyph(kind, stem-left, y - anchor-y, unit: unit, origin: true)
}

// A single beam segment between two stem-tip points; start/end give the
// beam's vertical center line.
#let draw-beam(start, end, thickness: beam-thickness, paint: black) = {
  import cetz.draw: *
  let (x1, y1) = start
  let (x2, y2) = end
  let h = thickness / 2
  merge-path(close: true, fill: paint, stroke: none, {
    line((x1, y1 - h), (x2, y2 - h))
    line((x2, y2 - h), (x2, y2 + h))
    line((x2, y2 + h), (x1, y1 + h))
  })
}

#let _brace-width = 0.82

#let draw-grand-brace(x, bottom-y, top-y, unit: 8pt) = {
  import cetz.draw: *
  let span = top-y - bottom-y
  let pad = calc.max(0.35, span * 0.025)
  content(
    (x, (bottom-y + top-y) / 2),
    image("assets/glyphs/brace.svg", width: _brace-width * unit, height: (span + pad * 2) * unit),
    anchor: "center",
    padding: 0pt,
  )
}

#let draw-staff-bracket(x, bottom-y, top-y, unit: 8pt) = {
  import cetz.draw: *
  let span = top-y - bottom-y
  let cap-width = 0.58
  let cap-height = 0.36
  let stem-width = 0.085
  let stem-height = calc.max(0.1, span - cap-height * 0.82)
  content(
    (x, (bottom-y + top-y) / 2),
    image("assets/glyphs/bracket-stem.svg", width: stem-width * unit, height: stem-height * unit),
    anchor: "center",
    padding: 0pt,
  )
  content(
    (x + cap-width / 2 - stem-width / 2, top-y),
    image("assets/glyphs/bracket-top.svg", width: cap-width * unit, height: cap-height * unit),
    anchor: "north",
    padding: 0pt,
  )
  content(
    (x + cap-width / 2 - stem-width / 2, bottom-y),
    image("assets/glyphs/bracket-bottom.svg", width: cap-width * unit, height: cap-height * unit),
    anchor: "south",
    padding: 0pt,
  )
}

// A tie/slur arc between two points. Positive height bulges upward,
// negative downward. Each boundary is one exact quadratic parabola:
// y(t) = (1-t) sy + t ey + 4 h t (1-t), with x(t) linear in t.
#let draw-curve(start, end, height: 0.7, thickness: 0.22) = {
  import cetz.draw: *
  let (sx, sy) = start
  let (ex, ey) = end
  let mx = (sx + ex) / 2
  let baseline-mid = (sy + ey) / 2
  let sign = if height < 0 { -1 } else { 1 }
  let inner-height = height - sign * thickness

  // A quadratic Bezier with control Q is a literal parabola. CeTZ accepts
  // cubics, so convert Q exactly: C1=P0+2/3(Q-P0), C2=P2+2/3(Q-P2).
  let outer-q = (mx, baseline-mid + 2 * height)
  let inner-q = (mx, baseline-mid + 2 * inner-height)
  let outer-c1 = (sx + (outer-q.at(0) - sx) * 2 / 3, sy + (outer-q.at(1) - sy) * 2 / 3)
  let outer-c2 = (ex + (outer-q.at(0) - ex) * 2 / 3, ey + (outer-q.at(1) - ey) * 2 / 3)
  let inner-c1 = (sx + (inner-q.at(0) - sx) * 2 / 3, sy + (inner-q.at(1) - sy) * 2 / 3)
  let inner-c2 = (ex + (inner-q.at(0) - ex) * 2 / 3, ey + (inner-q.at(1) - ey) * 2 / 3)
  merge-path(close: true, fill: black, stroke: none, {
    bezier(start, end, outer-c1, outer-c2)
    bezier(end, start, inner-c2, inner-c1)
  })
}

#let draw-staccato(x, y, placement: "above", unit: 8pt) = {
  _draw-bravura-glyph("staccato-" + placement, x, y, unit: unit)
}

#let draw-articulation(kind, x, y, placement: "above", unit: 8pt) = {
  _draw-bravura-glyph(kind + "-" + placement, x, y, unit: unit)
}

#let draw-ornament-turn(x, y, unit: 8pt, scale: 1.0) = {
  _draw-bravura-glyph("ornament-turn", x, y, unit: unit, glyph-scale: scale)
}

#let draw-pedal-mark(x, y, unit: 8pt, release: false, scale: 0.62) = {
  let kind = if release { "pedal-up" } else { "pedal-ped" }
  _draw-bravura-glyph(kind, x, y, unit: unit, glyph-scale: scale)
}

// Crescendo opens to the right; diminuendo opens to the left.
#let draw-hairpin(start, end, kind: "crescendo", spread: 0.65, unit: 8pt) = {
  import cetz.draw: *
  let (sx, sy) = start
  let (ex, ey) = end
  let stroke-style = 0.09 * unit + black
  if kind == "crescendo" {
    line((sx, sy), (ex, ey + spread / 2), stroke: stroke-style)
    line((sx, sy), (ex, ey - spread / 2), stroke: stroke-style)
  } else {
    line((sx, sy + spread / 2), (ex, ey), stroke: stroke-style)
    line((sx, sy - spread / 2), (ex, ey), stroke: stroke-style)
  }
}

#let draw-clef(clef, x, y, unit: 8pt) = {
  _draw-bravura-glyph(clef + "-clef", x, y, unit: unit, origin: true)
}

// (x, y) is the glyph origin: x at the accidental's left edge, y level
// with the notehead it modifies.
#let draw-accidental(accidental, x, y, unit: 8pt, scale: 1.0) = {
  let name = if accidental == "Natural" { "natural" }
    else if accidental == "Sharp" { "sharp" }
    else if accidental == "Flat" { "flat" }
    else { none }
  if name != none {
    _draw-bravura-glyph(name, x, y, unit: unit, origin: true, glyph-scale: scale)
  }
}

#let accidental-width(accidental) = {
  if accidental == "Natural" { _bravura-width("natural") }
  else if accidental == "Sharp" { _bravura-width("sharp") }
  else if accidental == "Flat" { _bravura-width("flat") }
  else { 0 }
}

// Rests attach to staff lines: the whole rest hangs from the 4th line,
// the half rest sits on the middle line, the rest glyphs center there.
#let draw-rest(duration-base, x, bottom-y: 0, line-gap: 1.0, unit: 8pt) = {
  let (name, origin-y) = if duration-base == "Whole" { ("rest-whole", 3) }
    else if duration-base == "Half" { ("rest-half", 2) }
    else if duration-base == "Quarter" { ("rest-quarter", 2) }
    else if duration-base == "Eighth" { ("rest-eighth", 2) }
    else if duration-base == "Sixteenth" { ("rest-sixteenth", 2) }
    else { ("rest-thirty-second", 2) }
  _draw-bravura-glyph(name, x, bottom-y + origin-y * line-gap, unit: unit, origin: true)
}

#let rest-width(duration-base) = {
  if duration-base == "Whole" { _bravura-width("rest-whole") }
  else if duration-base == "Half" { _bravura-width("rest-half") }
  else if duration-base == "Quarter" { _bravura-width("rest-quarter") }
  else if duration-base == "Eighth" { _bravura-width("rest-eighth") }
  else if duration-base == "Sixteenth" { _bravura-width("rest-sixteenth") }
  else { _bravura-width("rest-thirty-second") }
}

#let _time-sig-digit-gap = 0.08

#let _time-sig-row-width(digits) = {
  let width = 0
  for (i, d) in digits.codepoints().enumerate() {
    if i > 0 { width += _time-sig-digit-gap }
    width += _bravura-width("time-sig-" + d)
  }
  width
}

#let _draw-time-sig-row(digits, cx, y, unit) = {
  let total = _time-sig-row-width(digits)
  let x = cx - total / 2
  for (i, d) in digits.codepoints().enumerate() {
    let kind = "time-sig-" + d
    let bbox = _bravura-bbox(kind)
    _draw-bravura-glyph(kind, x - bbox.sw.at(0), y, unit: unit, origin: true)
    x += _bravura-width(kind) + _time-sig-digit-gap
  }
}

// Width (in staff spaces) the time signature occupies.
#let time-signature-width(time) = {
  if time == none { 0 } else {
    let parts = time.split("/")
    let top = _time-sig-row-width(parts.at(0))
    let bottom = if parts.len() > 1 { _time-sig-row-width(parts.at(1)) } else { 0 }
    calc.max(top, bottom)
  }
}

// Numerator centered on the 4th line, denominator on the 2nd; `x` is the
// left edge of the signature.
#let draw-time-signature(time, x, bottom-y: 0, unit: 8pt) = {
  if time != none {
    let parts = time.split("/")
    let cx = x + time-signature-width(time) / 2
    _draw-time-sig-row(parts.at(0), cx, bottom-y + 3, unit)
    if parts.len() > 1 {
      _draw-time-sig-row(parts.at(1), cx, bottom-y + 1, unit)
    }
  }
}
