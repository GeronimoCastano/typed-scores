//! Turn the normalized model into `score()` arguments, as JSON for Typst and
//! as equivalent Typst source that users can copy and edit.

use super::collections::Map;

use super::frac::Frac;
use super::model::*;

const INVISIBLE_HARMONY: &str = "\u{200B}";

/// A Typst value: the argument tree of a `score()` call.
#[derive(Clone, Debug)]
pub enum Val {
    Str(String),
    Int(i64),
    Bool(bool),
    Arr(Vec<Val>),
    /// Entries in order; `trailing` keeps a comma after the last entry in source.
    Dict(Vec<(String, Val)>, bool),
}

fn string(value: impl Into<String>) -> Val {
    Val::Str(value.into())
}

fn dict(entries: Vec<(&str, Val)>) -> Val {
    Val::Dict(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect(), false)
}

pub fn typst_string(value: &str) -> String {
    let mut quoted = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => quoted.push_str("\\\\"),
            '"' => quoted.push_str("\\\""),
            '\u{200B}' => quoted.push_str("\\u{200B}"),
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

impl Val {
    pub fn to_typst(&self) -> String {
        match self {
            Val::Str(value) => typst_string(value),
            Val::Int(value) => value.to_string(),
            Val::Bool(value) => value.to_string(),
            Val::Arr(items) => {
                let inner: Vec<String> = items.iter().map(Val::to_typst).collect();
                if items.len() == 1 { format!("({},)", inner[0]) } else { format!("({})", inner.join(", ")) }
            }
            Val::Dict(entries, trailing) => {
                let inner: Vec<String> = entries.iter().map(|(key, value)| format!("{key}: {}", value.to_typst())).collect();
                format!("({}{})", inner.join(", "), if *trailing { "," } else { "" })
            }
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            Val::Str(value) => serde_json::to_string(value).unwrap_or_else(|_| "\"\"".into()),
            Val::Int(value) => value.to_string(),
            Val::Bool(value) => value.to_string(),
            Val::Arr(items) => format!("[{}]", items.iter().map(Val::to_json).collect::<Vec<_>>().join(",")),
            Val::Dict(entries, _) => format!(
                "{{{}}}",
                entries
                    .iter()
                    .map(|(key, value)| format!("{}:{}", serde_json::to_string(key).unwrap_or_default(), value.to_json()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

fn beat_unit(time: &str) -> Frac {
    let (numerator, denominator) = time.split_once('/').unwrap_or(("4", "4"));
    let numerator: i64 = numerator.parse().unwrap_or(4);
    let denominator: i64 = denominator.parse().unwrap_or(4);
    if numerator > 3 && numerator % 3 == 0 && denominator >= 8 {
        Frac::new(3, denominator)
    } else {
        Frac::new(1, denominator)
    }
}

/// '-' or '/' before each event whose source beaming differs from the
/// parser's automatic beams, keyed by event index.
fn beam_markers(events: &[Event], beat: Frac) -> Map<usize, char> {
    let mut markers = Map::new();
    let mut group_beat: Option<i64> = None;
    let mut previous: Option<&Event> = None;
    for (index, event) in events.iter().enumerate() {
        if !event.flagged() {
            group_beat = None;
            previous = None;
            continue;
        }
        let beat_index = event.onset.floor_div(beat);
        if !event.graces.is_empty() || previous.is_some_and(|previous| previous.cue != event.cue) {
            group_beat = None;
        }
        let automatic = group_beat == Some(beat_index);
        let wanted = match (previous, group_beat) {
            (Some(previous), Some(_)) => previous.beam_next,
            _ => None,
        };
        if wanted == Some(true) && !automatic {
            markers.insert(index, '-');
        } else if wanted == Some(false) && automatic {
            markers.insert(index, '/');
        }
        group_beat = Some(beat_index);
        previous = Some(event);
    }
    markers
}

fn spacer_text(duration: Frac) -> ImportResult<String> {
    let tokens = |value: Frac| -> ImportResult<String> {
        Ok(binary_pieces(value)?
            .into_iter()
            .map(|(base, dots)| format!("s:{}", duration_code(base, dots)))
            .collect::<Vec<_>>()
            .join(" "))
    };
    if duration.is_binary() {
        return tokens(duration);
    }
    let mut odd = duration.d;
    while odd % 2 == 0 {
        odd /= 2;
    }
    let mut normal = 1;
    while normal * 2 < odd {
        normal *= 2;
    }
    let written = duration * Frac::new(odd, normal);
    if !written.is_binary() {
        return Err(format!("a gap of {duration} whole notes cannot be written"));
    }
    Ok(format!("tuplet {odd}:{normal}[bracket=never number=never] {{ {} }}", tokens(written)?))
}

struct VoiceWriter<'a> {
    home: &'a str,
    current: String,
    length: Frac,
    beat: Frac,
    staff_order: &'a [String],
    warnings: &'a mut Vec<String>,
    last_code: Option<String>,
    markers: Map<usize, char>,
}

impl<'a> VoiceWriter<'a> {
    fn write(&mut self, events: &mut [Event]) -> ImportResult<String> {
        if events.len() == 1 && events[0].kind == Kind::MeasureRest && events[0].duration == self.length {
            return Ok("_".into());
        }
        self.markers = beam_markers(events, self.beat);
        // Units: runs of events sharing a tuplet, or single events.
        let mut units: Vec<(usize, usize)> = Vec::new();
        for index in 0..events.len() {
            let joins = match (units.last(), &events[index].tuplet) {
                (Some((start, _)), Some(tuplet)) => events[*start].tuplet.as_ref().is_some_and(|first| first.id == tuplet.id),
                _ => false,
            };
            if joins {
                units.last_mut().expect("unit exists").1 = index + 1;
            } else {
                units.push((index, index + 1));
            }
        }
        let mut parts = Vec::new();
        let mut position = 0;
        while position < units.len() {
            let cue = events[units[position].0].cue;
            let mut run = Vec::new();
            while position < units.len() && events[units[position].0].cue == cue {
                let (start, end) = units[position];
                run.push(self.unit(events, start, end)?);
                position += 1;
            }
            if cue {
                self.last_code = None;
                parts.push(format!("cue {{ {} }}", run.join(" ")));
                self.last_code = None;
            } else {
                parts.extend(run);
            }
        }
        Ok(parts.into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" "))
    }

    fn unit(&mut self, events: &mut [Event], start: usize, end: usize) -> ImportResult<String> {
        let Some(tuplet) = events[start].tuplet.clone() else {
            return self.event(events, start);
        };
        let mut prefix = Vec::new();
        if !events[start].graces.is_empty() {
            prefix.push(self.switch(&events[start]));
            prefix.push(self.graces(&events[start]));
            events[start].graces.clear();
        }
        for event in &mut events[start + 1..end] {
            if !event.graces.is_empty() {
                self.warnings.push("grace notes inside tuplets were dropped".into());
                event.graces.clear();
            }
        }
        let options: Vec<String> = [("bracket", &tuplet.bracket), ("number", &tuplet.number)]
            .iter()
            .filter_map(|(name, value)| value.as_ref().map(|value| format!("{name}={value}")))
            .collect();
        let mut header = format!("tuplet {}:{}", tuplet.actual, tuplet.normal);
        if !options.is_empty() {
            header.push_str(&format!("[{}]", options.join(" ")));
        }
        self.last_code = None;
        let mut body = Vec::new();
        for index in start..end {
            body.push(self.event(events, index)?);
        }
        self.last_code = None;
        let prefix: Vec<String> = prefix.into_iter().filter(|part| !part.is_empty()).collect();
        let lead = if prefix.is_empty() { String::new() } else { format!("{} ", prefix.join(" ")) };
        Ok(format!("{lead}{header} {{ {} }}", body.join(" ")))
    }

    fn event_staff(&self, event: &Event) -> String {
        match event.kind {
            Kind::Rest | Kind::MeasureRest => event.staff.clone().unwrap_or_else(|| self.home.to_string()),
            Kind::Spacer => self.current.clone(),
            Kind::Note => {
                let mut staves: Vec<&str> = event.pitches.iter().map(|pitch| pitch.staff.as_deref().unwrap_or(self.home)).collect();
                staves.sort();
                staves.dedup();
                if staves.len() == 1 {
                    staves[0].to_string()
                } else if staves.contains(&self.current.as_str()) {
                    self.current.clone()
                } else {
                    staves.sort_by_key(|staff| self.staff_order.iter().position(|id| id == staff));
                    staves[0].to_string()
                }
            }
        }
    }

    fn switch(&mut self, event: &Event) -> String {
        let target = self.event_staff(event);
        if target == self.current {
            return String::new();
        }
        self.current = target.clone();
        format!("@{target}")
    }

    fn duration(&mut self, event: &Event) -> String {
        let code = duration_code(event.base.unwrap_or(Frac::new(1, 4)), event.dots);
        if self.last_code.as_deref() == Some(code.as_str()) {
            return String::new();
        }
        self.last_code = Some(code.clone());
        format!(":{code}")
    }

    fn pitches(&self, event: &Event) -> String {
        if event.pitches.len() == 1 {
            return event.pitches[0].text();
        }
        let mut sorted = event.pitches.clone();
        sorted.sort_by_key(Pitch::diatonic);
        let mut groups: Vec<(String, Vec<&Pitch>)> = Vec::new();
        for pitch in &sorted {
            let staff = pitch.staff.clone().unwrap_or_else(|| self.home.to_string());
            match groups.iter_mut().find(|(id, _)| *id == staff) {
                Some((_, members)) => members.push(pitch),
                None => groups.push((staff, vec![pitch])),
            }
        }
        let mut ordered = vec![self.current.clone()];
        ordered.extend(groups.iter().map(|(id, _)| id.clone()).filter(|id| *id != self.current));
        let mut texts = Vec::new();
        for (index, staff) in ordered.iter().enumerate() {
            let Some((_, members)) = groups.iter().find(|(id, _)| id == staff) else { continue };
            if index > 0 {
                texts.push(format!("@{staff}"));
            }
            texts.extend(members.iter().map(|pitch| pitch.text()));
        }
        format!("({})", texts.join(" "))
    }

    fn graces(&mut self, event: &Event) -> String {
        self.last_code = None;
        let mut body = Vec::new();
        for grace in &event.graces {
            let mut sorted = grace.pitches.clone();
            sorted.sort_by_key(Pitch::diatonic);
            let texts: Vec<String> = sorted.iter().map(Pitch::text).collect();
            // Grace groups cannot switch staves, so they sit on their main note's staff.
            let pitch = if texts.len() == 1 { texts[0].clone() } else { format!("({})", texts.join(" ")) };
            body.push(pitch + &self.duration(grace));
        }
        self.last_code = None;
        format!("{} {{ {} }}", event.grace_kind, body.join(" "))
    }

    fn event(&mut self, events: &[Event], index: usize) -> ImportResult<String> {
        let event = &events[index];
        match event.kind {
            Kind::Spacer => {
                self.last_code = None;
                if let (Some(_), Some(base)) = (&event.tuplet, event.base) {
                    return Ok(format!("s:{}", duration_code(base, event.dots)));
                }
                spacer_text(event.duration)
            }
            Kind::MeasureRest => {
                self.last_code = None;
                let mut parts = vec![self.switch(event)];
                for (base, dots) in binary_pieces(event.duration)? {
                    parts.push(format!("r:{}", duration_code(base, dots)));
                }
                Ok(parts.into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" "))
            }
            Kind::Note | Kind::Rest => {
                let mut parts = Vec::new();
                if let Some(marker) = self.markers.get(&index) {
                    parts.push(marker.to_string());
                }
                parts.push(self.switch(event));
                if !event.graces.is_empty() {
                    parts.push(self.graces(event));
                }
                let mut core = if event.kind == Kind::Note { self.pitches(event) } else { "r".to_string() };
                core.push_str(&self.duration(event));
                if !event.annotations.is_empty() {
                    core.push_str(&format!("[{}]", event.annotations.join(" ")));
                }
                parts.push(core);
                if event.tie {
                    parts.push("~".into());
                }
                Ok(parts.into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" "))
            }
        }
    }
}

fn usable(lyric: Option<&Lyric>) -> bool {
    lyric.is_some_and(|lyric| !matches!(lyric.text.trim(), "" | "_" | "__" | "--"))
}

/// Per bar and staff, one lyric string per verse (bars without text are omitted).
fn lyric_strings(score: &mut Score) -> Map<usize, Vec<(String, Vec<String>)>> {
    let mut result: Map<usize, Vec<(String, Vec<String>)>> = Map::new();
    let mut dropped = false;
    for staff in &score.staves {
        let mut per_measure: Vec<Vec<&Event>> = Vec::new();
        let mut verses = 0;
        for measure in &score.measures {
            let slots = measure.voices.get(&staff.id);
            let events: Vec<&Event> = slots
                .and_then(|slots| slots.first())
                .map(|slot| slot.iter().filter(|event| event.kind == Kind::Note).collect())
                .unwrap_or_default();
            if slots.is_some_and(|slots| slots.iter().skip(1).flatten().any(|event| !event.lyrics.is_empty())) {
                dropped = true;
            }
            for event in &events {
                if let Some(last) = event.lyrics.keys().max() {
                    verses = verses.max(last + 1);
                }
            }
            per_measure.push(events);
        }
        if verses == 0 {
            continue;
        }
        let mut tokens: Vec<Vec<Vec<String>>> = vec![vec![Vec::new(); verses]; score.measures.len()];
        let flat: Vec<(usize, &Event)> = per_measure
            .iter()
            .enumerate()
            .flat_map(|(measure, events)| events.iter().map(move |event| (measure, *event)))
            .collect();
        for verse in 0..verses {
            let syllables: Vec<Option<&Lyric>> = flat
                .iter()
                .map(|(_, event)| event.lyrics.get(&verse).filter(|lyric| usable(Some(lyric))))
                .collect();
            let mut remaining = vec![false; flat.len()];
            let mut later = false;
            for position in (0..flat.len()).rev() {
                remaining[position] = later;
                later = later || syllables[position].is_some();
            }
            let mut melisma = false;
            for (position, (measure, _)) in flat.iter().enumerate() {
                let bar_tokens = &mut tokens[*measure][verse];
                let Some(lyric) = syllables[position] else {
                    bar_tokens.push(if melisma { "__" } else { "_" }.into());
                    continue;
                };
                bar_tokens.push(lyric.text.split_whitespace().collect::<Vec<_>>().join("_"));
                let hyphen = lyric.hyphen_after && remaining[position];
                if hyphen {
                    bar_tokens.push("--".into());
                }
                melisma = lyric.extend && !hyphen;
            }
        }
        for (measure, bar) in tokens.into_iter().enumerate() {
            if bar.iter().flatten().any(|token| token != "_" && token != "__") {
                result
                    .or_default(measure)
                    .push((staff.id.clone(), bar.into_iter().map(|verse| verse.join(" ")).collect()));
            }
        }
    }
    if dropped {
        score.warn("lyrics under a staff's second or later voice were dropped");
    }
    result
}

fn harmony_text(measure: &Measure, warnings: &mut Vec<String>) -> ImportResult<Option<String>> {
    let mut sorted = measure.harmony.clone();
    sorted.sort_by_key(|(onset, _)| *onset);
    let mut changes: Vec<(Frac, String)> = Vec::new();
    for (onset, symbol) in sorted {
        let symbol: String = symbol.chars().filter(|c| *c != ':' && *c != ' ').collect();
        if symbol.is_empty() || onset >= measure.length || changes.last().is_some_and(|(last, _)| *last == onset) {
            continue;
        }
        changes.push((onset, symbol));
    }
    if changes.is_empty() {
        return Ok(None);
    }
    let mut boundaries: Vec<Frac> = changes.iter().map(|(onset, _)| *onset).collect();
    boundaries.push(measure.length);
    if !boundaries.iter().all(|value| value.is_binary()) {
        warnings.push(format!("bar {}: chord symbols at tuplet positions were dropped", measure.number));
        return Ok(None);
    }
    let mut tokens = Vec::new();
    if changes[0].0.is_positive() {
        for (base, dots) in binary_pieces(changes[0].0)? {
            tokens.push(format!("{INVISIBLE_HARMONY}:{}", duration_code(base, dots)));
        }
    }
    for (index, (onset, symbol)) in changes.iter().enumerate() {
        for (number, (base, dots)) in binary_pieces(boundaries[index + 1] - *onset)?.into_iter().enumerate() {
            let label = if number == 0 { symbol.as_str() } else { INVISIBLE_HARMONY };
            tokens.push(format!("{label}:{}", duration_code(base, dots)));
        }
    }
    Ok(Some(tokens.join(" ")))
}

fn tempo_value(tempo: &Tempo) -> Val {
    if tempo.beat.is_none() {
        if let Some(text) = &tempo.text {
            return string(text.clone());
        }
    }
    let mut entries = Vec::new();
    if let Some(text) = &tempo.text {
        entries.push(("text", string(text.clone())));
    }
    if let (Some(beat), Some(bpm)) = (tempo.beat, tempo.bpm) {
        entries.push(("beat", string(beat)));
        entries.push(("bpm", Val::Int(bpm as i64)));
    }
    dict(entries)
}

pub struct Emitted {
    pub title: Option<String>,
    pub warnings: Vec<String>,
    /// `score()` arguments, in order.
    pub arguments: Vec<(String, Val)>,
}

pub fn emit(score: &mut Score) -> ImportResult<Emitted> {
    let single = score.staves.len() == 1 && score.staves[0].label.is_none();
    let staff_order: Vec<String> = score.staves.iter().map(|staff| staff.id.clone()).collect();
    let lyrics = lyric_strings(score);
    let mut warnings = Vec::new();

    let mut bars = Vec::new();
    let mut time = score.time.clone();
    for (index, measure) in score.measures.iter_mut().enumerate() {
        if let Some(change) = &measure.time {
            time = change.clone();
        }
        let mut fields: Vec<(String, Val)> = Vec::new();
        let mut push = |name: &str, value: Val| fields.push((name.to_string(), value));
        if measure.length != meter_length(&time) {
            push("partial", string(format!("{}/{}", measure.length.n, measure.length.d)));
        }
        if let Some(value) = &measure.time {
            push("time", string(value.clone()));
        }
        if let Some(value) = &measure.key {
            push("key", string(value.clone()));
        }
        if !measure.clefs.is_empty() {
            if single {
                push("clef", string(measure.clefs[0].1.clone()));
            } else {
                push(
                    "clef",
                    Val::Dict(measure.clefs.iter().map(|(staff, clef)| (staff.clone(), string(clef.clone()))).collect(), true),
                );
            }
        }
        if let Some(tempo) = &measure.tempo {
            push("tempo", tempo_value(tempo));
        }
        if let Some(value) = &measure.rehearsal {
            push("rehearsal", string(value.clone()));
        }
        if let Some(value) = &measure.navigation {
            push("navigation", string(value.clone()));
        }
        let mut barline = Vec::new();
        if let Some(value) = &measure.barline_left {
            barline.push(("left", string(value.clone())));
        }
        if let Some(value) = &measure.barline_right {
            barline.push(("right", string(value.clone())));
        }
        if !barline.is_empty() {
            push("barline", dict(barline));
        }
        if let Some(ending) = &measure.ending {
            let mut entries = vec![("label", string(ending.label.clone()))];
            if ending.start {
                entries.push(("start", Val::Bool(true)));
            }
            if ending.stop {
                entries.push(("stop", Val::Bool(true)));
            }
            push("ending", dict(entries));
        }
        if let Some(harmony) = harmony_text(measure, &mut warnings)? {
            push("harmony", string(harmony));
        }
        let beat = beat_unit(&time);
        for staff in &staff_order {
            let length = measure.length;
            let slots = measure.voices.get_mut(staff).expect("every staff has voice slots");
            let mut texts = Vec::new();
            for slot in slots.iter_mut() {
                let mut writer = VoiceWriter {
                    home: staff,
                    current: staff.clone(),
                    length,
                    beat,
                    staff_order: &staff_order,
                    warnings: &mut warnings,
                    last_code: None,
                    markers: Map::new(),
                };
                texts.push(string(writer.write(slot)?));
            }
            let value = if texts.len() == 1 { texts.remove(0) } else { Val::Arr(texts) };
            fields.push((if single { "notes".to_string() } else { staff.clone() }, value));
        }
        if let Some(per_staff) = lyrics.get(&index) {
            let verses = |values: &Vec<String>| {
                if values.len() == 1 {
                    string(values[0].clone())
                } else {
                    Val::Arr(values.iter().cloned().map(Val::Str).collect())
                }
            };
            if single {
                fields.push(("lyrics".into(), verses(&per_staff[0].1)));
            } else {
                fields.push((
                    "lyrics".into(),
                    Val::Dict(per_staff.iter().map(|(staff, values)| (staff.clone(), verses(values))).collect(), true),
                ));
            }
        }
        bars.push(Val::Dict(fields, false));
    }
    for warning in warnings {
        score.warn(warning);
    }

    let mut arguments: Vec<(String, Val)> = Vec::new();
    if single {
        arguments.push(("clef".into(), string(score.staves[0].clef.clone())));
    } else {
        let staves = score
            .staves
            .iter()
            .map(|staff| {
                let mut entries = vec![("clef", string(staff.clef.clone()))];
                if let Some(label) = &staff.label {
                    entries.push(("label", string(label.clone())));
                }
                if let Some(label) = &staff.short_label {
                    entries.push(("short-label", string(label.clone())));
                }
                (staff.id.clone(), dict(entries))
            })
            .collect();
        arguments.push(("staves".into(), Val::Dict(staves, false)));
    }
    arguments.push(("key".into(), string(score.key.clone())));
    arguments.push(("time".into(), string(score.time.clone())));
    if let Some(tempo) = &score.tempo {
        arguments.push(("tempo".into(), tempo_value(tempo)));
    }
    if let Some(composer) = &score.composer {
        arguments.push(("composer".into(), string(composer.clone())));
    }
    arguments.push(("beams".into(), Val::Bool(true)));
    arguments.push(("bars".into(), Val::Arr(bars)));
    Ok(Emitted {
        title: score.title.clone(),
        warnings: score.warnings.clone(),
        arguments,
    })
}

impl Emitted {
    /// A standalone Typst file equivalent to the imported score.
    pub fn source(&self, package: &str, source_name: &str, scale: &str, measure_numbers: &[String]) -> String {
        let mut lines = vec![if source_name.is_empty() {
            "// Imported by typed-scores.".to_string()
        } else {
            format!("// Imported from {source_name} by typed-scores.")
        }];
        for warning in &self.warnings {
            lines.push(format!("// Import note: {warning}."));
        }
        lines.push(format!("#import {}: score", typst_string(package)));
        lines.push(String::new());
        if let Some(title) = &self.title {
            lines.push(format!("#align(center, text(size: 1.6em, weight: \"bold\", {}))", typst_string(title)));
            lines.push(String::new());
        }
        lines.push("#score(".into());
        for (name, value) in &self.arguments {
            match (name.as_str(), value) {
                ("staves", Val::Dict(staves, _)) => {
                    lines.push("  staves: (".into());
                    for (id, spec) in staves {
                        let Val::Dict(entries, _) = spec else { continue };
                        let fields: Vec<String> = entries.iter().map(|(key, value)| format!("{key}: {}", value.to_typst())).collect();
                        lines.push(format!("    {id}: ({}),", fields.join(", ")));
                    }
                    lines.push("  ),".into());
                }
                ("bars", Val::Arr(bars)) => {
                    lines.push(format!("  scale: {scale},"));
                    lines.push("  bars: (".into());
                    for (index, bar) in bars.iter().enumerate() {
                        let Val::Dict(fields, _) = bar else { continue };
                        let number = measure_numbers.get(index).cloned().unwrap_or_else(|| (index + 1).to_string());
                        lines.push(format!("    // m. {number}"));
                        lines.push("    (".into());
                        for (field, value) in fields {
                            lines.push(format!("      {field}: {},", value.to_typst()));
                        }
                        lines.push("    ),".into());
                    }
                    lines.push("  ),".into());
                }
                _ => lines.push(format!("  {name}: {},", value.to_typst())),
            }
        }
        lines.push(")".into());
        lines.push(String::new());
        lines.join("\n")
    }

    /// `(title, warnings, arguments)` as JSON for the Typst side.
    pub fn json(&self, source: &str) -> String {
        let arguments = Val::Dict(self.arguments.clone(), false);
        format!(
            "{{\"title\":{},\"warnings\":{},\"arguments\":{},\"source\":{}}}",
            self.title.as_deref().map(|title| serde_json::to_string(title).unwrap_or_default()).unwrap_or_else(|| "null".into()),
            serde_json::to_string(&self.warnings).unwrap_or_else(|_| "[]".into()),
            arguments.to_json(),
            serde_json::to_string(source).unwrap_or_default(),
        )
    }
}
