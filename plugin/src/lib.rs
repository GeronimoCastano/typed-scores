use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Letter {
    fn parse(ch: char) -> Option<Self> {
        match ch.to_ascii_uppercase() {
            'C' => Some(Self::C),
            'D' => Some(Self::D),
            'E' => Some(Self::E),
            'F' => Some(Self::F),
            'G' => Some(Self::G),
            'A' => Some(Self::A),
            'B' => Some(Self::B),
            _ => None,
        }
    }

    fn diatonic_index(self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 1,
            Self::E => 2,
            Self::F => 3,
            Self::G => 4,
            Self::A => 5,
            Self::B => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pitch {
    pub letter: Letter,
    pub accidental: Accidental,
    pub octave: i32,
}

impl Pitch {
    fn diatonic_index(&self) -> i32 {
        self.octave * 7 + self.letter.diatonic_index()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Clef {
    Treble,
    Bass,
    Alto,
    Tenor,
}

impl Clef {
    fn position_zero_diatonic_index(self) -> i32 {
        match self {
            // C4 is the first ledger line below treble staff.
            Self::Treble => 4 * 7 + Letter::C.diatonic_index(),
            // E2 is the first ledger line below bass staff.
            Self::Bass => 2 * 7 + Letter::E.diatonic_index(),
            // D3 is the first ledger line below alto staff.
            Self::Alto => 3 * 7 + Letter::D.diatonic_index(),
            // B2 is the first ledger line below tenor staff.
            Self::Tenor => 2 * 7 + Letter::B.diatonic_index(),
        }
    }

    fn default_relative_octave(self) -> i32 {
        match self {
            Self::Treble | Self::Alto | Self::Tenor => 4,
            Self::Bass => 3,
        }
    }
}

pub fn parse_clef(input: &str) -> Result<Clef, String> {
    match input.trim().to_ascii_lowercase().as_str() {
        "treble" => Ok(Clef::Treble),
        "bass" => Ok(Clef::Bass),
        "alto" => Ok(Clef::Alto),
        "tenor" => Ok(Clef::Tenor),
        other => Err(format!(
            "unknown clef {other:?}; expected treble, bass, alto, or tenor"
        )),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PitchSpec {
    letter: Letter,
    accidental: Accidental,
    octave: Option<i32>,
}

fn parse_pitch_spec(input: &str) -> Result<PitchSpec, String> {
    let input = input.trim();
    let mut chars = input.chars().peekable();

    let Some(letter_ch) = chars.next() else {
        return Err("empty pitch".to_string());
    };
    let letter = Letter::parse(letter_ch)
        .ok_or_else(|| format!("invalid pitch letter {letter_ch:?}; expected A-G or a-g"))?;

    let accidental = match chars.peek().copied() {
        Some('#') => {
            chars.next();
            if chars.peek() == Some(&'#') {
                chars.next();
                Accidental::DoubleSharp
            } else {
                Accidental::Sharp
            }
        }
        Some('b') => {
            chars.next();
            if chars.peek() == Some(&'b') {
                chars.next();
                Accidental::DoubleFlat
            } else {
                Accidental::Flat
            }
        }
        _ => Accidental::Natural,
    };

    if matches!(chars.peek(), Some('#' | 'b')) {
        return Err(format!(
            "invalid accidental in pitch {input:?}; use #, b, ##, or bb"
        ));
    }

    let octave_str: String = chars.collect();
    let octave = if octave_str.is_empty() {
        None
    } else {
        Some(
            octave_str
                .parse::<i32>()
                .map_err(|_| format!("invalid octave {octave_str:?} in pitch {input:?}"))?,
        )
    };

    Ok(PitchSpec {
        letter,
        accidental,
        octave,
    })
}

pub fn parse_pitch(input: &str) -> Result<Pitch, String> {
    let spec = parse_pitch_spec(input)?;
    let octave = spec
        .octave
        .ok_or_else(|| format!("missing octave in pitch {input:?}"))?;
    Ok(Pitch {
        letter: spec.letter,
        accidental: spec.accidental,
        octave,
    })
}

fn resolve_pitch(spec: PitchSpec, anchor: Option<&Pitch>, clef: Clef) -> Pitch {
    let octave = spec.octave.unwrap_or_else(|| {
        let Some(anchor) = anchor else {
            return clef.default_relative_octave();
        };
        let anchor_index = anchor.diatonic_index();
        let base_octave = anchor.octave;
        ((base_octave - 1)..=(base_octave + 1))
            .min_by_key(|octave| (octave * 7 + spec.letter.diatonic_index() - anchor_index).abs())
            .expect("three relative-octave candidates")
    });
    Pitch {
        letter: spec.letter,
        accidental: spec.accidental,
        octave,
    }
}

fn pitch_anchor_string(pitch: &Pitch) -> String {
    let letter = match pitch.letter {
        Letter::C => 'C',
        Letter::D => 'D',
        Letter::E => 'E',
        Letter::F => 'F',
        Letter::G => 'G',
        Letter::A => 'A',
        Letter::B => 'B',
    };
    format!("{letter}{}", pitch.octave)
}

pub fn pitch_to_staff_position(pitch: &Pitch, clef: Clef) -> i32 {
    pitch.diatonic_index() - clef.position_zero_diatonic_index()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurationBase {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}

impl DurationBase {
    fn parse(ch: char) -> Option<Self> {
        match ch {
            'w' => Some(Self::Whole),
            'h' => Some(Self::Half),
            'q' => Some(Self::Quarter),
            'e' => Some(Self::Eighth),
            's' => Some(Self::Sixteenth),
            't' => Some(Self::ThirtySecond),
            _ => None,
        }
    }

    fn denominator(self) -> u32 {
        match self {
            Self::Whole => 1,
            Self::Half => 2,
            Self::Quarter => 4,
            Self::Eighth => 8,
            Self::Sixteenth => 16,
            Self::ThirtySecond => 32,
        }
    }

    fn flag_count(self) -> u8 {
        match self {
            Self::Whole | Self::Half | Self::Quarter => 0,
            Self::Eighth => 1,
            Self::Sixteenth => 2,
            Self::ThirtySecond => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration {
    pub base: DurationBase,
    pub dots: u8,
}

pub fn parse_duration(input: &str) -> Result<Duration, String> {
    let input = input.trim();
    let mut chars = input.chars();

    let Some(base_ch) = chars.next() else {
        return Err("empty duration".to_string());
    };
    let base = DurationBase::parse(base_ch).ok_or_else(|| {
        format!("invalid duration base {base_ch:?}; expected w, h, q, e, s, or t")
    })?;

    let mut dots = 0_u8;
    for ch in chars {
        if ch != '.' {
            return Err(format!(
                "invalid duration suffix {ch:?} in {input:?}; only dots are allowed"
            ));
        }
        dots += 1;
    }

    if dots > 2 {
        return Err(format!(
            "duration {input:?} has {dots} dots; at most two dots are supported"
        ));
    }

    Ok(Duration { base, dots })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
}

impl Rational {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        assert!(denominator != 0, "rational denominator must not be zero");
        let g = gcd(numerator, denominator);
        Self {
            numerator: numerator / g,
            denominator: denominator / g,
        }
    }

    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }

    pub fn sub(self, other: Self) -> Option<Self> {
        let left = self.numerator * other.denominator;
        let right = other.numerator * self.denominator;
        if left < right {
            None
        } else {
            Some(Self::new(
                left - right,
                self.denominator * other.denominator,
            ))
        }
    }

    pub fn div_u32(self, divisor: u32) -> Self {
        Self::new(self.numerator, self.denominator * divisor)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.max(1)
}

pub fn duration_to_rational(duration: Duration) -> Rational {
    let base = Rational::new(1, duration.base.denominator());
    let dot_multiplier = match duration.dots {
        0 => Rational::new(1, 1),
        1 => Rational::new(3, 2),
        2 => Rational::new(7, 4),
        _ => unreachable!("parse_duration guarantees dots <= 2"),
    };
    base.mul(dot_multiplier)
}

fn supported_durations_descending() -> Vec<Duration> {
    let codes = [
        "w..", "w.", "w", "h..", "h.", "h", "q..", "q.", "q", "e..", "e.", "e", "s..", "s.", "s",
        "t..", "t.", "t",
    ];
    codes
        .into_iter()
        .map(|code| parse_duration(code).expect("hard-coded duration code is valid"))
        .collect()
}

fn rational_to_duration(value: Rational) -> Option<Duration> {
    supported_durations_descending()
        .into_iter()
        .find(|duration| duration_to_rational(*duration) == value)
}

fn distribute_auto_rests(remaining: Rational, count: usize) -> Option<Vec<Duration>> {
    if count == 0 {
        return if remaining.numerator == 0 {
            Some(Vec::new())
        } else {
            None
        };
    }

    let equal = remaining.div_u32(count as u32);
    if let Some(duration) = rational_to_duration(equal) {
        return Some(vec![duration; count]);
    }

    fn search(remaining: Rational, count: usize, choices: &[Duration]) -> Option<Vec<Duration>> {
        if count == 0 {
            return if remaining.numerator == 0 {
                Some(Vec::new())
            } else {
                None
            };
        }

        for duration in choices {
            let value = duration_to_rational(*duration);
            let Some(next_remaining) = remaining.sub(value) else {
                continue;
            };
            if let Some(mut tail) = search(next_remaining, count - 1, choices) {
                let mut out = vec![*duration];
                out.append(&mut tail);
                return Some(out);
            }
        }
        None
    }

    search(remaining, count, &supported_durations_descending())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
    pub tie_to_next: bool,
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rest {
    pub duration: Duration,
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ParsedEvent {
    Note(Note),
    Rest(Rest),
    Chord {
        notes: Vec<Note>,
        duration: Duration,
        tie_to_next: bool,
        annotations: Vec<String>,
    },
}

impl ParsedEvent {
    fn set_tie_to_next(&mut self) -> Result<(), String> {
        match self {
            Self::Note(note) => {
                if note.tie_to_next {
                    return Err("tie marker '~' is repeated for the same event".to_string());
                }
                note.tie_to_next = true;
            }
            Self::Rest(_) => return Err("tie marker '~' cannot follow a rest".to_string()),
            Self::Chord {
                notes, tie_to_next, ..
            } => {
                if *tie_to_next {
                    return Err("tie marker '~' is repeated for the same event".to_string());
                }
                *tie_to_next = true;
                for note in notes {
                    note.tie_to_next = true;
                }
            }
        }
        Ok(())
    }

    fn duration(&self) -> Duration {
        match self {
            Self::Note(note) => note.duration,
            Self::Rest(rest) => rest.duration,
            Self::Chord { duration, .. } => *duration,
        }
    }

    fn tie_to_next(&self) -> bool {
        match self {
            Self::Note(note) => note.tie_to_next,
            Self::Rest(_) => false,
            Self::Chord { tie_to_next, .. } => *tie_to_next,
        }
    }

    fn annotations(&self) -> Vec<String> {
        match self {
            Self::Note(note) => note.annotations.clone(),
            Self::Rest(rest) => rest.annotations.clone(),
            Self::Chord { annotations, .. } => annotations.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Event(String),
    Grace {
        style: String,
        contents: Vec<Token>,
    },
    Tremolo {
        subdivision: u32,
        contents: Vec<Token>,
    },
    Tuplet {
        numerator: u32,
        denominator: u32,
        bracket: String,
        side: String,
        contents: Vec<Token>,
    },
    Tie,
    AutoRest,
    BeamBreak,
    BeamJoin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BeamDirective {
    Auto,
    Break,
    Join,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSequenceState {
    events: Vec<SequencedEvent>,
    tuplets: Vec<ParsedTuplet>,
    tremolos: Vec<ParsedTremolo>,
    pitch_anchor: Option<Pitch>,
    duration_anchor: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GraceMeta {
    style: String,
    group: usize,
    index: usize,
    count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SequencedEvent {
    event: ParsedEvent,
    beam_directive: BeamDirective,
    duration_scale: Rational,
    grace: Option<GraceMeta>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedTuplet {
    numerator: u32,
    denominator: u32,
    bracket: String,
    side: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedTremolo {
    subdivision: u32,
    start: usize,
    end: usize,
}

pub fn parse_note(input: &str) -> Result<Note, String> {
    let event = parse_event(input)?;
    match event {
        ParsedEvent::Note(note) => Ok(note),
        _ => Err(format!("expected a single note, got {input:?}")),
    }
}

pub fn parse_chord(input: &str) -> Result<Vec<Note>, String> {
    let event = parse_event(input)?;
    match event {
        ParsedEvent::Chord { notes, .. } => Ok(notes),
        _ => Err(format!("expected a chord, got {input:?}")),
    }
}

pub fn parse_event(input: &str) -> Result<ParsedEvent, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("empty event".to_string());
    }

    if input.starts_with('(') {
        parse_chord_event(input)
    } else if input.starts_with("r:") {
        let (duration, annotations) = parse_duration_and_annotations(&input[2..])?;
        Ok(ParsedEvent::Rest(Rest {
            duration,
            annotations,
        }))
    } else {
        let (pitch_part, rest) = split_note_pitch_and_duration(input)?;
        let pitch = parse_pitch(pitch_part)?;
        let (duration, annotations) = parse_duration_and_annotations(rest)?;
        Ok(ParsedEvent::Note(Note {
            pitch,
            duration,
            tie_to_next: false,
            annotations,
        }))
    }
}

fn parse_event_relative(
    input: &str,
    pitch_anchor: &mut Option<Pitch>,
    duration_anchor: &mut Option<Duration>,
    clef: Clef,
) -> Result<ParsedEvent, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("empty event".to_string());
    }

    if input.starts_with('(') {
        let event =
            parse_chord_event_relative(input, pitch_anchor.as_ref(), duration_anchor, clef)?;
        if let ParsedEvent::Chord { notes, .. } = &event {
            *pitch_anchor = notes.first().map(|note| note.pitch.clone());
        }
        Ok(event)
    } else if input == "r" || input.starts_with("r:") || input.starts_with("r[") {
        parse_rest_event_relative(input, duration_anchor)
    } else {
        let (pitch_part, tail, duration_is_explicit) = split_note_pitch_and_tail(input)?;
        let pitch = resolve_pitch(parse_pitch_spec(pitch_part)?, pitch_anchor.as_ref(), clef);
        let (duration, annotations) =
            parse_duration_with_state(tail, duration_is_explicit, duration_anchor, input)?;
        *pitch_anchor = Some(pitch.clone());
        Ok(ParsedEvent::Note(Note {
            pitch,
            duration,
            tie_to_next: false,
            annotations,
        }))
    }
}

fn split_note_pitch_and_tail(input: &str) -> Result<(&str, &str, bool), String> {
    if let Some((pitch_part, rest)) = input.split_once(':') {
        return Ok((pitch_part, rest, true));
    }

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    if chars.is_empty() {
        return Err("empty note event".to_string());
    }

    let mut idx = 0_usize;
    let (_, first) = chars[idx];
    Letter::parse(first)
        .ok_or_else(|| format!("invalid pitch letter {first:?}; expected A-G or a-g"))?;
    idx += 1;

    let accidental_start = idx;
    while idx < chars.len() && idx - accidental_start < 2 && matches!(chars[idx].1, '#' | 'b') {
        idx += 1;
    }

    if idx < chars.len() && chars[idx].1 == '-' {
        idx += 1;
    }

    let octave_start = idx;
    while idx < chars.len() && chars[idx].1.is_ascii_digit() {
        idx += 1;
    }
    if idx == octave_start && idx > 0 && chars[idx - 1].1 == '-' {
        return Err(format!("invalid negative octave in note event {input:?}"));
    }
    if idx >= chars.len() {
        return Ok((input, "", false));
    }

    let split_byte = chars[idx].0;
    let tail = &input[split_byte..];
    if tail.starts_with('[') {
        Ok((&input[..split_byte], tail, false))
    } else if tail.chars().next().and_then(DurationBase::parse).is_some() {
        Ok((&input[..split_byte], tail, true))
    } else {
        Err(format!(
            "unexpected text {tail:?} after pitch in note event {input:?}"
        ))
    }
}

fn split_note_pitch_and_duration(input: &str) -> Result<(&str, &str), String> {
    let (pitch, tail, duration_is_explicit) = split_note_pitch_and_tail(input)?;
    if !duration_is_explicit {
        return Err(format!("missing duration in note event {input:?}"));
    }
    Ok((pitch, tail))
}

fn parse_chord_event(input: &str) -> Result<ParsedEvent, String> {
    let close = input
        .find(')')
        .ok_or_else(|| format!("missing ')' in chord event {input:?}"))?;
    let pitch_list = &input[1..close];
    let rest = input[close + 1..].trim_start();
    let duration_part = rest
        .strip_prefix(':')
        .ok_or_else(|| format!("missing chord duration after ')' in {input:?}"))?;
    let (duration, annotations) = parse_duration_and_annotations(duration_part)?;

    build_chord_event(pitch_list, duration, annotations, input)
}

fn parse_chord_event_relative(
    input: &str,
    external_pitch_anchor: Option<&Pitch>,
    duration_anchor: &mut Option<Duration>,
    clef: Clef,
) -> Result<ParsedEvent, String> {
    let close = input
        .find(')')
        .ok_or_else(|| format!("missing ')' in chord event {input:?}"))?;
    let pitch_list = &input[1..close];
    let rest = input[close + 1..].trim_start();
    let (tail, duration_is_explicit) = if let Some(tail) = rest.strip_prefix(':') {
        (tail, true)
    } else if rest.is_empty() || rest.starts_with('[') {
        (rest, false)
    } else {
        return Err(format!(
            "unexpected text {rest:?} after ')' in chord event {input:?}; use : followed by a duration"
        ));
    };
    let (duration, annotations) =
        parse_duration_with_state(tail, duration_is_explicit, duration_anchor, input)?;

    let mut notes = Vec::new();
    let mut internal_anchor = external_pitch_anchor.cloned();
    for pitch_str in pitch_list.split_whitespace() {
        let pitch = resolve_pitch(parse_pitch_spec(pitch_str)?, internal_anchor.as_ref(), clef);
        internal_anchor = Some(pitch.clone());
        notes.push(Note {
            pitch,
            duration,
            tie_to_next: false,
            annotations: annotations.clone(),
        });
    }
    if notes.is_empty() {
        return Err(format!("empty chord in {input:?}"));
    }

    Ok(ParsedEvent::Chord {
        notes,
        duration,
        tie_to_next: false,
        annotations,
    })
}

fn build_chord_event(
    pitch_list: &str,
    duration: Duration,
    annotations: Vec<String>,
    input: &str,
) -> Result<ParsedEvent, String> {
    let mut notes = Vec::new();
    for pitch_str in pitch_list.split_whitespace() {
        notes.push(Note {
            pitch: parse_pitch(pitch_str)?,
            duration,
            tie_to_next: false,
            annotations: annotations.clone(),
        });
    }
    if notes.is_empty() {
        return Err(format!("empty chord in {input:?}"));
    }

    Ok(ParsedEvent::Chord {
        notes,
        duration,
        tie_to_next: false,
        annotations,
    })
}

fn parse_duration_part_and_annotations(input: &str) -> Result<(&str, Vec<String>), String> {
    let input = input.trim();
    let (duration_part, annotation_part) = if let Some(open) = input.find('[') {
        let close = input
            .rfind(']')
            .ok_or_else(|| format!("missing closing ']' in annotations {input:?}"))?;
        if input[close + 1..].trim() != "" {
            return Err(format!("unexpected text after annotations in {input:?}"));
        }
        (&input[..open], Some(&input[open + 1..close]))
    } else {
        (input, None)
    };

    let annotations = annotation_part
        .map(|part| {
            part.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    for annotation in &annotations {
        validate_annotation(annotation)?;
    }

    Ok((duration_part.trim(), annotations))
}

fn parse_duration_and_annotations(input: &str) -> Result<(Duration, Vec<String>), String> {
    let (duration_part, annotations) = parse_duration_part_and_annotations(input)?;
    let duration = parse_duration(duration_part)?;

    Ok((duration, annotations))
}

fn parse_duration_with_state(
    input: &str,
    duration_is_explicit: bool,
    duration_anchor: &mut Option<Duration>,
    event: &str,
) -> Result<(Duration, Vec<String>), String> {
    let (duration_part, annotations) = parse_duration_part_and_annotations(input)?;
    let duration = if duration_part.is_empty() {
        if duration_is_explicit {
            return Err(format!(
                "missing duration after ':' in event {event:?}; omit the colon to inherit the previous duration"
            ));
        }
        duration_anchor.unwrap_or(Duration {
            base: DurationBase::Quarter,
            dots: 0,
        })
    } else {
        parse_duration(duration_part)?
    };
    *duration_anchor = Some(duration);

    Ok((duration, annotations))
}

fn parse_rest_event_relative(
    input: &str,
    duration_anchor: &mut Option<Duration>,
) -> Result<ParsedEvent, String> {
    let rest = &input[1..];
    let (tail, duration_is_explicit) = if let Some(tail) = rest.strip_prefix(':') {
        (tail, true)
    } else {
        (rest, false)
    };
    let (duration, annotations) =
        parse_duration_with_state(tail, duration_is_explicit, duration_anchor, input)?;

    Ok(ParsedEvent::Rest(Rest {
        duration,
        annotations,
    }))
}

fn valid_span_id(raw: &str, prefix: char, suffix: char) -> bool {
    let Some(id) = raw.strip_suffix(suffix) else {
        return false;
    };
    let Some(body) = id.strip_prefix(prefix) else {
        return false;
    };
    !body.is_empty()
        && body
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn validate_annotation(annotation: &str) -> Result<(), String> {
    const MARKS: &[&str] = &[
        "stacc",
        "staccatissimo",
        "tenuto",
        "legato",
        "accent",
        "marcato",
        "strong",
        "turn",
        "chromatic-turn",
        "fermata",
        "breath",
        "arpeggio",
    ];
    const DYNAMICS: &[&str] = &[
        "p", "pp", "ppp", "pppp", "ppppp", "pppppp", "f", "ff", "fff", "ffff", "fffff", "ffffff",
        "mp", "mf", "sf", "sfp", "sfpp", "fp", "rf", "rfz", "sfz", "sffz", "fz", "n", "pf", "sfzp",
    ];

    let valid = MARKS.contains(&annotation)
        || annotation
            .strip_prefix("f=")
            .is_some_and(|value| !value.is_empty())
        || annotation
            .strip_prefix("turn-f=")
            .is_some_and(|value| !value.is_empty())
        || annotation
            .strip_prefix("text=")
            .is_some_and(|value| !value.is_empty())
        || annotation
            .strip_prefix("text-below=")
            .is_some_and(|value| !value.is_empty())
        || annotation
            .strip_prefix("dyn=")
            .is_some_and(|value| DYNAMICS.contains(&value))
        || annotation
            .strip_prefix("arpeggio=")
            .is_some_and(|value| matches!(value, "up" | "down"))
        || annotation
            .strip_prefix("tremolo=")
            .is_some_and(|value| matches!(value, "8" | "16" | "32" | "64"))
        || valid_span_id(annotation, 's', '(')
        || valid_span_id(annotation, 's', ')')
        || valid_span_id(annotation, 'p', '(')
        || valid_span_id(annotation, 'p', ')')
        || valid_span_id(annotation, 'h', '<')
        || valid_span_id(annotation, 'h', '>')
        || valid_span_id(annotation, 'h', '!');

    if valid {
        Ok(())
    } else {
        Err(format!(
            "unknown annotation {annotation:?}; expected a documented mark, span marker, text=..., or dyn=..."
        ))
    }
}

pub fn parse_sequence(input: &str) -> Result<Vec<ParsedEvent>, String> {
    Ok(parse_sequence_marked(input, Clef::Treble, None, None)?
        .events
        .into_iter()
        .map(|item| item.event)
        .collect())
}

fn set_beam_directive(pending: &mut BeamDirective, directive: BeamDirective) -> Result<(), String> {
    if *pending != BeamDirective::Auto {
        return Err(
            "beam markers '/' and '-' cannot be repeated or combined before an event".to_string(),
        );
    }
    *pending = directive;
    Ok(())
}

fn parse_tuplet_tokens(
    numerator: u32,
    denominator: u32,
    bracket: String,
    side: String,
    tokens: &[Token],
    clef: Clef,
    pitch_anchor: &mut Option<Pitch>,
    duration_anchor: &mut Option<Duration>,
    scale: Rational,
    events: &mut Vec<SequencedEvent>,
    tuplets: &mut Vec<ParsedTuplet>,
) -> Result<(), String> {
    if numerator == 0 || denominator == 0 {
        return Err("tuplet ratio values must be positive".to_string());
    }
    let start = events.len();
    let scale = scale.mul(Rational::new(denominator, numerator));
    let mut pending_beam = BeamDirective::Auto;

    for token in tokens {
        match token {
            Token::Event(raw) => {
                events.push(SequencedEvent {
                    event: parse_event_relative(raw, pitch_anchor, duration_anchor, clef)?,
                    beam_directive: pending_beam,
                    duration_scale: scale,
                    grace: None,
                });
                pending_beam = BeamDirective::Auto;
            }
            Token::Tuplet {
                numerator,
                denominator,
                bracket,
                side,
                contents,
            } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before a tuplet".to_string());
                }
                parse_tuplet_tokens(
                    *numerator,
                    *denominator,
                    bracket.clone(),
                    side.clone(),
                    contents,
                    clef,
                    pitch_anchor,
                    duration_anchor,
                    scale,
                    events,
                    tuplets,
                )?;
            }
            Token::Grace { .. } => {
                return Err("grace groups cannot be nested inside tuplets".to_string())
            }
            Token::Tremolo { .. } => {
                return Err("alternating tremolos cannot be nested inside tuplets".to_string())
            }
            Token::AutoRest => {
                return Err("automatic rest placeholder '_' is not allowed inside a tuplet".to_string())
            }
            Token::BeamBreak => set_beam_directive(&mut pending_beam, BeamDirective::Break)?,
            Token::BeamJoin => set_beam_directive(&mut pending_beam, BeamDirective::Join)?,
            Token::Tie => {
                let Some(previous) = events.last_mut() else {
                    return Err("tie marker '~' cannot appear before a note or chord".to_string());
                };
                previous.event.set_tie_to_next()?;
            }
        }
    }

    if pending_beam != BeamDirective::Auto {
        return Err("beam marker '/' or '-' cannot end a tuplet".to_string());
    }
    if events.len() == start {
        return Err("tuplet must contain at least one note, chord, or written rest".to_string());
    }
    tuplets.push(ParsedTuplet {
        numerator,
        denominator,
        bracket,
        side,
        start,
        end: events.len(),
    });
    Ok(())
}

fn parse_grace_tokens(
    style: String,
    tokens: &[Token],
    group: usize,
    clef: Clef,
    pitch_anchor: &mut Option<Pitch>,
    duration_anchor: &mut Option<Duration>,
) -> Result<Vec<SequencedEvent>, String> {
    let mut parsed = Vec::new();
    let mut pending_beam = BeamDirective::Auto;
    for token in tokens {
        match token {
            Token::Event(raw) => {
                parsed.push(SequencedEvent {
                    event: parse_event_relative(raw, pitch_anchor, duration_anchor, clef)?,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(0, 1),
                    grace: None,
                });
                pending_beam = BeamDirective::Auto;
            }
            Token::BeamBreak => set_beam_directive(&mut pending_beam, BeamDirective::Break)?,
            Token::BeamJoin => set_beam_directive(&mut pending_beam, BeamDirective::Join)?,
            Token::Tie => {
                let Some(previous) = parsed.last_mut() else {
                    return Err("tie marker '~' cannot appear before a grace note".to_string());
                };
                previous.event.set_tie_to_next()?;
            }
            Token::Tuplet { .. } => {
                return Err("tuplets are not supported inside grace groups".to_string())
            }
            Token::Tremolo { .. } => {
                return Err("alternating tremolos are not supported inside grace groups".to_string())
            }
            Token::Grace { .. } => return Err("grace groups cannot nest".to_string()),
            Token::AutoRest => {
                return Err("automatic rest placeholder '_' is not allowed inside grace groups".to_string())
            }
        }
    }
    if pending_beam != BeamDirective::Auto {
        return Err("beam marker '/' or '-' cannot end a grace group".to_string());
    }
    if parsed.is_empty() {
        return Err("grace group must contain at least one note or chord".to_string());
    }
    if parsed.iter().any(|item| matches!(item.event, ParsedEvent::Rest(_))) {
        return Err("written rests are not supported inside grace groups".to_string());
    }
    let count = parsed.len();
    for (index, item) in parsed.iter_mut().enumerate() {
        item.grace = Some(GraceMeta {
            style: style.clone(),
            group,
            index,
            count,
        });
    }
    Ok(parsed)
}

fn parse_tremolo_tokens(
    subdivision: u32,
    tokens: &[Token],
    clef: Clef,
    pitch_anchor: &mut Option<Pitch>,
    duration_anchor: &mut Option<Duration>,
    events: &mut Vec<SequencedEvent>,
    tremolos: &mut Vec<ParsedTremolo>,
) -> Result<(), String> {
    if !matches!(subdivision, 8 | 16 | 32 | 64) {
        return Err("tremolo subdivision must be 8, 16, 32, or 64".to_string());
    }
    let start = events.len();
    let mut pending_beam = BeamDirective::Auto;
    for token in tokens {
        match token {
            Token::Event(raw) => {
                let event = parse_event_relative(raw, pitch_anchor, duration_anchor, clef)?;
                if matches!(event, ParsedEvent::Rest(_)) {
                    return Err("alternating tremolo requires notes or chords, not rests".to_string());
                }
                events.push(SequencedEvent {
                    event,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(1, 1),
                    grace: None,
                });
                pending_beam = BeamDirective::Auto;
            }
            Token::BeamBreak | Token::BeamJoin => {
                return Err("beam markers are not used inside alternating tremolos".to_string())
            }
            Token::Tie => {
                return Err("ties are not supported inside alternating tremolos".to_string())
            }
            Token::Tuplet { .. } | Token::Grace { .. } | Token::Tremolo { .. } => {
                return Err("alternating tremolo groups cannot contain nested groups".to_string())
            }
            Token::AutoRest => {
                return Err("automatic rests are not allowed inside alternating tremolos".to_string())
            }
        }
    }
    if pending_beam != BeamDirective::Auto {
        return Err("beam marker cannot end an alternating tremolo".to_string());
    }
    if events.len() != start + 2 {
        return Err("alternating tremolo must contain exactly two notes or chords".to_string());
    }
    if events[start].event.duration() != events[start + 1].event.duration() {
        return Err("alternating tremolo notes must have the same written duration".to_string());
    }
    if subdivision <= events[start].event.duration().base.denominator() {
        return Err("alternating tremolo subdivision must be shorter than its written notes".to_string());
    }
    tremolos.push(ParsedTremolo {
        subdivision,
        start,
        end: start + 1,
    });
    Ok(())
}

fn parse_sequence_marked(
    input: &str,
    clef: Clef,
    initial_pitch_anchor: Option<Pitch>,
    initial_duration_anchor: Option<Duration>,
) -> Result<ParsedSequenceState, String> {
    let tokens = tokenize_sequence(input)?;
    let mut events: Vec<SequencedEvent> = Vec::new();
    let mut tuplets = Vec::new();
    let mut tremolos = Vec::new();
    let mut pending_beam = BeamDirective::Auto;
    let mut pitch_anchor = initial_pitch_anchor;
    let mut duration_anchor = initial_duration_anchor;

    for token in tokens {
        match token {
            Token::Event(raw) => {
                events.push(SequencedEvent {
                    event: parse_event_relative(&raw, &mut pitch_anchor, &mut duration_anchor, clef)?,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(1, 1),
                    grace: None,
                });
                pending_beam = BeamDirective::Auto;
            }
            Token::Grace { style, contents } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before a grace group".to_string());
                }
                let group = events
                    .iter()
                    .filter_map(|item| item.grace.as_ref().map(|grace| grace.group))
                    .max()
                    .map_or(0, |value| value + 1);
                events.extend(parse_grace_tokens(
                    style,
                    &contents,
                    group,
                    clef,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                )?);
            }
            Token::Tremolo { subdivision, contents } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before an alternating tremolo".to_string());
                }
                parse_tremolo_tokens(
                    subdivision,
                    &contents,
                    clef,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                    &mut events,
                    &mut tremolos,
                )?;
            }
            Token::Tuplet {
                numerator,
                denominator,
                bracket,
                side,
                contents,
            } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before a tuplet".to_string());
                }
                parse_tuplet_tokens(
                    numerator,
                    denominator,
                    bracket,
                    side,
                    &contents,
                    clef,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                    Rational::new(1, 1),
                    &mut events,
                    &mut tuplets,
                )?;
            }
            Token::AutoRest => {
                return Err("auto rest placeholder '_' needs a time signature context".to_string())
            }
            Token::BeamBreak => {
                set_beam_directive(&mut pending_beam, BeamDirective::Break)?;
            }
            Token::BeamJoin => {
                set_beam_directive(&mut pending_beam, BeamDirective::Join)?;
            }
            Token::Tie => {
                let Some(previous) = events.last_mut() else {
                    return Err("tie marker '~' cannot appear before a note or chord".to_string());
                };
                previous.event.set_tie_to_next()?;
            }
        }
    }

    if pending_beam != BeamDirective::Auto {
        return Err("beam marker '/' or '-' cannot end a sequence".to_string());
    }
    if events.last().is_some_and(|item| item.grace.is_some()) {
        return Err("grace group must be followed by a main note, chord, or rest".to_string());
    }

    Ok(ParsedSequenceState {
        events,
        tuplets,
        tremolos,
        pitch_anchor,
        duration_anchor,
    })
}

fn tokenize_sequence(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0_usize;

    while i < chars.len() {
        match chars[i] {
            ch if ch.is_whitespace() => i += 1,
            '|' => {
                return Err(
                    "bar separator '|' is not allowed inside notes; create another bars entry"
                        .to_string(),
                )
            }
            '/' => {
                tokens.push(Token::BeamBreak);
                i += 1;
            }
            '-' => {
                tokens.push(Token::BeamJoin);
                i += 1;
            }
            '_' => {
                tokens.push(Token::AutoRest);
                i += 1;
            }
            '~' => {
                tokens.push(Token::Tie);
                i += 1;
            }
            '(' => {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != ')' {
                    i += 1;
                }
                if i == chars.len() {
                    return Err("unterminated chord in sequence".to_string());
                }
                i += 1;
                i = consume_event_suffix(&chars, i)?;
                tokens.push(Token::Event(chars[start..i].iter().collect()));
            }
            '{' | '}' => {
                return Err("tuplet braces must follow 'tuplet N:M'".to_string())
            }
            _ => {
                if chars[i..].starts_with(&['t', 'r', 'e', 'm', 'o', 'l', 'o'])
                    && i + 7 < chars.len()
                    && chars[i + 7].is_whitespace()
                {
                    tokens.push(parse_tremolo_token(&chars, &mut i)?);
                    continue;
                }
                let mut parsed_grace = false;
                for (keyword, style) in [
                    ("acciaccatura", "acciaccatura"),
                    ("appoggiatura", "appoggiatura"),
                    ("grace", "grace"),
                ] {
                    let keyword_chars: Vec<char> = keyword.chars().collect();
                    if chars[i..].starts_with(&keyword_chars)
                        && i + keyword_chars.len() < chars.len()
                        && chars[i + keyword_chars.len()].is_whitespace()
                    {
                        tokens.push(parse_grace_token(&chars, &mut i, keyword, style)?);
                        parsed_grace = true;
                        break;
                    }
                }
                if parsed_grace {
                    continue;
                }
                if chars[i..].starts_with(&['t', 'u', 'p', 'l', 'e', 't'])
                    && i + 6 < chars.len()
                    && chars[i + 6].is_whitespace()
                {
                    tokens.push(parse_tuplet_token(&chars, &mut i)?);
                    continue;
                }
                let start = i;
                while i < chars.len()
                    && !chars[i].is_whitespace()
                    && chars[i] != '|'
                    && chars[i] != '~'
                    && chars[i] != '/'
                    && chars[i] != '{'
                    && chars[i] != '}'
                {
                    if chars[i] == '[' {
                        while i < chars.len() && chars[i] != ']' {
                            i += 1;
                        }
                        if i == chars.len() {
                            return Err("unterminated annotation block in sequence".to_string());
                        }
                    }
                    i += 1;
                }
                tokens.push(Token::Event(chars[start..i].iter().collect()));
            }
        }
    }

    Ok(tokens)
}

fn parse_tremolo_token(chars: &[char], i: &mut usize) -> Result<Token, String> {
    *i += 7; // "tremolo"
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    let subdivision_start = *i;
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        *i += 1;
    }
    if subdivision_start == *i {
        return Err("tremolo must specify subdivision 8, 16, 32, or 64".to_string());
    }
    let subdivision: String = chars[subdivision_start..*i].iter().collect();
    let subdivision = subdivision
        .parse::<u32>()
        .map_err(|_| "invalid tremolo subdivision".to_string())?;
    if !matches!(subdivision, 8 | 16 | 32 | 64) {
        return Err("tremolo subdivision must be 8, 16, 32, or 64".to_string());
    }
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    if *i == chars.len() || chars[*i] != '{' {
        return Err("tremolo subdivision must be followed by '{ ... }'".to_string());
    }
    *i += 1;
    let contents_start = *i;
    let mut depth = 1_usize;
    while *i < chars.len() && depth > 0 {
        match chars[*i] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *i += 1;
    }
    if depth != 0 {
        return Err("unterminated alternating tremolo".to_string());
    }
    let contents: String = chars[contents_start..*i - 1].iter().collect();
    Ok(Token::Tremolo {
        subdivision,
        contents: tokenize_sequence(&contents)?,
    })
}

fn parse_grace_token(
    chars: &[char],
    i: &mut usize,
    keyword: &str,
    style: &str,
) -> Result<Token, String> {
    *i += keyword.chars().count();
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    if *i == chars.len() || chars[*i] != '{' {
        return Err(format!("{keyword} must be followed by '{{ ... }}'"));
    }
    *i += 1;
    let contents_start = *i;
    let mut depth = 1_usize;
    while *i < chars.len() && depth > 0 {
        match chars[*i] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *i += 1;
    }
    if depth != 0 {
        return Err(format!("unterminated {keyword} group"));
    }
    let contents: String = chars[contents_start..*i - 1].iter().collect();
    Ok(Token::Grace {
        style: style.to_string(),
        contents: tokenize_sequence(&contents)?,
    })
}

fn parse_tuplet_token(chars: &[char], i: &mut usize) -> Result<Token, String> {
    *i += 6; // "tuplet"
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    let numerator_start = *i;
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        *i += 1;
    }
    if numerator_start == *i || *i == chars.len() || chars[*i] != ':' {
        return Err("tuplet ratio must look like 3:2".to_string());
    }
    let numerator: String = chars[numerator_start..*i].iter().collect();
    *i += 1;
    let denominator_start = *i;
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        *i += 1;
    }
    if denominator_start == *i {
        return Err("tuplet ratio must look like 3:2".to_string());
    }
    let denominator: String = chars[denominator_start..*i].iter().collect();
    let numerator = numerator
        .parse::<u32>()
        .map_err(|_| "invalid tuplet numerator".to_string())?;
    let denominator = denominator
        .parse::<u32>()
        .map_err(|_| "invalid tuplet denominator".to_string())?;
    if numerator == 0 || denominator == 0 {
        return Err("tuplet ratio values must be positive".to_string());
    }

    let mut bracket = "auto".to_string();
    let mut side = "auto".to_string();
    if *i < chars.len() && chars[*i] == '[' {
        *i += 1;
        let options_start = *i;
        while *i < chars.len() && chars[*i] != ']' {
            *i += 1;
        }
        if *i == chars.len() {
            return Err("unterminated tuplet options".to_string());
        }
        let options: String = chars[options_start..*i].iter().collect();
        *i += 1;
        for option in options.replace(',', " ").split_whitespace() {
            if let Some(value) = option.strip_prefix("bracket=") {
                if !matches!(value, "auto" | "always" | "never") {
                    return Err("tuplet bracket must be auto, always, or never".to_string());
                }
                bracket = value.to_string();
            } else if let Some(value) = option.strip_prefix("side=") {
                if !matches!(value, "auto" | "above" | "below") {
                    return Err("tuplet side must be auto, above, or below".to_string());
                }
                side = value.to_string();
            } else {
                return Err(format!("unknown tuplet option {option:?}"));
            }
        }
    }
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    if *i == chars.len() || chars[*i] != '{' {
        return Err("tuplet ratio must be followed by '{ ... }'".to_string());
    }
    *i += 1;
    let contents_start = *i;
    let mut depth = 1_usize;
    while *i < chars.len() && depth > 0 {
        match chars[*i] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *i += 1;
    }
    if depth != 0 {
        return Err("unterminated tuplet".to_string());
    }
    let contents: String = chars[contents_start..*i - 1].iter().collect();
    Ok(Token::Tuplet {
        numerator,
        denominator,
        bracket,
        side,
        contents: tokenize_sequence(&contents)?,
    })
}

pub fn layout_sequence_with_time_native(
    input: &str,
    clef: Clef,
    time: &str,
) -> Result<Vec<NoteLayout>, String> {
    Ok(layout_sequence_with_time_relative_native(input, clef, time, None)?.layouts)
}

#[cfg(test)]
fn parse_sequence_with_auto_rests(
    input: &str,
    time: &str,
) -> Result<Vec<(ParsedEvent, BeamDirective)>, String> {
    Ok(parse_sequence_with_auto_rests_relative(input, time, Clef::Treble, None, None)?
        .events
        .into_iter()
        .map(|item| (item.event, item.beam_directive))
        .collect())
}

fn parse_sequence_with_auto_rests_relative(
    input: &str,
    time: &str,
    clef: Clef,
    initial_pitch_anchor: Option<Pitch>,
    initial_duration_anchor: Option<Duration>,
) -> Result<ParsedSequenceState, String> {
    let tokens = tokenize_sequence(input)?;
    let expected = parse_time_signature(time)?.value();
    let mut slots: Vec<Option<SequencedEvent>> = Vec::new();
    let mut tuplets = Vec::new();
    let mut tremolos = Vec::new();
    let mut known_total = Rational::new(0, 1);
    let mut auto_rest_count = 0_usize;
    let mut pending_beam = BeamDirective::Auto;
    let mut pitch_anchor = initial_pitch_anchor;
    let mut duration_anchor = initial_duration_anchor;

    for token in tokens {
        match token {
            Token::Event(raw) => {
                let event =
                    parse_event_relative(&raw, &mut pitch_anchor, &mut duration_anchor, clef)?;
                known_total = known_total.add(duration_to_rational(event.duration()));
                slots.push(Some(SequencedEvent {
                    event,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(1, 1),
                    grace: None,
                }));
                pending_beam = BeamDirective::Auto;
            }
            Token::Grace { style, contents } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before a grace group".to_string());
                }
                let group = slots
                    .iter()
                    .flatten()
                    .filter_map(|item| item.grace.as_ref().map(|grace| grace.group))
                    .max()
                    .map_or(0, |value| value + 1);
                for item in parse_grace_tokens(
                    style,
                    &contents,
                    group,
                    clef,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                )? {
                    slots.push(Some(item));
                }
            }
            Token::Tremolo { subdivision, contents } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before an alternating tremolo".to_string());
                }
                let start = slots.iter().filter(|slot| slot.is_some()).count();
                let mut group_events = Vec::new();
                let mut group_tremolos = Vec::new();
                parse_tremolo_tokens(
                    subdivision,
                    &contents,
                    clef,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                    &mut group_events,
                    &mut group_tremolos,
                )?;
                for item in group_events {
                    known_total = known_total.add(duration_to_rational(item.event.duration()));
                    slots.push(Some(item));
                }
                for mut tremolo in group_tremolos {
                    tremolo.start += start;
                    tremolo.end += start;
                    tremolos.push(tremolo);
                }
            }
            Token::Tuplet {
                numerator,
                denominator,
                bracket,
                side,
                contents,
            } => {
                if pending_beam != BeamDirective::Auto {
                    return Err("beam marker '/' or '-' cannot appear before a tuplet".to_string());
                }
                let start = slots.iter().filter(|slot| slot.is_some()).count();
                let tuplet_start = tuplets.len();
                let mut group_events = Vec::new();
                parse_tuplet_tokens(
                    numerator,
                    denominator,
                    bracket,
                    side,
                    &contents,
                    clef,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                    Rational::new(1, 1),
                    &mut group_events,
                    &mut tuplets,
                )?;
                for item in group_events {
                    known_total = known_total.add(
                        duration_to_rational(item.event.duration()).mul(item.duration_scale),
                    );
                    slots.push(Some(item));
                }
                for tuplet in tuplets.iter_mut().skip(tuplet_start) {
                    tuplet.start += start;
                    tuplet.end += start;
                }
            }
            Token::AutoRest => {
                if pending_beam != BeamDirective::Auto {
                    return Err(
                        "beam marker '/' or '-' cannot appear before an automatic rest".to_string(),
                    );
                }
                auto_rest_count += 1;
                slots.push(None);
            }
            Token::BeamBreak => {
                set_beam_directive(&mut pending_beam, BeamDirective::Break)?;
            }
            Token::BeamJoin => {
                set_beam_directive(&mut pending_beam, BeamDirective::Join)?;
            }
            Token::Tie => {
                let Some(last) = slots.last_mut() else {
                    return Err("tie marker '~' cannot appear before a note or chord".to_string());
                };
                let Some(previous) = last else {
                    return Err("tie marker '~' cannot follow an automatic rest".to_string());
                };
                previous.event.set_tie_to_next()?;
            }
        }
    }

    if pending_beam != BeamDirective::Auto {
        return Err("beam marker '/' or '-' cannot end a sequence".to_string());
    }
    if slots.iter().rev().flatten().next().is_some_and(|item| item.grace.is_some()) {
        return Err("grace group must be followed by a main note, chord, or rest".to_string());
    }

    let remaining = expected.sub(known_total).ok_or_else(|| {
        format!(
            "durations sum to {}, expected {} before auto rests",
            known_total, expected
        )
    })?;

    if auto_rest_count == 0 {
        if remaining.numerator != 0 {
            return Err(format!(
                "durations sum to {}, expected {}",
                known_total, expected
            ));
        }
        return Ok(ParsedSequenceState {
            events: slots.into_iter().flatten().collect(),
            tuplets,
            tremolos,
            pitch_anchor,
            duration_anchor,
        });
    }

    let rest_durations = distribute_auto_rests(remaining, auto_rest_count).ok_or_else(|| {
        format!(
            "cannot express remaining auto-rest duration {} across {} placeholders",
            remaining, auto_rest_count
        )
    })?;
    let mut rest_iter = rest_durations.into_iter();
    let mut out = Vec::new();
    for slot in slots {
        match slot {
            Some(event) => out.push(event),
            None => out.push(SequencedEvent {
                event: ParsedEvent::Rest(Rest {
                    duration: rest_iter
                        .next()
                        .expect("one generated rest duration per placeholder"),
                    annotations: Vec::new(),
                }),
                beam_directive: BeamDirective::Auto,
                duration_scale: Rational::new(1, 1),
                grace: None,
            }),
        }
    }
    Ok(ParsedSequenceState {
        events: out,
        tuplets,
        tremolos,
        pitch_anchor,
        duration_anchor,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature {
    pub numerator: u32,
    pub denominator: u32,
}

impl TimeSignature {
    fn value(self) -> Rational {
        Rational::new(self.numerator, self.denominator)
    }

    /// The beat used to group beams: a dotted unit in compound meters
    /// (6/8, 9/8, 12/8, ...), one denominator unit otherwise.
    fn beat_unit(self) -> Rational {
        if self.numerator > 3 && self.numerator % 3 == 0 && self.denominator >= 8 {
            Rational::new(3, self.denominator)
        } else {
            Rational::new(1, self.denominator)
        }
    }
}

fn parse_time_signature(input: &str) -> Result<TimeSignature, String> {
    let (numerator, denominator) = input
        .trim()
        .split_once('/')
        .ok_or_else(|| format!("time signature must look like 4/4 or 12/8, got {input:?}"))?;
    let numerator = numerator
        .parse::<u32>()
        .map_err(|_| format!("invalid time signature numerator {numerator:?}"))?;
    let denominator = denominator
        .parse::<u32>()
        .map_err(|_| format!("invalid time signature denominator {denominator:?}"))?;
    if numerator == 0 || denominator == 0 {
        return Err(format!(
            "time signature values must be positive, got {input:?}"
        ));
    }
    Ok(TimeSignature {
        numerator,
        denominator,
    })
}

fn consume_event_suffix(chars: &[char], mut i: usize) -> Result<usize, String> {
    while i < chars.len()
        && !chars[i].is_whitespace()
        && chars[i] != '|'
        && chars[i] != '~'
        && chars[i] != '/'
    {
        if chars[i] == '[' {
            while i < chars.len() && chars[i] != ']' {
                i += 1;
            }
            if i == chars.len() {
                return Err("unterminated annotation block in chord suffix".to_string());
            }
        }
        i += 1;
    }
    Ok(i)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionedPitch {
    pub pitch: Pitch,
    pub staff_position: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteLayout {
    pub kind: String,
    pub clef: Clef,
    pub duration: Duration,
    pub duration_value: Rational,
    /// Time elapsed before this event, as a fraction of a whole note.
    pub onset: Rational,
    pub pitches: Vec<PositionedPitch>,
    pub tie_to_next: bool,
    pub annotations: Vec<String>,
    pub notehead: String,
    pub stem: bool,
    pub flags: u8,
    pub rest: bool,
    pub beam_break_before: bool,
    pub beam_join_before: bool,
    /// Events sharing a group id are joined by a beam.
    pub beam_group: Option<usize>,
    /// Grace events occupy horizontal space before an onset but no bar time.
    pub grace: bool,
    pub grace_style: Option<String>,
    pub grace_group: Option<usize>,
    pub grace_index: usize,
    pub grace_count: usize,
    /// Number of grace events immediately preceding this main event.
    pub grace_before: usize,
    /// Tuplets that start at this event and end at the given event index.
    pub tuplet_starts: Vec<TupletLayout>,
    /// Alternating tremolos that start at this event.
    pub tremolo_starts: Vec<TremoloLayout>,
    pub alternating_tremolo: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupletLayout {
    pub numerator: u32,
    pub denominator: u32,
    pub bracket: String,
    pub side: String,
    pub end_index: usize,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TremoloLayout {
    pub subdivision: u32,
    pub end_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeLayoutResponse {
    pub layouts: Vec<NoteLayout>,
    pub anchor: Option<String>,
    pub duration_anchor: Option<String>,
}

fn layout_event(
    event: ParsedEvent,
    clef: Clef,
    beam_directive: BeamDirective,
    onset: Rational,
    duration_scale: Rational,
    grace: Option<GraceMeta>,
) -> Result<NoteLayout, String> {
    let duration = event.duration();
    let duration_value = duration_to_rational(duration).mul(duration_scale);
    let notehead = match duration.base {
        DurationBase::Whole => "whole",
        DurationBase::Half => "half",
        DurationBase::Quarter
        | DurationBase::Eighth
        | DurationBase::Sixteenth
        | DurationBase::ThirtySecond => "black",
    }
    .to_string();
    let stem = !matches!(duration.base, DurationBase::Whole);
    let flags = duration.base.flag_count();

    let pitches = match &event {
        ParsedEvent::Note(note) => vec![PositionedPitch {
            pitch: note.pitch.clone(),
            staff_position: pitch_to_staff_position(&note.pitch, clef),
        }],
        ParsedEvent::Chord { notes, .. } => notes
            .iter()
            .map(|note| PositionedPitch {
                pitch: note.pitch.clone(),
                staff_position: pitch_to_staff_position(&note.pitch, clef),
            })
            .collect(),
        ParsedEvent::Rest(_) => Vec::new(),
    };

    let tie_to_next = event.tie_to_next();
    let annotations = event.annotations();
    let kind = match &event {
        ParsedEvent::Note(_) => "note",
        ParsedEvent::Rest(_) => "rest",
        ParsedEvent::Chord { .. } => "chord",
    }
    .to_string();
    if kind != "chord"
        && annotations
            .iter()
            .any(|mark| mark == "arpeggio" || mark.starts_with("arpeggio="))
    {
        return Err("arpeggio annotation requires a chord".to_string());
    }
    for mark in &annotations {
        if let Some(value) = mark.strip_prefix("tremolo=") {
            let subdivision = value
                .parse::<u32>()
                .map_err(|_| "invalid tremolo subdivision".to_string())?;
            if subdivision <= duration.base.denominator() {
                return Err(format!(
                    "tremolo subdivision {subdivision} must be shorter than the written note"
                ));
            }
        }
    }

    let (is_grace, grace_style, grace_group, grace_index, grace_count) = match grace {
        Some(meta) => (true, Some(meta.style), Some(meta.group), meta.index, meta.count),
        None => (false, None, None, 0, 0),
    };

    Ok(NoteLayout {
        rest: kind == "rest",
        kind,
        clef,
        duration,
        duration_value,
        onset,
        pitches,
        tie_to_next,
        annotations,
        notehead,
        stem,
        flags,
        beam_break_before: beam_directive == BeamDirective::Break,
        beam_join_before: beam_directive == BeamDirective::Join,
        beam_group: None,
        grace: is_grace,
        grace_style,
        grace_group,
        grace_index,
        grace_count,
        grace_before: 0,
        tuplet_starts: Vec::new(),
        tremolo_starts: Vec::new(),
        alternating_tremolo: false,
    })
}

fn layout_events(
    events: Vec<SequencedEvent>,
    clef: Clef,
) -> Result<Vec<NoteLayout>, String> {
    let mut onset = Rational::new(0, 1);
    let mut out = Vec::with_capacity(events.len());
    for item in events {
        let duration_value = duration_to_rational(item.event.duration()).mul(item.duration_scale);
        out.push(layout_event(
            item.event,
            clef,
            item.beam_directive,
            onset,
            item.duration_scale,
            item.grace,
        )?);
        onset = onset.add(duration_value);
    }
    let mut pending_graces = 0_usize;
    for layout in &mut out {
        if layout.grace {
            pending_graces += 1;
        } else {
            layout.grace_before = pending_graces;
            pending_graces = 0;
        }
    }
    Ok(out)
}

fn attach_tuplets(layouts: &mut [NoteLayout], tuplets: Vec<ParsedTuplet>) {
    for tuplet in &tuplets {
        if tuplet.start >= layouts.len() || tuplet.end == 0 || tuplet.end > layouts.len() {
            continue;
        }
        let depth = tuplets
            .iter()
            .filter(|other| {
                other.start >= tuplet.start
                    && other.end <= tuplet.end
                    && (other.start != tuplet.start || other.end != tuplet.end)
            })
            .count();
        layouts[tuplet.start].tuplet_starts.push(TupletLayout {
            numerator: tuplet.numerator,
            denominator: tuplet.denominator,
            bracket: tuplet.bracket.clone(),
            side: tuplet.side.clone(),
            end_index: tuplet.end - 1,
            depth,
        });
    }
}

fn attach_tremolos(layouts: &mut [NoteLayout], tremolos: Vec<ParsedTremolo>) {
    for tremolo in tremolos {
        if tremolo.start < layouts.len() && tremolo.end < layouts.len() {
            layouts[tremolo.start].tremolo_starts.push(TremoloLayout {
                subdivision: tremolo.subdivision,
                end_index: tremolo.end,
            });
            layouts[tremolo.start].alternating_tremolo = true;
            layouts[tremolo.end].alternating_tremolo = true;
            layouts[tremolo.start].beam_group = None;
            layouts[tremolo.end].beam_group = None;
        }
    }
}

/// Group consecutive flagged notes into beams. With a beat unit, groups also
/// break at beat boundaries; rests and explicit '/' breaks always split.
fn assign_beam_groups(layouts: &mut [NoteLayout], beat: Option<Rational>) -> Result<(), String> {
    let mut next_group = 0_usize;
    let mut current: Option<(usize, u64)> = None;
    let mut grace_group: Option<(usize, usize)> = None;
    let mut previous_was_grace = false;
    for (index, layout) in layouts.iter_mut().enumerate() {
        if layout.beam_join_before && (layout.rest || layout.flags == 0) {
            return Err(format!(
                "beam join marker '-' before event {} requires a flagged note or chord",
                index + 1
            ));
        }
        if layout.rest || layout.flags == 0 {
            current = None;
            grace_group = None;
            previous_was_grace = layout.grace;
            continue;
        }
        if layout.grace {
            current = None;
            let source_group = layout.grace_group.expect("grace event has a group id");
            if !matches!(grace_group, Some((group, _)) if group == source_group) {
                grace_group = Some((source_group, next_group));
                next_group += 1;
            }
            layout.beam_group = grace_group.map(|(_, beam_group)| beam_group);
            previous_was_grace = true;
            continue;
        }
        grace_group = None;
        if previous_was_grace {
            current = None;
        }
        previous_was_grace = false;
        let beat_index = beat
            .map(|beat| {
                u64::from(layout.onset.numerator) * u64::from(beat.denominator)
                    / (u64::from(layout.onset.denominator) * u64::from(beat.numerator))
            })
            .unwrap_or(0);
        if layout.beam_join_before && current.is_none() {
            return Err(format!(
                "beam join marker '-' before event {} requires an adjacent flagged note or chord before it",
                index + 1
            ));
        }
        let continues = layout.beam_join_before
            || (matches!(current, Some((_, index)) if index == beat_index)
                && !layout.beam_break_before);
        if !continues {
            current = Some((next_group, beat_index));
            next_group += 1;
        } else if layout.beam_join_before {
            current = current.map(|(group, _)| (group, beat_index));
        }
        layout.beam_group = Some(current.expect("group was just ensured").0);
    }
    Ok(())
}

pub fn layout_note_native(input: &str, clef: Clef) -> Result<NoteLayout, String> {
    layout_event(
        parse_event(input)?,
        clef,
        BeamDirective::Auto,
        Rational::new(0, 1),
        Rational::new(1, 1),
        None,
    )
}

pub fn layout_sequence_native(input: &str, clef: Clef) -> Result<Vec<NoteLayout>, String> {
    Ok(layout_sequence_relative_native(input, clef, None)?.layouts)
}

fn parse_anchor(anchor: Option<&str>) -> Result<Option<Pitch>, String> {
    anchor
        .filter(|value| !value.is_empty())
        .map(parse_pitch)
        .transpose()
}

fn parse_duration_anchor(anchor: Option<&str>) -> Result<Option<Duration>, String> {
    anchor
        .filter(|value| !value.is_empty())
        .map(parse_duration)
        .transpose()
}

fn duration_anchor_string(duration: Duration) -> String {
    let base = match duration.base {
        DurationBase::Whole => 'w',
        DurationBase::Half => 'h',
        DurationBase::Quarter => 'q',
        DurationBase::Eighth => 'e',
        DurationBase::Sixteenth => 's',
        DurationBase::ThirtySecond => 't',
    };
    format!("{base}{}", ".".repeat(duration.dots.into()))
}

pub fn layout_sequence_relative_native(
    input: &str,
    clef: Clef,
    anchor: Option<&str>,
) -> Result<RelativeLayoutResponse, String> {
    layout_sequence_relative_with_state_native(input, clef, anchor, None)
}

pub fn layout_sequence_relative_with_state_native(
    input: &str,
    clef: Clef,
    pitch_anchor: Option<&str>,
    duration_anchor: Option<&str>,
) -> Result<RelativeLayoutResponse, String> {
    let parsed = parse_sequence_marked(
        input,
        clef,
        parse_anchor(pitch_anchor)?,
        parse_duration_anchor(duration_anchor)?,
    )?;
    let mut layouts = layout_events(parsed.events, clef)?;
    assign_beam_groups(&mut layouts, None)?;
    attach_tuplets(&mut layouts, parsed.tuplets);
    attach_tremolos(&mut layouts, parsed.tremolos);
    Ok(RelativeLayoutResponse {
        layouts,
        anchor: parsed.pitch_anchor.as_ref().map(pitch_anchor_string),
        duration_anchor: parsed.duration_anchor.map(duration_anchor_string),
    })
}

pub fn layout_sequence_with_time_relative_native(
    input: &str,
    clef: Clef,
    time: &str,
    anchor: Option<&str>,
) -> Result<RelativeLayoutResponse, String> {
    layout_sequence_with_time_relative_state_native(input, clef, time, anchor, None)
}

pub fn layout_sequence_with_time_relative_state_native(
    input: &str,
    clef: Clef,
    time: &str,
    pitch_anchor: Option<&str>,
    duration_anchor: Option<&str>,
) -> Result<RelativeLayoutResponse, String> {
    let signature = parse_time_signature(time)?;
    let parsed = parse_sequence_with_auto_rests_relative(
        input,
        time,
        clef,
        parse_anchor(pitch_anchor)?,
        parse_duration_anchor(duration_anchor)?,
    )?;
    let mut layouts = layout_events(parsed.events, clef)?;
    assign_beam_groups(&mut layouts, Some(signature.beat_unit()))?;
    attach_tuplets(&mut layouts, parsed.tuplets);
    attach_tremolos(&mut layouts, parsed.tremolos);
    Ok(RelativeLayoutResponse {
        layouts,
        anchor: parsed.pitch_anchor.as_ref().map(pitch_anchor_string),
        duration_anchor: parsed.duration_anchor.map(duration_anchor_string),
    })
}

#[cfg(target_arch = "wasm32")]
mod wasm_entrypoint {
    use super::*;
    use wasm_minimal_protocol::*;

    initiate_protocol!();

    #[wasm_func]
    pub fn layout_note(input: &[u8]) -> Result<Vec<u8>, String> {
        let s = core::str::from_utf8(input).map_err(|e| format!("UTF-8 error: {e}"))?;
        let (clef_str, note_str) = s
            .split_once('\n')
            .map(|(clef, note)| (clef, note))
            .unwrap_or(("treble", s));
        let clef = parse_clef(clef_str)?;
        let out = layout_note_native(note_str, clef)?;
        serde_json::to_vec(&out).map_err(|e| format!("JSON error: {e}"))
    }

    #[wasm_func]
    pub fn parse_events(input: &[u8]) -> Result<Vec<u8>, String> {
        let s = core::str::from_utf8(input).map_err(|e| format!("UTF-8 error: {e}"))?;
        let out = parse_sequence(s)?;
        serde_json::to_vec(&out).map_err(|e| format!("JSON error: {e}"))
    }

    #[wasm_func]
    pub fn layout_sequence(input: &[u8]) -> Result<Vec<u8>, String> {
        let s = core::str::from_utf8(input).map_err(|e| format!("UTF-8 error: {e}"))?;
        let (clef_str, sequence_str) = s
            .split_once('\n')
            .map(|(clef, sequence)| (clef, sequence))
            .unwrap_or(("treble", s));
        let clef = parse_clef(clef_str)?;
        let out = layout_sequence_native(sequence_str, clef)?;
        serde_json::to_vec(&out).map_err(|e| format!("JSON error: {e}"))
    }

    #[wasm_func]
    pub fn layout_sequence_timed(input: &[u8]) -> Result<Vec<u8>, String> {
        let s = core::str::from_utf8(input).map_err(|e| format!("UTF-8 error: {e}"))?;
        let mut parts = s.splitn(3, '\n');
        let clef_str = parts.next().unwrap_or("treble");
        let time = parts.next().unwrap_or("4/4");
        let sequence_str = parts.next().unwrap_or("");
        let clef = parse_clef(clef_str)?;
        let out = layout_sequence_with_time_native(sequence_str, clef, time)?;
        serde_json::to_vec(&out).map_err(|e| format!("JSON error: {e}"))
    }

    #[wasm_func]
    pub fn layout_sequence_relative(input: &[u8]) -> Result<Vec<u8>, String> {
        let s = core::str::from_utf8(input).map_err(|e| format!("UTF-8 error: {e}"))?;
        let mut parts = s.splitn(4, '\n');
        let clef_str = parts.next().unwrap_or("treble");
        let pitch_anchor = parts.next().filter(|value| !value.is_empty());
        let duration_anchor = parts.next().filter(|value| !value.is_empty());
        let sequence_str = parts.next().unwrap_or("");
        let clef = parse_clef(clef_str)?;
        let out = layout_sequence_relative_with_state_native(
            sequence_str,
            clef,
            pitch_anchor,
            duration_anchor,
        )?;
        serde_json::to_vec(&out).map_err(|e| format!("JSON error: {e}"))
    }

    #[wasm_func]
    pub fn layout_sequence_timed_relative(input: &[u8]) -> Result<Vec<u8>, String> {
        let s = core::str::from_utf8(input).map_err(|e| format!("UTF-8 error: {e}"))?;
        let mut parts = s.splitn(5, '\n');
        let clef_str = parts.next().unwrap_or("treble");
        let time = parts.next().unwrap_or("4/4");
        let pitch_anchor = parts.next().filter(|value| !value.is_empty());
        let duration_anchor = parts.next().filter(|value| !value.is_empty());
        let sequence_str = parts.next().unwrap_or("");
        let clef = parse_clef(clef_str)?;
        let out = layout_sequence_with_time_relative_state_native(
            sequence_str,
            clef,
            time,
            pitch_anchor,
            duration_anchor,
        )?;
        serde_json::to_vec(&out).map_err(|e| format!("JSON error: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(pitch: &str, clef: Clef) -> i32 {
        pitch_to_staff_position(&parse_pitch(pitch).unwrap(), clef)
    }

    #[test]
    fn parses_plain_and_accidental_pitches() {
        assert_eq!(
            parse_pitch("C4").unwrap(),
            Pitch {
                letter: Letter::C,
                accidental: Accidental::Natural,
                octave: 4
            }
        );
        assert_eq!(parse_pitch("c4").unwrap(), parse_pitch("C4").unwrap());
        assert_eq!(parse_pitch("f#5").unwrap(), parse_pitch("F#5").unwrap());
        assert_eq!(parse_pitch("D#4").unwrap().accidental, Accidental::Sharp);
        assert_eq!(parse_pitch("C-1").unwrap().octave, -1);
        assert_eq!(parse_pitch("Eb4").unwrap().accidental, Accidental::Flat);
        assert_eq!(
            parse_pitch("F##5").unwrap().accidental,
            Accidental::DoubleSharp
        );
        assert_eq!(
            parse_pitch("Bbb3").unwrap().accidental,
            Accidental::DoubleFlat
        );
        assert_eq!(parse_pitch("Bb3").unwrap().letter, Letter::B);
    }

    #[test]
    fn rejects_invalid_pitches() {
        assert!(parse_pitch("").is_err());
        assert!(parse_pitch("H4").is_err());
        assert!(parse_pitch("C").is_err());
        assert!(parse_pitch("Cq").is_err());
        assert!(parse_pitch("C###4").is_err());
        assert!(parse_pitch("C#b4").is_err());
    }

    #[test]
    fn treble_staff_positions_are_diatonic() {
        let cases = [
            ("C4", 0),
            ("D4", 1),
            ("E4", 2),
            ("F4", 3),
            ("G4", 4),
            ("A4", 5),
            ("B4", 6),
            ("C5", 7),
            ("D5", 8),
            ("E5", 9),
            ("F5", 10),
            ("G5", 11),
            ("A5", 12),
        ];
        for (pitch, expected) in cases {
            assert_eq!(pos(pitch, Clef::Treble), expected, "{pitch}");
        }
    }

    #[test]
    fn clef_anchor_positions_are_correct() {
        assert_eq!(pos("C4", Clef::Treble), 0);
        assert_eq!(pos("E4", Clef::Treble), 2);
        assert_eq!(pos("F5", Clef::Treble), 10);
        assert_eq!(pos("A5", Clef::Treble), 12);

        assert_eq!(pos("E2", Clef::Bass), 0);
        assert_eq!(pos("G2", Clef::Bass), 2);
        assert_eq!(pos("A3", Clef::Bass), 10);
        assert_eq!(pos("C4", Clef::Bass), 12);

        assert_eq!(pos("D3", Clef::Alto), 0);
        assert_eq!(pos("F3", Clef::Alto), 2);
        assert_eq!(pos("C4", Clef::Alto), 6);
        assert_eq!(pos("G4", Clef::Alto), 10);
        assert_eq!(pos("B4", Clef::Alto), 12);

        assert_eq!(pos("B2", Clef::Tenor), 0);
        assert_eq!(pos("D3", Clef::Tenor), 2);
        assert_eq!(pos("C4", Clef::Tenor), 8);
        assert_eq!(pos("E4", Clef::Tenor), 10);
        assert_eq!(pos("G4", Clef::Tenor), 12);
    }

    #[test]
    fn accidentals_do_not_change_staff_position() {
        let chromatic_spellings = [
            ("C4", 0),
            ("C#4", 0),
            ("Db4", 1),
            ("D4", 1),
            ("D#4", 1),
            ("Eb4", 2),
            ("E4", 2),
            ("F4", 3),
            ("F#4", 3),
            ("Gb4", 4),
            ("G4", 4),
            ("G#4", 4),
            ("Ab4", 5),
            ("A4", 5),
            ("A#4", 5),
            ("Bb4", 6),
            ("B4", 6),
        ];
        for (pitch, expected) in chromatic_spellings {
            assert_eq!(pos(pitch, Clef::Treble), expected, "{pitch}");
        }
    }

    #[test]
    fn parses_duration_bases_and_dots() {
        assert_eq!(
            parse_duration("w").unwrap(),
            Duration {
                base: DurationBase::Whole,
                dots: 0
            }
        );
        assert_eq!(parse_duration("h").unwrap().base, DurationBase::Half);
        assert_eq!(parse_duration("q").unwrap().base, DurationBase::Quarter);
        assert_eq!(parse_duration("e.").unwrap().dots, 1);
        assert_eq!(parse_duration("s..").unwrap().dots, 2);
        assert_eq!(
            parse_duration("t").unwrap().base,
            DurationBase::ThirtySecond
        );
    }

    #[test]
    fn rejects_invalid_durations() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("x").is_err());
        assert!(parse_duration("q...").is_err());
        assert!(parse_duration("q-").is_err());
    }

    #[test]
    fn duration_rationals_are_exact() {
        let cases = [
            ("w", Rational::new(1, 1)),
            ("h", Rational::new(1, 2)),
            ("q", Rational::new(1, 4)),
            ("e", Rational::new(1, 8)),
            ("s", Rational::new(1, 16)),
            ("t", Rational::new(1, 32)),
            ("q.", Rational::new(3, 8)),
            ("e.", Rational::new(3, 16)),
            ("h..", Rational::new(7, 8)),
            ("q..", Rational::new(7, 16)),
        ];
        for (raw, expected) in cases {
            assert_eq!(duration_to_rational(parse_duration(raw).unwrap()), expected);
        }
    }

    #[test]
    fn parses_single_notes_and_annotations() {
        let note = parse_note("Bb4:e.[s1( s2)]").unwrap();
        assert_eq!(note.pitch.letter, Letter::B);
        assert_eq!(note.pitch.accidental, Accidental::Flat);
        assert_eq!(note.duration.base, DurationBase::Eighth);
        assert_eq!(note.duration.dots, 1);
        assert_eq!(note.annotations, vec!["s1(", "s2)"]);
    }

    #[test]
    fn preserves_combined_accent_and_articulation_annotations() {
        let note = parse_note("C6:q[marcato strong accent tenuto legato stacc staccatissimo f=4]")
            .unwrap();
        assert_eq!(
            note.annotations,
            vec![
                "marcato",
                "strong",
                "accent",
                "tenuto",
                "legato",
                "stacc",
                "staccatissimo",
                "f=4",
            ]
        );
    }

    #[test]
    fn parses_compact_note_duration_syntax() {
        let note = parse_note("Eb4q").unwrap();
        assert_eq!(note.pitch.letter, Letter::E);
        assert_eq!(note.pitch.accidental, Accidental::Flat);
        assert_eq!(note.duration.base, DurationBase::Quarter);
        let note = parse_note("C5e.[s1(]").unwrap();
        assert_eq!(note.duration.base, DurationBase::Eighth);
        assert_eq!(note.duration.dots, 1);
        assert_eq!(note.annotations, vec!["s1("]);
    }

    #[test]
    fn lowercase_and_mixed_case_notes_have_identical_pitch_semantics() {
        let lowercase = parse_sequence("c4:e d E f").unwrap();
        let uppercase = parse_sequence("C4:e D E F").unwrap();
        assert_eq!(lowercase, uppercase);
    }

    #[test]
    fn compact_lowercase_durations_are_distinct_from_whitespace_separated_notes() {
        let compact = parse_sequence("ce").unwrap();
        let colon = parse_sequence("c:e").unwrap();
        assert_eq!(compact, colon);
        assert_eq!(compact.len(), 1);
        assert_eq!(compact[0].duration().base, DurationBase::Eighth);

        let two_notes = parse_sequence("c e").unwrap();
        assert_eq!(two_notes.len(), 2);
        assert!(two_notes
            .iter()
            .all(|event| event.duration().base == DurationBase::Quarter));

        let e_eighth = parse_sequence("ee").unwrap();
        let ParsedEvent::Note(note) = &e_eighth[0] else {
            panic!("expected note");
        };
        assert_eq!(note.pitch.letter, Letter::E);
        assert_eq!(note.duration.base, DurationBase::Eighth);
    }

    #[test]
    fn compact_lowercase_pitches_support_flat_and_double_accidentals() {
        let events = parse_sequence("bb4e bbb4e f##5q").unwrap();
        let ParsedEvent::Note(b_flat) = &events[0] else {
            panic!("expected note");
        };
        let ParsedEvent::Note(b_double_flat) = &events[1] else {
            panic!("expected note");
        };
        let ParsedEvent::Note(f_double_sharp) = &events[2] else {
            panic!("expected note");
        };
        assert_eq!(b_flat.pitch.accidental, Accidental::Flat);
        assert_eq!(b_double_flat.pitch.accidental, Accidental::DoubleFlat);
        assert_eq!(f_double_sharp.pitch.accidental, Accidental::DoubleSharp);
        assert_eq!(b_flat.duration.base, DurationBase::Eighth);
        assert_eq!(b_double_flat.duration.base, DurationBase::Eighth);
        assert_eq!(f_double_sharp.duration.base, DurationBase::Quarter);
    }

    #[test]
    fn parses_rests() {
        let ParsedEvent::Rest(rest) = parse_event("r:q").unwrap() else {
            panic!("expected rest");
        };
        assert_eq!(rest.duration.base, DurationBase::Quarter);
    }

    #[test]
    fn parses_chords_with_shared_duration() {
        let notes = parse_chord("(Bb3 Eb4 G4):e.").unwrap();
        assert_eq!(notes.len(), 3);
        assert!(notes
            .iter()
            .all(|note| note.duration.base == DurationBase::Eighth && note.duration.dots == 1));
        assert_eq!(notes[0].pitch.accidental, Accidental::Flat);
    }

    #[test]
    fn tokenizes_bars_and_sets_tie_flags() {
        let events = parse_sequence("G4:e ~ G4:q. r:e (C4 E4 G4):q").unwrap();
        assert_eq!(events.len(), 4);
        assert!(events[0].tie_to_next());
        assert!(!events[1].tie_to_next());
        assert!(matches!(events[2], ParsedEvent::Rest(_)));
        assert!(matches!(events[3], ParsedEvent::Chord { .. }));
    }

    #[test]
    fn omitted_octaves_follow_the_nearest_diatonic_pitch() {
        let events = parse_sequence("G4:q A:q B:q C:q Dq E:q F#:q G:q").unwrap();
        let pitches: Vec<_> = events
            .iter()
            .map(|event| match event {
                ParsedEvent::Note(note) => pitch_anchor_string(&note.pitch),
                _ => panic!("expected notes"),
            })
            .collect();
        assert_eq!(
            pitches,
            vec!["G4", "A4", "B4", "C5", "D5", "E5", "F5", "G5"]
        );
    }

    #[test]
    fn omitted_durations_inherit_and_default_to_quarters() {
        let events = parse_sequence("C4 D E:e F G:q. A[stacc]").unwrap();
        let durations: Vec<_> = events.iter().map(ParsedEvent::duration).collect();
        assert_eq!(
            durations,
            vec![
                parse_duration("q").unwrap(),
                parse_duration("q").unwrap(),
                parse_duration("e").unwrap(),
                parse_duration("e").unwrap(),
                parse_duration("q.").unwrap(),
                parse_duration("q.").unwrap(),
            ]
        );
        assert_eq!(events[5].annotations(), vec!["stacc"]);
    }

    #[test]
    fn explicit_rests_and_chords_update_the_inherited_duration() {
        let events =
            parse_sequence("C4:e r (E4 G4) A4:q r:s B4 (C5 E5 G5):h (D5 F5 A5) B5").unwrap();
        let codes: Vec<_> = events
            .iter()
            .map(|event| duration_anchor_string(event.duration()))
            .collect();
        assert_eq!(codes, vec!["e", "e", "e", "q", "s", "s", "h", "h", "h"]);
    }

    #[test]
    fn duration_anchor_continues_across_measures() {
        let first = layout_sequence_with_time_relative_state_native(
            "B4:e C D",
            Clef::Treble,
            "3/8",
            None,
            None,
        )
        .unwrap();
        assert_eq!(first.anchor.as_deref(), Some("D5"));
        assert_eq!(first.duration_anchor.as_deref(), Some("e"));

        let second = layout_sequence_with_time_relative_state_native(
            "E F G",
            Clef::Treble,
            "3/8",
            first.anchor.as_deref(),
            first.duration_anchor.as_deref(),
        )
        .unwrap();
        assert_eq!(second.anchor.as_deref(), Some("G5"));
        assert_eq!(second.duration_anchor.as_deref(), Some("e"));
        assert!(second
            .layouts
            .iter()
            .all(|layout| layout.duration == parse_duration("e").unwrap()));
    }

    #[test]
    fn automatic_rests_do_not_change_the_duration_anchor() {
        let out = layout_sequence_with_time_relative_state_native(
            "C4:e _",
            Clef::Treble,
            "4/4",
            None,
            None,
        )
        .unwrap();
        assert_eq!(out.duration_anchor.as_deref(), Some("e"));
        assert_eq!(out.layouts[1].duration, parse_duration("h..").unwrap());
    }

    #[test]
    fn explicit_octaves_reset_relative_pitch_and_clefs_have_defaults() {
        let treble =
            layout_sequence_relative_native("B:q C:q C3:q D:q", Clef::Treble, None).unwrap();
        let treble_pitches: Vec<_> = treble
            .layouts
            .iter()
            .map(|layout| pitch_anchor_string(&layout.pitches[0].pitch))
            .collect();
        assert_eq!(treble_pitches, vec!["B4", "C5", "C3", "D3"]);

        let bass = layout_sequence_relative_native("G:q A:q", Clef::Bass, None).unwrap();
        assert_eq!(pitch_anchor_string(&bass.layouts[0].pitches[0].pitch), "G3");
        assert_eq!(bass.anchor.as_deref(), Some("A3"));
    }

    #[test]
    fn relative_anchor_can_continue_across_measures() {
        let first =
            layout_sequence_with_time_relative_native("B4:e C:e D:e", Clef::Treble, "3/8", None)
                .unwrap();
        assert_eq!(first.anchor.as_deref(), Some("D5"));
        let second = layout_sequence_with_time_relative_native(
            "E:e F:e G:e",
            Clef::Treble,
            "3/8",
            first.anchor.as_deref(),
        )
        .unwrap();
        assert_eq!(second.anchor.as_deref(), Some("G5"));
    }

    #[test]
    fn relative_octaves_resolve_inside_chords_like_lilypond() {
        let out =
            layout_sequence_relative_native("(a4 c E):q (f A c) d (C4 g4 c5)", Clef::Treble, None)
                .unwrap();
        let chord_pitches = |index: usize| {
            out.layouts[index]
                .pitches
                .iter()
                .map(|pitch| pitch_anchor_string(&pitch.pitch))
                .collect::<Vec<_>>()
        };
        assert_eq!(chord_pitches(0), vec!["A4", "C5", "E5"]);
        assert_eq!(chord_pitches(1), vec!["F4", "A4", "C5"]);
        assert_eq!(pitch_anchor_string(&out.layouts[2].pitches[0].pitch), "D4");
        assert_eq!(chord_pitches(3), vec!["C4", "G4", "C5"]);
    }

    #[test]
    fn chords_anchor_following_events_from_their_first_written_pitch() {
        let out = layout_sequence_relative_native("(C4 E G):q A", Clef::Treble, None).unwrap();
        assert_eq!(pitch_anchor_string(&out.layouts[1].pitches[0].pitch), "A3");
        assert_eq!(out.anchor.as_deref(), Some("A3"));
    }

    #[test]
    fn an_unanchored_chord_uses_the_clef_default_for_its_first_pitch() {
        let out = layout_sequence_relative_native("(C E G):q", Clef::Bass, None).unwrap();
        let pitches: Vec<_> = out.layouts[0]
            .pitches
            .iter()
            .map(|pitch| pitch_anchor_string(&pitch.pitch))
            .collect();
        assert_eq!(pitches, vec!["C3", "E3", "G3"]);
        assert_eq!(out.anchor.as_deref(), Some("C3"));
    }

    #[test]
    fn rejects_legacy_bars_and_malformed_markers() {
        assert!(parse_sequence("C4:q | D4:q").is_err());
        assert!(parse_sequence("C4:q /").is_err());
        assert!(parse_sequence("C4:q / / D4:q").is_err());
        assert!(parse_sequence("r:q ~ C4:q").is_err());
        assert!(parse_sequence("C4:q ~ ~ C4:q").is_err());
        assert!(parse_sequence("C4: D")
            .unwrap_err()
            .contains("omit the colon"));
    }

    #[test]
    fn validates_annotation_language() {
        assert!(parse_note("C4:q[dyn=pp]").is_ok());
        assert!(parse_note("C4:q[dyn=sfz]").is_ok());
        assert!(parse_note("C4:q[fermata breath]").is_ok());
        assert!(parse_note("C4:q[text=dolce p1( h2<]").is_ok());
        assert!(parse_note("C4:q[unknown]").is_err());
        assert!(parse_note("C4:q[dyn=quiet]").is_err());
        assert!(parse_note("C4:q[s(]").is_err());
    }

    #[test]
    fn lays_out_single_note_for_typst() {
        let out = layout_note_native("C4:q", Clef::Treble).unwrap();
        assert_eq!(out.kind, "note");
        assert_eq!(out.pitches[0].staff_position, 0);
        assert_eq!(out.notehead, "black");
        assert!(out.stem);
        assert_eq!(out.flags, 0);
    }

    #[test]
    fn lays_out_sequence_for_typst() {
        let out = layout_sequence_native("C4:q D4:e E4:e r:q", Clef::Treble).unwrap();
        assert_eq!(out.len(), 4);
        assert_eq!(out[0].pitches[0].staff_position, 0);
        assert_eq!(out[1].flags, 1);
        assert!(out[3].rest);
    }

    #[test]
    fn slash_marks_next_note_as_beam_break() {
        let out = layout_sequence_native("E4:e / F4:e G4:e", Clef::Treble).unwrap();
        assert!(!out[0].beam_break_before);
        assert!(out[1].beam_break_before);
        assert!(!out[2].beam_break_before);
    }

    #[test]
    fn onsets_accumulate_durations() {
        let out = layout_sequence_native("C4:q D4:e E4:e F4:h", Clef::Treble).unwrap();
        assert_eq!(out[0].onset, Rational::new(0, 1));
        assert_eq!(out[1].onset, Rational::new(1, 4));
        assert_eq!(out[2].onset, Rational::new(3, 8));
        assert_eq!(out[3].onset, Rational::new(1, 2));
    }

    #[test]
    fn untimed_sequences_beam_consecutive_flagged_notes() {
        let out = layout_sequence_native("C4:e D4:e E4:q F4:e G4:e", Clef::Treble).unwrap();
        assert_eq!(out[0].beam_group, Some(0));
        assert_eq!(out[1].beam_group, Some(0));
        assert_eq!(out[2].beam_group, None);
        assert_eq!(out[3].beam_group, Some(1));
        assert_eq!(out[4].beam_group, Some(1));
    }

    #[test]
    fn simple_meter_beams_break_at_quarter_beats() {
        let out = layout_sequence_with_time_native(
            "C4:e D4:e E4:e F4:e G4:e A4:e B4:e C5:e",
            Clef::Treble,
            "4/4",
        )
        .unwrap();
        let groups: Vec<_> = out.iter().map(|l| l.beam_group).collect();
        assert_eq!(
            groups,
            vec![
                Some(0),
                Some(0),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3)
            ]
        );
    }

    #[test]
    fn compound_meter_beams_eighths_in_threes() {
        let out =
            layout_sequence_with_time_native("C4:e D4:e E4:e F4:e G4:e A4:e", Clef::Treble, "6/8")
                .unwrap();
        let groups: Vec<_> = out.iter().map(|l| l.beam_group).collect();
        assert_eq!(
            groups,
            vec![Some(0), Some(0), Some(0), Some(1), Some(1), Some(1)]
        );
    }

    #[test]
    fn hyphens_force_local_beam_joins_across_metric_boundaries() {
        let out =
            layout_sequence_with_time_native("C5:s D:s - E:s / F:s - G:s A:s", Clef::Treble, "3/8")
                .unwrap();
        let groups: Vec<_> = out.iter().map(|layout| layout.beam_group).collect();
        assert_eq!(
            groups,
            vec![Some(0), Some(0), Some(0), Some(1), Some(1), Some(1)]
        );
    }

    #[test]
    fn beam_joins_reject_non_adjacent_or_unflagged_events() {
        assert!(
            layout_sequence_with_time_native("- C4:e D:e", Clef::Treble, "2/8")
                .unwrap_err()
                .contains("adjacent flagged")
        );
        assert!(
            layout_sequence_with_time_native("C4:e r:e - D4:e E:e", Clef::Treble, "4/8")
                .unwrap_err()
                .contains("adjacent flagged")
        );
        assert!(
            layout_sequence_with_time_native("C4:q - D4:q", Clef::Treble, "2/4")
                .unwrap_err()
                .contains("flagged note")
        );
        assert!(
            layout_sequence_with_time_native("C4:e / - D4:e", Clef::Treble, "2/8")
                .unwrap_err()
                .contains("cannot be repeated or combined")
        );
    }

    #[test]
    fn rests_and_slashes_split_beam_groups() {
        let out = layout_sequence_with_time_native(
            "C4:e D4:e r:e E4:e / F4:e G4:e r:q",
            Clef::Treble,
            "4/4",
        )
        .unwrap();
        let groups: Vec<_> = out.iter().map(|l| l.beam_group).collect();
        assert_eq!(
            groups,
            vec![Some(0), Some(0), None, Some(1), Some(2), Some(2), None]
        );
    }

    #[test]
    fn tuplets_scale_time_and_preserve_their_written_durations() {
        let out = layout_sequence_with_time_native(
            "tuplet 3:2 { C4:e D E } F:q",
            Clef::Treble,
            "2/4",
        )
        .unwrap();
        assert_eq!(out.len(), 4);
        assert_eq!(out[0].duration_value, Rational::new(1, 12));
        assert_eq!(out[1].onset, Rational::new(1, 12));
        assert_eq!(out[2].onset, Rational::new(1, 6));
        assert_eq!(out[3].onset, Rational::new(1, 4));
        assert_eq!(out[0].tuplet_starts.len(), 1);
        assert_eq!(out[0].tuplet_starts[0].numerator, 3);
        assert_eq!(out[0].tuplet_starts[0].denominator, 2);
        assert_eq!(out[0].tuplet_starts[0].end_index, 2);
        assert_eq!(out[0].beam_group, Some(0));
        assert_eq!(out[2].beam_group, Some(0));
    }

    #[test]
    fn tuplets_accept_options_and_nesting() {
        let out = layout_sequence_native(
            "tuplet 5:4[bracket=always side=above] { C4:s D tuplet 3:2 { E F G } A B }",
            Clef::Treble,
        )
        .unwrap();
        assert_eq!(out.len(), 7);
        assert_eq!(out[0].tuplet_starts[0].bracket, "always");
        assert_eq!(out[0].tuplet_starts[0].side, "above");
        assert_eq!(out[2].tuplet_starts[0].numerator, 3);
        assert_eq!(out[0].tuplet_starts[0].depth, 1);
        assert_eq!(out[2].tuplet_starts[0].depth, 0);
    }

    #[test]
    fn tuplets_report_malformed_groups() {
        assert!(layout_sequence_native("tuplet 3:2 C4:e D E", Clef::Treble)
            .unwrap_err()
            .contains("must be followed"));
        assert!(layout_sequence_native("tuplet 3:2 { }", Clef::Treble)
            .unwrap_err()
            .contains("must contain"));
        assert!(layout_sequence_native("tuplet 3:2[bracket=sideways] { C4:e D E }", Clef::Treble)
            .unwrap_err()
            .contains("bracket must be"));
    }

    #[test]
    fn grace_groups_take_no_bar_time_and_attach_to_the_following_event() {
        let out = layout_sequence_with_time_native(
            "C5:q acciaccatura { D5:s E } F5:q G5:h",
            Clef::Treble,
            "4/4",
        )
        .unwrap();

        assert_eq!(out.len(), 5);
        assert!(out[1].grace);
        assert!(out[2].grace);
        assert_eq!(out[1].grace_style.as_deref(), Some("acciaccatura"));
        assert_eq!(out[1].grace_group, out[2].grace_group);
        assert_eq!(out[1].grace_index, 0);
        assert_eq!(out[2].grace_index, 1);
        assert_eq!(out[1].grace_count, 2);
        assert_eq!(out[1].duration_value, Rational::new(0, 1));
        assert_eq!(out[1].onset, Rational::new(1, 4));
        assert_eq!(out[2].onset, Rational::new(1, 4));
        assert_eq!(out[3].onset, Rational::new(1, 4));
        assert_eq!(out[3].grace_before, 2);
        assert_eq!(out[1].beam_group, out[2].beam_group);
        assert!(out[1].beam_group.is_some());
    }

    #[test]
    fn grace_groups_reject_ambiguous_contents_and_missing_principal_notes() {
        let rest_error = layout_sequence_native("grace { r:e } C5:q", Clef::Treble).unwrap_err();
        assert!(rest_error.contains("rests are not supported"), "{rest_error}");
        assert!(layout_sequence_native("C5:q grace { D5:e }", Clef::Treble)
            .unwrap_err()
            .contains("followed by a main"));
        let nested_error =
            layout_sequence_native("grace { grace { C5:e } } D5:q", Clef::Treble).unwrap_err();
        assert!(nested_error.contains("cannot nest"), "{nested_error}");
    }

    #[test]
    fn alternating_tremolos_attach_to_two_equal_events() {
        let out = layout_sequence_with_time_native(
            "tremolo 16 { C5:h G5:h }",
            Clef::Treble,
            "4/4",
        )
        .unwrap();

        assert_eq!(out.len(), 2);
        assert_eq!(out[0].tremolo_starts.len(), 1);
        assert_eq!(out[0].tremolo_starts[0].subdivision, 16);
        assert_eq!(out[0].tremolo_starts[0].end_index, 1);
        assert!(out[0].alternating_tremolo);
        assert!(out[1].alternating_tremolo);
        assert_eq!(out[0].beam_group, None);
        assert_eq!(out[1].beam_group, None);

        assert!(layout_sequence_native("tremolo 16 { C5:h }", Clef::Treble)
            .unwrap_err()
            .contains("exactly two"));
        let unequal_error =
            layout_sequence_native("tremolo 16 { C5:h G5:q }", Clef::Treble).unwrap_err();
        assert!(unequal_error.contains("same written duration"), "{unequal_error}");
    }

    #[test]
    fn arpeggio_and_single_note_tremolo_annotations_are_validated() {
        assert!(layout_sequence_native("(C5 E G):h[arpeggio=up]", Clef::Treble).is_ok());
        assert!(layout_sequence_native("C5:h[tremolo=16]", Clef::Treble).is_ok());
        assert!(layout_sequence_native("C5:h[arpeggio]", Clef::Treble)
            .unwrap_err()
            .contains("requires a chord"));
        let short_error = layout_sequence_native("C5:s[tremolo=8]", Clef::Treble).unwrap_err();
        assert!(short_error.contains("must be shorter"), "{short_error}");
    }

    #[test]
    fn expands_auto_rests_evenly_when_possible() {
        let out = parse_sequence_with_auto_rests("_ Eb4:h _", "4/4").unwrap();
        assert_eq!(out.len(), 3);
        assert!(matches!(out[0].0, ParsedEvent::Rest(_)));
        assert!(matches!(out[1].0, ParsedEvent::Note(_)));
        assert!(matches!(out[2].0, ParsedEvent::Rest(_)));
        assert_eq!(out[0].0.duration().base, DurationBase::Quarter);
        assert_eq!(out[2].0.duration().base, DurationBase::Quarter);
    }

    #[test]
    fn expands_ambiguous_auto_rests_with_representable_durations() {
        let out = parse_sequence_with_auto_rests("_ E4:q _", "4/4").unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(
            duration_to_rational(out[0].0.duration()),
            Rational::new(3, 8)
        );
        assert_eq!(
            duration_to_rational(out[2].0.duration()),
            Rational::new(3, 8)
        );
    }
}
