#import "render.typ": *

#let score-plugin = plugin("plugin.wasm")

// ---------------------------------------------------------------------------
// Plugin calls
// ---------------------------------------------------------------------------

#let _layout-note(note-str, clef: "treble") = {
  json(score-plugin.layout_note(bytes(clef + "\n" + note-str)))
}

#let _layout-sequence(sequence-str, clef: "treble", time: none) = {
  if time == none {
    json(score-plugin.layout_sequence(bytes(clef + "\n" + sequence-str)))
  } else {
    json(score-plugin.layout_sequence_timed(bytes(clef + "\n" + time + "\n" + sequence-str)))
  }
}

// ---------------------------------------------------------------------------
// Layout constants (staff spaces)
// ---------------------------------------------------------------------------

#let _default-stem-length = 3.5 - stem-anchor-dy
#let _accidental-gap = 0.25
#let _dot-gap-from-head = 0.4
#let _dot-step = 0.55
#let _clef-advance = 3.45
#let _prologue-gap = 0.7
#let _content-lead-in = 1.2
#let _min-onset-step = 1.0
#let _grand-brace-x = 0.55
#let _grand-brace-to-bar-gap = 0.4
#let _grand-clef-after-bar-gap = 0.8

#let _flat-order-positions = (
  treble: (8, 11, 7, 10, 6, 9, 5),
  bass: (6, 9, 5, 8, 4, 7, 3),
  alto: (7, 10, 6, 9, 5, 8, 4),
  tenor: (8, 11, 7, 10, 6, 9, 5),
)
#let _sharp-order-positions = (
  treble: (10, 7, 11, 8, 5, 9, 6),
  bass: (8, 5, 9, 6, 3, 7, 4),
  alto: (9, 6, 10, 7, 4, 8, 5),
  tenor: (10, 7, 11, 8, 5, 9, 6),
)

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

#let _duration-base(layout) = str(layout.duration.base)

#let _stem-direction(positions) = {
  if positions.len() == 0 {
    "up"
  } else {
    let sum = 0
    for p in positions {
      sum += p
    }
    if sum / positions.len() < 6 { "up" } else { "down" }
  }
}

#let _clef-origin-y(clef, bottom-y: 0, line-gap: 1.0) = {
  if clef == "treble" { staff-y(4, bottom-y: bottom-y, line-gap: line-gap) }
  else if clef == "bass" { staff-y(8, bottom-y: bottom-y, line-gap: line-gap) }
  else if clef == "alto" { staff-y(6, bottom-y: bottom-y, line-gap: line-gap) }
  else if clef == "tenor" { staff-y(8, bottom-y: bottom-y, line-gap: line-gap) }
  else { panic("unknown clef " + clef) }
}

#let _head-half-width(layout) = {
  if layout.notehead == "whole" { 0.844 } else { notehead-half-width }
}

// Dots sit in the space above line notes.
#let _dot-y(position, y, line-gap) = {
  if calc.rem(position, 2) == 0 { y + line-gap / 2 } else { y }
}

#let _draw-dots(x, y, dots, unit: 8pt) = {
  for i in range(dots) {
    draw-augmentation-dot(x + i * _dot-step, y, unit: unit)
  }
}

// ---------------------------------------------------------------------------
// Key signatures
// ---------------------------------------------------------------------------

#let _key-signature-name(key) = {
  if key == none { none }
  else if key == "Am" { "C" }
  else if key == "Em" { "G" }
  else if key == "Bm" { "D" }
  else if key == "F#m" { "A" }
  else if key == "C#m" { "E" }
  else if key == "G#m" { "B" }
  else if key == "D#m" { "F#" }
  else if key == "A#m" { "C#" }
  else if key == "Dm" { "F" }
  else if key == "Gm" { "Bb" }
  else if key == "Cm" { "Eb" }
  else if key == "Fm" { "Ab" }
  else if key == "Bbm" { "Db" }
  else if key == "Ebm" { "Gb" }
  else if key == "Abm" { "Cb" }
  else { key }
}

#let _key-accidentals(key) = {
  let key = _key-signature-name(key)
  let flats = ("F": 1, "Bb": 2, "Eb": 3, "Ab": 4, "Db": 5, "Gb": 6, "Cb": 7)
  let sharps = ("G": 1, "D": 2, "A": 3, "E": 4, "B": 5, "F#": 6, "C#": 7)
  if key == "C" or key == none {
    (kind: none, count: 0)
  } else if key in flats {
    (kind: "Flat", count: flats.at(key))
  } else if key in sharps {
    (kind: "Sharp", count: sharps.at(key))
  } else {
    panic("unsupported key signature " + key)
  }
}

#let _key-suppresses-accidental(pitch, key) = {
  let sig = _key-accidentals(key)
  if sig.count == 0 or pitch.accidental != sig.kind {
    false
  } else {
    let flat-letters = ("B", "E", "A", "D", "G", "C", "F")
    let sharp-letters = ("F", "C", "G", "D", "A", "E", "B")
    let letters = if sig.kind == "Flat" { flat-letters } else { sharp-letters }
    letters.slice(0, sig.count).contains(pitch.letter)
  }
}

#let _key-alters-natural(pitch, key) = {
  let sig = _key-accidentals(key)
  if pitch.accidental != "Natural" or sig.count == 0 {
    false
  } else {
    let flat-letters = ("B", "E", "A", "D", "G", "C", "F")
    let sharp-letters = ("F", "C", "G", "D", "A", "E", "B")
    let letters = if sig.kind == "Flat" { flat-letters } else { sharp-letters }
    letters.slice(0, sig.count).contains(pitch.letter)
  }
}

#let _key-default-accidental(letter, key) = {
  let sig = _key-accidentals(key)
  if sig.count == 0 {
    "Natural"
  } else {
    let flat-letters = ("B", "E", "A", "D", "G", "C", "F")
    let sharp-letters = ("F", "C", "G", "D", "A", "E", "B")
    let letters = if sig.kind == "Flat" { flat-letters } else { sharp-letters }
    if letters.slice(0, sig.count).contains(letter) { sig.kind } else { "Natural" }
  }
}

#let _key-accidental-step(kind) = accidental-width(kind) + 0.12

#let _key-signature-width(key) = {
  let sig = _key-accidentals(key)
  if sig.count == 0 { 0 } else { sig.count * _key-accidental-step(sig.kind) }
}

#let _draw-key-signature(clef, key, x, bottom-y: 0, unit: 8pt) = {
  let sig = _key-accidentals(key)
  if sig.count > 0 {
    let positions = if sig.kind == "Flat" {
      _flat-order-positions.at(clef)
    } else {
      _sharp-order-positions.at(clef)
    }
    for i in range(sig.count) {
      draw-accidental(
        sig.kind,
        x + i * _key-accidental-step(sig.kind),
        staff-y(positions.at(i), bottom-y: bottom-y),
        unit: unit,
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Prologue (clef + key + time) geometry
// ---------------------------------------------------------------------------

// Absolute x where measure content coordinate 0 begins on the first
// measure of a system.
#let _prologue-start-x(key, time, staff-x: 0, clef-x: 0.35) = {
  let x = clef-x + _clef-advance
  let key-width = _key-signature-width(key)
  if key-width > 0 { x += key-width + _prologue-gap }
  let time-width = time-signature-width(time)
  if time-width > 0 { x += time-width + _prologue-gap }
  x + _content-lead-in
}

#let _draw-prologue(clef, key, time, bottom-y: 0, unit: 8pt, staff-x: 0, clef-x: 0.35) = {
  draw-clef(clef, clef-x, _clef-origin-y(clef, bottom-y: bottom-y), unit: unit)
  let x = clef-x + _clef-advance
  _draw-key-signature(clef, key, x, bottom-y: bottom-y, unit: unit)
  let key-width = _key-signature-width(key)
  if key-width > 0 { x += key-width + _prologue-gap }
  draw-time-signature(time, x, bottom-y: bottom-y, unit: unit)
}

// Mid-score key/time changes shown at the start of a measure.
#let _inline-signature-note-start(measure-start, key, time, show-key, show-time) = {
  let x = measure-start + 0.8
  let has-signature = false
  if show-key {
    let key-width = _key-signature-width(key)
    if key-width > 0 {
      x += key-width + _prologue-gap
      has-signature = true
    }
  }
  if show-time and time != none {
    x += time-signature-width(time) + _prologue-gap
    has-signature = true
  }
  if has-signature { x + 0.4 } else { x }
}

#let _draw-inline-signature(clef, key, time, measure-start, bottom-y: 0, unit: 8pt, show-key: false, show-time: false) = {
  let x = measure-start + 0.8
  if show-key {
    _draw-key-signature(clef, key, x, bottom-y: bottom-y, unit: unit)
    let key-width = _key-signature-width(key)
    if key-width > 0 { x += key-width + _prologue-gap }
  }
  if show-time {
    draw-time-signature(time, x, bottom-y: bottom-y, unit: unit)
  }
}

// ---------------------------------------------------------------------------
// Horizontal spacing: onset-aligned positions shared by all voices
// ---------------------------------------------------------------------------

// Widest accidental this event actually displays.
#let _visible-accidental-width(layout, key) = {
  let width = 0
  for item in layout.pitches {
    let visible-kind = if _key-alters-natural(item.pitch, key) {
      "Natural"
    } else if item.pitch.accidental != "Natural" and not _key-suppresses-accidental(item.pitch, key) {
      item.pitch.accidental
    } else {
      none
    }
    if visible-kind != none {
      width = calc.max(width, accidental-width(visible-kind))
    }
  }
  width
}

// Space needed to the left of the notehead center (accidentals, wider
// whole-note heads).
#let _left-pad(layout, key) = {
  let acc = _visible-accidental-width(layout, key)
  let pad = _head-half-width(layout) - notehead-half-width
  if acc > 0 { pad + acc + _accidental-gap } else { pad }
}

// Space the event's own ink needs to the right of the notehead center.
#let _right-extent(layout, beamed) = {
  let x = _head-half-width(layout)
  if layout.duration.dots > 0 {
    x += _dot-gap-from-head + layout.duration.dots * _dot-step
  }
  if layout.flags > 0 and not beamed {
    x += 1.1
  }
  if layout.rest { x += rest-width(_duration-base(layout)) }
  x
}

// Ideal advance after this event, scaled by duration: longer notes get
// more room, but sub-linearly.
#let _duration-spacing(layout, note-spacing) = {
  let value = layout.duration_value.numerator / layout.duration_value.denominator
  let factor = calc.clamp(calc.pow(value / 0.25, 0.55), 0.55, 2.6)
  note-spacing * factor
}

#let _advance(layout, note-spacing, beamed) = {
  calc.max(_duration-spacing(layout, note-spacing), _right-extent(layout, beamed) + 1.1)
}

#let _is-beamed(layout, beams) = {
  beams and layout.at("beam_group", default: none) != none
}

// Exact integer key for an onset rational (all durations divide 4096).
#let _onset-key(onset) = {
  int(onset.numerator * 4096 / onset.denominator)
}

// Compute shared x positions for every distinct onset across all voices
// of one measure, in content coordinates (0 = measure content start).
// Returns (positions: onset-key -> x, width: total content width).
#let _measure-positions(voices-layouts, note-spacing: 3.1, beams: false, key: "C") = {
  let start = 0
  let onset-keys = ()
  let seen = (:)
  for layouts in voices-layouts {
    if layouts.len() > 0 {
      start = calc.max(start, _left-pad(layouts.first(), key))
    }
    for layout in layouts {
      let k = _onset-key(layout.onset)
      if str(k) not in seen {
        seen.insert(str(k), true)
        onset-keys.push(k)
      }
    }
  }
  onset-keys = onset-keys.sorted()

  // For each onset, the spacing constraints imposed by events that end
  // there: x[onset] >= x[event onset] + advance(event) + left pad of next.
  let ending = (:)
  let end-demands = ()
  for layouts in voices-layouts {
    for i in range(layouts.len()) {
      let layout = layouts.at(i)
      let k1 = _onset-key(layout.onset)
      let adv = _advance(layout, note-spacing, _is-beamed(layout, beams))
      if i + 1 < layouts.len() {
        let next = layouts.at(i + 1)
        let k2 = str(_onset-key(next.onset))
        let demand = (from: k1, distance: adv + _left-pad(next, key))
        if k2 in ending {
          ending.at(k2).push(demand)
        } else {
          ending.insert(k2, (demand,))
        }
      } else {
        // The bar ends after the last event's own ink plus a modest pad,
        // not its full duration-proportional advance.
        let pad = calc.max(
          _right-extent(layout, _is-beamed(layout, beams)) + 1.2,
          note-spacing * 0.6,
        )
        end-demands.push((from: k1, distance: calc.min(adv, pad)))
      }
    }
  }

  let positions = (:)
  let prev-x = none
  for k in onset-keys {
    let x = if prev-x == none { start } else { prev-x + _min-onset-step }
    for demand in ending.at(str(k), default: ()) {
      x = calc.max(x, positions.at(str(demand.from)) + demand.distance)
    }
    positions.insert(str(k), x)
    prev-x = x
  }

  let width = if prev-x == none { 0 } else { prev-x }
  for demand in end-demands {
    width = calc.max(width, positions.at(str(demand.from)) + demand.distance)
  }
  (positions: positions, width: width)
}

// Place one voice's layouts at the shared onset positions.
#let _place-at-positions(layouts, positions, note-start) = {
  layouts.map(layout => (
    layout: layout,
    x: note-start + positions.at(str(_onset-key(layout.onset))),
  ))
}

// Legacy single-voice placement used by `bar` and `note`.
#let _place-layouts(layouts, start-x, note-spacing: 3.1, beams: false, key: "C") = {
  let result = _measure-positions((layouts,), note-spacing: note-spacing, beams: beams, key: key)
  (
    placed: _place-at-positions(layouts, result.positions, start-x),
    width: result.width,
  )
}

// ---------------------------------------------------------------------------
// Drawing single events
// ---------------------------------------------------------------------------

#let _draw-pitch-accidental(pitch, x, y, head-half-width, unit, key: "C", visible-kind: auto) = {
  let visible-kind = if visible-kind != auto {
    visible-kind
  } else if _key-alters-natural(pitch, key) {
    "Natural"
  } else if pitch.accidental != "Natural" and not _key-suppresses-accidental(pitch, key) {
    pitch.accidental
  } else {
    none
  }
  if visible-kind != none {
    let width = accidental-width(visible-kind)
    draw-accidental(visible-kind, x - head-half-width - _accidental-gap - width, y, unit: unit)
  }
}

#let _draw-layout-note(
  layout,
  x: 0,
  bottom-y: 0,
  line-gap: 1.0,
  unit: 8pt,
  suppress-flags: false,
  stem-length-override: none,
  stem-direction-override: none,
  key: "C",
) = {
  if layout.rest {
    draw-rest(_duration-base(layout), x, bottom-y: bottom-y, line-gap: line-gap, unit: unit)
    let dot-x = x + rest-width(_duration-base(layout)) + _dot-gap-from-head
    _draw-dots(dot-x, bottom-y + 2.5 * line-gap, layout.duration.dots, unit: unit)
  } else {
    let positions = layout.pitches.map(p => p.staff_position)
    let y-values = positions.map(p => staff-y(p, bottom-y: bottom-y, line-gap: line-gap))
    let direction = if stem-direction-override == none {
      _stem-direction(positions)
    } else {
      stem-direction-override
    }
    let head-half-width = _head-half-width(layout)

    for (i, item) in layout.pitches.enumerate() {
      let pos = item.staff_position
      let y = y-values.at(i)
      draw-ledger-lines(x, pos, bottom-y: bottom-y, line-gap: line-gap, head-half-width: head-half-width, unit: unit)
      let visible-accidentals = layout.at("visible-accidentals", default: none)
      let visible-kind = if visible-accidentals == none { auto } else { visible-accidentals.at(i) }
      _draw-pitch-accidental(item.pitch, x, y, head-half-width, unit, key: key, visible-kind: visible-kind)
      if layout.notehead == "whole" {
        draw-whole-notehead(x, y, unit: unit)
      } else if layout.notehead == "half" {
        draw-open-notehead(x, y, unit: unit)
      } else {
        draw-filled-notehead(x, y, unit: unit)
      }
      let dot-x = x + head-half-width + _dot-gap-from-head + 0.2
      _draw-dots(dot-x, _dot-y(pos, y, line-gap), layout.duration.dots, unit: unit)
    }

    if layout.stem {
      let low-y = calc.min(..y-values)
      let high-y = calc.max(..y-values)
      let stem-start-y = if direction == "up" { low-y } else { high-y }
      let stem-length = if stem-length-override == none {
        (high-y - low-y) + _default-stem-length
      } else {
        stem-length-override
      }
      draw-stem(x, stem-start-y, direction: direction, length: stem-length, unit: unit)
      if layout.flags > 0 and not suppress-flags {
        let tip = stem-tip(x, stem-start-y, direction: direction, length: stem-length)
        draw-flag(tip.at(0), tip.at(1), direction: direction, count: layout.flags, unit: unit)
      }
    }
  }
}

// Stem tip and geometry of an event, or none for rests/whole notes.
#let _stem-data-for-layout(layout, x, bottom-y: 0, line-gap: 1.0, direction-override: none) = {
  if layout.rest or not layout.stem {
    none
  } else {
    let positions = layout.pitches.map(p => p.staff_position)
    let direction = if direction-override == none { _stem-direction(positions) } else { direction-override }
    let y-values = positions.map(p => staff-y(p, bottom-y: bottom-y, line-gap: line-gap))
    let low-y = calc.min(..y-values)
    let high-y = calc.max(..y-values)
    let stem-start-y = if direction == "up" { low-y } else { high-y }
    let stem-length = (high-y - low-y) + _default-stem-length
    (
      point: stem-tip(x, stem-start-y, direction: direction, length: stem-length),
      direction: direction,
      flags: layout.flags,
    )
  }
}

// ---------------------------------------------------------------------------
// Beam groups
// ---------------------------------------------------------------------------

#let _max-beam-rise = 1.0
#let _ideal-beamed-stem = 3.25
#let _min-beamed-stem = 2.5
#let _beam-stub-length = 0.75

#let _draw-beam-group(group, bottom-y: 0, line-gap: 1.0, unit: 8pt, key: "C") = {
  if group.len() == 0 { return }
  if group.len() == 1 {
    let item = group.first()
    _draw-layout-note(item.layout, x: item.x, bottom-y: bottom-y, unit: unit, key: key)
    return
  }

  let all-positions = ()
  for item in group {
    for pitch in item.layout.pitches {
      all-positions.push(pitch.staff_position)
    }
  }
  let direction = _stem-direction(all-positions)
  let sign = if direction == "up" { 1 } else { -1 }

  let items = group.map(item => {
    let y-values = item.layout.pitches.map(p => staff-y(p.staff_position, bottom-y: bottom-y, line-gap: line-gap))
    (
      layout: item.layout,
      x: item.x,
      sx: item.x + sign * stem-center-dx,
      base-y: if direction == "up" { calc.min(..y-values) } else { calc.max(..y-values) },
      extreme-y: if direction == "up" { calc.max(..y-values) } else { calc.min(..y-values) },
      flags: item.layout.flags,
    )
  })

  // Beam slant follows the first/last outer noteheads, capped to one
  // staff space across the whole group.
  let first = items.first()
  let last = items.last()
  let dx = last.sx - first.sx
  let rise = calc.clamp(last.extreme-y - first.extreme-y, -_max-beam-rise, _max-beam-rise)
  let slope = if dx == 0 { 0 } else { rise / dx }

  // Place the beam so every stem keeps a workable length.
  let intercept = none
  for item in items {
    let by-base = item.base-y + sign * ((item.extreme-y - item.base-y) * sign + _ideal-beamed-stem) - slope * item.sx
    let by-extreme = item.extreme-y + sign * _min-beamed-stem - slope * item.sx
    let candidate = if direction == "up" { calc.max(by-base, by-extreme) } else { calc.min(by-base, by-extreme) }
    intercept = if intercept == none {
      candidate
    } else if direction == "up" {
      calc.max(intercept, candidate)
    } else {
      calc.min(intercept, candidate)
    }
  }

  let beam-y(sx) = slope * sx + intercept

  for item in items {
    let tip-y = beam-y(item.sx)
    let length = sign * (tip-y - item.base-y) - stem-anchor-dy
    _draw-layout-note(
      item.layout,
      x: item.x,
      bottom-y: bottom-y,
      unit: unit,
      suppress-flags: true,
      stem-length-override: length,
      stem-direction-override: direction,
      key: key,
    )
  }

  // Beam centers step inward (toward the noteheads) from the stem tips.
  let level-offset(level) = -sign * (beam-thickness / 2 + level * (beam-thickness + beam-spacing))

  // Full segments between neighbors.
  for i in range(items.len() - 1) {
    let a = items.at(i)
    let b = items.at(i + 1)
    let count = calc.min(a.flags, b.flags)
    for level in range(count) {
      let dy = level-offset(level)
      draw-beam((a.sx, beam-y(a.sx) + dy), (b.sx, beam-y(b.sx) + dy))
    }
  }

  // Stubs for notes with more flags than both neighbors share.
  for i in range(items.len()) {
    let item = items.at(i)
    let left = if i > 0 { calc.min(items.at(i - 1).flags, item.flags) } else { 0 }
    let right = if i + 1 < items.len() { calc.min(items.at(i + 1).flags, item.flags) } else { 0 }
    let covered = calc.max(left, right)
    if item.flags > covered {
      let toward-left = i > 0
      let x2 = if toward-left { item.sx - _beam-stub-length } else { item.sx + _beam-stub-length }
      for level in range(covered, item.flags) {
        let dy = level-offset(level)
        draw-beam((item.sx, beam-y(item.sx) + dy), (x2, beam-y(x2) + dy))
      }
    }
  }
}

// Draw a placed voice, joining beam groups computed by the plugin.
#let _resolve-measure-accidentals(placed, key) = {
  let state = (:)
  let out = ()
  for item in placed {
    let visible = ()
    for pitch in item.layout.pitches {
      let pitch-key = pitch.pitch.letter + str(pitch.pitch.octave)
      let current = state.at(
        pitch-key,
        default: _key-default-accidental(pitch.pitch.letter, key),
      )
      let actual = pitch.pitch.accidental
      if actual == current {
        visible.push(none)
      } else {
        visible.push(actual)
        state.insert(pitch-key, actual)
      }
    }
    out.push((
      x: item.x,
      layout: item.layout + (visible-accidentals: visible,),
    ))
  }
  out
}

#let _draw-placed-sequence(placed, bottom-y: 0, unit: 8pt, beams: false, key: "C") = {
  let placed = _resolve-measure-accidentals(placed, key)
  let i = 0
  while i < placed.len() {
    let item = placed.at(i)
    let group-id = item.layout.at("beam_group", default: none)
    if beams and group-id != none {
      let group = (item,)
      let j = i + 1
      while j < placed.len() and placed.at(j).layout.at("beam_group", default: none) == group-id {
        group.push(placed.at(j))
        j += 1
      }
      _draw-beam-group(group, bottom-y: bottom-y, unit: unit, key: key)
      i = j
    } else {
      _draw-layout-note(item.layout, x: item.x, bottom-y: bottom-y, unit: unit, key: key)
      i += 1
    }
  }
}

// ---------------------------------------------------------------------------
// Event annotations and direction spanners
// ---------------------------------------------------------------------------

#let _annotation-with-prefix(layout, prefix) = {
  for annotation in layout.annotations {
    let raw = str(annotation)
    if raw.starts-with(prefix) {
      return raw.slice(prefix.len())
    }
  }
  none
}

#let _has-annotation(layout, expected) = {
  layout.annotations.any(annotation => str(annotation) == expected)
}

#let _event-top(item, bottom-y: 0, line-gap: 1.0) = {
  if item.layout.rest or item.layout.pitches.len() == 0 {
    bottom-y + 3 * line-gap
  } else {
    let y-values = item.layout.pitches.map(p => staff-y(p.staff_position, bottom-y: bottom-y, line-gap: line-gap))
    let data = _stem-data-for-layout(item.layout, item.x, bottom-y: bottom-y, line-gap: line-gap)
    if data != none and data.direction == "up" {
      calc.max(calc.max(..y-values), data.point.at(1))
    } else {
      calc.max(..y-values)
    }
  }
}

#let _annotation-stem-direction(item, placed, beams: false, bottom-y: 0) = {
  let group-id = item.layout.at("beam_group", default: none)
  if beams and group-id != none {
    let group = placed.filter(candidate => candidate.layout.at("beam_group", default: none) == group-id)
    if group.len() > 1 {
      let positions = ()
      for candidate in group {
        for pitch in candidate.layout.pitches {
          positions.push(pitch.staff_position)
        }
      }
      return _stem-direction(positions)
    }
  }
  let stem = _stem-data-for-layout(item.layout, item.x, bottom-y: bottom-y)
  if stem == none { none } else { stem.direction }
}

#let _articulation-height(kind) = {
  if kind == "staccato" { 0.336 }
  else if kind == "tenuto" { 0.192 }
  else if kind == "staccatissimo" { 1.16 }
  else if kind == "accent" { 0.98 }
  else if kind == "marcato" { 1.016 }
  else { panic("unknown articulation " + kind) }
}

#let _event-articulations(layout) = {
  let marks = ()
  // Near-note duration articulations come first; force accents sit outside.
  if _has-annotation(layout, "stacc") { marks.push("staccato") }
  if _has-annotation(layout, "staccatissimo") { marks.push("staccatissimo") }
  if _has-annotation(layout, "tenuto") or _has-annotation(layout, "legato") { marks.push("tenuto") }
  if _has-annotation(layout, "accent") { marks.push("accent") }
  if _has-annotation(layout, "marcato") or _has-annotation(layout, "strong") { marks.push("marcato") }
  marks
}

#let _draw-placed-annotations(placed, bottom-y: 0, unit: 8pt, beams: false) = {
  import cetz.draw: *
  for item in placed {
    if item.layout.rest { continue }
    let top = _event-top(item, bottom-y: bottom-y)
    let y-values = item.layout.pitches.map(p => staff-y(p.staff_position, bottom-y: bottom-y))
    let articulations = _event-articulations(item.layout)
    let articulation-placement = none
    let articulation-cursor = none
    if articulations.len() > 0 {
      let stem-direction = _annotation-stem-direction(item, placed, beams: beams, bottom-y: bottom-y)
      // The entire stack belongs on the notehead ("bubble") side: below for
      // stem-up notes and above for stem-down notes, including beam groups.
      articulation-placement = if stem-direction == "up" { "below" } else { "above" }
      let sign = if articulation-placement == "above" { 1 } else { -1 }
      articulation-cursor = if articulation-placement == "above" {
        calc.max(..y-values) + 0.64
      } else {
        calc.min(..y-values) - 0.64
      }
      for mark in articulations {
        let mark-height = _articulation-height(mark)
        let mark-y = articulation-cursor + sign * mark-height / 2
        draw-articulation(mark, item.x, mark-y, placement: articulation-placement, unit: unit)
        articulation-cursor += sign * (mark-height + 0.18)
      }
    }
    let fingering = _annotation-with-prefix(item.layout, "f=")
    if fingering != none {
      let fingering-y = if articulations.len() > 0 {
        articulation-cursor
      } else {
        top + 0.72
      }
      let fingering-anchor = if articulations.len() > 0 and articulation-placement == "below" {
        "north"
      } else {
        "south"
      }
      content(
        (item.x, fingering-y),
        text(size: unit * 0.82, weight: "bold", fingering),
        anchor: fingering-anchor,
        padding: 0pt,
      )
    }
    if _has-annotation(item.layout, "turn") or _has-annotation(item.layout, "chromatic-turn") {
      let turn-y = top + 1.22
      draw-ornament-turn(item.x, turn-y, unit: unit, scale: 0.82)
      if _has-annotation(item.layout, "chromatic-turn") {
        draw-accidental("Flat", item.x - 0.72, turn-y + 0.58, unit: unit, scale: 0.38)
        draw-accidental("Natural", item.x + 0.34, turn-y - 0.74, unit: unit, scale: 0.38)
      }
      let turn-fingering = _annotation-with-prefix(item.layout, "turn-f=")
      if turn-fingering != none {
        content(
          (item.x, turn-y + 1.08),
          text(size: unit * 0.62, weight: "bold", turn-fingering),
          anchor: "south",
          padding: 0pt,
        )
      }
    }
    let marking = _annotation-with-prefix(item.layout, "text=")
    if marking != none {
      content(
        (item.x, bottom-y - 1.35),
        text(size: unit * 0.9, style: "italic", marking.replace("_", " ")),
        anchor: "north-west",
        padding: 0pt,
      )
    }
    let below-marking = _annotation-with-prefix(item.layout, "text-below=")
    if below-marking != none {
      content(
        (item.x, bottom-y - 6.8),
        text(size: unit * 0.9, style: "italic", below-marking.replace("_", " ")),
        anchor: "north-west",
        padding: 0pt,
      )
    }
  }
}

#let _collect-pedal-spans(placed-bars) = {
  let open = (:)
  let spans = ()
  for placed in placed-bars {
    for item in placed {
      for annotation in item.layout.annotations {
        let raw = str(annotation)
        if raw.starts-with("p") and raw.ends-with("(") {
          open.insert(raw.slice(0, -1), item.x)
        } else if raw.starts-with("p") and raw.ends-with(")") {
          let id = raw.slice(0, -1)
          if id in open {
            spans.push((start: open.at(id), end: item.x))
            let _ = open.remove(id)
          }
        }
      }
    }
  }
  spans
}

#let _draw-pedal-spans(spans, y, unit: 8pt) = {
  import cetz.draw: *
  let stroke-style = 0.08 * unit + black
  for span in spans {
    draw-pedal-mark(span.start, y, unit: unit)
    let line-start = span.start + 1.72
    let line-end = span.end + 0.55
    if line-end > line-start {
      line((line-start, y), (line-end, y), stroke: stroke-style)
      line((line-end, y), (line-end, y + 0.48), stroke: stroke-style)
    }
  }
}

#let _collect-hairpins(placed-bars) = {
  let open = (:)
  let spans = ()
  for placed in placed-bars {
    for item in placed {
      for annotation in item.layout.annotations {
        let raw = str(annotation)
        if raw.starts-with("h") and (raw.ends-with("<") or raw.ends-with(">")) {
          open.insert(raw.slice(0, -1), (
            x: item.x,
            kind: if raw.ends-with("<") { "crescendo" } else { "diminuendo" },
          ))
        } else if raw.starts-with("h") and raw.ends-with("!") {
          let id = raw.slice(0, -1)
          if id in open {
            let start = open.at(id)
            spans.push((start: start.x, end: item.x, kind: start.kind))
            let _ = open.remove(id)
          }
        }
      }
    }
  }
  spans
}

#let _draw-hairpins(spans, y, unit: 8pt) = {
  for span in spans {
    draw-hairpin((span.start, y), (span.end, y), kind: span.kind, unit: unit)
  }
}

// ---------------------------------------------------------------------------
// Ties
// ---------------------------------------------------------------------------

// Collect tie curves for a flat list of placed events (one voice across
// a whole system). Open ties at the system edge run to the right margin.
#let _collect-ties(placed, bottom-y: 0, line-gap: 1.0, continuation-right-x: none) = {
  let ties = ()
  for i in range(placed.len()) {
    let item = placed.at(i)
    if item.layout.rest or not item.layout.tie_to_next { continue }
    let positions = item.layout.pitches.map(p => p.staff_position)
    let direction = _stem-direction(positions)
    let sign = if direction == "up" { -1 } else { 1 }
    let next = if i + 1 < placed.len() { placed.at(i + 1) } else { none }
    for pitch in item.layout.pitches {
      let y = staff-y(pitch.staff_position, bottom-y: bottom-y, line-gap: line-gap)
      let start-x = item.x + _head-half-width(item.layout) + 0.2
      let end-x = if next != none {
        next.x - _head-half-width(next.layout) - 0.2
      } else if continuation-right-x != none {
        continuation-right-x
      } else {
        none
      }
      if end-x != none and end-x > start-x + 0.2 {
        let height = sign * calc.clamp((end-x - start-x) * 0.18, 0.4, 0.8)
        ties.push((
          start: (start-x, y + sign * 0.55),
          end: (end-x, y + sign * 0.55),
          height: height,
        ))
      }
    }
  }
  ties
}

#let _draw-ties(ties) = {
  for tie in ties {
    draw-curve(tie.start, tie.end, height: tie.height, thickness: 0.18)
  }
}

// ---------------------------------------------------------------------------
// Slurs
// ---------------------------------------------------------------------------

// Slurs arc above the staff: above the stem tip for stem-up notes,
// above the top notehead otherwise.
#let _slur-anchor(item, bottom-y: 0, line-gap: 1.0) = {
  if item.layout.rest or item.layout.pitches.len() == 0 {
    none
  } else {
    let data = _stem-data-for-layout(item.layout, item.x, bottom-y: bottom-y, line-gap: line-gap)
    if data != none and data.direction == "up" {
      (data.point.at(0) - 0.2, data.point.at(1) + 0.4)
    } else {
      let y-values = item.layout.pitches.map(p => staff-y(p.staff_position, bottom-y: bottom-y, line-gap: line-gap))
      (item.x, calc.max(..y-values) + 0.8)
    }
  }
}

#let _slur-overlaps(a, b) = {
  a.start.at(0) < b.end.at(0) and b.start.at(0) < a.end.at(0)
}

#let _validate-staff-slurs(layout-bars, staff-name) = {
  let open = (:)
  for bar-index in range(layout-bars.len()) {
    for layout in layout-bars.at(bar-index) {
      for annotation in layout.annotations {
        let raw = str(annotation)
        if raw.starts-with("s") and raw.ends-with("(") {
          let id = raw.slice(0, -1)
          if id in open {
            panic("typed-scores error: slur " + id + " opened twice in " + staff-name + " bar " + str(bar-index + 1))
          }
          open.insert(id, bar-index + 1)
        } else if raw.starts-with("s") and raw.ends-with(")") {
          let id = raw.slice(0, -1)
          if id not in open {
            panic("typed-scores error: slur " + id + " closes without opening in " + staff-name + " bar " + str(bar-index + 1))
          }
          let _ = open.remove(id)
        }
      }
    }
  }
  if open.len() > 0 {
    panic("typed-scores error: slur " + open.keys().first() + " was opened but never closed in " + staff-name)
  }
}

// Collect slurs for one staff across a system. Slurs that continue past
// the system run to the continuation edges.
#let _collect-system-slurs(
  placed-bars,
  staff-name,
  bottom-y: 0,
  continuation-left-x: 0,
  continuation-right-x: 0,
) = {
  let open = (:)
  let closed = ()
  for bar-index in range(placed-bars.len()) {
    for item in placed-bars.at(bar-index) {
      let anchor = _slur-anchor(item, bottom-y: bottom-y)
      if anchor != none {
        for annotation in item.layout.annotations {
          let raw = str(annotation)
          if raw.starts-with("s") and raw.ends-with("(") {
            let id = raw.slice(0, -1)
            open.insert(id, (point: anchor, bar: bar-index + 1))
          } else if raw.starts-with("s") and raw.ends-with(")") {
            let id = raw.slice(0, -1)
            if id in open {
              let start = open.at(id)
              let _ = open.remove(id)
              closed.push((
                id: id,
                start: start.point,
                end: anchor,
                span: anchor.at(0) - start.point.at(0),
              ))
            } else {
              closed.push((
                id: id,
                start: (continuation-left-x, anchor.at(1)),
                end: anchor,
                span: anchor.at(0) - continuation-left-x,
              ))
            }
          }
        }
      }
    }
  }
  for id in open.keys() {
    let start = open.at(id)
    closed.push((
      id: id,
      start: start.point,
      end: (continuation-right-x, start.point.at(1)),
      span: continuation-right-x - start.point.at(0),
    ))
  }
  closed
}

// `obstacles` lists (x, y) points the arcs must clear: the slur anchors
// of every event on the staff.
#let _draw-slurs(slurs, obstacles: ()) = {
  for slur in slurs {
    let longer-rank = 0
    for other in slurs {
      if _slur-overlaps(slur, other) and other.span < slur.span {
        longer-rank += 1
      }
    }
    let height = calc.clamp(slur.span * 0.09, 0.6, 1.8)
    // Raise the single quadratic parabola until it clears every interior
    // obstacle. Its chord-relative clearance is exactly 4 h t (1-t).
    let (sx, sy) = slur.start
    let (ex, ey) = slur.end
    for (ox, oy) in obstacles {
      if ox > sx + 0.5 and ox < ex - 0.5 {
        let t = (ox - sx) / (ex - sx)
        let baseline = sy + (ey - sy) * t
        let needed = (oy - baseline) / (4 * t * (1 - t))
        height = calc.max(height, calc.min(needed, 4.0))
      }
    }
    height += longer-rank * 0.4
    // A centered peak must clear the higher endpoint as well. Keep enough
    // clearance for the midpoint to read as the unique peak at normal sizes.
    let endpoint-difference = if sy > ey { sy - ey } else { ey - sy }
    height = calc.max(height, endpoint-difference / 2 + 0.5)
    draw-curve(slur.start, slur.end, height: height, thickness: 0.22)
  }
}

// ---------------------------------------------------------------------------
// Rational helpers for bar validation
// ---------------------------------------------------------------------------

#let _gcd(a, b) = {
  if b == 0 { a } else { _gcd(b, calc.rem(a, b)) }
}

#let _rational(n, d) = {
  let g = _gcd(n, d)
  (numerator: n / g, denominator: d / g)
}

#let _rational-add(a, b) = {
  _rational(
    a.numerator * b.denominator + b.numerator * a.denominator,
    a.denominator * b.denominator,
  )
}

#let _rational-eq(a, b) = {
  a.numerator * b.denominator == b.numerator * a.denominator
}

#let _format-rational(value) = {
  if value.denominator == 1 {
    str(value.numerator)
  } else {
    str(value.numerator) + "/" + str(value.denominator)
  }
}

#let _parse-time-rational(time) = {
  if time == none {
    none
  } else {
    let parts = time.split("/")
    if parts.len() != 2 {
      panic("typed-scores error: time signature must look like 4/4 or 12/8, got " + repr(time))
    }
    let numerator = int(parts.at(0))
    let denominator = int(parts.at(1))
    if numerator <= 0 or denominator <= 0 {
      panic("typed-scores error: time signature values must be positive, got " + repr(time))
    }
    _rational(numerator, denominator)
  }
}

#let _duration-sum(layouts) = {
  let total = (numerator: 0, denominator: 1)
  for layout in layouts {
    total = _rational-add(total, layout.duration_value)
  }
  total
}

#let _validate-bar-duration(layouts, time, staff-name, bar-number) = {
  let expected = _parse-time-rational(time)
  if expected != none {
    let actual = _duration-sum(layouts)
    if not _rational-eq(actual, expected) {
      panic(
        "typed-scores error in "
          + staff-name
          + " bar "
          + str(bar-number)
          + ": durations sum to "
          + _format-rational(actual)
          + ", expected "
          + time
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Score input normalization
// ---------------------------------------------------------------------------

#let _bar-count-label(count) = {
  str(count) + if count == 1 { " bar" } else { " bars" }
}

#let _required-string(value, label) = {
  if type(value) != str {
    panic("typed-scores error: " + label + " must be a string")
  }
  value
}

#let _as-bar-array(value, label) = {
  if type(value) == str {
    (value,)
  } else if type(value) == array {
    value
  } else {
    panic("typed-scores error: " + label + " must be a string or an array of bars")
  }
}

#let _bar-item-notes(item, label, index) = {
  if type(item) == str {
    item
  } else if type(item) == dictionary {
    let notes = item.at("notes", default: none)
    if notes == none {
      panic("typed-scores error: " + label + " bar " + str(index) + " is missing a notes field")
    }
    _required-string(notes, label + " bar " + str(index) + " notes")
  } else {
    panic("typed-scores error: " + label + " bar " + str(index) + " must be a string or dictionary")
  }
}

#let _bar-item-field(item, field) = {
  if type(item) == dictionary {
    item.at(field, default: none)
  } else {
    none
  }
}

#let _voice-id(voice, index) = {
  if type(voice) == dictionary and voice.at("name", default: none) != none {
    voice.name
  } else {
    "voice " + str(index)
  }
}

#let _voice-clef(voice, index) = {
  if type(voice) != dictionary {
    panic("typed-scores error: voice " + str(index) + " must be a dictionary")
  }
  voice.at("clef", default: "treble")
}

#let _voice-notes-array(voice, index) = {
  if type(voice) != dictionary {
    panic("typed-scores error: voice " + str(index) + " must be a dictionary")
  }
  let notes = voice.at("notes", default: none)
  if notes == none {
    panic("typed-scores error: voice " + str(index) + " is missing a notes field")
  }
  _as-bar-array(notes, "voice " + str(index) + " notes")
}

#let _normalize-voices-bars(voices, key, time) = {
  if type(voices) != array or voices.len() == 0 {
    panic("typed-scores error: voices must be a non-empty array")
  }
  let voice-specs = ()
  let bar-count = none
  for i in range(voices.len()) {
    let voice = voices.at(i)
    let notes = _voice-notes-array(voice, i + 1)
    if bar-count == none {
      bar-count = notes.len()
    } else if notes.len() != bar-count {
      panic(
        "typed-scores error: "
          + _voice-id(voice, i + 1)
          + " has "
          + _bar-count-label(notes.len())
          + " but expected "
          + _bar-count-label(bar-count)
      )
    }
    voice-specs.push((
      id: _voice-id(voice, i + 1),
      clef: _voice-clef(voice, i + 1),
      notes: notes,
    ))
  }

  let out = ()
  let current-key = key
  let current-time = time
  for bar-index in range(bar-count) {
    let bar-key = none
    let bar-time = none
    let bar-partial = none
    for spec in voice-specs {
      let item = spec.notes.at(bar-index)
      let item-key = _bar-item-field(item, "key")
      let item-time = _bar-item-field(item, "time")
      let item-partial = _bar-item-field(item, "partial")
      if item-key != none {
        if bar-key != none and bar-key != item-key {
          panic("typed-scores error: conflicting key changes in bar " + str(bar-index + 1))
        }
        bar-key = item-key
      }
      if item-time != none {
        if bar-time != none and bar-time != item-time {
          panic("typed-scores error: conflicting time changes in bar " + str(bar-index + 1))
        }
        bar-time = item-time
      }
      if item-partial != none {
        if bar-partial != none and bar-partial != item-partial {
          panic("typed-scores error: conflicting partial durations in bar " + str(bar-index + 1))
        }
        bar-partial = item-partial
      }
    }
    current-key = if bar-key != none { bar-key } else { current-key }
    current-time = if bar-time != none { bar-time } else { current-time }
    let bar-voices = ()
    for spec in voice-specs {
      let item = spec.notes.at(bar-index)
      bar-voices.push((
        id: spec.id,
        clef: spec.clef,
        notes: _bar-item-notes(item, spec.id, bar-index + 1),
      ))
    }
    out.push((key: current-key, time: current-time, partial: bar-partial, voices: bar-voices))
  }
  out
}

#let _legacy-voices-from-treble-bass(treble, bass) = (
  (name: "treble", clef: "treble", notes: treble),
  (name: "bass", clef: "bass", notes: bass),
)

#let _normalize-parallel-bars(treble, bass, key, time) = {
  if treble == none or bass == none {
    panic("typed-scores error: score needs voices, sections, treble/bass, or a bars array")
  }
  _normalize-voices-bars(_legacy-voices-from-treble-bass(treble, bass), key, time)
}

#let _normalize-measure-bars(bars, key, time) = {
  if type(bars) != array {
    panic("typed-scores error: bars must be an array of measure dictionaries")
  }
  let out = ()
  let current-key = key
  let current-time = time
  for i in range(bars.len()) {
    let item = bars.at(i)
    if type(item) != dictionary {
      panic("typed-scores error: bars item " + str(i + 1) + " must be a dictionary")
    }
    current-key = item.at("key", default: current-key)
    current-time = item.at("time", default: current-time)
    let item-voices = if item.at("voices", default: none) != none {
      item.voices
    } else if item.at("treble", default: none) != none and item.at("bass", default: none) != none {
      _legacy-voices-from-treble-bass(item.treble, item.bass)
    } else {
      panic("typed-scores error: bars item " + str(i + 1) + " needs voices or treble/bass fields")
    }
    if type(item-voices) != array or item-voices.len() == 0 {
      panic("typed-scores error: bars item " + str(i + 1) + " voices must be a non-empty array")
    }
    let bar-voices = ()
    for voice-index in range(item-voices.len()) {
      let voice = item-voices.at(voice-index)
      let voice-notes = voice.at("notes", default: none)
      if voice-notes == none {
        panic("typed-scores error: bars item " + str(i + 1) + " voice " + str(voice-index + 1) + " is missing notes")
      }
      bar-voices.push((
        id: _voice-id(voice, voice-index + 1),
        clef: _voice-clef(voice, voice-index + 1),
        notes: _required-string(voice-notes, "bars item " + str(i + 1) + " voice " + str(voice-index + 1) + " notes"),
      ))
    }
    out.push((
      key: current-key,
      time: current-time,
      partial: item.at("partial", default: none),
      voices: bar-voices,
    ))
  }
  out
}

#let _tempo-text(tempo, bpm) = {
  if tempo == none and bpm == none { none }
  else if tempo != none and bpm != none { str(tempo) + " = " + str(bpm) }
  else if tempo != none { tempo }
  else { str(bpm) }
}

#let _with-first-tempo(bars, tempo) = {
  if tempo != none and bars.len() > 0 {
    let _ = bars.at(0).insert("tempo", tempo)
  }
  bars
}

#let _normalize-sections(sections, key, time, tempo) = {
  if type(sections) != array or sections.len() == 0 {
    panic("typed-scores error: sections must be a non-empty array")
  }
  let out = ()
  let current-key = key
  let current-time = time
  let current-tempo = tempo
  for section-index in range(sections.len()) {
    let section = sections.at(section-index)
    if type(section) != dictionary {
      panic("typed-scores error: section " + str(section-index + 1) + " must be a dictionary")
    }
    current-key = section.at("key", default: current-key)
    current-time = section.at("time", default: current-time)
    current-tempo = section.at("tempo", default: current-tempo)
    let section-bars = if section.at("voices", default: none) != none {
      _normalize-voices-bars(section.voices, current-key, current-time)
    } else if section.at("bars", default: none) != none {
      _normalize-measure-bars(section.bars, current-key, current-time)
    } else if section.at("treble", default: none) != none and section.at("bass", default: none) != none {
      _normalize-parallel-bars(section.treble, section.bass, current-key, current-time)
    } else {
      panic("typed-scores error: section " + str(section-index + 1) + " needs voices, bars, or treble/bass")
    }
    for bar in _with-first-tempo(section-bars, current-tempo) {
      out.push(bar)
      current-key = bar.key
      current-time = bar.time
    }
  }
  out
}

#let _normalize-grand-bars(voices, sections, treble, bass, bars, key, time, tempo) = {
  if sections != none {
    _normalize-sections(sections, key, time, tempo)
  } else if voices != none {
    _with-first-tempo(_normalize-voices-bars(voices, key, time), tempo)
  } else if bars != none {
    _with-first-tempo(_normalize-measure-bars(bars, key, time), tempo)
  } else {
    _with-first-tempo(_normalize-parallel-bars(treble, bass, key, time), tempo)
  }
}

// Parse, validate, and pre-compute shared positions for every measure.
#let _prepare-grand-measures(voices, sections, treble, bass, bars, key, time, tempo, note-spacing: 3.1, beams: false) = {
  let raw-bars = _normalize-grand-bars(voices, sections, treble, bass, bars, key, time, tempo)
  let out = ()
  let previous-key = none
  let previous-time = none
  let voice-shape = none
  for i in range(raw-bars.len()) {
    let item = raw-bars.at(i)
    let validation-time = item.at("partial", default: none)
    if validation-time == none {
      validation-time = item.time
    }
    let prepared-voices = ()
    for voice in item.voices {
      let layouts = _layout-sequence(voice.notes, clef: voice.clef, time: validation-time)
      _validate-bar-duration(layouts, validation-time, voice.id, i + 1)
      prepared-voices.push((
        id: voice.id,
        clef: voice.clef,
        notes: voice.notes,
        layouts: layouts,
      ))
    }
    let current-shape = prepared-voices.map(voice => (id: voice.id, clef: voice.clef))
    if voice-shape == none {
      voice-shape = current-shape
    } else if current-shape != voice-shape {
      panic("typed-scores error: bar " + str(i + 1) + " changes the score voice structure; start a separate score for a different ensemble")
    }
    let spacing = _measure-positions(
      prepared-voices.map(voice => voice.layouts),
      note-spacing: note-spacing,
      beams: beams,
      key: item.key,
    )
    out.push((
      key: item.key,
      time: item.time,
      partial: item.at("partial", default: none),
      tempo: item.at("tempo", default: none),
      voices: prepared-voices,
      positions: spacing.positions,
      content-width: spacing.width,
      show-key: i == 0 or item.key != previous-key,
      show-time: i == 0 or item.time != previous-time,
    ))
    previous-key = item.key
    previous-time = item.time
  }
  out
}

// ---------------------------------------------------------------------------
// System packing and rendering
// ---------------------------------------------------------------------------

#let _min-staff-position(layouts) = {
  let out = none
  for layout in layouts {
    for item in layout.pitches {
      if out == none or item.staff_position < out {
        out = item.staff_position
      }
    }
  }
  if out == none { 2 } else { out }
}

#let _max-staff-position(layouts) = {
  let out = none
  for layout in layouts {
    for item in layout.pitches {
      if out == none or item.staff_position > out {
        out = item.staff_position
      }
    }
  }
  if out == none { 10 } else { out }
}

#let _measure-width-in-system(measure, is-system-start, staff-x, clef-x) = {
  let prefix = if is-system-start {
    _prologue-start-x(measure.key, measure.time, staff-x: staff-x, clef-x: clef-x) - staff-x
  } else {
    _inline-signature-note-start(0, measure.key, measure.time, measure.at("show-key"), measure.at("show-time"))
  }
  calc.max(4.5, prefix + measure.content-width)
}

#let _pack-score-systems(measures, max-width, staff-x, clef-x) = {
  let systems = ()
  let system-start = 0
  let system-widths = ()
  let system-width = staff-x

  for i in range(measures.len()) {
    let measure = measures.at(i)
    let is-first-in-system = system-widths.len() == 0
    let measure-width = _measure-width-in-system(measure, is-first-in-system, staff-x, clef-x)
    let would-overflow = system-width + measure-width > max-width
    let can-break = i > system-start
    if would-overflow and can-break {
      systems.push((start: system-start, widths: system-widths, width: system-width))
      system-start = i
      system-widths = ()
      let first-width = _measure-width-in-system(measure, true, staff-x, clef-x)
      system-widths.push(first-width)
      system-width = staff-x + first-width
    } else {
      system-widths.push(measure-width)
      system-width += measure-width
    }
  }

  if system-widths.len() > 0 {
    systems.push((start: system-start, widths: system-widths, width: system-width))
  }
  systems
}

#let _render-grand-system(
  measures,
  system,
  unit,
  beams: false,
  staff-gap: none,
  composer: none,
) = {
  let system-measures = measures.slice(system.start, system.start + system.widths.len())
  let voice-count = system-measures.first().voices.len()
  let voice-layouts = ()
  for voice-index in range(voice-count) {
    let layouts = ()
    for measure in system-measures {
      for layout in measure.voices.at(voice-index).layouts {
        layouts.push(layout)
      }
    }
    voice-layouts.push(layouts)
  }

  // Stack staves bottom-up, leaving room for ledger-line excursions.
  // Extents are relative to each staff's own bottom line.
  let bottom-map = (:)
  let current-bottom = 0
  let lower-rel-high = calc.max(4, staff-y(_max-staff-position(voice-layouts.last())))
  bottom-map.insert(str(voice-count - 1), current-bottom)
  if voice-count > 1 {
    for voice-index in range(voice-count - 2, -1, step: -1) {
      let upper-rel-low = calc.min(0, staff-y(_min-staff-position(voice-layouts.at(voice-index))))
      let gap = if staff-gap == none { calc.max(7, lower-rel-high - upper-rel-low + 2.2) } else { staff-gap }
      current-bottom += gap
      bottom-map.insert(str(voice-index), current-bottom)
      lower-rel-high = calc.max(4, staff-y(_max-staff-position(voice-layouts.at(voice-index))))
    }
  }
  let system-bottom = bottom-map.at(str(voice-count - 1))
  let system-top = bottom-map.at("0") + 4
  let left-bar-x = _grand-brace-x + _brace-width / 2 + _grand-brace-to-bar-gap
  let clef-x = left-bar-x + _grand-clef-after-bar-gap
  let system-width = system.width

  let measure-starts = ()
  let current-x = left-bar-x
  for measure-width in system.widths {
    measure-starts.push(current-x)
    current-x += measure-width
  }

  let placed-by-voice = ()
  for _ in range(voice-count) {
    placed-by-voice.push(())
  }
  let continuation-left-x = _prologue-start-x(
    system-measures.first().key,
    system-measures.first().time,
    staff-x: left-bar-x,
    clef-x: clef-x,
  ) - 0.7
  let continuation-right-x = system-width - 0.85
  for i in range(system-measures.len()) {
    let measure = system-measures.at(i)
    let measure-start = measure-starts.at(i)
    let note-start = if i == 0 {
      _prologue-start-x(measure.key, measure.time, staff-x: left-bar-x, clef-x: clef-x)
    } else {
      _inline-signature-note-start(measure-start, measure.key, measure.time, measure.at("show-key"), measure.at("show-time"))
    }
    for voice-index in range(voice-count) {
      let voice = measure.voices.at(voice-index)
      placed-by-voice.at(voice-index).push(
        _place-at-positions(voice.layouts, measure.positions, note-start)
      )
    }
  }
  let slurs-by-voice = ()
  for voice-index in range(voice-count) {
    slurs-by-voice.push(_collect-system-slurs(
      placed-by-voice.at(voice-index),
      system-measures.first().voices.at(voice-index).id,
      bottom-y: bottom-map.at(str(voice-index)),
      continuation-left-x: continuation-left-x,
      continuation-right-x: continuation-right-x,
    ))
  }

  music-canvas(length: unit, {
    if voice-count == 2 {
      draw-grand-brace(_grand-brace-x, system-bottom, system-top, unit: unit)
    } else if voice-count > 2 {
      draw-staff-bracket(left-bar-x - 0.34, system-bottom, system-top, unit: unit)
    }
    for voice-index in range(voice-count) {
      draw-staff-lines(system-width - left-bar-x, x: left-bar-x, bottom-y: bottom-map.at(str(voice-index)), unit: unit)
    }

    if composer != none {
      import cetz.draw: *
      content(
        (system-width - 0.3, system-top + 5.4),
        text(size: unit * 1.18, composer),
        anchor: "east",
        padding: 0pt,
      )
    }

    for i in range(system-measures.len()) {
      let measure = system-measures.at(i)
      let measure-start = measure-starts.at(i)
      if measure.tempo != none {
        import cetz.draw: *
        content(
          (measure-start + 0.4, system-top + 4.6),
          text(size: unit * 1.25, style: "italic", measure.tempo),
          anchor: "west",
          padding: 0pt,
        )
      }
      for voice-index in range(voice-count) {
        let voice = measure.voices.at(voice-index)
        let bottom-y = bottom-map.at(str(voice-index))
        if i == 0 {
          _draw-prologue(voice.clef, measure.key, measure.time, bottom-y: bottom-y, unit: unit, staff-x: left-bar-x, clef-x: clef-x)
        } else {
          _draw-inline-signature(
            voice.clef,
            measure.key,
            measure.time,
            measure-start,
            bottom-y: bottom-y,
            unit: unit,
            show-key: measure.at("show-key"),
            show-time: measure.at("show-time"),
          )
        }
        _draw-placed-sequence(placed-by-voice.at(voice-index).at(i), bottom-y: bottom-y, unit: unit, beams: beams, key: measure.key)
        _draw-placed-annotations(placed-by-voice.at(voice-index).at(i), bottom-y: bottom-y, unit: unit, beams: beams)
      }
    }

    for voice-index in range(voice-count) {
      let bottom-y = bottom-map.at(str(voice-index))
      let placed-flat = ()
      for placed in placed-by-voice.at(voice-index) {
        placed-flat += placed
      }
      _draw-ties(_collect-ties(
        placed-flat,
        bottom-y: bottom-y,
        continuation-right-x: continuation-right-x,
      ))
      let obstacles = placed-flat
        .map(item => _slur-anchor(item, bottom-y: bottom-y))
        .filter(anchor => anchor != none)
      _draw-slurs(slurs-by-voice.at(voice-index), obstacles: obstacles)
      _draw-hairpins(
        _collect-hairpins(placed-by-voice.at(voice-index)),
        bottom-y - 3.2,
        unit: unit,
      )
      _draw-pedal-spans(
        _collect-pedal-spans(placed-by-voice.at(voice-index)),
        bottom-y - 7.0,
        unit: unit,
      )
    }

    import cetz.draw: *
    let thin = thin-barline-thickness * unit + black
    if voice-count == 2 {
      line((left-bar-x, system-bottom), (left-bar-x, system-top), stroke: thin)
    } else {
      for voice-index in range(voice-count) {
        let bottom-y = bottom-map.at(str(voice-index))
        line((left-bar-x, bottom-y), (left-bar-x, bottom-y + 4), stroke: thin)
      }
    }
    let bar-x = left-bar-x
    for measure-width in system.widths {
      bar-x += measure-width
      if voice-count == 2 {
        line((bar-x, system-bottom), (bar-x, system-top), stroke: thin)
      } else {
        for voice-index in range(voice-count) {
          let bottom-y = bottom-map.at(str(voice-index))
          line((bar-x, bottom-y), (bar-x, bottom-y + 4), stroke: thin)
        }
      }
    }
  })
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

#let note(
  note-str,
  clef: "treble",
  width: none,
  scale: 1.0,
) = context {
  let layout = _layout-note(note-str, clef: clef)
  let unit = 8pt * scale
  let start-x = _prologue-start-x(none, none, clef-x: 0.35)
  let x = start-x + _left-pad(layout, "C")
  let staff-width = if width == none {
    x + _advance(layout, 3.1, false)
  } else {
    width
  }
  music-canvas(length: unit, {
    draw-staff-lines(staff-width, unit: unit)
    draw-clef(clef, 0.35, _clef-origin-y(clef), unit: unit)
    _draw-layout-note(layout, x: x, unit: unit)
    _draw-placed-annotations(((layout: layout, x: x),), unit: unit)
  })
}

#let bar(
  sequence-str,
  clef: "treble",
  width: none,
  scale: 1.0,
  note-spacing: 3.1,
  beams: false,
  key: "C",
  time: none,
) = context {
  let unit = 8pt * scale
  let layouts = _layout-sequence(sequence-str, clef: clef, time: time)
  _validate-bar-duration(layouts, time, clef, 1)
  _validate-staff-slurs((layouts,), clef)
  let start-x = _prologue-start-x(key, time, clef-x: 0.35)
  let result = _place-layouts(layouts, start-x, note-spacing: note-spacing, beams: beams, key: key)
  let bar-width = if width == none {
    calc.max(6, start-x + result.width + 0.6)
  } else {
    width
  }
  music-canvas(length: unit, {
    draw-staff-lines(bar-width, unit: unit)
    _draw-prologue(clef, key, time, unit: unit, clef-x: 0.35)
    _draw-placed-sequence(result.placed, unit: unit, beams: beams, key: key)
    _draw-placed-annotations(result.placed, unit: unit, beams: beams)
    _draw-ties(_collect-ties(result.placed, continuation-right-x: bar-width - 0.3))
    _draw-slurs(
      _collect-system-slurs(
        (result.placed,),
        clef,
        continuation-left-x: start-x - 0.7,
        continuation-right-x: bar-width - 0.5,
      ),
      obstacles: result.placed
        .map(item => _slur-anchor(item))
        .filter(anchor => anchor != none),
    )
    _draw-hairpins(_collect-hairpins((result.placed,)), -3.2, unit: unit)
    _draw-pedal-spans(_collect-pedal-spans((result.placed,)), -7.0, unit: unit)
    import cetz.draw: *
    line((bar-width, 0), (bar-width, 4), stroke: thin-barline-thickness * unit + black)
  })
}

#let staff(
  body: none,
  clef: "treble",
  note-str: "C4:q",
  width: none,
  scale: 1.0,
) = {
  if body != none {
    panic("staff body parsing starts in Layer 5; use note-str for the Layer 1/2 renderer")
  }
  note(note-str, clef: clef, width: width, scale: scale)
}

#let score(
  body: none,
  note-str: "C4:q",
  clef: "treble",
  voices: none,
  sections: none,
  treble: none,
  bass: none,
  bars: none,
  key: "C",
  time: "4/4",
  tempo: none,
  bpm: none,
  composer: none,
  width: none,
  scale: 1.0,
  note-spacing: 3.1,
  beams: false,
  staff-gap: none,
  wrap: true,
  system-gap: 1.2em,
) = {
  if voices == none and sections == none and treble == none and bass == none and bars == none {
    if body != none {
      panic("score body parsing starts after Layer 2; use note-str, voices, sections, or treble/bass/bars arguments")
    }
    note(note-str, clef: clef, width: width, scale: scale)
  } else {
    if body != none {
      panic("score body parsing starts in Layer 6; use voices, sections, treble/bass arrays, or bars dictionaries for the current renderer")
    }
    layout(size => context {
      let unit = 8pt * scale
      let measures = _prepare-grand-measures(
        voices, sections, treble, bass, bars, key, time,
        _tempo-text(tempo, bpm),
        note-spacing: note-spacing,
        beams: beams,
      )
      let voice-count = measures.first().voices.len()
      for voice-index in range(voice-count) {
        _validate-staff-slurs(
          measures.map(measure => measure.voices.at(voice-index).layouts),
          measures.first().voices.at(voice-index).id,
        )
      }
      let left-bar-x = _grand-brace-x + _brace-width / 2 + _grand-brace-to-bar-gap
      let clef-x = left-bar-x + _grand-clef-after-bar-gap
      let max-width = if width == none { size.width / unit } else { width }
      let systems = if wrap {
        _pack-score-systems(measures, max-width, left-bar-x, clef-x)
      } else {
        let widths = ()
        let system-width = left-bar-x
        for i in range(measures.len()) {
          let measure-width = _measure-width-in-system(measures.at(i), i == 0, left-bar-x, clef-x)
          widths.push(measure-width)
          system-width += measure-width
        }
        ((start: 0, widths: widths, width: system-width),)
      }

      for i in range(systems.len()) {
        _render-grand-system(
          measures,
          systems.at(i),
          unit,
          beams: beams,
          staff-gap: staff-gap,
          composer: if i == 0 { composer } else { none },
        )
        if i + 1 < systems.len() {
          v(system-gap)
        }
      }
    })
  }
}

#let grand-staff(
  body: none,
  treble: none,
  bass: none,
  bars: none,
  key: "C",
  time: "4/4",
  width: none,
  scale: 1.0,
  note-spacing: 3.1,
  beams: false,
  staff-gap: none,
  wrap: true,
  system-gap: 1.2em,
  composer: none,
) = {
  score(
    body: body,
    treble: treble,
    bass: bass,
    bars: bars,
    key: key,
    time: time,
    tempo: none,
    composer: composer,
    width: width,
    scale: scale,
    note-spacing: note-spacing,
    beams: beams,
    staff-gap: staff-gap,
    wrap: wrap,
    system-gap: system-gap,
  )
}

#let voice(body: none, id: 1) = {
  panic("voice starts in Layer 5")
}

#let pedal(from: none, to: none) = {
  panic("pedal marks start in Layer 5")
}
