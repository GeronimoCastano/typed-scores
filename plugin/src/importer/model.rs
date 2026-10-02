//! Format-neutral score model shared by the MusicXML and ABC readers.
//!
//! Durations are fractions of a whole note. Readers fill the model; the
//! normalizer repairs what typed-scores cannot express and the emitter turns
//! the result into `score()` arguments.

use super::collections::Map;

use super::frac::Frac;

pub const DURATION_BASES: [(Frac, &str); 6] = [
    (Frac { n: 1, d: 1 }, "w"),
    (Frac { n: 1, d: 2 }, "h"),
    (Frac { n: 1, d: 4 }, "q"),
    (Frac { n: 1, d: 8 }, "e"),
    (Frac { n: 1, d: 16 }, "s"),
    (Frac { n: 1, d: 32 }, "t"),
];

pub const DYNAMICS: &[&str] = &[
    "p", "pp", "ppp", "pppp", "ppppp", "pppppp", "f", "ff", "fff", "ffff", "fffff", "ffffff", "mp",
    "mf", "sf", "sfp", "sfpp", "fp", "rf", "rfz", "sfz", "sffz", "fz", "n", "pf", "sfzp",
];

const KEY_MAJOR: [&str; 15] = [
    "Cb", "Gb", "Db", "Ab", "Eb", "Bb", "F", "C", "G", "D", "A", "E", "B", "F#", "C#",
];
const KEY_MINOR: [&str; 15] = [
    "Abm", "Ebm", "Bbm", "Fm", "Cm", "Gm", "Dm", "Am", "Em", "Bm", "F#m", "C#m", "G#m", "D#m",
    "A#m",
];
pub const STEPS: &str = "CDEFGAB";

pub type ImportResult<T> = Result<T, String>;

pub fn tempo_beat(value: Frac) -> Option<&'static str> {
    [
        (Frac::new(1, 1), "whole"),
        (Frac::new(1, 2), "half"),
        (Frac::new(1, 4), "quarter"),
        (Frac::new(1, 8), "eighth"),
        (Frac::new(1, 16), "sixteenth"),
        (Frac::new(1, 32), "thirty-second"),
    ]
    .iter()
    .find(|(base, _)| *base == value)
    .map(|(_, name)| *name)
}

pub fn is_dynamic(mark: &str) -> bool {
    DYNAMICS.contains(&mark)
}

pub fn key_name(fifths: i32, minor: bool) -> String {
    let index = (fifths.clamp(-7, 7) + 7) as usize;
    (if minor {
        KEY_MINOR[index]
    } else {
        KEY_MAJOR[index]
    })
    .to_string()
}

/// Key-signature alteration of each pitch letter, indexed like `STEPS`.
pub fn key_alterations(fifths: i32) -> [i8; 7] {
    let mut alterations = [0_i8; 7];
    if fifths > 0 {
        for letter in "FCGDAEB".chars().take(fifths as usize) {
            alterations[step_index(letter)] = 1;
        }
    } else if fifths < 0 {
        for letter in "BEADGCF".chars().take((-fifths) as usize) {
            alterations[step_index(letter)] = -1;
        }
    }
    alterations
}

pub fn step_index(step: char) -> usize {
    STEPS.find(step.to_ascii_uppercase()).unwrap_or(0)
}

pub fn base_code(base: Frac) -> Option<&'static str> {
    DURATION_BASES
        .iter()
        .find(|(value, _)| *value == base)
        .map(|(_, code)| *code)
}

pub fn written_value(base: Frac, dots: u8) -> Frac {
    base * (Frac::int(2) - Frac::new(1, 1 << dots))
}

/// (base, dots) for a value one duration token can write.
pub fn split_written(value: Frac) -> Option<(Frac, u8)> {
    for (dots, factor) in [
        (0_u8, Frac::ONE),
        (1, Frac::new(3, 2)),
        (2, Frac::new(7, 4)),
    ] {
        let base = value / factor;
        if base_code(base).is_some() {
            return Some((base, dots));
        }
    }
    None
}

pub fn duration_code(base: Frac, dots: u8) -> String {
    format!(
        "{}{}",
        base_code(base).unwrap_or("q"),
        ".".repeat(dots as usize)
    )
}

/// Greedily split a binary duration into writable (base, dots) tokens.
pub fn binary_pieces(value: Frac) -> ImportResult<Vec<(Frac, u8)>> {
    let mut candidates: Vec<(Frac, Frac, u8)> = DURATION_BASES
        .iter()
        .flat_map(|(base, _)| (0..3).map(move |dots| (written_value(*base, dots), *base, dots)))
        .collect();
    candidates.sort_by(|a, b| b.cmp(a));
    let mut pieces = Vec::new();
    let mut remaining = value;
    while remaining.is_positive() {
        if pieces.len() >= 256 {
            return Err(format!("duration {value} needs more than 256 tied note values; divide it across shorter measures"));
        }
        let piece = candidates
            .iter()
            .find(|(amount, _, _)| *amount <= remaining);
        match piece {
            Some((amount, base, dots)) => {
                pieces.push((*base, *dots));
                remaining -= *amount;
            }
            None => {
                return Err(format!(
                    "duration {value} is shorter than a thirty-second note"
                ))
            }
        }
    }
    Ok(pieces)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pitch {
    pub step: char,
    pub alter: i8,
    pub octave: i32,
    /// Display staff; None follows the voice.
    pub staff: Option<String>,
}

impl Pitch {
    pub fn text(&self) -> String {
        let accidental = match self.alter {
            -2 => "bb",
            -1 => "b",
            1 => "#",
            2 => "##",
            _ => "",
        };
        format!("{}{}{}", self.step, accidental, self.octave)
    }

    pub fn key(&self) -> (char, i8, i32) {
        (self.step, self.alter, self.octave)
    }

    pub fn diatonic(&self) -> i32 {
        self.octave * 7 + step_index(self.step) as i32
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tuplet {
    /// Events sharing an id belong to one tuplet group.
    pub id: usize,
    pub actual: u32,
    pub normal: u32,
    pub bracket: Option<String>,
    pub number: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Lyric {
    pub text: String,
    pub hyphen_after: bool,
    pub extend: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Note,
    Rest,
    Spacer,
    MeasureRest,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: Kind,
    /// Sounding length in whole notes.
    pub duration: Frac,
    pub base: Option<Frac>,
    pub dots: u8,
    pub pitches: Vec<Pitch>,
    /// Rest placement staff.
    pub staff: Option<String>,
    pub tie: bool,
    /// Beamed to the following event, when the source says.
    pub beam_next: Option<bool>,
    pub annotations: Vec<String>,
    pub tuplet: Option<Tuplet>,
    pub cue: bool,
    pub graces: Vec<Event>,
    pub grace_kind: &'static str,
    pub lyrics: Map<usize, Lyric>,
    pub onset: Frac,
}

impl Event {
    pub fn new(kind: Kind, duration: Frac) -> Event {
        Event {
            kind,
            duration,
            base: None,
            dots: 0,
            pitches: Vec::new(),
            staff: None,
            tie: false,
            beam_next: None,
            annotations: Vec::new(),
            tuplet: None,
            cue: false,
            graces: Vec::new(),
            grace_kind: "grace",
            lyrics: Map::new(),
            onset: Frac::ZERO,
        }
    }

    pub fn spacer(duration: Frac, onset: Frac) -> Event {
        let mut event = Event::new(Kind::Spacer, duration);
        event.onset = onset;
        event
    }

    pub fn flagged(&self) -> bool {
        self.kind == Kind::Note
            && self.base.is_some_and(|base| {
                base == Frac::new(1, 8) || base == Frac::new(1, 16) || base == Frac::new(1, 32)
            })
    }

    pub fn add(&mut self, annotation: impl Into<String>) {
        let annotation = annotation.into();
        if !self.annotations.contains(&annotation) {
            self.annotations.push(annotation);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Staff {
    pub id: String,
    pub clef: String,
    pub label: Option<String>,
    pub short_label: Option<String>,
}

/// A timed mark that the normalizer attaches to the nearest event.
#[derive(Clone, Debug)]
pub struct Direction {
    pub staff: String,
    pub onset: Frac,
    pub annotation: String,
    /// A span end attaches to the event it reaches.
    pub stop: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tempo {
    pub text: Option<String>,
    pub beat: Option<&'static str>,
    pub bpm: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ending {
    pub label: String,
    pub start: bool,
    pub stop: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Measure {
    pub number: String,
    pub length: Frac,
    pub time: Option<String>,
    pub key: Option<String>,
    pub clefs: Vec<(String, String)>,
    pub tempo: Option<Tempo>,
    pub rehearsal: Option<String>,
    pub navigation: Option<String>,
    pub barline_left: Option<String>,
    pub barline_right: Option<String>,
    pub ending: Option<Ending>,
    pub harmony: Vec<(Frac, String)>,
    pub directions: Vec<Direction>,
    /// Staff id -> voice slots -> events.
    pub voices: Map<String, Vec<Vec<Event>>>,
}

impl Default for Frac {
    fn default() -> Frac {
        Frac::ZERO
    }
}

impl Measure {
    pub fn set_clef(&mut self, staff: &str, clef: &str) {
        if let Some(entry) = self.clefs.iter_mut().find(|(id, _)| id == staff) {
            entry.1 = clef.to_string();
        } else {
            self.clefs.push((staff.to_string(), clef.to_string()));
        }
    }
}

#[derive(Clone, Debug)]
pub struct Score {
    pub title: Option<String>,
    pub composer: Option<String>,
    pub staves: Vec<Staff>,
    pub key: String,
    pub time: String,
    pub tempo: Option<Tempo>,
    pub measures: Vec<Measure>,
    pub warnings: Vec<String>,
}

impl Default for Score {
    fn default() -> Score {
        Score {
            title: None,
            composer: None,
            staves: Vec::new(),
            key: "C".to_string(),
            time: "4/4".to_string(),
            tempo: None,
            measures: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

impl Score {
    pub fn warn(&mut self, message: impl Into<String>) {
        let message = message.into();
        if !self.warnings.contains(&message) {
            self.warnings.push(message);
        }
    }
}

pub fn meter_length(time: &str) -> Frac {
    let (numerator, denominator) = time.split_once('/').unwrap_or(("4", "4"));
    Frac::new(
        numerator.trim().parse().unwrap_or(4),
        denominator.trim().parse().unwrap_or(4),
    )
}

/// A lowercase ASCII staff ID that Typst accepts as a dictionary key.
pub fn slug(name: &str) -> String {
    let mut value = String::new();
    let mut pending_dash = false;
    for character in fold_to_ascii(name).chars() {
        if !character.is_ascii() {
            continue;
        }
        if character.is_ascii_alphanumeric() {
            if pending_dash && !value.is_empty() {
                value.push('-');
            }
            pending_dash = false;
            value.push(character.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    if value.is_empty() {
        value = "staff".to_string();
    } else if !value.starts_with(|c: char| c.is_ascii_alphabetic()) {
        value = format!("staff-{value}");
    }
    const RESERVED: &[&str] = &[
        "key",
        "time",
        "clef",
        "partial",
        "tempo",
        "harmony",
        "figures",
        "barline",
        "ending",
        "rehearsal",
        "navigation",
        "lyrics",
        "notes",
    ];
    if RESERVED.contains(&value.as_str()) {
        value.push_str("-staff");
    }
    value
}

/// Drop diacritics from common Latin letters; other symbols become separators.
fn fold_to_ascii(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
            'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => 'A',
            'è' | 'é' | 'ê' | 'ë' => 'e',
            'È' | 'É' | 'Ê' | 'Ë' => 'E',
            'ì' | 'í' | 'î' | 'ï' => 'i',
            'Ì' | 'Í' | 'Î' | 'Ï' => 'I',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => 'o',
            'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => 'O',
            'ù' | 'ú' | 'û' | 'ü' => 'u',
            'Ù' | 'Ú' | 'Û' | 'Ü' => 'U',
            'ñ' => 'n',
            'Ñ' => 'N',
            'ç' => 'c',
            'Ç' => 'C',
            'ý' | 'ÿ' => 'y',
            other => other,
        })
        .collect()
}
