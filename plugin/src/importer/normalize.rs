//! Repair a freshly read score so every construct is one typed-scores accepts.
//!
//! Readers keep what the source says; this pass decides bar lengths, fills
//! voice gaps with spacers, anchors timed directions to events, and removes
//! ties, spans, and marks that typed-scores would reject, recording a warning
//! for each kind of loss.

use super::collections::Map;

use super::frac::Frac;
use super::model::*;

const NOTE_ONLY_MARKS: &[&str] = &[
    "stacc", "staccatissimo", "tenuto", "legato", "accent", "marcato", "strong",
    "turn", "chromatic-turn", "inverted-turn", "trill", "mordent", "inverted-mordent",
    "arpeggio",
];
const ORNAMENTS: &[&str] = &["turn", "chromatic-turn", "inverted-turn", "trill", "mordent", "inverted-mordent"];
const SINGLE_PREFIXES: &[&str] = &["f=", "turn-f=", "text=", "text-below=", "dyn=", "arpeggio", "tremolo="];

/// Split a span marker such as `s3(` or `h1!` into its name and symbol.
pub fn span_parts(mark: &str) -> Option<(&str, char)> {
    let mut characters = mark.chars();
    let first = characters.next()?;
    let last = mark.chars().last()?;
    if !matches!(first, 's' | 'p' | 'h') || !matches!(last, '(' | '<' | '>' | ')' | '!') || mark.len() < 3 {
        return None;
    }
    let body = &mark[1..mark.len() - 1];
    body.chars()
        .all(|c| c.is_alphanumeric() || c == '_')
        .then_some((&mark[..mark.len() - 1], last))
}

pub fn normalize(score: &mut Score) -> ImportResult<()> {
    set_lengths(score)?;
    fill_slots(score);
    attach_directions(score);
    clean_annotations(score);
    check_ties(score);
    check_spans(score);
    Ok(())
}

fn staff_order(score: &Score) -> Vec<String> {
    score.staves.iter().map(|staff| staff.id.clone()).collect()
}

fn set_lengths(score: &mut Score) -> ImportResult<()> {
    let mut time = score.time.clone();
    for measure in &mut score.measures {
        if let Some(change) = &measure.time {
            time = change.clone();
        }
        let full = meter_length(&time);
        let content = measure
            .voices
            .values()
            .flatten()
            .flatten()
            .map(|event| event.onset + event.duration)
            .max()
            .unwrap_or(Frac::ZERO);
        if content > full {
            let denominator: i64 = time.split('/').nth(1).and_then(|value| value.parse().ok()).unwrap_or(4);
            let beats = content * Frac::int(denominator);
            let held = if beats.d == 1 { format!("{}/{denominator}", beats.n) } else { content.to_string() };
            return Err(format!(
                "bar {} holds {held} of music, more than its {time} meter allows; check the source for a missing barline or meter change",
                measure.number
            ));
        }
        measure.length = if content.is_positive() { content } else { full };
    }
    Ok(())
}

fn fill_slots(score: &mut Score) {
    let mut overlaps = false;
    let order = staff_order(score);
    for measure in &mut score.measures {
        let length = measure.length;
        for staff in &order {
            let Some(slots) = measure.voices.get_mut(staff) else { continue };
            let staff_empty = slots.iter().all(Vec::is_empty);
            for (index, slot) in slots.iter_mut().enumerate() {
                if slot.is_empty() {
                    let kind = if index == 0 && staff_empty { Kind::MeasureRest } else { Kind::Spacer };
                    *slot = vec![Event::new(kind, length)];
                    continue;
                }
                let mut events = std::mem::take(slot);
                events.sort_by_key(|event| event.onset);
                let mut position = Frac::ZERO;
                for event in events {
                    if event.onset < position {
                        overlaps = true;
                        continue;
                    }
                    if event.onset > position {
                        slot.push(Event::spacer(event.onset - position, position));
                    }
                    position = event.onset + event.duration;
                    slot.push(event);
                }
                if position < length {
                    slot.push(Event::spacer(length - position, position));
                }
            }
        }
    }
    if overlaps {
        score.warn("overlapping notes inside one voice were dropped");
    }
}

/// Indices of events a direction may attach to, converting a lone full-bar
/// rest into an ordinary rest when that is the only anchor.
fn anchor_candidates(slot: &mut [Event]) -> Vec<usize> {
    let candidates: Vec<usize> = slot
        .iter()
        .enumerate()
        .filter(|(_, event)| matches!(event.kind, Kind::Note | Kind::Rest))
        .map(|(index, _)| index)
        .collect();
    if !candidates.is_empty() {
        return candidates;
    }
    for (index, event) in slot.iter_mut().enumerate() {
        if event.kind == Kind::MeasureRest {
            if let Some((base, dots)) = split_written(event.duration) {
                event.kind = Kind::Rest;
                event.base = Some(base);
                event.dots = dots;
                return vec![index];
            }
        }
    }
    Vec::new()
}

fn pick_anchor(slot: &[Event], candidates: &[usize], onset: Frac, stop: bool) -> usize {
    if stop {
        if let Some(index) = candidates.iter().find(|index| slot[**index].onset == onset) {
            return *index;
        }
        return candidates
            .iter()
            .rev()
            .find(|index| slot[**index].onset < onset)
            .copied()
            .unwrap_or(candidates[0]);
    }
    candidates
        .iter()
        .find(|index| slot[**index].onset >= onset)
        .copied()
        .unwrap_or(*candidates.last().expect("candidates are not empty"))
}

fn attach_directions(score: &mut Score) {
    let mut warnings = Vec::new();
    for measure in &mut score.measures {
        let directions = std::mem::take(&mut measure.directions);
        for direction in directions {
            let Some(slots) = measure.voices.get_mut(&direction.staff) else { continue };
            if slots.is_empty() {
                continue;
            }
            let is_span = span_parts(&direction.annotation).is_some();
            let mut chosen = None;
            for (slot_index, slot) in slots.iter_mut().enumerate() {
                if slot_index > 0 && is_span {
                    break;
                }
                let candidates = anchor_candidates(slot);
                if !candidates.is_empty() {
                    chosen = Some((slot_index, candidates));
                    break;
                }
            }
            let Some((slot_index, candidates)) = chosen else {
                warnings.push(format!("bar {}: a direction with no note to attach to was dropped", measure.number));
                continue;
            };
            let slot = &mut slots[slot_index];
            let target = pick_anchor(slot, &candidates, direction.onset, direction.stop);
            let event = &mut slot[target];
            if direction.annotation.starts_with("dyn=") && event.annotations.iter().any(|mark| mark.starts_with("dyn=")) {
                warnings.push(format!("bar {}: a second dynamic on one note was dropped", measure.number));
                continue;
            }
            event.add(direction.annotation);
        }
    }
    for warning in warnings {
        score.warn(warning);
    }
}

fn clean_annotations(score: &mut Score) {
    let order = staff_order(score);
    let mut warnings = Vec::new();
    for measure in &mut score.measures {
        for staff in &order {
            let Some(slots) = measure.voices.get_mut(staff) else { continue };
            for event in slots.iter_mut().flatten() {
                for grace in &mut event.graces {
                    grace.annotations.clear();
                    grace.lyrics.clear();
                    grace.tie = false;
                }
                event.annotations = clean_marks(event, &mut warnings);
            }
        }
    }
    for warning in warnings {
        score.warn(warning);
    }
}

fn clean_marks(event: &Event, warnings: &mut Vec<String>) -> Vec<String> {
    if !matches!(event.kind, Kind::Note | Kind::Rest) {
        if !event.annotations.is_empty() {
            warnings.push("marks on invisible notes or full-bar rests were dropped".into());
        }
        return Vec::new();
    }
    let mut kept: Vec<String> = Vec::new();
    for mark in &event.annotations {
        let mut mark = mark.clone();
        if mark.starts_with("text=") || mark.starts_with("text-below=") {
            let (prefix, value) = mark.split_once('=').expect("text marks contain '='");
            let value = value.replace('[', "(").replace(']', ")");
            let value = value.split_whitespace().collect::<Vec<_>>().join("_");
            let value = value.trim_matches('_');
            if value.is_empty() {
                continue;
            }
            mark = format!("{prefix}={value}");
        }
        if event.kind == Kind::Rest
            && (NOTE_ONLY_MARKS.contains(&mark.as_str())
                || ["f=", "turn-f=", "arpeggio", "tremolo="].iter().any(|prefix| mark.starts_with(prefix))
                || span_parts(&mark).is_some_and(|(name, _)| name.starts_with('s')))
        {
            continue;
        }
        if mark.starts_with("arpeggio") && event.pitches.len() < 2 {
            continue;
        }
        if ORNAMENTS.contains(&mark.as_str()) && kept.iter().any(|existing| ORNAMENTS.contains(&existing.as_str())) {
            warnings.push("only one ornament per note is supported; extra ornaments were dropped".into());
            continue;
        }
        if let Some(prefix) = SINGLE_PREFIXES.iter().find(|prefix| mark.starts_with(**prefix)) {
            if kept.iter().any(|existing| existing.starts_with(prefix)) {
                continue;
            }
        }
        if !kept.contains(&mark) {
            kept.push(mark);
        }
    }
    kept
}

/// Each voice slot as (staff id, slot, [(measure index, event index)]).
fn voice_sequences(score: &Score) -> Vec<(String, usize, Vec<(usize, usize)>)> {
    let mut sequences = Vec::new();
    for staff in &score.staves {
        let slot_count = score
            .measures
            .iter()
            .map(|measure| measure.voices.get(&staff.id).map(Vec::len).unwrap_or(0))
            .max()
            .unwrap_or(0);
        for slot in 0..slot_count {
            let mut sequence = Vec::new();
            for (measure_index, measure) in score.measures.iter().enumerate() {
                if let Some(events) = measure.voices.get(&staff.id).and_then(|slots| slots.get(slot)) {
                    sequence.extend((0..events.len()).map(|event_index| (measure_index, event_index)));
                }
            }
            sequences.push((staff.id.clone(), slot, sequence));
        }
    }
    sequences
}

fn event_at<'a>(score: &'a Score, staff: &str, slot: usize, position: (usize, usize)) -> &'a Event {
    &score.measures[position.0].voices[staff][slot][position.1]
}

fn event_at_mut<'a>(score: &'a mut Score, staff: &str, slot: usize, position: (usize, usize)) -> &'a mut Event {
    &mut score.measures[position.0].voices.get_mut(staff).expect("staff exists")[slot][position.1]
}

pub fn display_staff(event: &Event, home: &str) -> String {
    let mut staves: Vec<&str> = event.pitches.iter().map(|pitch| pitch.staff.as_deref().unwrap_or(home)).collect();
    staves.sort();
    staves.dedup();
    if staves.len() == 1 { staves[0].to_string() } else { "split".to_string() }
}

fn check_ties(score: &mut Score) {
    let mut dropped = false;
    for (home, slot, sequence) in voice_sequences(score) {
        for (position, location) in sequence.iter().enumerate() {
            let event = event_at(score, &home, slot, *location);
            if !event.tie {
                continue;
            }
            let mut valid = false;
            if let (Some(following), Kind::Note) = (sequence.get(position + 1), event.kind) {
                let target = event_at(score, &home, slot, *following);
                let source_staff = display_staff(event, &home);
                let target_staff = display_staff(target, &home);
                let mut source_keys: Vec<_> = event.pitches.iter().map(Pitch::key).collect();
                let mut target_keys: Vec<_> = target.pitches.iter().map(Pitch::key).collect();
                source_keys.sort();
                target_keys.sort();
                valid = target.kind == Kind::Note
                    && target.graces.is_empty()
                    && source_keys == target_keys
                    && source_staff == target_staff
                    && target_staff != "split"
                    && (following.0 == location.0 || source_staff == home);
            }
            if !valid {
                event_at_mut(score, &home, slot, *location).tie = false;
                dropped = true;
            }
        }
    }
    if dropped {
        score.warn("ties typed-scores cannot draw (partial chord ties, ties across staves or voices) were dropped");
    }
}

fn check_spans(score: &mut Score) {
    // name -> [(sequence index, position, symbol, staff, slot, location)]
    let mut placements: Vec<(String, Vec<(usize, usize, char, String, usize, (usize, usize))>)> = Vec::new();
    let mut lookup: Map<String, usize> = Map::new();
    for (sequence_index, (staff, slot, sequence)) in voice_sequences(score).into_iter().enumerate() {
        for (position, location) in sequence.iter().enumerate() {
            for mark in &event_at(score, &staff, slot, *location).annotations {
                if let Some((name, symbol)) = span_parts(mark) {
                    let entry = *lookup.or_insert_with(name.to_string(), || {
                        placements.push((name.to_string(), Vec::new()));
                        placements.len() - 1
                    });
                    placements[entry].1.push((sequence_index, position, symbol, staff.clone(), slot, *location));
                }
            }
        }
    }
    let mut dropped = false;
    for (name, marks) in placements {
        let opens: Vec<_> = marks.iter().filter(|mark| matches!(mark.2, '(' | '<' | '>')).collect();
        let closes: Vec<_> = marks.iter().filter(|mark| matches!(mark.2, ')' | '!')).collect();
        let valid = opens.len() == 1 && closes.len() == 1 && opens[0].0 == closes[0].0 && opens[0].1 < closes[0].1;
        if !valid {
            dropped = true;
            for (_, _, symbol, staff, slot, location) in &marks {
                let removed = format!("{name}{symbol}");
                event_at_mut(score, staff, *slot, *location).annotations.retain(|mark| *mark != removed);
            }
        }
    }
    if dropped {
        score.warn("slurs, hairpins, or pedal marks without a matching end in the same voice were dropped");
    }
}
