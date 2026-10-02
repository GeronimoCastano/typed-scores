//! Read one ABC 2.1 tune into the score model.

use super::collections::{Map, Set};

use super::frac::Frac;
use super::model::*;

const TONIC_FIFTHS: [(char, i32); 7] = [
    ('F', -1),
    ('C', 0),
    ('G', 1),
    ('D', 2),
    ('A', 3),
    ('E', 4),
    ('B', 5),
];
const MODE_OFFSETS: [(&str, i32); 11] = [
    ("maj", 0),
    ("ion", 0),
    ("mix", -1),
    ("dor", -2),
    ("min", -3),
    ("aeo", -3),
    ("m", -3),
    ("phr", -4),
    ("loc", -5),
    ("lyd", 1),
    ("", 0),
];
const LONG_DECORATIONS: &[(&str, &str)] = &[
    ("staccato", "stacc"),
    ("tenuto", "tenuto"),
    ("accent", "accent"),
    (">", "accent"),
    ("emphasis", "accent"),
    ("marcato", "marcato"),
    ("^", "marcato"),
    ("wedge", "staccatissimo"),
    ("fermata", "fermata"),
    ("invertedfermata", "fermata"),
    ("trill", "trill"),
    ("mordent", "mordent"),
    ("lowermordent", "mordent"),
    ("pralltriller", "inverted-mordent"),
    ("uppermordent", "inverted-mordent"),
    ("turn", "turn"),
    ("roll", "turn"),
    ("invertedturn", "inverted-turn"),
    ("turnx", "inverted-turn"),
    ("breath", "breath"),
    ("arpeggio", "arpeggio"),
];
const IGNORED_DECORATIONS: &[&str] = &[
    "upbow",
    "downbow",
    "open",
    "thumb",
    "snap",
    "slide",
    "plus",
    "+",
    "trill(",
    "trill)",
    "shortphrase",
    "mediumphrase",
    "longphrase",
    "editorial",
    "courtesy",
    "dot",
    "invisible",
    "xstem",
    "beambr1",
    "beambr2",
    "beamon",
    "8va(",
    "8va)",
    "8vb(",
    "8vb)",
];
const NAVIGATION_DECORATIONS: &[(&str, &str)] = &[
    ("segno", "segno"),
    ("coda", "coda"),
    ("D.S.", "D.S."),
    ("D.C.", "D.C."),
    ("dacapo", "D.C."),
    ("dacoda", "Da Coda"),
    ("fine", "Fine"),
];
const BODY_FIELDS: &str = "IKLMmNPQRrsTUVWw";

fn lookup<'a>(table: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    table
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, value)| *value)
}

fn mode_offset(mode: &str) -> Option<i32> {
    MODE_OFFSETS
        .iter()
        .find(|(name, _)| !name.is_empty() && *name == mode)
        .map(|(_, offset)| *offset)
}

fn digits_at(chars: &[char], start: usize) -> usize {
    let mut end = start;
    while end < chars.len() && chars[end].is_ascii_digit() {
        end += 1;
    }
    end
}

/// Length of `\d+(?:[-,]\d+)*` at `start`, or 0.
fn ending_numbers_at(chars: &[char], start: usize) -> usize {
    let mut end = digits_at(chars, start);
    if end == start {
        return 0;
    }
    loop {
        if end < chars.len() && matches!(chars[end], '-' | ',') {
            let next = digits_at(chars, end + 1);
            if next > end + 1 {
                end = next;
                continue;
            }
        }
        return end - start;
    }
}

/// Length of `[\d/]*` at `start`.
fn length_text_at(chars: &[char], start: usize) -> usize {
    let mut end = start;
    while end < chars.len() && (chars[end].is_ascii_digit() || chars[end] == '/') {
        end += 1;
    }
    end - start
}

/// An accidental prefix at `start`: (alteration text, length).
fn accidental_at(chars: &[char], start: usize) -> (&'static str, usize) {
    let at = |offset: usize| chars.get(start + offset).copied();
    match (at(0), at(1)) {
        (Some('^'), Some('^')) => ("^^", 2),
        (Some('^'), _) => ("^", 1),
        (Some('_'), Some('_')) => ("__", 2),
        (Some('_'), _) => ("_", 1),
        (Some('='), _) => ("=", 1),
        _ => ("", 0),
    }
}

fn is_pitch_letter(character: char) -> bool {
    matches!(character.to_ascii_uppercase(), 'A'..='G') && character.is_ascii_alphabetic()
}

struct NoteToken {
    accidental: &'static str,
    letter: char,
    octave_marks: String,
    length_text: String,
    tie: bool,
    end: usize,
}

/// Match `(accidental)?letter[',]*` plus an optional length (and tie) at `start`.
fn note_token_at(
    chars: &[char],
    start: usize,
    with_length: bool,
    with_tie: bool,
) -> Option<NoteToken> {
    let (accidental, accidental_length) = accidental_at(chars, start);
    let letter_at = start + accidental_length;
    let letter = *chars.get(letter_at)?;
    if !is_pitch_letter(letter) {
        return None;
    }
    let mut end = letter_at + 1;
    while end < chars.len() && matches!(chars[end], '\'' | ',') {
        end += 1;
    }
    let octave_marks: String = chars[letter_at + 1..end].iter().collect();
    let mut length_text = String::new();
    if with_length {
        let length = length_text_at(chars, end);
        length_text = chars[end..end + length].iter().collect();
        end += length;
    }
    let mut tie = false;
    if with_tie && chars.get(end) == Some(&'-') {
        tie = true;
        end += 1;
    }
    Some(NoteToken {
        accidental,
        letter,
        octave_marks,
        length_text,
        tie,
        end,
    })
}

/// All note tokens in a text, skipping anything else (like a regex findall).
fn find_note_tokens(chars: &[char], with_length: bool) -> Vec<NoteToken> {
    let mut tokens = Vec::new();
    let mut position = 0;
    while position < chars.len() {
        match note_token_at(chars, position, with_length, with_length) {
            Some(token) => {
                position = token.end;
                tokens.push(token);
            }
            None => position += 1,
        }
    }
    tokens
}

pub fn parse_length(text: &str) -> ImportResult<Frac> {
    if text.is_empty() {
        return Ok(Frac::ONE);
    }
    let chars: Vec<char> = text.chars().collect();
    let first = digits_at(&chars, 0);
    let mut slashes_end = first;
    while slashes_end < chars.len() && chars[slashes_end] == '/' {
        slashes_end += 1;
    }
    let last = digits_at(&chars, slashes_end);
    let invalid = || {
        format!(
            "invalid ABC duration {text:?}; use positive whole numbers and a nonzero denominator"
        )
    };
    if last != chars.len() {
        return Err(invalid());
    }
    let numerator = if first > 0 {
        text[..first].parse::<i64>().map_err(|_| invalid())?
    } else {
        1
    };
    let slashes = slashes_end - first;
    let denominator = if last > slashes_end {
        text[slashes_end..].parse::<i64>().map_err(|_| invalid())?
    } else {
        1_i64
            .checked_shl(slashes as u32)
            .filter(|value| *value > 0)
            .ok_or_else(invalid)?
    };
    if numerator <= 0 || denominator <= 0 {
        return Err(invalid());
    }
    Ok(Frac::new(numerator, denominator))
}

fn clef_from_token(token: &str) -> Option<String> {
    let value = token
        .strip_prefix("clef=")
        .unwrap_or(token)
        .to_ascii_lowercase();
    let value = if let Some(stripped) = value
        .strip_suffix("-8")
        .or_else(|| value.strip_suffix("+8"))
    {
        stripped.to_string()
    } else if value.ends_with(|c: char| c.is_ascii_digit()) {
        value[..value.len() - 1].to_string()
    } else {
        value
    };
    matches!(value.as_str(), "treble" | "bass" | "alto" | "tenor").then_some(value)
}

/// Quoted segments of a field value, and the value with them removed.
fn split_quoted(value: &str) -> (Vec<String>, String) {
    let mut words = Vec::new();
    let mut rest = String::new();
    let mut remaining = value;
    while let Some(start) = remaining.find('"') {
        let after = &remaining[start + 1..];
        let Some(end) = after.find('"') else { break };
        rest.push_str(&remaining[..start]);
        words.push(after[..end].to_string());
        remaining = &after[end + 1..];
    }
    rest.push_str(remaining);
    (words, rest)
}

pub fn parse_tempo(value: &str, unit: Frac) -> Option<Tempo> {
    let mut tempo = Tempo::default();
    let (words, rest) = split_quoted(value);
    let text = words
        .iter()
        .map(|word| word.trim())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if !text.is_empty() {
        tempo.text = Some(text);
    }
    let rest = rest.trim();
    let chars: Vec<char> = rest.chars().collect();
    let numerator_end = digits_at(&chars, 0);
    let mut matched = false;
    if numerator_end > 0 && chars.get(numerator_end) == Some(&'/') {
        let denominator_end = digits_at(&chars, numerator_end + 1);
        let mut position = denominator_end;
        while chars.get(position).is_some_and(|c| c.is_whitespace()) {
            position += 1;
        }
        if denominator_end > numerator_end + 1 && chars.get(position) == Some(&'=') {
            position += 1;
            while chars.get(position).is_some_and(|c| c.is_whitespace()) {
                position += 1;
            }
            let bpm_end = digits_at(&chars, position);
            if bpm_end > position {
                matched = true;
                let beat = Frac::new(
                    rest[..numerator_end].parse().unwrap_or(1),
                    chars[numerator_end + 1..denominator_end]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap_or(4),
                );
                if let Some(name) = tempo_beat(beat) {
                    tempo.beat = Some(name);
                    tempo.bpm = chars[position..bpm_end]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .ok();
                }
            }
        }
    }
    if !matched && !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
        if let Some(name) = tempo_beat(unit) {
            tempo.beat = Some(name);
            tempo.bpm = rest.parse().ok();
        }
    }
    (tempo != Tempo::default()).then_some(tempo)
}

#[derive(Clone, Default)]
struct Context {
    meter: Option<String>,
    unit: Option<Frac>,
    key_fifths: i32,
    key_minor: bool,
    key_extra: Map<char, i8>,
    free_meter: bool,
}

#[derive(Clone, Default)]
struct Changes {
    key: Option<String>,
    time: Option<String>,
    clef: Option<String>,
    tempo: Option<Tempo>,
}

impl Changes {
    fn is_empty(&self) -> bool {
        self.key.is_none() && self.time.is_none() && self.clef.is_none() && self.tempo.is_none()
    }

    fn update(&mut self, other: Changes) {
        if other.key.is_some() {
            self.key = other.key;
        }
        if other.time.is_some() {
            self.time = other.time;
        }
        if other.clef.is_some() {
            self.clef = other.clef;
        }
        if other.tempo.is_some() {
            self.tempo = other.tempo;
        }
    }
}

struct Bar {
    slots: Vec<Vec<usize>>,
    right: Option<String>,
    left: Option<String>,
    ending: Option<Ending>,
    harmony: Vec<(Frac, String)>,
    changes: Changes,
    rehearsal: Option<String>,
    navigation: Option<String>,
}

impl Bar {
    fn new() -> Bar {
        Bar {
            slots: vec![Vec::new()],
            right: None,
            left: None,
            ending: None,
            harmony: Vec::new(),
            changes: Changes::default(),
            rehearsal: None,
            navigation: None,
        }
    }

    fn empty(&self) -> bool {
        self.slots.iter().all(Vec::is_empty)
    }
}

#[derive(Default)]
struct Pending {
    marks: Vec<String>,
    slurs: usize,
    graces: Option<(Vec<Event>, &'static str)>,
    chord: Option<String>,
    changes: Changes,
    rehearsal: Option<String>,
}

struct Voice {
    id: String,
    name: Option<String>,
    short_name: Option<String>,
    clef: String,
    ctx: Context,
    events: Vec<Event>,
    bars: Vec<Bar>,
    bar: Bar,
    slot: usize,
    cursor: Frac,
    accidentals: Map<(char, i32), i8>,
    last_note: Option<usize>,
    beam_gap: bool,
    tuplet: Option<(Tuplet, i64, i64, i64)>,
    broken: Option<Frac>,
    slur_stack: Vec<String>,
    hairpins: Vec<String>,
    pedal: Option<String>,
    lyric_notes: Vec<(usize, usize)>,
    lyric_block: Option<Vec<(usize, usize)>>,
    verse: usize,
    ending_open: bool,
    ending_label: String,
    pending_ending: Option<String>,
    pending: Pending,
}

fn stopped(ending: Option<Ending>, label: &str) -> Option<Ending> {
    let mut ending = ending.unwrap_or(Ending {
        label: label.to_string(),
        start: false,
        stop: false,
    });
    ending.stop = true;
    Some(ending)
}

struct AbcReader<'s> {
    score: &'s mut Score,
    defaults: Context,
    voices: Vec<Voice>,
    voice: Option<usize>,
    default_clef: String,
    spans: Map<char, usize>,
    tuplets: usize,
    staff_groups: Vec<Vec<String>>,
    header: bool,
    warned: Set<String>,
}

impl<'s> AbcReader<'s> {
    fn ctx(&mut self) -> &mut Context {
        if self.header {
            &mut self.defaults
        } else {
            let index = self.current();
            &mut self.voices[index].ctx
        }
    }

    fn span(&mut self, prefix: char) -> String {
        let count = self.spans.or_default(prefix);
        *count += 1;
        format!("{prefix}{count}")
    }

    fn unit_length(&mut self) -> Frac {
        let ctx = self.ctx().clone();
        if let Some(unit) = ctx.unit {
            return unit;
        }
        match (&ctx.meter, ctx.free_meter) {
            (Some(meter), false) => {
                if meter_length(meter) < Frac::new(3, 4) {
                    Frac::new(1, 16)
                } else {
                    Frac::new(1, 8)
                }
            }
            _ => Frac::new(1, 8),
        }
    }

    fn meter_length(&mut self) -> Frac {
        let meter = self.ctx().meter.clone().unwrap_or_else(|| "4/4".into());
        meter_length(&meter)
    }

    fn key_alter(&mut self, letter: char) -> i8 {
        let ctx = self.ctx();
        ctx.key_extra
            .get(&letter)
            .copied()
            .unwrap_or_else(|| key_alterations(ctx.key_fifths)[step_index(letter)])
    }

    fn current(&mut self) -> usize {
        if let Some(index) = self.voice {
            return index;
        }
        let index = if self.voices.is_empty() {
            self.get_voice("1")
        } else {
            0
        };
        self.voice = Some(index);
        index
    }

    fn get_voice(&mut self, id: &str) -> usize {
        if let Some(index) = self.voices.iter().position(|voice| voice.id == id) {
            return index;
        }
        self.voices.push(Voice {
            id: id.to_string(),
            name: None,
            short_name: None,
            clef: self.default_clef.clone(),
            ctx: self.defaults.clone(),
            events: Vec::new(),
            bars: Vec::new(),
            bar: Bar::new(),
            slot: 0,
            cursor: Frac::ZERO,
            accidentals: Map::new(),
            last_note: None,
            beam_gap: true,
            tuplet: None,
            broken: None,
            slur_stack: Vec::new(),
            hairpins: Vec::new(),
            pedal: None,
            lyric_notes: Vec::new(),
            lyric_block: None,
            verse: 0,
            ending_open: false,
            ending_label: "1.".into(),
            pending_ending: None,
            pending: Pending::default(),
        });
        self.voices.len() - 1
    }

    fn voice(&mut self) -> &mut Voice {
        let index = self.current();
        &mut self.voices[index]
    }

    fn warn_once(&mut self, key: &str, message: String) {
        if self.warned.insert(key.to_string()) {
            self.score.warn(message);
        }
    }

    // -- fields ----------------------------------------------------------

    fn field(&mut self, letter: char, value: &str) -> ImportResult<()> {
        let value = value.trim();
        match letter {
            'T' if self.header && self.score.title.is_none() => {
                self.score.title = Some(value.to_string())
            }
            'C' if self.header => {
                self.score.composer = Some(match &self.score.composer {
                    Some(existing) => format!("{existing}, {value}"),
                    None => value.to_string(),
                });
            }
            'M' => self.set_meter(value)?,
            'L' => {
                let compact: String = value.chars().filter(|c| !c.is_whitespace()).collect();
                let Some((numerator, denominator)) = compact.split_once('/') else {
                    return Err(format!(
                        "invalid ABC L: field {value:?}; use a positive fraction such as 1/8"
                    ));
                };
                if numerator.is_empty()
                    || denominator.is_empty()
                    || !numerator.chars().all(|c| c.is_ascii_digit())
                    || !denominator.chars().all(|c| c.is_ascii_digit())
                {
                    return Err(format!(
                        "invalid ABC L: field {value:?}; use a positive fraction such as 1/8"
                    ));
                }
                self.ctx().unit = Some(parse_length(&compact)?);
            }
            'Q' => {
                let unit = self.unit_length();
                if let Some(tempo) = parse_tempo(value, unit) {
                    if self.header {
                        self.score.tempo = Some(tempo);
                    } else {
                        self.change(Changes {
                            tempo: Some(tempo),
                            ..Changes::default()
                        });
                    }
                }
            }
            'K' => {
                self.set_key(value);
                if self.header {
                    self.score.key = key_name(self.defaults.key_fifths, self.defaults.key_minor);
                    self.header = false;
                    for voice in &mut self.voices {
                        voice.ctx = self.defaults.clone();
                    }
                }
            }
            'V' => self.set_voice(value),
            'P' if !self.header => {
                let voice = self.voice();
                if voice.bar.empty() {
                    voice.bar.rehearsal = Some(value.to_string());
                } else {
                    voice.pending.rehearsal = Some(value.to_string());
                }
            }
            'I' if value.starts_with("score") || value.starts_with("staves") => {
                self.directive(&format!("%%{value}"))
            }
            _ => {}
        }
        Ok(())
    }

    fn set_meter(&mut self, value: &str) -> ImportResult<()> {
        let value = value.trim();
        let meter = if value == "C" {
            "4/4".to_string()
        } else if value == "C|" {
            "2/2".to_string()
        } else if value.eq_ignore_ascii_case("none") || value.is_empty() {
            self.ctx().free_meter = true;
            if !self.header {
                self.score
                    .warn("free meter sections are barred against the previous meter");
            }
            return Ok(());
        } else {
            let chars: Vec<char> = value.chars().collect();
            let mut position = usize::from(chars.first() == Some(&'('));
            let start = position;
            while chars
                .get(position)
                .is_some_and(|c| c.is_ascii_digit() || *c == '+')
            {
                position += 1;
            }
            let numerator_text: String = chars[start..position].iter().collect();
            if chars.get(position) == Some(&')') {
                position += 1;
            }
            while chars.get(position).is_some_and(|c| c.is_whitespace()) {
                position += 1;
            }
            let mut parsed = None;
            if position > start && chars.get(position) == Some(&'/') {
                position += 1;
                while chars.get(position).is_some_and(|c| c.is_whitespace()) {
                    position += 1;
                }
                let denominator_end = digits_at(&chars, position);
                if denominator_end > position {
                    let numerator = numerator_text
                        .split('+')
                        .try_fold(0_i64, |total, part| {
                            let beat = part.parse::<i64>().ok()?;
                            if beat <= 0 {
                                return None;
                            }
                            total.checked_add(beat)
                        })
                        .ok_or_else(|| {
                            format!("invalid ABC M: field {value:?}; use positive meter values")
                        })?;
                    let denominator: String = chars[position..denominator_end].iter().collect();
                    if denominator
                        .parse::<i64>()
                        .ok()
                        .filter(|value| *value > 0)
                        .is_none()
                        || denominator_end != chars.len()
                    {
                        return Err(format!("invalid ABC M: field {value:?}; use a positive numerator and nonzero denominator"));
                    }
                    parsed = Some(format!("{numerator}/{denominator}"));
                }
            }
            match parsed {
                Some(meter) => meter,
                None => {
                    return Err(format!(
                        "invalid ABC M: field {value:?}; use a meter such as 4/4"
                    ));
                }
            }
        };
        self.ctx().free_meter = false;
        self.ctx().meter = Some(meter.clone());
        if self.header {
            self.score.time = meter;
        } else {
            self.change(Changes {
                time: Some(meter),
                ..Changes::default()
            });
        }
        Ok(())
    }

    fn set_key(&mut self, value: &str) {
        let mut tokens: Vec<&str> = value.split_whitespace().collect();
        let mut clef = None;
        let mut extra: Map<char, i8> = Map::new();
        let rest: Vec<&str>;
        let first = tokens.first().copied().unwrap_or("");
        if !first.is_empty()
            && !first.eq_ignore_ascii_case("none")
            && first.chars().next().is_some_and(is_pitch_letter)
            && !first.starts_with("clef")
        {
            let mut chars = first.chars();
            let tonic = chars.next().expect("tonic exists").to_ascii_uppercase();
            let remainder: String = chars.collect();
            let (accidental, mut mode) = match remainder.chars().next() {
                Some('#') => (7, remainder[1..].to_string()),
                Some('b') => (-7, remainder[1..].to_string()),
                _ => (0, remainder.clone()),
            };
            let mut fifths = TONIC_FIFTHS
                .iter()
                .find(|(letter, _)| *letter == tonic)
                .map(|(_, value)| *value)
                .unwrap_or(0)
                + accidental;
            if mode.is_empty()
                && tokens.len() > 1
                && tokens[1].chars().all(|c| c.is_ascii_alphabetic())
            {
                let candidate: String = tokens[1]
                    .chars()
                    .take(3)
                    .collect::<String>()
                    .to_ascii_lowercase();
                if mode_offset(&candidate).is_some() {
                    mode = tokens[1].to_string();
                    tokens.remove(0);
                }
            }
            let mut mode_key = if mode.is_empty() {
                "maj".to_string()
            } else {
                mode.chars()
                    .take(3)
                    .collect::<String>()
                    .to_ascii_lowercase()
            };
            if mode_offset(&mode_key).is_none() {
                mode_key = if mode.eq_ignore_ascii_case("m") {
                    "m".into()
                } else {
                    "maj".into()
                };
            }
            fifths += mode_offset(&mode_key).unwrap_or(0);
            if fifths.abs() > 7 {
                self.score.warn(format!(
                    "key {} needs more than seven accidentals; its enharmonic key is used",
                    tokens[0]
                ));
                fifths = if fifths > 0 { fifths - 12 } else { fifths + 12 };
            }
            let ctx = self.ctx();
            ctx.key_fifths = fifths;
            ctx.key_minor = matches!(mode_key.as_str(), "min" | "aeo" | "m");
            rest = tokens[1..].to_vec();
        } else if first.eq_ignore_ascii_case("none") || first.eq_ignore_ascii_case("hp") {
            let ctx = self.ctx();
            ctx.key_fifths = 0;
            ctx.key_minor = false;
            rest = tokens[1..].to_vec();
        } else {
            rest = tokens.clone();
        }
        for token in rest {
            let chars: Vec<char> = token.chars().collect();
            let (accidental, length) = accidental_at(&chars, 0);
            if length > 0 && chars.len() == length + 1 && is_pitch_letter(chars[length]) {
                let alter = match accidental {
                    "^^" => 2,
                    "^" => 1,
                    "__" => -2,
                    "_" => -1,
                    _ => 0,
                };
                extra.insert(chars[length].to_ascii_uppercase(), alter);
                continue;
            }
            if let Some(found) = clef_from_token(token) {
                clef = Some(found);
            }
        }
        if !extra.is_empty() {
            self.score
                .warn("explicit key-signature accidentals are written as accidentals on the notes");
        }
        self.ctx().key_extra = extra;
        if let Some(clef) = clef {
            if self.header {
                self.default_clef = clef.clone();
                for voice in &mut self.voices {
                    voice.clef = clef.clone();
                }
            } else {
                self.change(Changes {
                    clef: Some(clef),
                    ..Changes::default()
                });
            }
        }
        if !self.header {
            let ctx = self.ctx().clone();
            self.change(Changes {
                key: Some(key_name(ctx.key_fifths, ctx.key_minor)),
                ..Changes::default()
            });
        }
    }

    fn set_voice(&mut self, value: &str) {
        let value = value.trim();
        let (id, options) = match value.split_once(char::is_whitespace) {
            Some((id, options)) => (id, options),
            None => (value, ""),
        };
        if id.is_empty() {
            return;
        }
        let index = self.get_voice(id);
        // name="..." style options, then bare clef tokens.
        let mut remaining = String::new();
        let mut rest = options;
        while let Some(equals) = rest.find('=') {
            let before = &rest[..equals];
            let name_start = before
                .trim_end()
                .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
                .map(|i| i + 1)
                .unwrap_or(0);
            let name = before.trim_end()[name_start..].to_string();
            let after = rest[equals + 1..].trim_start();
            if !name.is_empty() && after.starts_with('"') {
                if let Some(close) = after[1..].find('"') {
                    let text = after[1..1 + close].to_string();
                    remaining.push_str(&before[..name_start]);
                    let voice = &mut self.voices[index];
                    match name.as_str() {
                        "name" | "nm" => voice.name = (!text.is_empty()).then_some(text),
                        "subname" | "sname" | "snm" => {
                            voice.short_name = (!text.is_empty()).then_some(text)
                        }
                        _ => {}
                    }
                    rest = &after[close + 2..];
                    continue;
                }
            }
            remaining.push_str(&rest[..equals + 1]);
            rest = &rest[equals + 1..];
        }
        remaining.push_str(rest);
        for token in remaining.split_whitespace() {
            if let Some(found) = clef_from_token(token) {
                let voice = &mut self.voices[index];
                if self.header || (voice.bars.is_empty() && voice.bar.empty()) {
                    voice.clef = found;
                } else {
                    self.voice = Some(index);
                    self.change(Changes {
                        clef: Some(found),
                        ..Changes::default()
                    });
                }
            }
        }
        if !self.header {
            self.voice = Some(index);
        }
    }

    fn change(&mut self, changes: Changes) {
        let moves_layout =
            changes.key.is_some() || changes.time.is_some() || changes.clef.is_some();
        let voice = self.voice();
        if voice.bar.empty() {
            voice.bar.changes.update(changes);
        } else {
            voice.pending.changes.update(changes);
            if moves_layout {
                self.warn_once(
                    "mid-bar",
                    "mid-bar key, meter, or clef changes were moved to the following barline"
                        .into(),
                );
            }
        }
    }

    fn directive(&mut self, line: &str) {
        let Some(rest) = line
            .strip_prefix("%%score")
            .or_else(|| line.strip_prefix("%%staves"))
        else {
            return;
        };
        if !rest.starts_with(char::is_whitespace) {
            return;
        }
        let mut groups = Vec::new();
        let mut remaining = rest;
        while let Some(open) = remaining.find('(') {
            let after = &remaining[open + 1..];
            let Some(close) = after.find(')') else { break };
            groups.push(
                after[..close]
                    .split_whitespace()
                    .map(str::to_string)
                    .collect(),
            );
            remaining = &after[close + 1..];
        }
        self.staff_groups = groups;
    }

    // -- music -----------------------------------------------------------

    fn music_line(&mut self, line: &str) -> ImportResult<()> {
        let chars: Vec<char> = line.chars().collect();
        {
            let voice = self.voice();
            if voice.lyric_block.is_some() {
                voice.lyric_block = None;
                voice.verse = 0;
            }
        }
        let length = chars.len();
        let mut position = 0;
        let next = |position: usize| chars.get(position + 1).copied();
        while position < length {
            let character = chars[position];
            match character {
                ' ' | '\t' => {
                    self.voice().beam_gap = true;
                    position += 1;
                }
                '%' => break,
                '\\' if chars[position..].iter().collect::<String>().trim() == "\\" => break,
                '"' => {
                    let Some(end) = chars[position + 1..].iter().position(|c| *c == '"') else {
                        break;
                    };
                    let text: String = chars[position + 1..position + 1 + end].iter().collect();
                    self.quoted(&text);
                    position += end + 2;
                }
                '!' | '+' => {
                    let end = chars[position + 1..].iter().position(|c| *c == character);
                    match end {
                        Some(end)
                            if !(character == '!'
                                && chars[position + 1..position + 1 + end].contains(&' ')) =>
                        {
                            let name: String =
                                chars[position + 1..position + 1 + end].iter().collect();
                            self.decoration(&name);
                            position += end + 2;
                        }
                        // A deprecated '!' line break.
                        _ => position += 1,
                    }
                }
                '[' if next(position).is_some_and(|c| c.is_ascii_alphabetic())
                    && chars.get(position + 2) == Some(&':') =>
                {
                    let Some(end) = chars[position..].iter().position(|c| *c == ']') else {
                        break;
                    };
                    let value: String = chars[position + 3..position + end].iter().collect();
                    self.field(chars[position + 1], &value)?;
                    position += end + 1;
                }
                '[' if next(position).is_some_and(|c| c.is_ascii_digit()) => {
                    position = self.ending(&chars, position + 1);
                }
                '|' | ':' => position = self.barline(&chars, position),
                '[' if next(position) == Some('|') => position = self.barline(&chars, position),
                '.' if next(position) == Some('|') => position = self.barline(&chars, position),
                '(' if next(position).is_some_and(|c| c.is_ascii_digit()) => {
                    position = self.tuplet(&chars, position + 1)?
                }
                '(' => {
                    self.voice().pending.slurs += 1;
                    position += 1;
                }
                ')' => {
                    let voice = self.voice();
                    if let (false, Some(last)) = (voice.slur_stack.is_empty(), voice.last_note) {
                        let name = voice.slur_stack.pop().expect("slur is open");
                        voice.events[last].add(format!("{name})"));
                    }
                    position += 1;
                }
                '{' => {
                    let Some(end) = chars[position..].iter().position(|c| *c == '}') else {
                        break;
                    };
                    let inner: Vec<char> = chars[position + 1..position + end].to_vec();
                    self.grace(&inner);
                    position += end + 1;
                }
                '-' => {
                    let voice = self.voice();
                    if let Some(last) = voice.last_note {
                        voice.events[last].tie = true;
                    }
                    position += 1;
                }
                '<' | '>' => {
                    let mut count = 1;
                    while position + count < length && chars[position + count] == character {
                        count += 1;
                    }
                    self.broken(character, count as u32);
                    position += count;
                }
                '.' | '~' | 'H' | 'L' | 'M' | 'P' | 'T' | 'u' | 'v' => {
                    let mark = match character {
                        '.' => Some("stacc"),
                        '~' => Some("turn"),
                        'H' => Some("fermata"),
                        'L' => Some("accent"),
                        'M' => Some("mordent"),
                        'P' => Some("inverted-mordent"),
                        'T' => Some("trill"),
                        _ => None,
                    };
                    if let Some(mark) = mark {
                        self.voice().pending.marks.push(mark.into());
                    }
                    position += 1;
                }
                'S' | 'O' => {
                    self.navigation(if character == 'S' { "segno" } else { "coda" });
                    position += 1;
                }
                '&' => {
                    let voice = self.voice();
                    voice.slot += 1;
                    voice.cursor = Frac::ZERO;
                    voice.last_note = None;
                    while voice.bar.slots.len() <= voice.slot {
                        voice.bar.slots.push(Vec::new());
                    }
                    position += 1;
                }
                _ if character == '['
                    || matches!(character, '^' | '_' | '=')
                    || is_pitch_letter(character)
                    || matches!(character, 'z' | 'x' | 'Z' | 'X') =>
                {
                    position = self.note(&chars, position)?;
                }
                _ => position += 1, // y spacer, ` and unsupported symbols
            }
        }
        Ok(())
    }

    fn quoted(&mut self, text: &str) {
        let Some(first) = text.chars().next() else {
            return;
        };
        let voice = self.voice();
        if matches!(first, '^' | '<' | '>' | '@') {
            voice
                .pending
                .marks
                .push(format!("text={}", &text[first.len_utf8()..]));
        } else if first == '_' {
            voice
                .pending
                .marks
                .push(format!("text-below={}", &text[1..]));
        } else {
            let symbol = text
                .split(';')
                .next()
                .unwrap_or("")
                .split('\n')
                .next()
                .unwrap_or("")
                .trim();
            if !symbol.is_empty() {
                voice.pending.chord = Some(symbol.to_string());
            }
        }
    }

    fn decoration(&mut self, name: &str) {
        if let Some(mark) = lookup(LONG_DECORATIONS, name) {
            self.voice().pending.marks.push(mark.into());
        } else if is_dynamic(name) {
            self.voice().pending.marks.push(format!("dyn={name}"));
        } else if name.len() == 1 && name.chars().all(|c| c.is_ascii_digit()) {
            self.voice().pending.marks.push(format!("f={name}"));
        } else if matches!(name, "crescendo(" | "<(" | "diminuendo(" | ">(") {
            let hairpin = self.span('h');
            let symbol = if name.starts_with('c') || name.starts_with('<') {
                '<'
            } else {
                '>'
            };
            let voice = self.voice();
            voice.hairpins.push(hairpin.clone());
            voice.pending.marks.push(format!("{hairpin}{symbol}"));
        } else if matches!(name, "crescendo)" | "<)" | "diminuendo)" | ">)") {
            let voice = self.voice();
            if let Some(hairpin) = voice.hairpins.pop() {
                voice.pending.marks.push(format!("{hairpin}!"));
            }
        } else if name == "ped" {
            let pedal = self.span('p');
            let voice = self.voice();
            if let Some(open) = voice.pedal.take() {
                voice.pending.marks.push(format!("{open})"));
            }
            voice.pending.marks.push(format!("{pedal}("));
            voice.pedal = Some(pedal);
        } else if name == "ped-up" {
            let voice = self.voice();
            if let Some(open) = voice.pedal.take() {
                voice.pending.marks.push(format!("{open})"));
            }
        } else if let Some(mark) = lookup(NAVIGATION_DECORATIONS, name) {
            if matches!(name, "segno" | "coda") || self.voice().bar.empty() {
                self.navigation(mark);
            } else {
                // Navigation marks sit at a bar's left edge; later ones become text.
                self.voice().pending.marks.push(format!("text={mark}"));
            }
        } else if !IGNORED_DECORATIONS.contains(&name) {
            self.warn_once(
                &format!("decoration {name}"),
                format!("decoration !{name}! is not supported and was ignored"),
            );
        }
    }

    fn navigation(&mut self, mark: &str) {
        let voice = self.voice();
        if voice.bar.navigation.is_none() {
            voice.bar.navigation = Some(mark.to_string());
        }
    }

    fn ending(&mut self, chars: &[char], position: usize) -> usize {
        let length = ending_numbers_at(chars, position);
        let label = format!(
            "{}.",
            chars[position..position + length]
                .iter()
                .collect::<String>()
        );
        let voice_index = self.current();
        self.close_ending(voice_index);
        let voice = &mut self.voices[voice_index];
        if voice.bar.empty() {
            voice.bar.ending = Some(Ending {
                label: label.clone(),
                start: true,
                stop: false,
            });
            voice.ending_open = true;
            voice.ending_label = label;
        } else {
            voice.pending_ending = Some(label);
        }
        position + length
    }

    fn close_ending(&mut self, index: usize) {
        let voice = &mut self.voices[index];
        if !voice.ending_open {
            return;
        }
        let label = voice.ending_label.clone();
        let target = if !voice.bar.empty() || voice.bars.is_empty() {
            &mut voice.bar
        } else {
            voice.bars.last_mut().expect("a bar exists")
        };
        target.ending = stopped(target.ending.take(), &label);
        voice.ending_open = false;
    }

    fn barline(&mut self, chars: &[char], position: usize) -> usize {
        let mut end = position;
        if chars.get(end) == Some(&'.') {
            end += 1;
        }
        if chars.get(end) == Some(&'[') {
            end += 1;
        }
        while chars.get(end).is_some_and(|c| matches!(c, '|' | ':' | ']')) {
            end += 1;
        }
        let mut text: String = chars[position..end].iter().collect();
        // A trailing ']' belongs to the barline only as '|]'.
        if text.ends_with(']') && !text.ends_with("|]") {
            text.pop();
            end -= 1;
        }
        let stripped = text.trim_start_matches('.');
        let (mut right, mut left) = (None, None);
        if stripped.starts_with(':') && stripped.ends_with(':') && stripped.len() > 1 {
            right = Some("repeat-end");
            left = Some("repeat-start");
        } else if stripped.starts_with(':') {
            right = Some("repeat-end");
        } else if stripped.ends_with(':') {
            left = Some("repeat-start");
        } else if stripped == "||" {
            right = Some("double");
        } else if stripped == "|]" || stripped == "||]" {
            right = Some("final");
        } else if text.starts_with('.') {
            right = Some("dashed");
        }
        let bracket = usize::from(chars.get(end) == Some(&'['));
        let numbers = ending_numbers_at(chars, end + bracket);
        let mut new_ending = None;
        if numbers > 0 && (bracket == 1 || text.ends_with('|')) {
            new_ending = Some(format!(
                "{}.",
                chars[end + bracket..end + bracket + numbers]
                    .iter()
                    .collect::<String>()
            ));
            end += bracket + numbers;
        }
        let index = self.current();
        let voice = &mut self.voices[index];
        if voice.bar.empty() && voice.pending.changes.is_empty() {
            if let (Some(right), Some(previous)) = (right, voice.bars.last_mut()) {
                if previous.right.is_none() {
                    previous.right = Some(right.into());
                }
                if voice.ending_open {
                    previous.ending = stopped(previous.ending.take(), &voice.ending_label);
                    voice.ending_open = false;
                }
            }
        } else {
            if let Some(right) = right {
                voice.bar.right = Some(right.into());
            }
            if voice.ending_open && (right.is_some() || left.is_some() || new_ending.is_some()) {
                voice.bar.ending = stopped(voice.bar.ending.take(), &voice.ending_label);
                voice.ending_open = false;
            }
            self.close_bar(index);
        }
        if left.is_some() {
            self.voices[index].bar.left = Some("repeat-start".into());
            self.close_ending(index);
        }
        if let Some(label) = new_ending {
            self.close_ending(index);
            let voice = &mut self.voices[index];
            voice.bar.ending = Some(Ending {
                label: label.clone(),
                start: true,
                stop: false,
            });
            voice.ending_open = true;
            voice.ending_label = label;
        }
        end
    }

    fn close_bar(&mut self, index: usize) {
        let voice = &mut self.voices[index];
        let bar = std::mem::replace(&mut voice.bar, Bar::new());
        voice.bars.push(bar);
        voice.slot = 0;
        voice.cursor = Frac::ZERO;
        voice.accidentals.clear();
        voice.beam_gap = true;
        voice.tuplet = None;
        let changes = std::mem::take(&mut voice.pending.changes);
        voice.bar.changes.update(changes);
        if let Some(rehearsal) = voice.pending.rehearsal.take() {
            voice.bar.rehearsal = Some(rehearsal);
        }
        if let Some(label) = voice.pending_ending.take() {
            voice.bar.ending = Some(Ending {
                label: label.clone(),
                start: true,
                stop: false,
            });
            voice.ending_open = true;
            voice.ending_label = label;
        }
    }

    fn tuplet(&mut self, chars: &[char], position: usize) -> ImportResult<usize> {
        let p_end = digits_at(chars, position);
        let invalid = || "invalid ABC tuplet; use positive counts such as (3 or (3:2:3".to_string();
        let p: i64 = chars[position..p_end]
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| invalid())?;
        let mut end = p_end;
        let mut q = None;
        let mut r = p;
        if chars.get(end) == Some(&':') {
            let q_end = digits_at(chars, end + 1);
            if q_end > end + 1 {
                q = Some(
                    chars[end + 1..q_end]
                        .iter()
                        .collect::<String>()
                        .parse::<i64>()
                        .map_err(|_| invalid())?,
                );
            }
            end = q_end;
            if chars.get(end) == Some(&':') {
                let r_end = digits_at(chars, end + 1);
                if r_end > end + 1 {
                    r = chars[end + 1..r_end]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .map_err(|_| invalid())?;
                }
                end = r_end;
            }
        }
        let q = q.unwrap_or_else(|| {
            let numerator: i64 = self
                .ctx()
                .meter
                .as_deref()
                .and_then(|meter| meter.split('/').next())
                .and_then(|value| value.parse().ok())
                .unwrap_or(4);
            let compound = numerator % 3 == 0 && numerator > 3;
            match p {
                2 => 3,
                3 => 2,
                4 => 3,
                6 => 2,
                8 => 3,
                _ => {
                    if compound {
                        3
                    } else {
                        2
                    }
                }
            }
        });
        if p < 2
            || q <= 0
            || r <= 0
            || p > u32::MAX as i64
            || q > u32::MAX as i64
            || r > u32::MAX as i64
        {
            return Err(invalid());
        }
        if p > 1 && q > 0 {
            self.tuplets += 1;
            let tuplet = Tuplet {
                id: self.tuplets,
                actual: p as u32,
                normal: q as u32,
                bracket: None,
                number: None,
            };
            self.voice().tuplet = Some((tuplet, r, q, p));
        }
        Ok(end)
    }

    fn broken(&mut self, character: char, count: u32) {
        let voice_index = self.current();
        let voice = &mut self.voices[voice_index];
        let Some(last) = voice.last_note else { return };
        if voice.events[last].tuplet.is_some() {
            return;
        }
        let shorter = Frac::new(1, 1 << count.min(30));
        let longer = Frac::int(2) - shorter;
        let (first, second) = if character == '>' {
            (longer, shorter)
        } else {
            (shorter, longer)
        };
        let event = &mut voice.events[last];
        let new_duration = event.duration * first;
        match split_written(new_duration) {
            Some((base, dots)) => {
                voice.cursor += new_duration - event.duration;
                event.duration = new_duration;
                event.base = Some(base);
                event.dots = dots;
            }
            None => self
                .score
                .warn("broken rhythms that produce unwritable durations were ignored"),
        }
        self.voices[voice_index].broken = Some(second);
    }

    fn grace(&mut self, chars: &[char]) {
        let slash = chars.first() == Some(&'/');
        let tokens = find_note_tokens(chars, false);
        let base = if tokens.len() == 1 {
            Frac::new(1, 8)
        } else {
            Frac::new(1, 16)
        };
        let mut graces = Vec::new();
        for token in tokens {
            let pitch = self.pitch(token.accidental, token.letter, &token.octave_marks);
            let mut event = Event::new(Kind::Note, Frac::ZERO);
            event.base = Some(base);
            event.pitches = vec![pitch];
            graces.push(event);
        }
        if !graces.is_empty() {
            self.voice().pending.graces =
                Some((graces, if slash { "acciaccatura" } else { "grace" }));
        }
    }

    fn pitch(&mut self, accidental: &str, letter: char, octave_marks: &str) -> Pitch {
        let step = letter.to_ascii_uppercase();
        let mut octave = if letter.is_ascii_uppercase() { 4 } else { 5 };
        octave +=
            octave_marks.matches('\'').count() as i32 - octave_marks.matches(',').count() as i32;
        let key_alter = self.key_alter(step);
        let voice = self.voice();
        let alter = if !accidental.is_empty() {
            let alter = match accidental {
                "^^" => 2,
                "^" => 1,
                "__" => -2,
                "_" => -1,
                _ => 0,
            };
            voice.accidentals.insert((step, octave), alter);
            alter
        } else if let Some(alter) = voice.accidentals.get(&(step, octave)) {
            *alter
        } else if let Some(tied) = voice
            .last_note
            .map(|index| &voice.events[index])
            .filter(|event| event.tie)
            .and_then(|event| {
                event
                    .pitches
                    .iter()
                    .find(|pitch| pitch.step == step && pitch.octave == octave)
            })
        {
            tied.alter
        } else {
            key_alter
        };
        Pitch {
            step,
            alter,
            octave,
            staff: None,
        }
    }

    fn note(&mut self, chars: &[char], position: usize) -> ImportResult<usize> {
        let character = chars[position];
        let mut pitches: Vec<Pitch> = Vec::new();
        let mut kind = Kind::Note;
        let mut multi_bars = 0;
        let mut tied = false;
        let multiplier;
        let mut position = position;
        if character == '[' {
            let Some(end) = chars[position..]
                .iter()
                .position(|c| *c == ']')
                .map(|offset| position + offset)
            else {
                return Ok(chars.len());
            };
            let members = find_note_tokens(&chars[position + 1..end], true);
            tied = !members.is_empty() && members.iter().all(|member| member.tie);
            let mut inner_length = None;
            for member in &members {
                let pitch = self.pitch(member.accidental, member.letter, &member.octave_marks);
                if pitches.iter().all(|existing| existing.key() != pitch.key()) {
                    pitches.push(pitch);
                }
                if inner_length.is_none() {
                    inner_length = Some(parse_length(&member.length_text)?);
                }
            }
            position = end + 1;
            let length = length_text_at(chars, position);
            let text: String = chars[position..position + length].iter().collect();
            multiplier = inner_length.unwrap_or(Frac::ONE) * parse_length(&text)?;
            position += length;
            if pitches.is_empty() {
                return Ok(position);
            }
        } else if matches!(character, 'z' | 'x' | 'Z' | 'X') {
            let length = length_text_at(chars, position + 1);
            let text: String = chars[position + 1..position + 1 + length].iter().collect();
            position += 1 + length;
            if matches!(character, 'Z' | 'X') {
                multi_bars = if !text.is_empty() && text.chars().all(|c| c.is_ascii_digit()) {
                    text.parse().unwrap_or(1)
                } else {
                    1
                };
                multiplier = Frac::ONE;
            } else {
                multiplier = parse_length(&text)?;
            }
            kind = if matches!(character, 'z' | 'Z') {
                Kind::Rest
            } else {
                Kind::Spacer
            };
        } else {
            let Some(token) = note_token_at(chars, position, true, false) else {
                return Ok(position + 1);
            };
            pitches.push(self.pitch(token.accidental, token.letter, &token.octave_marks));
            multiplier = parse_length(&token.length_text)?;
            position = token.end;
        }

        if multi_bars > 0 {
            self.multi_bar_rest(multi_bars, kind);
            return Ok(position);
        }
        let mut written = self.unit_length() * multiplier;
        let voice = self.voice();
        if let Some(broken) = voice.broken.take() {
            written = written * broken;
        }
        let mut tuplet = None;
        let mut sounding = written;
        if let Some((current, remaining, q, p)) = &mut voice.tuplet {
            tuplet = Some(current.clone());
            sounding = written * Frac::new(*q, *p);
            *remaining -= 1;
            if *remaining <= 0 {
                voice.tuplet = None;
            }
        }
        self.add_event(kind, pitches, written, sounding, tuplet, tied)?;
        Ok(position)
    }

    fn add_event(
        &mut self,
        kind: Kind,
        pitches: Vec<Pitch>,
        written: Frac,
        sounding: Frac,
        tuplet: Option<Tuplet>,
        tied: bool,
    ) -> ImportResult<()> {
        if !written.is_positive() || !sounding.is_positive() {
            return Err(
                "ABC note durations must be positive; check the L: field and note lengths".into(),
            );
        }
        let pieces = match split_written(written) {
            Some(piece) => vec![piece],
            None if written.is_binary() => binary_pieces(written)?,
            None => return Err(format!("a note of {written} whole notes cannot be written")),
        };
        let ratio = sounding / written;
        let mut slurs = Vec::new();
        {
            let voice = self.voice();
            for _ in 0..voice.pending.slurs {
                slurs.push(());
            }
        }
        let slur_names: Vec<String> = if kind == Kind::Note {
            slurs.iter().map(|_| self.span('s')).collect()
        } else {
            Vec::new()
        };
        let voice = self.voice();
        let pending = std::mem::take(&mut voice.pending);
        voice.pending.changes = pending.changes.clone();
        voice.pending.rehearsal = pending.rehearsal.clone();
        let mut indices = Vec::new();
        let count = pieces.len();
        for (index, (base, dots)) in pieces.into_iter().enumerate() {
            let value = written_value(base, dots) * ratio;
            let mut event = Event::new(kind, value);
            event.base = Some(base);
            event.dots = dots;
            event.pitches = pitches.clone();
            event.tuplet = tuplet.clone();
            event.onset = voice.cursor;
            if kind == Kind::Note && index < count - 1 {
                event.tie = true;
            }
            voice.cursor += value;
            voice.events.push(event);
            indices.push(voice.events.len() - 1);
        }
        let first = indices[0];
        let last = *indices.last().expect("at least one piece");
        if kind == Kind::Note {
            voice.events[last].tie = tied;
            for mark in &pending.marks {
                voice.events[first].add(mark.clone());
            }
            for name in slur_names {
                voice.events[first].add(format!("{name}("));
                voice.slur_stack.push(name);
            }
            if let Some((graces, grace_kind)) = pending.graces {
                voice.events[first].graces = graces;
                voice.events[first].grace_kind = grace_kind;
            }
            let continuation = voice
                .last_note
                .is_some_and(|previous| voice.events[previous].tie);
            if let Some(previous) = voice.last_note {
                if voice.events[first].flagged() {
                    let previous_flagged = voice.events[previous].flagged();
                    voice.events[previous].beam_next = Some(!voice.beam_gap && previous_flagged);
                }
            }
            if !continuation {
                voice.lyric_notes.push((voice.bars.len(), first));
            }
            voice.last_note = Some(last);
            voice.beam_gap = false;
            for index in &indices {
                if voice.events[*index].flagged() && voice.events[*index].beam_next.is_none() {
                    voice.events[*index].beam_next = Some(*index != last);
                }
            }
        } else {
            if kind == Kind::Rest {
                for mark in &pending.marks {
                    if ["dyn=", "text=", "text-below=", "h", "p"]
                        .iter()
                        .any(|prefix| mark.starts_with(prefix))
                    {
                        voice.events[first].add(mark.clone());
                    }
                }
            }
            if let Some(previous) = voice.last_note {
                let flagged = voice.events[previous].flagged();
                voice.events[previous].beam_next = if flagged { Some(false) } else { None };
            }
            voice.last_note = None;
            voice.beam_gap = true;
        }
        if let Some(chord) = pending.chord {
            let onset = voice.events[first].onset;
            voice.bar.harmony.push((onset, chord));
        }
        while voice.bar.slots.len() <= voice.slot {
            voice.bar.slots.push(Vec::new());
        }
        let slot = voice.slot;
        voice.bar.slots[slot].extend(indices);
        Ok(())
    }

    fn multi_bar_rest(&mut self, count: usize, kind: Kind) {
        let length = self.meter_length();
        let index = self.current();
        for number in 0..count {
            if number > 0 {
                self.close_bar(index);
            }
            let voice = &mut self.voices[index];
            let event = Event::new(
                if kind == Kind::Rest {
                    Kind::MeasureRest
                } else {
                    Kind::Spacer
                },
                length,
            );
            voice.events.push(event);
            let event_index = voice.events.len() - 1;
            voice.bar.slots[0].push(event_index);
            voice.cursor = length;
        }
        let voice = &mut self.voices[index];
        voice.last_note = None;
        voice.pending.marks.clear();
    }

    fn lyrics(&mut self, text: &str) {
        let voice = self.voice();
        match voice.lyric_block {
            None => {
                voice.lyric_block = Some(std::mem::take(&mut voice.lyric_notes));
                voice.verse = 0;
            }
            Some(_) => voice.verse += 1,
        }
        let notes = voice.lyric_block.clone().unwrap_or_default();
        let verse = voice.verse;
        let mut index = 0;
        let mut previous: Option<usize> = None;
        let mut syllable = String::new();
        let chars: Vec<char> = text.chars().collect();
        let flush = |hyphen: bool,
                     index: &mut usize,
                     previous: &mut Option<usize>,
                     syllable: &mut String,
                     voice: &mut Voice| {
            if !syllable.is_empty() {
                if *index < notes.len() {
                    let event = notes[*index].1;
                    voice.events[event].lyrics.insert(
                        verse,
                        Lyric {
                            text: syllable.replace('~', " "),
                            hyphen_after: hyphen,
                            extend: false,
                        },
                    );
                    *previous = Some(event);
                }
                *index += 1;
                syllable.clear();
            } else if hyphen {
                if let Some(event) = previous {
                    if let Some(lyric) = voice.events[*event].lyrics.get_mut(&verse) {
                        lyric.hyphen_after = true;
                    }
                }
            }
        };
        let mut position = 0;
        while position < chars.len() {
            let character = chars[position];
            if character == '\\' && chars.get(position + 1) == Some(&'-') {
                syllable.push('-');
                position += 2;
                continue;
            }
            match character {
                ' ' | '\t' => flush(false, &mut index, &mut previous, &mut syllable, voice),
                '-' => flush(true, &mut index, &mut previous, &mut syllable, voice),
                '_' => {
                    flush(false, &mut index, &mut previous, &mut syllable, voice);
                    if let Some(event) = previous {
                        if let Some(lyric) = voice.events[event].lyrics.get_mut(&verse) {
                            lyric.extend = true;
                        }
                    }
                    index += 1;
                }
                '*' => {
                    flush(false, &mut index, &mut previous, &mut syllable, voice);
                    index += 1;
                }
                '|' => {
                    flush(false, &mut index, &mut previous, &mut syllable, voice);
                    if index < notes.len() {
                        let bar = if index > 0 {
                            notes[index - 1].0 as i64
                        } else {
                            notes[0].0 as i64 - 1
                        };
                        while index < notes.len() && notes[index].0 as i64 <= bar {
                            index += 1;
                        }
                    }
                }
                other => syllable.push(other),
            }
            position += 1;
        }
        flush(false, &mut index, &mut previous, &mut syllable, voice);
    }

    fn finish(&mut self) {
        for index in 0..self.voices.len() {
            if !self.voices[index].bar.empty() {
                self.close_bar(index);
            }
            let voice = &mut self.voices[index];
            if voice.ending_open {
                let label = voice.ending_label.clone();
                if let Some(last) = voice.bars.last_mut() {
                    last.ending = stopped(last.ending.take(), &label);
                }
            }
        }
    }
}

/// The text of one tune, chosen by its X: number or the first one.
fn select_tune(text: &str, tune: Option<&str>) -> ImportResult<String> {
    let mut tunes: Vec<String> = Vec::new();
    for line in text.split_inclusive('\n') {
        if line.starts_with("X:") {
            tunes.push(String::new());
        }
        if let Some(current) = tunes.last_mut() {
            current.push_str(line);
        }
    }
    if tunes.is_empty() {
        if text.lines().any(|line| line.starts_with("K:")) {
            return Ok(text.to_string());
        }
        return Err("no ABC tune (X: field) was found".into());
    }
    let Some(tune) = tune else {
        return Ok(tunes.remove(0));
    };
    tunes
        .into_iter()
        .find(|chunk| chunk[2..].lines().next().unwrap_or("").trim() == tune)
        .ok_or_else(|| format!("no tune with X:{tune}"))
}

pub fn read(bytes: &[u8], tune: Option<&str>) -> ImportResult<Score> {
    let text = std::str::from_utf8(bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes))
        .map_err(|_| "the ABC file is not valid UTF-8".to_string())?;
    let source = select_tune(text, tune)?;
    let mut score = Score::default();
    let mut reader = AbcReader {
        score: &mut score,
        defaults: Context {
            meter: Some("4/4".into()),
            ..Context::default()
        },
        voices: Vec::new(),
        voice: None,
        default_clef: "treble".into(),
        spans: Map::new(),
        tuplets: 0,
        staff_groups: Vec::new(),
        header: true,
        warned: Set::new(),
    };
    for raw in source.lines() {
        let line = raw.trim_end();
        if line.starts_with("%%") {
            reader.directive(line);
            continue;
        }
        if line.trim().is_empty() || line.trim_start().starts_with('%') {
            continue;
        }
        let mut chars = line.chars();
        let letter = chars.next().unwrap_or(' ');
        let is_field = letter.is_ascii_alphabetic() && chars.next() == Some(':');
        if is_field && (reader.header || BODY_FIELDS.contains(letter)) {
            let value = &line[2..];
            let value = value.split('%').next().unwrap_or("");
            if matches!(letter, 'w' | 'W') && !reader.header {
                if letter == 'w' {
                    reader.lyrics(value.trim());
                }
                continue;
            }
            reader.field(letter, value)?;
            continue;
        }
        if reader.header {
            continue;
        }
        reader.music_line(line)?;
    }
    reader.finish();
    let AbcReader {
        voices,
        staff_groups,
        ..
    } = reader;
    build_score(voices, staff_groups, score)
}

fn build_score(
    voices: Vec<Voice>,
    staff_groups: Vec<Vec<String>>,
    mut score: Score,
) -> ImportResult<Score> {
    let voices: Vec<Voice> = voices
        .into_iter()
        .filter(|voice| !voice.bars.is_empty())
        .collect();
    if voices.is_empty() {
        return Err("the tune contains no music".into());
    }

    // %%score (1 2) puts voices on one staff; every other voice gets its own.
    let mut staff_of: Map<String, String> = Map::new();
    let mut staff_voices: Vec<(String, Vec<usize>)> = Vec::new();
    let mut used: Set<String> = Set::new();
    for (index, voice) in voices.iter().enumerate() {
        let group = staff_groups.iter().find(|group| group.contains(&voice.id));
        let owner = group
            .and_then(|group| group.first())
            .filter(|owner| **owner != voice.id);
        if let Some(staff) = owner.and_then(|owner| staff_of.get(owner)).cloned() {
            staff_of.insert(voice.id.clone(), staff.clone());
            staff_voices
                .iter_mut()
                .find(|(id, _)| *id == staff)
                .expect("staff exists")
                .1
                .push(index);
            continue;
        }
        let base = slug(voice.name.as_deref().unwrap_or(&if voices.len() > 1 {
            format!("voice {}", voice.id)
        } else {
            "melody".into()
        }));
        let mut staff_id = base.clone();
        let mut counter = 2;
        while used.contains(&staff_id) {
            staff_id = format!("{base}-{counter}");
            counter += 1;
        }
        used.insert(staff_id.clone());
        staff_of.insert(voice.id.clone(), staff_id.clone());
        staff_voices.push((staff_id.clone(), vec![index]));
        score.staves.push(Staff {
            id: staff_id,
            clef: voice.clef.clone(),
            label: voice.name.clone(),
            short_label: voice.short_name.clone(),
        });
    }

    let measure_count = voices
        .iter()
        .map(|voice| voice.bars.len())
        .max()
        .unwrap_or(0);
    for index in 0..measure_count {
        let mut measure = Measure {
            number: (index + 1).to_string(),
            ..Measure::default()
        };
        for voice in &voices {
            let Some(bar) = voice.bars.get(index) else {
                continue;
            };
            if measure.barline_right.is_none() {
                measure.barline_right = bar.right.clone();
            }
            if measure.barline_left.is_none() {
                measure.barline_left = bar.left.clone();
            }
            if measure.ending.is_none() {
                measure.ending = bar.ending.clone();
            }
            if measure.rehearsal.is_none() {
                measure.rehearsal = bar.rehearsal.clone();
            }
            if measure.navigation.is_none() {
                measure.navigation = bar.navigation.clone();
            }
            if measure.harmony.is_empty() {
                measure.harmony = bar.harmony.clone();
            }
            if measure.time.is_none() {
                measure.time = bar.changes.time.clone();
            }
            if measure.key.is_none() {
                measure.key = bar.changes.key.clone();
            }
            if measure.tempo.is_none() {
                measure.tempo = bar.changes.tempo.clone();
            }
            if let Some(clef) = &bar.changes.clef {
                measure.set_clef(&staff_of[&voice.id], clef);
            }
        }
        for (staff_id, members) in &staff_voices {
            let mut slots: Vec<Vec<Event>> = Vec::new();
            for member in members {
                let voice = &voices[*member];
                let width = voice
                    .bars
                    .iter()
                    .map(|bar| bar.slots.len())
                    .max()
                    .unwrap_or(1);
                for slot in 0..width {
                    let events = voice
                        .bars
                        .get(index)
                        .and_then(|bar| bar.slots.get(slot))
                        .map(|indices| {
                            indices
                                .iter()
                                .map(|event| voice.events[*event].clone())
                                .collect()
                        })
                        .unwrap_or_default();
                    slots.push(events);
                }
            }
            if slots.len() > 4 {
                score.warn(format!(
                    "staff {staff_id} has more than four voices; extra voices were dropped"
                ));
                slots.truncate(4);
            }
            measure.voices.insert(staff_id.clone(), slots);
        }
        score.measures.push(measure);
    }

    let first = &mut score.measures[0];
    if let Some(key) = first.key.take() {
        score.key = key;
    }
    let first = &mut score.measures[0];
    if let Some(time) = first.time.take() {
        score.time = time;
    }
    let first = &mut score.measures[0];
    if score.tempo.is_none() {
        score.tempo = first.tempo.take();
    }
    let clefs = std::mem::take(&mut score.measures[0].clefs);
    for (staff_id, clef) in clefs {
        if let Some(staff) = score.staves.iter_mut().find(|staff| staff.id == staff_id) {
            staff.clef = clef;
        }
    }
    if voices.iter().any(|voice| voice.ctx.free_meter) {
        let longest = score
            .measures
            .iter()
            .flat_map(|measure| measure.voices.values())
            .flatten()
            .flatten()
            .map(|event| event.onset + event.duration)
            .max()
            .unwrap_or(Frac::ONE);
        score.time = format!("{}/{}", longest.n, longest.d);
        score.warn("free-meter music is written in the longest bar's meter with shorter bars as partial bars");
    }
    Ok(score)
}
