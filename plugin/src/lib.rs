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
        match ch {
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

pub fn parse_pitch(input: &str) -> Result<Pitch, String> {
    let input = input.trim();
    let mut chars = input.chars().peekable();

    let Some(letter_ch) = chars.next() else {
        return Err("empty pitch".to_string());
    };
    let letter = Letter::parse(letter_ch)
        .ok_or_else(|| format!("invalid pitch letter {letter_ch:?}; expected A-G"))?;

    let accidental = match chars.peek().copied() {
        Some('#') => {
            chars.next();
            Accidental::Sharp
        }
        Some('b') => {
            chars.next();
            Accidental::Flat
        }
        _ => Accidental::Natural,
    };

    let octave_str: String = chars.collect();
    if octave_str.is_empty() {
        return Err(format!("missing octave in pitch {input:?}"));
    }
    let octave = octave_str
        .parse::<i32>()
        .map_err(|_| format!("invalid octave {octave_str:?} in pitch {input:?}"))?;

    Ok(Pitch {
        letter,
        accidental,
        octave,
    })
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
            Some(Self::new(left - right, self.denominator * other.denominator))
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
        "w..", "w.", "w", "h..", "h.", "h", "q..", "q.", "q", "e..", "e.", "e", "s..", "s.",
        "s", "t..", "t.", "t",
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
    fn set_tie_to_next(&mut self) {
        match self {
            Self::Note(note) => note.tie_to_next = true,
            Self::Rest(_) => {}
            Self::Chord {
                notes, tie_to_next, ..
            } => {
                *tie_to_next = true;
                for note in notes {
                    note.tie_to_next = true;
                }
            }
        }
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
    Tie,
    AutoRest,
    BeamBreak,
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

fn split_note_pitch_and_duration(input: &str) -> Result<(&str, &str), String> {
    if let Some((pitch_part, rest)) = input.split_once(':') {
        return Ok((pitch_part, rest));
    }

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    if chars.is_empty() {
        return Err("empty note event".to_string());
    }

    let mut idx = 0_usize;
    let (_, first) = chars[idx];
    Letter::parse(first)
        .ok_or_else(|| format!("invalid pitch letter {first:?}; expected A-G"))?;
    idx += 1;

    if idx < chars.len() && matches!(chars[idx].1, '#' | 'b') {
        idx += 1;
    }

    if idx < chars.len() && chars[idx].1 == '-' {
        idx += 1;
    }

    let octave_start = idx;
    while idx < chars.len() && chars[idx].1.is_ascii_digit() {
        idx += 1;
    }
    if idx == octave_start {
        return Err(format!("missing octave in note event {input:?}"));
    }
    if idx >= chars.len() {
        return Err(format!("missing ':' in note event {input:?}"));
    }

    let split_byte = chars[idx].0;
    Ok((&input[..split_byte], &input[split_byte..]))
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

fn parse_duration_and_annotations(input: &str) -> Result<(Duration, Vec<String>), String> {
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

    let duration = parse_duration(duration_part)?;
    let annotations = annotation_part
        .map(|part| {
            part.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok((duration, annotations))
}

pub fn parse_sequence(input: &str) -> Result<Vec<ParsedEvent>, String> {
    Ok(parse_sequence_marked(input)?
        .into_iter()
        .map(|(event, _)| event)
        .collect())
}

fn parse_sequence_marked(input: &str) -> Result<Vec<(ParsedEvent, bool)>, String> {
    let tokens = tokenize_sequence(input)?;
    let mut events: Vec<(ParsedEvent, bool)> = Vec::new();
    let mut beam_break_before_next = false;

    for token in tokens {
        match token {
            Token::Event(raw) => {
                events.push((parse_event(&raw)?, beam_break_before_next));
                beam_break_before_next = false;
            }
            Token::AutoRest => {
                return Err(
                    "auto rest placeholder '_' needs a time signature context".to_string(),
                )
            }
            Token::BeamBreak => {
                beam_break_before_next = true;
            }
            Token::Tie => {
                let Some((previous, _)) = events.last_mut() else {
                    return Err("tie marker '~' cannot appear before a note or chord".to_string());
                };
                previous.set_tie_to_next();
            }
        }
    }

    Ok(events)
}

fn tokenize_sequence(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0_usize;

    while i < chars.len() {
        match chars[i] {
            ch if ch.is_whitespace() || ch == '|' => i += 1,
            '/' => {
                tokens.push(Token::BeamBreak);
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
            _ => {
                let start = i;
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

pub fn layout_sequence_with_time_native(
    input: &str,
    clef: Clef,
    time: &str,
) -> Result<Vec<NoteLayout>, String> {
    let signature = parse_time_signature(time)?;
    let mut layouts = layout_events(parse_sequence_with_auto_rests(input, time)?, clef)?;
    assign_beam_groups(&mut layouts, Some(signature.beat_unit()));
    Ok(layouts)
}

pub fn parse_sequence_with_auto_rests(
    input: &str,
    time: &str,
) -> Result<Vec<(ParsedEvent, bool)>, String> {
    let tokens = tokenize_sequence(input)?;
    let expected = parse_time_signature(time)?.value();
    let mut slots: Vec<Option<(ParsedEvent, bool)>> = Vec::new();
    let mut known_total = Rational::new(0, 1);
    let mut auto_rest_count = 0_usize;
    let mut beam_break_before_next = false;

    for token in tokens {
        match token {
            Token::Event(raw) => {
                let event = parse_event(&raw)?;
                known_total = known_total.add(duration_to_rational(event.duration()));
                slots.push(Some((event, beam_break_before_next)));
                beam_break_before_next = false;
            }
            Token::AutoRest => {
                auto_rest_count += 1;
                slots.push(None);
                beam_break_before_next = false;
            }
            Token::BeamBreak => {
                beam_break_before_next = true;
            }
            Token::Tie => {
                let Some(Some((previous, _))) = slots.iter_mut().rev().find(|slot| slot.is_some())
                else {
                    return Err(
                        "tie marker '~' cannot appear before a note or chord".to_string(),
                    );
                };
                previous.set_tie_to_next();
            }
        }
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
        return Ok(slots.into_iter().flatten().collect());
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
            None => out.push((
                ParsedEvent::Rest(Rest {
                    duration: rest_iter
                        .next()
                        .expect("one generated rest duration per placeholder"),
                    annotations: Vec::new(),
                }),
                false,
            )),
        }
    }
    Ok(out)
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
    /// Events sharing a group id are joined by a beam.
    pub beam_group: Option<usize>,
}

fn layout_event(
    event: ParsedEvent,
    clef: Clef,
    beam_break_before: bool,
    onset: Rational,
) -> Result<NoteLayout, String> {
    let duration = event.duration();
    let duration_value = duration_to_rational(duration);
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
        beam_break_before,
        beam_group: None,
    })
}

fn layout_events(
    events: Vec<(ParsedEvent, bool)>,
    clef: Clef,
) -> Result<Vec<NoteLayout>, String> {
    let mut onset = Rational::new(0, 1);
    let mut out = Vec::with_capacity(events.len());
    for (event, beam_break_before) in events {
        let duration_value = duration_to_rational(event.duration());
        out.push(layout_event(event, clef, beam_break_before, onset)?);
        onset = onset.add(duration_value);
    }
    Ok(out)
}

/// Group consecutive flagged notes into beams. With a beat unit, groups also
/// break at beat boundaries; rests and explicit '/' breaks always split.
fn assign_beam_groups(layouts: &mut [NoteLayout], beat: Option<Rational>) {
    let mut next_group = 0_usize;
    let mut current: Option<(usize, u64)> = None;
    for layout in layouts.iter_mut() {
        if layout.rest || layout.flags == 0 {
            current = None;
            continue;
        }
        let beat_index = beat
            .map(|beat| {
                u64::from(layout.onset.numerator) * u64::from(beat.denominator)
                    / (u64::from(layout.onset.denominator) * u64::from(beat.numerator))
            })
            .unwrap_or(0);
        let continues = matches!(current, Some((_, index)) if index == beat_index)
            && !layout.beam_break_before;
        if !continues {
            current = Some((next_group, beat_index));
            next_group += 1;
        }
        layout.beam_group = Some(current.expect("group was just ensured").0);
    }
}

pub fn layout_note_native(input: &str, clef: Clef) -> Result<NoteLayout, String> {
    layout_event(parse_event(input)?, clef, false, Rational::new(0, 1))
}

pub fn layout_sequence_native(input: &str, clef: Clef) -> Result<Vec<NoteLayout>, String> {
    let mut layouts = layout_events(parse_sequence_marked(input)?, clef)?;
    assign_beam_groups(&mut layouts, None);
    Ok(layouts)
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
        assert_eq!(parse_pitch("D#4").unwrap().accidental, Accidental::Sharp);
        assert_eq!(parse_pitch("Eb4").unwrap().accidental, Accidental::Flat);
        assert_eq!(parse_pitch("Bb3").unwrap().letter, Letter::B);
    }

    #[test]
    fn rejects_invalid_pitches() {
        assert!(parse_pitch("").is_err());
        assert!(parse_pitch("H4").is_err());
        assert!(parse_pitch("C").is_err());
        assert!(parse_pitch("Cq").is_err());
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
        let note = parse_note(
            "C6:q[marcato strong accent tenuto legato stacc staccatissimo f=4]",
        )
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
        let events = parse_sequence("G4:e ~ G4:q. | r:e (C4 E4 G4):q").unwrap();
        assert_eq!(events.len(), 4);
        assert!(events[0].tie_to_next());
        assert!(!events[1].tie_to_next());
        assert!(matches!(events[2], ParsedEvent::Rest(_)));
        assert!(matches!(events[3], ParsedEvent::Chord { .. }));
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
        let out = layout_sequence_with_time_native(
            "C4:e D4:e E4:e F4:e G4:e A4:e",
            Clef::Treble,
            "6/8",
        )
        .unwrap();
        let groups: Vec<_> = out.iter().map(|l| l.beam_group).collect();
        assert_eq!(
            groups,
            vec![Some(0), Some(0), Some(0), Some(1), Some(1), Some(1)]
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
        assert_eq!(duration_to_rational(out[0].0.duration()), Rational::new(3, 8));
        assert_eq!(duration_to_rational(out[2].0.duration()), Rational::new(3, 8));
    }
}
