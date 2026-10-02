//! Read MusicXML (partwise, timewise, or compressed .mxl) into the score model.

use super::collections::{Map, Set};

use super::xml::{self, Node};

use super::frac::Frac;
use super::model::*;
use super::zip;

const NOTE_TYPES: [(&str, Frac); 6] = [
    ("whole", Frac { n: 1, d: 1 }),
    ("half", Frac { n: 1, d: 2 }),
    ("quarter", Frac { n: 1, d: 4 }),
    ("eighth", Frac { n: 1, d: 8 }),
    ("16th", Frac { n: 1, d: 16 }),
    ("32nd", Frac { n: 1, d: 32 }),
];
const UNSUPPORTED_TYPES: &[&str] = &[
    "maxima", "long", "breve", "64th", "128th", "256th", "512th", "1024th",
];
const ORNAMENTS: &[(&str, &str)] = &[
    ("trill-mark", "trill"),
    ("turn", "turn"),
    ("delayed-turn", "turn"),
    ("inverted-turn", "inverted-turn"),
    ("delayed-inverted-turn", "inverted-turn"),
    ("mordent", "mordent"),
    ("inverted-mordent", "inverted-mordent"),
];
const HARMONY_KINDS: &[(&str, &str)] = &[
    ("major", ""),
    ("minor", "m"),
    ("augmented", "+"),
    ("diminished", "dim"),
    ("dominant", "7"),
    ("major-seventh", "maj7"),
    ("minor-seventh", "m7"),
    ("diminished-seventh", "dim7"),
    ("augmented-seventh", "+7"),
    ("half-diminished", "m7b5"),
    ("major-minor", "m(maj7)"),
    ("major-sixth", "6"),
    ("minor-sixth", "m6"),
    ("dominant-ninth", "9"),
    ("major-ninth", "maj9"),
    ("minor-ninth", "m9"),
    ("dominant-11th", "11"),
    ("major-11th", "maj11"),
    ("minor-11th", "m11"),
    ("dominant-13th", "13"),
    ("major-13th", "maj13"),
    ("minor-13th", "m13"),
    ("suspended-second", "sus2"),
    ("suspended-fourth", "sus4"),
    ("power", "5"),
    ("pedal", "ped"),
    ("none", "N.C."),
    ("other", ""),
];

fn note_type(name: &str) -> Option<Frac> {
    NOTE_TYPES
        .iter()
        .find(|(type_name, _)| *type_name == name)
        .map(|(_, value)| *value)
}

// -- XML helpers -----------------------------------------------------------

type XNode<'a> = Node<'a>;

fn child<'a>(node: XNode<'a>, name: &str) -> Option<XNode<'a>> {
    node.elements().find(|item| item.name() == name)
}

fn children<'a>(node: XNode<'a>, name: &'a str) -> impl Iterator<Item = XNode<'a>> + 'a {
    node.elements().filter(move |item| item.name() == name)
}

fn elements<'a>(node: XNode<'a>) -> impl Iterator<Item = XNode<'a>> {
    node.elements()
}

fn find<'a>(node: XNode<'a>, path: &str) -> Option<XNode<'a>> {
    let mut current = node;
    for part in path.split('/') {
        current = child(current, part)?;
    }
    Some(current)
}

fn element_text(node: XNode<'_>) -> String {
    node.text()
}

fn text_at(node: XNode<'_>, path: &str) -> Option<String> {
    let found = find(node, path)?;
    Some(element_text(found).trim().to_string())
}

fn has_tie_start(node: XNode<'_>) -> bool {
    children(node, "tie").any(|tie| tie.attribute("type") == Some("start"))
}

/// Nearest integer, ties to even like Python's round().
fn round_half_even(value: Frac) -> i32 {
    let floor = value.n.div_euclid(value.d);
    let remainder = value - Frac::int(floor);
    let half = Frac::new(1, 2);
    let rounded = if remainder > half || (remainder == half && floor % 2 != 0) {
        floor + 1
    } else {
        floor
    };
    rounded as i32
}

fn parse_number(text: &str) -> Option<Frac> {
    let text = text.trim();
    if let Some((whole, fraction)) = text.split_once('.') {
        let whole: i64 = if whole.is_empty() || whole == "-" {
            0
        } else {
            whole.parse().ok()?
        };
        if fraction.is_empty() || !fraction.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let digits = fraction.to_string();
        let scale = 10_i64.checked_pow(digits.len() as u32)?;
        let fraction_value: i64 = if digits.is_empty() {
            0
        } else {
            digits.parse().ok()?
        };
        let sign = if text.starts_with('-') { -1 } else { 1 };
        Some(Frac::int(whole) + Frac::new(sign * fraction_value, scale))
    } else {
        text.parse::<i64>().ok().map(Frac::int)
    }
}

// -- loading -----------------------------------------------------------------

fn decode_text(bytes: &[u8]) -> ImportResult<String> {
    if bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) {
        if (bytes.len() - 2) % 2 != 0 {
            return Err("the MusicXML file has an incomplete UTF-16 code unit".into());
        }
        let little = bytes[0] == 0xFF;
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|pair| {
                if little {
                    u16::from_le_bytes([pair[0], pair[1]])
                } else {
                    u16::from_be_bytes([pair[0], pair[1]])
                }
            })
            .collect();
        return String::from_utf16(&units)
            .map_err(|_| "the MusicXML file is not valid UTF-16".to_string());
    }
    let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
    String::from_utf8(bytes.to_vec())
        .map_err(|_| "the MusicXML file is not valid UTF-8".to_string())
}

/// Extract the score document from a compressed .mxl archive.
fn unpack_mxl(bytes: &[u8]) -> ImportResult<Vec<u8>> {
    let entries = zip::entries(bytes)?;
    let mut rootfile = None;
    if let Some(container) = entries
        .iter()
        .find(|entry| entry.name == "META-INF/container.xml")
    {
        let text = decode_text(&zip::read(bytes, container)?)?;
        let document = parse_document(&text)?;
        rootfile = document
            .root()
            .descendants()
            .into_iter()
            .find(|node| node.name() == "rootfile" && node.attribute("full-path").is_some())
            .and_then(|node| node.attribute("full-path"))
            .map(str::to_string);
    }
    let name = match rootfile {
        Some(name) => name,
        None => entries
            .iter()
            .find(|entry| {
                let lower = entry.name.to_ascii_lowercase();
                (lower.ends_with(".xml") || lower.ends_with(".musicxml"))
                    && !entry.name.starts_with("META-INF")
            })
            .map(|entry| entry.name.clone())
            .ok_or("the .mxl archive contains no MusicXML score")?,
    };
    let entry = entries
        .iter()
        .find(|entry| entry.name == name)
        .ok_or_else(|| format!("the .mxl archive is missing {name}"))?;
    zip::read(bytes, entry)
}

fn parse_document(text: &str) -> ImportResult<xml::Document> {
    let document = xml::parse(text)?;
    for node in document.root().descendants() {
        let value = element_text(node);
        let value = value.trim();
        let invalid = || {
            format!(
                "MusicXML <{}> has invalid value {value:?}; correct it in the source score",
                node.name()
            )
        };
        match node.name() {
            "divisions" | "duration" | "actual-notes" | "normal-notes" | "beat-type" | "staff"
            | "staves" => {
                let number = value.parse::<i64>().map_err(|_| invalid())?;
                let maximum = if node.name() == "divisions" {
                    i64::MAX / 4
                } else {
                    u32::MAX as i64
                };
                if number <= 0 || number > maximum {
                    return Err(invalid());
                }
            }
            "beats" => {
                value
                    .split('+')
                    .try_fold(0_u32, |total, component| {
                        let count = component
                            .trim()
                            .parse::<u32>()
                            .ok()
                            .filter(|count| *count > 0)?;
                        total.checked_add(count)
                    })
                    .ok_or_else(invalid)?;
            }
            "step" | "display-step" | "root-step" | "bass-step" => {
                if value.len() != 1 || !STEPS.contains(value) {
                    return Err(invalid());
                }
            }
            "octave" | "display-octave" => {
                if value
                    .parse::<i32>()
                    .ok()
                    .filter(|octave| (-1..=9).contains(octave))
                    .is_none()
                {
                    return Err(invalid());
                }
            }
            "alter" | "root-alter" | "bass-alter" | "offset" => {
                if parse_number(value).is_none() {
                    return Err(invalid());
                }
            }
            "fifths" => {
                if value
                    .parse::<i32>()
                    .ok()
                    .filter(|fifths| (-7..=7).contains(fifths))
                    .is_none()
                {
                    return Err(invalid());
                }
            }
            "note" => {
                if child(node, "grace").is_none() && child(node, "duration").is_none() {
                    return Err("MusicXML note is missing <duration>; provide its positive duration in divisions".into());
                }
                if child(node, "pitch").is_none()
                    && child(node, "unpitched").is_none()
                    && child(node, "rest").is_none()
                {
                    return Err("MusicXML note needs <pitch>, <unpitched>, or <rest>; correct the source note".into());
                }
                if children(node, "dot").count() > 2 {
                    return Err("MusicXML notes support at most two augmentation dots; simplify the written duration".into());
                }
            }
            "pitch" => {
                if child(node, "step").is_none() || child(node, "octave").is_none() {
                    return Err(
                        "MusicXML pitch needs <step> and <octave>; complete the source pitch"
                            .into(),
                    );
                }
            }
            _ => {}
        }
    }
    Ok(document)
}

struct MeasureSource<'a> {
    number: Option<String>,
    children: Vec<XNode<'a>>,
}

struct PartSource<'a> {
    id: String,
    measures: Vec<MeasureSource<'a>>,
}

fn part_sources<'a>(root: XNode<'a>) -> Vec<PartSource<'a>> {
    if root.name() == "score-timewise" {
        let mut parts: Vec<PartSource<'a>> = Vec::new();
        for measure in children(root, "measure") {
            for part in children(measure, "part") {
                let id = part.attribute("id").unwrap_or("").to_string();
                let index = match parts.iter().position(|source| source.id == id) {
                    Some(index) => index,
                    None => {
                        parts.push(PartSource {
                            id,
                            measures: Vec::new(),
                        });
                        parts.len() - 1
                    }
                };
                parts[index].measures.push(MeasureSource {
                    number: measure.attribute("number").map(str::to_string),
                    children: elements(part).collect(),
                });
            }
        }
        return parts;
    }
    children(root, "part")
        .map(|part| PartSource {
            id: part.attribute("id").unwrap_or("").to_string(),
            measures: children(part, "measure")
                .map(|measure| MeasureSource {
                    number: measure.attribute("number").map(str::to_string),
                    children: elements(measure).collect(),
                })
                .collect(),
        })
        .collect()
}

fn clef_name(sign: Option<&str>, line: Option<&str>, score: &mut Score) -> String {
    let line_number: Option<i32> = line.and_then(|value| value.parse().ok());
    match (sign, line_number) {
        (Some("G"), None | Some(2)) => return "treble".into(),
        (Some("F"), None | Some(4)) => return "bass".into(),
        (Some("C"), Some(3)) => return "alto".into(),
        (Some("C"), Some(4)) => return "tenor".into(),
        _ => {}
    }
    let fallback = if sign == Some("F") { "bass" } else { "treble" };
    score.warn(format!(
        "clef {}{} is not supported; using {fallback}",
        sign.unwrap_or("None"),
        line.unwrap_or("")
    ));
    fallback.into()
}

// -- per-part reading --------------------------------------------------------

struct NoteRecord {
    voice: String,
    staff: usize,
    measure: usize,
    event: usize,
}

#[derive(Default)]
struct MeasureInfo {
    number: String,
    clefs: Vec<(String, String)>,
    clef_later: Vec<(String, Frac, String)>,
    clef_mid_bar: bool,
    directions: Vec<Direction>,
    harmony: Vec<(Frac, String)>,
    time: Option<String>,
    key: Option<String>,
    tempo: Option<Tempo>,
    rehearsal: Option<String>,
    navigation: Option<String>,
    barline_left: Option<String>,
    barline_right: Option<String>,
    ending: Option<Ending>,
}

struct OpenTuplet {
    tuplet: Tuplet,
    target: Frac,
    accumulated: Frac,
}

struct Counters {
    spans: Map<char, usize>,
    tuplets: usize,
}

impl Counters {
    fn span(&mut self, prefix: char) -> String {
        let count = self.spans.or_default(prefix);
        *count += 1;
        format!("{prefix}{count}")
    }
}

struct PartReader<'s> {
    staff_ids: Vec<String>,
    score: &'s mut Score,
    counters: &'s mut Counters,
    divisions: i64,
    events: Vec<Event>,
    records: Vec<NoteRecord>,
    measures: Vec<MeasureInfo>,
    slur_names: Map<String, Option<String>>,
    wedge_names: Map<String, String>,
    pedal_name: Option<String>,
    has_beams: bool,
    verse_numbers: Vec<String>,
}

struct MeasureState {
    cursor: Frac,
    last_notes: Map<String, usize>,
    pending_graces: Map<String, Vec<Event>>,
    pending_slash: Map<String, bool>,
    tuplets: Map<String, OpenTuplet>,
}

impl<'s> PartReader<'s> {
    fn staff_id(&self, number: usize) -> String {
        self.staff_ids[number.clamp(1, self.staff_ids.len()) - 1].clone()
    }

    fn number(&self) -> String {
        self.measures
            .last()
            .map(|info| info.number.clone())
            .unwrap_or_default()
    }

    fn length(&self, node: XNode<'_>, tag: &str) -> Frac {
        text_at(node, tag)
            .and_then(|value| parse_number(&value))
            .map(|value| value / Frac::int(4 * self.divisions))
            .unwrap_or(Frac::ZERO)
    }

    fn read(&mut self, part: &PartSource<'_>) -> ImportResult<()> {
        self.has_beams = part.measures.iter().any(|measure| {
            measure.children.iter().any(|node| {
                node.descendants()
                    .into_iter()
                    .any(|item| item.name() == "beam")
            })
        });
        for (index, measure) in part.measures.iter().enumerate() {
            self.read_measure(index, measure)?;
        }
        Ok(())
    }

    fn read_measure(&mut self, index: usize, measure: &MeasureSource<'_>) -> ImportResult<()> {
        self.measures.push(MeasureInfo {
            number: measure
                .number
                .clone()
                .unwrap_or_else(|| (index + 1).to_string()),
            ..MeasureInfo::default()
        });
        let mut state = MeasureState {
            cursor: Frac::ZERO,
            last_notes: Map::new(),
            pending_graces: Map::new(),
            pending_slash: Map::new(),
            tuplets: Map::new(),
        };
        let mut measure_end = Frac::ZERO;
        for node in &measure.children {
            match node.name() {
                "attributes" => self.read_attributes(*node, state.cursor),
                "backup" => state.cursor -= self.length(*node, "duration"),
                "forward" => state.cursor += self.length(*node, "duration"),
                "note" => self.read_note(*node, index, &mut state)?,
                "direction" => self.read_direction(*node, state.cursor),
                "harmony" => {
                    if let Some(symbol) = harmony_symbol(*node) {
                        let offset = self.length(*node, "offset");
                        self.info().harmony.push((state.cursor + offset, symbol));
                    }
                }
                "barline" => self.read_barline(*node),
                _ => {}
            }
            measure_end = measure_end.max(state.cursor);
        }
        let info = self.info();
        info.clef_mid_bar = info
            .clef_later
            .iter()
            .any(|(_, onset, _)| *onset < measure_end);
        if state
            .pending_graces
            .values()
            .any(|graces| !graces.is_empty())
        {
            let number = self.number();
            self.score.warn(format!(
                "bar {number}: grace notes with no following note were dropped"
            ));
        }
        Ok(())
    }

    fn info(&mut self) -> &mut MeasureInfo {
        self.measures.last_mut().expect("a measure is being read")
    }

    fn read_attributes(&mut self, node: XNode<'_>, cursor: Frac) {
        if let Some(divisions) = text_at(node, "divisions").and_then(|value| parse_number(&value)) {
            let whole = divisions.n / divisions.d;
            if whole > 0 {
                self.divisions = whole;
            }
        }
        if let Some(key) = child(node, "key") {
            if let Some(fifths) = text_at(key, "fifths").and_then(|value| value.parse::<i32>().ok())
            {
                let mode = text_at(key, "mode").unwrap_or_else(|| "major".into());
                self.info().key = Some(key_name(fifths, mode == "minor" || mode == "aeolian"));
            }
        }
        if let Some(time) = child(node, "time") {
            if child(time, "beats").is_some() {
                let beats = text_at(time, "beats").unwrap_or_default();
                let beat_type = text_at(time, "beat-type").unwrap_or_default();
                let numerator: Option<i64> = beats
                    .split('+')
                    .map(|part| part.trim().parse::<i64>().ok())
                    .sum();
                match (numerator, beat_type.parse::<i64>()) {
                    (Some(numerator), Ok(denominator)) => {
                        self.info().time = Some(format!("{numerator}/{denominator}"));
                    }
                    _ => self.score.warn(format!(
                        "time signature {beats}/{beat_type} is not supported"
                    )),
                }
            }
        }
        for clef in children(node, "clef") {
            let number: usize = clef
                .attribute("number")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1);
            let sign = text_at(clef, "sign");
            let line = text_at(clef, "line");
            let name = clef_name(sign.as_deref(), line.as_deref(), self.score);
            if !matches!(
                text_at(clef, "clef-octave-change").as_deref(),
                None | Some("0")
            ) {
                self.score
                    .warn("octave-transposing clefs are drawn without their octave mark");
            }
            let staff_id = self.staff_id(number);
            let info = self.info();
            if cursor == Frac::ZERO {
                set_pair(&mut info.clefs, &staff_id, name);
            } else {
                info.clef_later.retain(|(id, _, _)| *id != staff_id);
                info.clef_later.push((staff_id, cursor, name));
            }
        }
    }

    fn read_note(
        &mut self,
        node: XNode<'_>,
        measure: usize,
        state: &mut MeasureState,
    ) -> ImportResult<()> {
        let voice = text_at(node, "voice").unwrap_or_else(|| "1".into());
        let staff: usize = text_at(node, "staff")
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        let staff_id = self.staff_id(staff);
        if let Some(notehead) = text_at(node, "notehead") {
            if notehead != "normal" {
                self.score.warn(format!(
                    "notehead {notehead:?} is imported as a round notehead"
                ));
            }
        }
        let is_chord = child(node, "chord").is_some();
        let is_grace = child(node, "grace").is_some();
        let duration = if is_grace {
            Frac::ZERO
        } else {
            self.length(node, "duration")
        };
        let pitch = self.read_pitch(node, &staff_id);
        let cursor = state.cursor;

        if is_chord {
            if let Some(pitch) = pitch.clone() {
                let on_grace = is_grace
                    && state
                        .pending_graces
                        .get(&voice)
                        .is_some_and(|graces| !graces.is_empty());
                let head = if on_grace {
                    state
                        .pending_graces
                        .get_mut(&voice)
                        .and_then(|graces| graces.pop())
                } else {
                    state.last_notes.get(&voice).map(|index| {
                        std::mem::replace(
                            &mut self.events[*index],
                            Event::new(Kind::Note, Frac::ZERO),
                        )
                    })
                };
                if let Some(mut previous) = head {
                    let joined = previous.kind == Kind::Note;
                    if joined {
                        if previous
                            .pitches
                            .iter()
                            .all(|existing| existing.key() != pitch.key())
                        {
                            previous.pitches.push(pitch);
                        }
                        if !has_tie_start(node) {
                            previous.tie = false;
                        }
                        self.read_notations(node, &mut previous);
                    }
                    if on_grace {
                        state
                            .pending_graces
                            .get_mut(&voice)
                            .expect("grace group exists")
                            .push(previous);
                    } else {
                        self.events[state.last_notes[&voice]] = previous;
                    }
                    if joined {
                        return Ok(());
                    }
                }
            }
            // A chord member without a head note starts its own event.
        }

        let (base, dots) = self.written_duration(node, duration);
        let visible = node.attribute("print-object") != Some("no");
        let rest = child(node, "rest");
        let mut event = if !visible {
            Event::new(Kind::Spacer, duration)
        } else if let Some(rest) = rest {
            let kind = if rest.attribute("measure") == Some("yes") || base.is_none() {
                Kind::MeasureRest
            } else {
                Kind::Rest
            };
            let mut event = Event::new(kind, duration);
            event.base = base;
            event.dots = dots;
            event.staff = Some(staff_id.clone());
            event
        } else {
            let Some(pitch) = pitch else {
                state.cursor = cursor + duration;
                return Ok(());
            };
            let Some(base) = base else {
                let name = text_at(node, "type").unwrap_or_else(|| "?".into());
                return Err(format!(
                    "bar {}: {name} notes cannot be written in typed-scores",
                    self.number()
                ));
            };
            let mut event = Event::new(Kind::Note, duration);
            event.base = Some(base);
            event.dots = dots;
            event.pitches = vec![pitch];
            event.tie = has_tie_start(node);
            event
        };
        event.cue = child(node, "cue").is_some()
            || child(node, "type").is_some_and(|value| value.attribute("size") == Some("cue"));
        event.onset = cursor;
        if event.kind == Kind::Note {
            self.read_beam(node, &mut event);
            self.read_notations(node, &mut event);
            self.read_lyrics(node, &mut event);
        }
        if is_grace {
            if event.kind == Kind::Note {
                let graces = state.pending_graces.or_default(voice.clone());
                if graces.is_empty() {
                    let slash = child(node, "grace").and_then(|grace| grace.attribute("slash"))
                        == Some("yes");
                    state.pending_slash.insert(voice.clone(), slash);
                }
                graces.push(event);
            }
            return Ok(());
        }
        self.read_tuplet(node, &mut event, &voice, state);
        if let Some(graces) = state.pending_graces.get_mut(&voice) {
            if !graces.is_empty() {
                if event.kind == Kind::Note {
                    event.graces = std::mem::take(graces);
                    event.grace_kind = if state.pending_slash.get(&voice).copied().unwrap_or(false)
                    {
                        "acciaccatura"
                    } else {
                        "grace"
                    };
                } else {
                    graces.clear();
                    let number = self.number();
                    self.score.warn(format!(
                        "bar {number}: grace notes before a rest were dropped"
                    ));
                }
            }
        }
        let is_note = event.kind == Kind::Note;
        self.events.push(event);
        let index = self.events.len() - 1;
        if is_note {
            state.last_notes.insert(voice.clone(), index);
        }
        self.records.push(NoteRecord {
            voice,
            staff,
            measure,
            event: index,
        });
        state.cursor = cursor + duration;
        Ok(())
    }

    fn read_pitch(&mut self, node: XNode<'_>, staff_id: &str) -> Option<Pitch> {
        if let Some(pitch) = child(node, "pitch") {
            let alter_value = text_at(pitch, "alter")
                .and_then(|value| parse_number(&value))
                .unwrap_or(Frac::ZERO);
            let mut alter = round_half_even(alter_value);
            if alter_value.d != 1 || !(-2..=2).contains(&alter) {
                self.score.warn("microtonal alterations were rounded");
                alter = alter.clamp(-2, 2);
            }
            return Some(Pitch {
                step: text_at(pitch, "step")?.chars().next()?.to_ascii_uppercase(),
                alter: alter as i8,
                octave: text_at(pitch, "octave")?.parse().ok()?,
                staff: Some(staff_id.to_string()),
            });
        }
        if let Some(unpitched) = child(node, "unpitched") {
            self.score
                .warn("unpitched notes are placed at their display pitch");
            return Some(Pitch {
                step: text_at(unpitched, "display-step")
                    .and_then(|value| value.chars().next())
                    .unwrap_or('B'),
                alter: 0,
                octave: text_at(unpitched, "display-octave")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(4),
                staff: Some(staff_id.to_string()),
            });
        }
        None
    }

    fn written_duration(&self, node: XNode<'_>, duration: Frac) -> (Option<Frac>, u8) {
        let name = text_at(node, "type");
        let dots = children(node, "dot").count().min(2) as u8;
        if let Some(value) = name.as_deref().and_then(note_type) {
            return (Some(value), dots);
        }
        if name
            .as_deref()
            .is_some_and(|value| UNSUPPORTED_TYPES.contains(&value))
        {
            return (None, 0);
        }
        // No <type>: infer the written value from the sounding duration.
        let mut written = duration;
        if let Some(modification) = child(node, "time-modification") {
            let actual = text_at(modification, "actual-notes")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1);
            let normal = text_at(modification, "normal-notes")
                .and_then(|value| value.parse().ok())
                .unwrap_or(1);
            written = duration * Frac::new(actual, normal);
        }
        match split_written(written) {
            Some((base, dots)) if written.is_positive() => (Some(base), dots),
            _ => (None, 0),
        }
    }

    fn read_beam(&self, node: XNode<'_>, event: &mut Event) {
        if !self.has_beams || !event.flagged() {
            return;
        }
        let beam = children(node, "beam")
            .find(|beam| beam.attribute("number") == Some("1"))
            .or_else(|| child(node, "beam"));
        event.beam_next = Some(
            beam.is_some_and(|beam| matches!(element_text(beam).trim(), "begin" | "continue")),
        );
    }

    fn read_tuplet(
        &mut self,
        node: XNode<'_>,
        event: &mut Event,
        voice: &str,
        state: &mut MeasureState,
    ) {
        let Some(modification) = child(node, "time-modification") else {
            state.tuplets.remove(voice);
            return;
        };
        let markers: Vec<XNode<'_>> = children(node, "notations")
            .flat_map(|notations| children(notations, "tuplet"))
            .collect();
        let starts: Vec<XNode<'_>> = markers
            .iter()
            .copied()
            .filter(|marker| marker.attribute("type") == Some("start"))
            .collect();
        let stops = markers
            .iter()
            .any(|marker| marker.attribute("type") == Some("stop"));
        let actual: u32 = text_at(modification, "actual-notes")
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        let normal: u32 = text_at(modification, "normal-notes")
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        if actual == normal {
            return;
        }
        if starts.len() > 1 || (child(modification, "normal-type").is_some() && markers.len() > 1) {
            self.score
                .warn("nested tuplets are written as one combined tuplet");
        }
        let needs_new = match state.tuplets.get(voice) {
            None => true,
            Some(open) => {
                !starts.is_empty() || (open.tuplet.actual, open.tuplet.normal) != (actual, normal)
            }
        };
        if needs_new {
            self.counters.tuplets += 1;
            let mut tuplet = Tuplet {
                id: self.counters.tuplets,
                actual,
                normal,
                bracket: None,
                number: None,
            };
            if let Some(start) = starts.first() {
                if start.attribute("bracket") == Some("no") {
                    tuplet.bracket = Some("never".into());
                }
                if start.attribute("show-number") == Some("none") {
                    tuplet.number = Some("never".into());
                }
            }
            let unit = text_at(modification, "normal-type")
                .as_deref()
                .and_then(note_type)
                .or(event.base)
                .unwrap_or(Frac::new(1, 8));
            let dotted = if child(modification, "normal-dot").is_some() {
                Frac::new(3, 2)
            } else {
                Frac::ONE
            };
            state.tuplets.insert(
                voice.to_string(),
                OpenTuplet {
                    tuplet,
                    target: unit * Frac::int(normal as i64) * dotted,
                    accumulated: Frac::ZERO,
                },
            );
        }
        let open = state
            .tuplets
            .get_mut(voice)
            .expect("tuplet was just opened");
        event.tuplet = Some(open.tuplet.clone());
        open.accumulated += event.duration;
        if stops || open.accumulated >= open.target {
            state.tuplets.remove(voice);
        }
    }

    fn read_notations(&mut self, node: XNode<'_>, event: &mut Event) {
        let is_grace = child(node, "grace").is_some();
        for notations in children(node, "notations") {
            for slur in children(notations, "slur") {
                let number = slur.attribute("number").unwrap_or("1").to_string();
                match slur.attribute("type") {
                    Some("start") => {
                        if is_grace {
                            self.slur_names.insert(number, None);
                        } else {
                            let name = self.counters.span('s');
                            event.add(format!("{name}("));
                            self.slur_names.insert(number, Some(name));
                        }
                    }
                    Some("stop") => {
                        if let Some(Some(name)) = self.slur_names.remove(&number) {
                            if !is_grace {
                                event.add(format!("{name})"));
                            }
                        }
                    }
                    _ => {}
                }
            }
            for group in children(notations, "articulations") {
                for mark in elements(group) {
                    let annotations: &[&str] = match mark.name() {
                        "staccato" => &["stacc"],
                        "staccatissimo" | "spiccato" => &["staccatissimo"],
                        "tenuto" => &["tenuto"],
                        "accent" => &["accent"],
                        "strong-accent" => &["marcato"],
                        "detached-legato" => &["tenuto", "stacc"],
                        "breath-mark" | "caesura" => &["breath"],
                        _ => &[],
                    };
                    for annotation in annotations {
                        event.add(*annotation);
                    }
                }
            }
            for group in children(notations, "ornaments") {
                for mark in elements(group) {
                    let name = mark.name();
                    if let Some((_, annotation)) = ORNAMENTS.iter().find(|(tag, _)| *tag == name) {
                        event.add(*annotation);
                    } else if name == "tremolo"
                        && mark.attribute("type").unwrap_or("single") == "single"
                    {
                        let strokes: u32 = element_text(mark).trim().parse().unwrap_or(3);
                        if (1..=4).contains(&strokes) {
                            event.add(format!("tremolo={}", 8 * 2_u32.pow(strokes - 1)));
                        }
                    } else if name == "tremolo" {
                        self.score
                            .warn("two-note tremolos are written as plain notes");
                    }
                }
            }
            if child(notations, "fermata").is_some() {
                event.add("fermata");
            }
            for technical in children(notations, "technical") {
                for fingering in children(technical, "fingering") {
                    let digits = element_text(fingering).trim().to_string();
                    if !digits.is_empty()
                        && digits.chars().all(|c| c.is_ascii_digit())
                        && !event.annotations.iter().any(|mark| mark.starts_with("f="))
                    {
                        event.add(format!("f={digits}"));
                    }
                }
            }
            if let Some(arpeggiate) = child(notations, "arpeggiate") {
                match arpeggiate.attribute("direction") {
                    Some(direction @ ("up" | "down")) => event.add(format!("arpeggio={direction}")),
                    _ => event.add("arpeggio"),
                }
            }
            for dynamics in children(notations, "dynamics") {
                for mark in elements(dynamics) {
                    if is_dynamic(mark.name()) {
                        event.add(format!("dyn={}", mark.name()));
                    }
                }
            }
        }
    }

    fn read_lyrics(&mut self, node: XNode<'_>, event: &mut Event) {
        for lyric in children(node, "lyric") {
            let number = lyric.attribute("number").unwrap_or("1").to_string();
            let verse = match self.verse_numbers.iter().position(|known| *known == number) {
                Some(index) => index,
                None => {
                    self.verse_numbers.push(number);
                    self.verse_numbers.len() - 1
                }
            };
            let texts: Vec<String> = children(lyric, "text").map(element_text).collect();
            if texts.is_empty() {
                continue;
            }
            // Elided syllables share one note; join them with an undertie.
            let text = if texts.len() > 1 {
                texts
                    .iter()
                    .map(|part| part.trim())
                    .collect::<Vec<_>>()
                    .join("\u{203f}")
            } else {
                texts[0].clone()
            };
            let syllabic = text_at(lyric, "syllabic").unwrap_or_else(|| "single".into());
            let extend = child(lyric, "extend")
                .is_some_and(|extend| extend.attribute("type").unwrap_or("start") != "stop");
            event.lyrics.insert(
                verse,
                Lyric {
                    text: text.trim().to_string(),
                    hyphen_after: syllabic == "begin" || syllabic == "middle",
                    extend,
                },
            );
        }
    }

    fn read_direction(&mut self, node: XNode<'_>, cursor: Frac) {
        let offset = self.length(node, "offset");
        let onset = (cursor + offset).max(Frac::ZERO);
        let staff_number: usize = text_at(node, "staff")
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        let staff_id = self.staff_id(staff_number);
        let below = node.attribute("placement") == Some("below");
        let mut words: Vec<String> = Vec::new();
        let mut metronome = None;
        let sound = child(node, "sound");
        for direction_type in children(node, "direction-type") {
            for item in elements(direction_type) {
                match item.name() {
                    "dynamics" => {
                        for mark in elements(item) {
                            let name = mark.name();
                            if is_dynamic(name) {
                                self.info().directions.push(Direction {
                                    staff: staff_id.clone(),
                                    onset,
                                    annotation: format!("dyn={name}"),
                                    stop: false,
                                });
                            } else if name == "other-dynamics" {
                                let text = element_text(mark).trim().to_string();
                                if !text.is_empty() {
                                    words.push(text);
                                }
                            }
                        }
                    }
                    "wedge" => {
                        let number = item.attribute("number").unwrap_or("1").to_string();
                        match item.attribute("type") {
                            Some(kind @ ("crescendo" | "diminuendo")) => {
                                let name = self.counters.span('h');
                                let symbol = if kind == "crescendo" { "<" } else { ">" };
                                self.wedge_names.insert(number, name.clone());
                                self.info().directions.push(Direction {
                                    staff: staff_id.clone(),
                                    onset,
                                    annotation: format!("{name}{symbol}"),
                                    stop: false,
                                });
                            }
                            Some("stop") => {
                                if let Some(name) = self.wedge_names.remove(&number) {
                                    self.info().directions.push(Direction {
                                        staff: staff_id.clone(),
                                        onset,
                                        annotation: format!("{name}!"),
                                        stop: true,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                    "words" => {
                        let text = element_text(item)
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ");
                        if !text.is_empty() {
                            words.push(text);
                        }
                    }
                    "metronome" => metronome = Some(item),
                    "rehearsal" => {
                        let text = element_text(item).trim().to_string();
                        if !text.is_empty() {
                            self.info().rehearsal = Some(text);
                        }
                    }
                    tag @ ("segno" | "coda") => self.info().navigation = Some(tag.to_string()),
                    "pedal" => self.read_pedal(item, &staff_id, onset),
                    unsupported => self
                        .score
                        .warn(format!("direction {unsupported:?} could not be reproduced")),
                }
            }
        }
        let mut tempo = None;
        if metronome.is_some()
            || (sound.is_some_and(|sound| sound.attribute("tempo").is_some()) && !words.is_empty())
        {
            let mut value = Tempo::default();
            if !words.is_empty() {
                value.text = Some(words.join(" "));
            }
            if let Some(metronome) = metronome {
                let unit = text_at(metronome, "beat-unit")
                    .as_deref()
                    .and_then(note_type);
                let per_minute = text_at(metronome, "per-minute");
                let beat = unit.and_then(tempo_beat);
                let digits: String = per_minute
                    .unwrap_or_default()
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                if let (Some(beat), true, false) = (
                    beat,
                    child(metronome, "beat-unit-dot").is_none(),
                    digits.is_empty(),
                ) {
                    value.beat = Some(beat);
                    value.bpm = digits.parse().ok();
                } else {
                    self.score
                        .warn("metronome marks with dotted or unusual beats are omitted");
                }
            }
            if value != Tempo::default() {
                tempo = Some(value);
            }
        }
        if let Some(tempo) = tempo {
            let info = self.info();
            if info.tempo.is_none() {
                info.tempo = Some(tempo);
            }
            return;
        }
        for text in words {
            let info = self.info();
            // Navigation marks sit at the bar's left edge; later ones stay text.
            if is_navigation_word(&text) && onset == Frac::ZERO && info.navigation.is_none() {
                info.navigation = Some(text);
            } else {
                let prefix = if below { "text-below=" } else { "text=" };
                info.directions.push(Direction {
                    staff: staff_id.clone(),
                    onset,
                    annotation: format!("{prefix}{text}"),
                    stop: false,
                });
            }
        }
    }

    fn read_pedal(&mut self, node: XNode<'_>, staff_id: &str, onset: Frac) {
        let kind = node.attribute("type").unwrap_or("");
        if matches!(kind, "stop" | "change") {
            if let Some(name) = self.pedal_name.take() {
                self.info().directions.push(Direction {
                    staff: staff_id.to_string(),
                    onset,
                    annotation: format!("{name})"),
                    stop: true,
                });
            }
        }
        if matches!(kind, "start" | "change") {
            let name = self.counters.span('p');
            self.pedal_name = Some(name.clone());
            self.info().directions.push(Direction {
                staff: staff_id.to_string(),
                onset,
                annotation: format!("{name}("),
                stop: false,
            });
        }
    }

    fn read_barline(&mut self, node: XNode<'_>) {
        let location = node.attribute("location").unwrap_or("right");
        let style = text_at(node, "bar-style");
        let repeat = child(node, "repeat").and_then(|repeat| repeat.attribute("direction"));
        let info = self.info();
        if location == "left" {
            if repeat == Some("forward") {
                info.barline_left = Some("repeat-start".into());
            }
        } else if location == "right" {
            if repeat == Some("backward") {
                info.barline_right = Some("repeat-end".into());
            } else {
                match style.as_deref() {
                    Some("light-light") => info.barline_right = Some("double".into()),
                    Some("light-heavy") => info.barline_right = Some("final".into()),
                    Some("dashed" | "dotted") => info.barline_right = Some("dashed".into()),
                    _ => {}
                }
            }
        }
        if let Some(ending) = child(node, "ending") {
            let text = element_text(ending).trim().to_string();
            let label = if text.is_empty() {
                ending
                    .attribute("number")
                    .unwrap_or("1")
                    .split(',')
                    .map(str::trim)
                    .collect::<Vec<_>>()
                    .join(", ")
                    + "."
            } else {
                text
            };
            let current = info.ending.get_or_insert(Ending {
                label: label.clone(),
                start: false,
                stop: false,
            });
            match ending.attribute("type") {
                Some("start") => {
                    current.start = true;
                    current.label = label;
                }
                Some("stop" | "discontinue") => current.stop = true,
                _ => {}
            }
        }
    }
}

fn set_pair(pairs: &mut Vec<(String, String)>, key: &str, value: String) {
    if let Some(entry) = pairs.iter_mut().find(|(id, _)| id == key) {
        entry.1 = value;
    } else {
        pairs.push((key.to_string(), value));
    }
}

fn is_navigation_word(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let compact: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    compact.starts_with("d.c.")
        || compact.starts_with("d.s.")
        || lower.starts_with("da capo")
        || lower.starts_with("dal segno")
        || lower.starts_with("to coda")
        || lower == "fine"
}

fn accidental_text(alter: Option<String>) -> &'static str {
    match alter
        .and_then(|value| parse_number(&value))
        .map(round_half_even)
    {
        Some(-2) => "bb",
        Some(-1) => "b",
        Some(1) => "#",
        Some(2) => "##",
        _ => "",
    }
}

fn harmony_symbol(node: XNode<'_>) -> Option<String> {
    let kind = child(node, "kind");
    let Some(root) = child(node, "root") else {
        if kind.is_some_and(|kind| element_text(kind).trim() == "none") {
            return Some("N.C.".into());
        }
        return None;
    };
    let mut symbol = text_at(root, "root-step").unwrap_or_default();
    symbol.push_str(accidental_text(text_at(root, "root-alter")));
    if let Some(kind) = kind {
        if let Some(text) = kind.attribute("text") {
            symbol.push_str(text);
        } else {
            let name = element_text(kind);
            let name = name.trim();
            symbol.push_str(
                HARMONY_KINDS
                    .iter()
                    .find(|(kind, _)| *kind == name)
                    .map(|(_, text)| *text)
                    .unwrap_or(""),
            );
        }
    }
    for degree in children(node, "degree") {
        let value = text_at(degree, "degree-value").unwrap_or_default();
        let alter = accidental_text(text_at(degree, "degree-alter"));
        let prefix = match text_at(degree, "degree-type").as_deref().unwrap_or("add") {
            "add" => "add",
            "subtract" => "no",
            _ => "",
        };
        symbol.push_str(&format!("{prefix}{alter}{value}"));
    }
    if let Some(bass) = child(node, "bass") {
        symbol.push('/');
        symbol.push_str(&text_at(bass, "bass-step").unwrap_or_default());
        symbol.push_str(accidental_text(text_at(bass, "bass-alter")));
    }
    let symbol: String = symbol.chars().filter(|c| !c.is_whitespace()).collect();
    (!symbol.is_empty()).then_some(symbol)
}

// -- whole score ---------------------------------------------------------------

pub fn read(bytes: &[u8]) -> ImportResult<Score> {
    let bytes = if bytes.starts_with(b"PK") {
        unpack_mxl(bytes)?
    } else {
        bytes.to_vec()
    };
    let text = decode_text(&bytes)?;
    let document = parse_document(&text)?;
    let root = document.root();
    let root_name = root.name();
    if root_name != "score-partwise" && root_name != "score-timewise" {
        return Err(format!(
            "the file is not a MusicXML score (root element <{root_name}>)"
        ));
    }
    let mut score = Score {
        title: text_at(root, "work/work-title").or_else(|| text_at(root, "movement-title")),
        ..Score::default()
    };
    if let Some(identification) = child(root, "identification") {
        if let Some(creator) = children(identification, "creator")
            .find(|creator| creator.attribute("type") == Some("composer"))
        {
            let text = element_text(creator).trim().to_string();
            if !text.is_empty() {
                score.composer = Some(text);
            }
        }
    }

    let mut part_names: Map<String, (Option<String>, Option<String>)> = Map::new();
    if let Some(part_list) = child(root, "part-list") {
        for part in children(part_list, "score-part") {
            let visible_text = |name: &str| {
                child(part, name)
                    .filter(|node| node.attribute("print-object") != Some("no"))
                    .map(|node| element_text(node).trim().to_string())
                    .filter(|text| !text.is_empty())
            };
            part_names.insert(
                part.attribute("id").unwrap_or("").to_string(),
                (visible_text("part-name"), visible_text("part-abbreviation")),
            );
        }
    }

    let parts = part_sources(root);
    if parts.is_empty() {
        return Err("the MusicXML score has no parts".into());
    }
    let mut used_ids: Set<String> = Set::new();
    let mut counters = Counters {
        spans: Map::new(),
        tuplets: 0,
    };
    let mut readers: Vec<(Vec<String>, Vec<MeasureInfo>, Vec<NoteRecord>, Vec<Event>)> = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        let (label, short) = part_names.get(&part.id).cloned().unwrap_or((None, None));
        let staff_count = part
            .measures
            .iter()
            .flat_map(|measure| measure.children.iter())
            .flat_map(|node| node.descendants())
            .filter(|node| node.name() == "staves")
            .filter_map(|node| element_text(node).trim().parse::<usize>().ok())
            .max()
            .unwrap_or(1)
            .max(1);
        let base = slug(label.as_deref().unwrap_or(&format!("part {}", index + 1)));
        let names: Vec<String> = match staff_count {
            1 => vec![base],
            2 => vec![format!("{base}-upper"), format!("{base}-lower")],
            _ => (1..=staff_count)
                .map(|number| format!("{base}-{number}"))
                .collect(),
        };
        let mut ids = Vec::new();
        for name in names {
            let mut unique = name.clone();
            let mut counter = 2;
            while used_ids.contains(&unique) {
                unique = format!("{name}-{counter}");
                counter += 1;
            }
            used_ids.insert(unique.clone());
            ids.push(unique);
        }
        for (number, id) in ids.iter().enumerate() {
            score.staves.push(Staff {
                id: id.clone(),
                clef: "treble".into(),
                label: if number == 0 { label.clone() } else { None },
                short_label: if number == 0 { short.clone() } else { None },
            });
        }
        let mut reader = PartReader {
            staff_ids: ids.clone(),
            score: &mut score,
            counters: &mut counters,
            divisions: 1,
            events: Vec::new(),
            records: Vec::new(),
            measures: Vec::new(),
            slur_names: Map::new(),
            wedge_names: Map::new(),
            pedal_name: None,
            has_beams: false,
            verse_numbers: Vec::new(),
        };
        reader.read(part)?;
        let PartReader {
            measures,
            records,
            events,
            ..
        } = reader;
        readers.push((ids, measures, records, events));
    }

    let measure_count = readers
        .iter()
        .map(|(_, measures, _, _)| measures.len())
        .max()
        .unwrap_or(0);
    score.measures = (0..measure_count)
        .map(|index| Measure {
            number: (index + 1).to_string(),
            ..Measure::default()
        })
        .collect();
    let first_part_keys: Vec<Option<String>> =
        readers[0].1.iter().map(|info| info.key.clone()).collect();
    for (part_index, (ids, infos, records, events)) in readers.iter_mut().enumerate() {
        let mut pending_clefs: Vec<(String, String)> = Vec::new();
        for (index, info) in infos.iter_mut().enumerate() {
            let measure = &mut score.measures[index];
            if part_index == 0 {
                measure.number = info.number.clone();
                measure.time = info.time.clone();
                measure.key = info.key.clone();
            } else if info.key.is_some()
                && first_part_keys
                    .get(index)
                    .is_some_and(|key| *key != info.key)
            {
                score.warn("parts with different key signatures share the first part's key");
            }
            let measure = &mut score.measures[index];
            if measure.tempo.is_none() {
                measure.tempo = info.tempo.take();
            }
            if measure.rehearsal.is_none() {
                measure.rehearsal = info.rehearsal.take();
            }
            if measure.navigation.is_none() {
                measure.navigation = info.navigation.take();
            }
            if measure.barline_left.is_none() {
                measure.barline_left = info.barline_left.take();
            }
            if measure.barline_right.is_none() {
                measure.barline_right = info.barline_right.take();
            }
            if measure.ending.is_none() {
                measure.ending = info.ending.take();
            }
            let mut clefs = std::mem::take(&mut pending_clefs);
            for (staff, clef) in &info.clefs {
                set_pair(&mut clefs, staff, clef.clone());
            }
            for (staff, _, clef) in &info.clef_later {
                set_pair(&mut pending_clefs, staff, clef.clone());
            }
            for (staff, clef) in clefs {
                measure.set_clef(&staff, &clef);
            }
            if measure.harmony.is_empty() {
                let mut harmony = info.harmony.clone();
                harmony.sort_by_key(|(onset, _)| *onset);
                measure.harmony = harmony;
            }
            measure.directions.extend(info.directions.drain(..));
        }
        assign_voices(ids, infos.len(), records, events, &mut score);
    }

    let moved = readers
        .iter()
        .any(|(_, infos, _, _)| infos.iter().any(|info| info.clef_mid_bar));
    if let Some(first) = score.measures.first_mut() {
        let clefs = std::mem::take(&mut first.clefs);
        let time = first.time.take();
        let key = first.key.take();
        let tempo = first.tempo.take();
        for (staff_id, clef) in clefs {
            if let Some(staff) = score.staves.iter_mut().find(|staff| staff.id == staff_id) {
                staff.clef = clef;
            }
        }
        if let Some(time) = time {
            score.time = time;
        }
        if let Some(key) = key {
            score.key = key;
        }
        if tempo.is_some() {
            score.tempo = tempo;
        }
    }
    if moved {
        score.warn("mid-bar clef changes were moved to the following barline");
    }
    drop_redundant_changes(&mut score);
    Ok(score)
}

fn drop_redundant_changes(score: &mut Score) {
    let mut key = score.key.clone();
    let mut time = score.time.clone();
    let mut clefs: Map<String, String> = score
        .staves
        .iter()
        .map(|staff| (staff.id.clone(), staff.clef.clone()))
        .collect();
    for measure in &mut score.measures {
        match &measure.key {
            Some(value) if *value == key => measure.key = None,
            Some(value) => key = value.clone(),
            None => {}
        }
        match &measure.time {
            Some(value) if *value == time => measure.time = None,
            Some(value) => time = value.clone(),
            None => {}
        }
        measure.clefs.retain(|(staff, clef)| {
            if clefs.get(staff) == Some(clef) {
                false
            } else {
                clefs.insert(staff.clone(), clef.clone());
                true
            }
        });
    }
}

type VoiceKey = (String, Option<usize>);

/// Map MusicXML voices onto a fixed set of per-staff voice slots.
fn assign_voices(
    ids: &[String],
    measure_count: usize,
    records: &[NoteRecord],
    events: &mut [Event],
    score: &mut Score,
) {
    let staff_id = |number: usize| ids[number.clamp(1, ids.len()) - 1].clone();
    let mut by_measure_voice: Map<(usize, String), Vec<&NoteRecord>> = Map::new();
    for record in records {
        by_measure_voice
            .or_default((record.measure, record.voice.clone()))
            .push(record);
    }

    // A voice number reused on two staves at once is two logical voices.
    let mut logical: Vec<(usize, VoiceKey, Vec<&NoteRecord>)> = Vec::new();
    for ((measure, voice), group) in by_measure_voice.iter() {
        let mut spans: Vec<(Frac, Frac, usize)> = group
            .iter()
            .filter(|record| events[record.event].duration.is_positive())
            .map(|record| {
                let event = &events[record.event];
                (event.onset, event.onset + event.duration, record.staff)
            })
            .collect();
        spans.sort();
        let overlapping = spans.iter().enumerate().any(|(position, earlier)| {
            spans[position + 1..]
                .iter()
                .any(|later| later.0 < earlier.1 && later.2 != earlier.2)
        });
        if overlapping {
            let mut staves: Vec<usize> = Vec::new();
            for record in group {
                if !staves.contains(&record.staff) {
                    staves.push(record.staff);
                }
            }
            for staff in staves {
                let members = group
                    .iter()
                    .copied()
                    .filter(|record| record.staff == staff)
                    .collect();
                logical.push((*measure, (voice.clone(), Some(staff)), members));
            }
        } else {
            logical.push((*measure, (voice.clone(), None), group.clone()));
        }
    }

    let mut staff_counts: Map<VoiceKey, Map<usize, usize>> = Map::new();
    let mut presence: Map<VoiceKey, Set<usize>> = Map::new();
    for (measure, key, members) in &logical {
        presence.or_default(key.clone()).insert(*measure);
        let counts = staff_counts.or_default(key.clone());
        for record in members {
            let weight = usize::from(events[record.event].kind != Kind::Spacer);
            *counts.or_default(record.staff) += weight;
        }
    }
    let home = |key: &VoiceKey| -> usize {
        if let Some(staff) = key.1 {
            return staff;
        }
        let counts = &staff_counts[key];
        counts
            .iter()
            .min_by_key(|(staff, count)| (std::cmp::Reverse(**count), **staff))
            .map(|(staff, _)| *staff)
            .unwrap_or(1)
    };

    let mut keys: Vec<VoiceKey> = presence.keys().cloned().collect();
    keys.sort_by_key(|key| {
        (
            key.0.parse::<u32>().unwrap_or(999),
            key.0.clone(),
            key.1.unwrap_or(0),
        )
    });
    let mut slots: Map<VoiceKey, (usize, usize)> = Map::new();
    let mut occupied: Map<usize, Vec<Set<usize>>> = Map::new();
    for key in keys {
        let staff = home(&key);
        let staff_slots = occupied.or_default(staff);
        let here = &presence[&key];
        if let Some(index) = staff_slots.iter().position(|used| used.is_disjoint(here)) {
            staff_slots[index].extend_from(here);
            slots.insert(key, (staff, index));
        } else if staff_slots.len() >= 4 {
            score.warn(format!(
                "staff {} has more than four voices; extra voices were dropped",
                staff_id(staff)
            ));
        } else {
            staff_slots.push(here.clone());
            slots.insert(key, (staff, staff_slots.len() - 1));
        }
    }

    for measure in score.measures.iter_mut().take(measure_count) {
        for (number, id) in ids.iter().enumerate() {
            let count = occupied
                .get(&(number + 1))
                .map(Vec::len)
                .unwrap_or(0)
                .max(1);
            measure.voices.insert(id.clone(), vec![Vec::new(); count]);
        }
    }
    for (measure_index, key, members) in logical {
        let Some((staff, slot)) = slots.get(&key) else {
            continue;
        };
        let id = staff_id(*staff);
        for record in members {
            let mut event = events[record.event].clone();
            if matches!(event.kind, Kind::Rest | Kind::MeasureRest)
                && event.staff.as_deref() == Some(id.as_str())
            {
                event.staff = None;
            }
            score.measures[measure_index]
                .voices
                .get_mut(&id)
                .expect("staff slots exist")[*slot]
                .push(event);
        }
    }
}
