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
        let octave = octave_str
            .parse::<i32>()
            .map_err(|_| format!("invalid octave {octave_str:?} in pitch {input:?}"))?;
        if !(-1..=9).contains(&octave) {
            return Err(format!(
                "octave {octave} in pitch {input:?} is outside the supported range -1 through 9; use a renderable staff octave"
            ));
        }
        Some(octave)
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
            .unwrap_or(base_octave)
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

    let mut dots = 0_usize;
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

    Ok(Duration {
        base,
        dots: dots as u8,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rational {
    pub numerator: u64,
    pub denominator: u64,
}

impl Rational {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        assert!(denominator != 0, "rational denominator must not be zero");
        let common_divisor = gcd(numerator, denominator);
        Self {
            numerator: numerator / common_divisor,
            denominator: denominator / common_divisor,
        }
    }

    pub fn checked_mul(self, other: Self) -> Result<Self, String> {
        rational_from_u128(
            u128::from(self.numerator) * u128::from(other.numerator),
            u128::from(self.denominator) * u128::from(other.denominator),
        )
    }

    pub fn checked_add(self, other: Self) -> Result<Self, String> {
        let left = u128::from(self.numerator) * u128::from(other.denominator);
        let right = u128::from(other.numerator) * u128::from(self.denominator);
        let numerator = left.checked_add(right).ok_or_else(rhythmic_overflow)?;
        let denominator = u128::from(self.denominator) * u128::from(other.denominator);
        rational_from_u128(numerator, denominator)
    }

    pub fn sub(self, other: Self) -> Option<Self> {
        let left = u128::from(self.numerator) * u128::from(other.denominator);
        let right = u128::from(other.numerator) * u128::from(self.denominator);
        if left < right {
            None
        } else {
            rational_from_u128(
                left - right,
                u128::from(self.denominator) * u128::from(other.denominator),
            )
            .ok()
        }
    }

    pub fn checked_div_u64(self, divisor: u64) -> Option<Self> {
        rational_from_u128(
            u128::from(self.numerator),
            u128::from(self.denominator) * u128::from(divisor),
        )
        .ok()
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

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn gcd_u128(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn rhythmic_overflow() -> String {
    "rhythmic arithmetic exceeds the supported range; simplify nested tuplets or use smaller reduced ratios"
        .to_string()
}

fn rational_from_u128(numerator: u128, denominator: u128) -> Result<Rational, String> {
    if denominator == 0 {
        return Err(rhythmic_overflow());
    }
    let common_divisor = gcd_u128(numerator, denominator);
    let numerator = numerator / common_divisor;
    let denominator = denominator / common_divisor;
    if numerator > u128::from(u64::MAX) || denominator > u128::from(u64::MAX) {
        return Err(rhythmic_overflow());
    }
    Ok(Rational::new(numerator as u64, denominator as u64))
}

pub fn duration_to_rational(duration: Duration) -> Rational {
    let (dot_numerator, dot_denominator) = match duration.dots {
        0 => (1, 1),
        1 => (3, 2),
        2 => (7, 4),
        _ => unreachable!("parse_duration guarantees dots <= 2"),
    };
    Rational::new(
        dot_numerator,
        u64::from(duration.base.denominator()) * dot_denominator,
    )
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

    if let Some(equal_share) = remaining.checked_div_u64(count as u64) {
        if let Some(duration) = rational_to_duration(equal_share) {
            return Some(vec![duration; count]);
        }
    }

    fn find_rest_durations(
        remaining: Rational,
        count: usize,
        supported_durations: &[Duration],
    ) -> Option<Vec<Duration>> {
        if count == 0 {
            return if remaining.numerator == 0 {
                Some(Vec::new())
            } else {
                None
            };
        }

        for duration in supported_durations {
            let duration_value = duration_to_rational(*duration);
            let Some(next_remaining) = remaining.sub(duration_value) else {
                continue;
            };
            if let Some(mut remaining_durations) =
                find_rest_durations(next_remaining, count - 1, supported_durations)
            {
                let mut rest_durations = vec![*duration];
                rest_durations.append(&mut remaining_durations);
                return Some(rest_durations);
            }
        }
        None
    }

    find_rest_durations(remaining, count, &supported_durations_descending())
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
    let parsed_event = parse_event(input)?;
    match parsed_event {
        ParsedEvent::Note(note) => Ok(note),
        _ => Err(format!("expected a single note, got {input:?}")),
    }
}

pub fn parse_chord(input: &str) -> Result<Vec<Note>, String> {
    let parsed_event = parse_event(input)?;
    match parsed_event {
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
    } else if let Some(rest_duration) = input.strip_prefix("r:") {
        let (duration, annotations) = parse_duration_and_annotations(rest_duration)?;
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
        let chord_event =
            parse_chord_event_relative(input, pitch_anchor.as_ref(), duration_anchor, clef)?;
        if let ParsedEvent::Chord { notes, .. } = &chord_event {
            *pitch_anchor = notes.first().map(|note| note.pitch.clone());
        }
        Ok(chord_event)
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

fn parse_sequence_event_relative(
    event_text: &str,
    pitch_anchor: &mut Option<Pitch>,
    duration_anchor: &mut Option<Duration>,
    clef: Clef,
) -> Result<ParsedEvent, String> {
    parse_event_relative(event_text, pitch_anchor, duration_anchor, clef)
        .map_err(|error| format!("invalid event {event_text:?}: {error}"))
}

fn split_note_pitch_and_tail(input: &str) -> Result<(&str, &str, bool), String> {
    if let Some((pitch_part, duration_and_annotations)) = input.split_once(':') {
        return Ok((pitch_part, duration_and_annotations, true));
    }

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    if chars.is_empty() {
        return Err("empty note event".to_string());
    }

    let mut cursor = 0_usize;
    let (_, pitch_letter) = chars[cursor];
    Letter::parse(pitch_letter)
        .ok_or_else(|| format!("invalid pitch letter {pitch_letter:?}; expected A-G or a-g"))?;
    cursor += 1;

    let accidental_start = cursor;
    while cursor < chars.len()
        && cursor - accidental_start < 2
        && matches!(chars[cursor].1, '#' | 'b')
    {
        cursor += 1;
    }

    if cursor < chars.len() && chars[cursor].1 == '-' {
        cursor += 1;
    }

    let octave_start = cursor;
    while cursor < chars.len() && chars[cursor].1.is_ascii_digit() {
        cursor += 1;
    }
    if cursor == octave_start && cursor > 0 && chars[cursor - 1].1 == '-' {
        return Err(format!("invalid negative octave in note event {input:?}"));
    }
    if cursor >= chars.len() {
        return Ok((input, "", false));
    }

    let split_byte = chars[cursor].0;
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
    let closing_parenthesis = input
        .find(')')
        .ok_or_else(|| format!("missing ')' in chord event {input:?}"))?;
    let pitch_list = &input[1..closing_parenthesis];
    let chord_suffix = input[closing_parenthesis + 1..].trim_start();
    let duration_part = chord_suffix
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
    let closing_parenthesis = input
        .find(')')
        .ok_or_else(|| format!("missing ')' in chord event {input:?}"))?;
    let pitch_list = &input[1..closing_parenthesis];
    let chord_suffix = input[closing_parenthesis + 1..].trim_start();
    let (duration_and_annotations, duration_is_explicit) = if let Some(duration_and_annotations) =
        chord_suffix.strip_prefix(':')
    {
        (duration_and_annotations, true)
    } else if chord_suffix.is_empty() || chord_suffix.starts_with('[') {
        (chord_suffix, false)
    } else {
        return Err(format!(
                "unexpected text {chord_suffix:?} after ')' in chord event {input:?}; use : followed by a duration"
            ));
    };
    let (duration, annotations) = parse_duration_with_state(
        duration_and_annotations,
        duration_is_explicit,
        duration_anchor,
        input,
    )?;

    let mut notes = Vec::new();
    let mut chord_pitch_anchor = external_pitch_anchor.cloned();
    for pitch_text in pitch_list.split_whitespace() {
        let pitch = resolve_pitch(
            parse_pitch_spec(pitch_text)?,
            chord_pitch_anchor.as_ref(),
            clef,
        );
        chord_pitch_anchor = Some(pitch.clone());
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
    validate_chord_pitches(&notes, input)?;

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
    for pitch_text in pitch_list.split_whitespace() {
        notes.push(Note {
            pitch: parse_pitch(pitch_text)?,
            duration,
            tie_to_next: false,
            annotations: annotations.clone(),
        });
    }
    if notes.is_empty() {
        return Err(format!("empty chord in {input:?}"));
    }
    validate_chord_pitches(&notes, input)?;

    Ok(ParsedEvent::Chord {
        notes,
        duration,
        tie_to_next: false,
        annotations,
    })
}

fn validate_chord_pitches(notes: &[Note], input: &str) -> Result<(), String> {
    if notes.len() < 2 {
        return Err(format!(
            "chord {input:?} must contain at least two pitches; remove the parentheses for a single note"
        ));
    }
    for (index, note) in notes.iter().enumerate() {
        if notes[..index]
            .iter()
            .any(|previous| previous.pitch == note.pitch)
        {
            return Err(format!(
                "chord {input:?} repeats the same written pitch; remove the duplicate pitch"
            ));
        }
    }
    Ok(())
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

    if annotation_part.is_some_and(|part| part.trim().is_empty()) {
        return Err(
            "empty annotation block []; remove it or add a documented annotation".to_string(),
        );
    }
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
    validate_annotation_combinations(&annotations)?;

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

fn is_valid_span_id(annotation: &str, prefix: char, suffix: char) -> bool {
    let Some(identifier) = annotation.strip_suffix(suffix) else {
        return false;
    };
    let Some(identifier_body) = identifier.strip_prefix(prefix) else {
        return false;
    };
    !identifier_body.is_empty()
        && identifier_body
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn is_turn_ornament(annotation: &str) -> bool {
    matches!(annotation, "turn" | "chromatic-turn" | "inverted-turn")
}

fn is_ornament(annotation: &str) -> bool {
    is_turn_ornament(annotation) || matches!(annotation, "trill" | "mordent" | "inverted-mordent")
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
        "inverted-turn",
        "trill",
        "mordent",
        "inverted-mordent",
        "fermata",
        "breath",
        "arpeggio",
    ];
    const DYNAMICS: &[&str] = &[
        "p", "pp", "ppp", "pppp", "ppppp", "pppppp", "f", "ff", "fff", "ffff", "fffff", "ffffff",
        "mp", "mf", "sf", "sfp", "sfpp", "fp", "rf", "rfz", "sfz", "sffz", "fz", "n", "pf", "sfzp",
    ];

    let annotation_is_supported = MARKS.contains(&annotation)
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
        || is_valid_span_id(annotation, 's', '(')
        || is_valid_span_id(annotation, 's', ')')
        || is_valid_span_id(annotation, 'p', '(')
        || is_valid_span_id(annotation, 'p', ')')
        || is_valid_span_id(annotation, 'h', '<')
        || is_valid_span_id(annotation, 'h', '>')
        || is_valid_span_id(annotation, 'h', '!');

    if annotation_is_supported {
        Ok(())
    } else {
        Err(format!(
            "unknown annotation {annotation:?}; expected a documented mark, span marker, text=..., or dyn=..."
        ))
    }
}

fn validate_annotation_combinations(annotations: &[String]) -> Result<(), String> {
    for (index, annotation) in annotations.iter().enumerate() {
        if annotations[..index].contains(annotation) {
            return Err(format!(
                "annotation {annotation:?} is repeated on the same event; keep it only once"
            ));
        }
    }

    for (label, prefix) in [
        ("fingering", "f="),
        ("turn fingering", "turn-f="),
        ("text direction", "text="),
        ("below-staff text direction", "text-below="),
        ("dynamic", "dyn="),
        ("arpeggio", "arpeggio"),
        ("single-note tremolo", "tremolo="),
    ] {
        let count = annotations
            .iter()
            .filter(|annotation| {
                if prefix == "arpeggio" {
                    annotation.as_str() == "arpeggio" || annotation.starts_with("arpeggio=")
                } else {
                    annotation.starts_with(prefix)
                }
            })
            .count();
        if count > 1 {
            return Err(format!(
                "event has more than one {label} annotation; keep exactly one"
            ));
        }
    }

    Ok(())
}

pub fn parse_sequence(input: &str) -> Result<Vec<ParsedEvent>, String> {
    Ok(parse_sequence_marked(input, Clef::Treble, None, None)?
        .events
        .into_iter()
        .map(|item| item.event)
        .collect())
}

fn set_pending_beam_directive(
    pending_beam_directive: &mut BeamDirective,
    directive: BeamDirective,
) -> Result<(), String> {
    if *pending_beam_directive != BeamDirective::Auto {
        return Err(
            "beam markers '/' and '-' cannot be repeated or combined before an event".to_string(),
        );
    }
    *pending_beam_directive = directive;
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
    let scale = scale.checked_mul(Rational::new(u64::from(denominator), u64::from(numerator)))?;
    let mut pending_beam = BeamDirective::Auto;

    for token in tokens {
        match token {
            Token::Event(event_text) => {
                events.push(SequencedEvent {
                    event: parse_sequence_event_relative(
                        event_text,
                        pitch_anchor,
                        duration_anchor,
                        clef,
                    )?,
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
                return Err(
                    "automatic rest placeholder '_' is not allowed inside a tuplet".to_string(),
                )
            }
            Token::BeamBreak => {
                set_pending_beam_directive(&mut pending_beam, BeamDirective::Break)?
            }
            Token::BeamJoin => set_pending_beam_directive(&mut pending_beam, BeamDirective::Join)?,
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
    let mut grace_events = Vec::new();
    let mut pending_beam = BeamDirective::Auto;
    for token in tokens {
        match token {
            Token::Event(event_text) => {
                grace_events.push(SequencedEvent {
                    event: parse_sequence_event_relative(
                        event_text,
                        pitch_anchor,
                        duration_anchor,
                        clef,
                    )?,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(0, 1),
                    grace: None,
                });
                pending_beam = BeamDirective::Auto;
            }
            Token::BeamBreak => {
                set_pending_beam_directive(&mut pending_beam, BeamDirective::Break)?
            }
            Token::BeamJoin => set_pending_beam_directive(&mut pending_beam, BeamDirective::Join)?,
            Token::Tie => {
                let Some(previous) = grace_events.last_mut() else {
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
                return Err(
                    "automatic rest placeholder '_' is not allowed inside grace groups".to_string(),
                )
            }
        }
    }
    if pending_beam != BeamDirective::Auto {
        return Err("beam marker '/' or '-' cannot end a grace group".to_string());
    }
    if grace_events.is_empty() {
        return Err("grace group must contain at least one note or chord".to_string());
    }
    if grace_events
        .iter()
        .any(|item| matches!(item.event, ParsedEvent::Rest(_)))
    {
        return Err("written rests are not supported inside grace groups".to_string());
    }
    let count = grace_events.len();
    for (index, grace_event) in grace_events.iter_mut().enumerate() {
        grace_event.grace = Some(GraceMeta {
            style: style.clone(),
            group,
            index,
            count,
        });
    }
    Ok(grace_events)
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
            Token::Event(event_text) => {
                let tremolo_event =
                    parse_sequence_event_relative(event_text, pitch_anchor, duration_anchor, clef)?;
                if matches!(tremolo_event, ParsedEvent::Rest(_)) {
                    return Err(
                        "alternating tremolo requires notes or chords, not rests".to_string()
                    );
                }
                events.push(SequencedEvent {
                    event: tremolo_event,
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
                return Err(
                    "automatic rests are not allowed inside alternating tremolos".to_string(),
                )
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
        return Err(
            "alternating tremolo subdivision must be shorter than its written notes".to_string(),
        );
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
            Token::Event(event_text) => {
                events.push(SequencedEvent {
                    event: parse_sequence_event_relative(
                        &event_text,
                        &mut pitch_anchor,
                        &mut duration_anchor,
                        clef,
                    )?,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(1, 1),
                    grace: None,
                });
                pending_beam = BeamDirective::Auto;
            }
            Token::Grace { style, contents } => {
                if pending_beam != BeamDirective::Auto {
                    return Err(
                        "beam marker '/' or '-' cannot appear before a grace group".to_string()
                    );
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
            Token::Tremolo {
                subdivision,
                contents,
            } => {
                if pending_beam != BeamDirective::Auto {
                    return Err(
                        "beam marker '/' or '-' cannot appear before an alternating tremolo"
                            .to_string(),
                    );
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
                set_pending_beam_directive(&mut pending_beam, BeamDirective::Break)?;
            }
            Token::BeamJoin => {
                set_pending_beam_directive(&mut pending_beam, BeamDirective::Join)?;
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
    tokenize_sequence_with_depth(input, 0)
}

fn tokenize_sequence_with_depth(input: &str, nesting_depth: usize) -> Result<Vec<Token>, String> {
    if nesting_depth > 8 {
        return Err(
            "notation groups may nest at most eight levels; simplify or split the nested tuplets"
                .to_string(),
        );
    }
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut cursor = 0_usize;

    while cursor < chars.len() {
        match chars[cursor] {
            ch if ch.is_whitespace() => cursor += 1,
            '|' => {
                return Err(
                    "bar separator '|' is not allowed inside notes; create another bars entry"
                        .to_string(),
                )
            }
            '/' => {
                tokens.push(Token::BeamBreak);
                cursor += 1;
            }
            '-' => {
                tokens.push(Token::BeamJoin);
                cursor += 1;
            }
            '_' => {
                tokens.push(Token::AutoRest);
                cursor += 1;
            }
            '~' => {
                tokens.push(Token::Tie);
                cursor += 1;
            }
            '(' => {
                let event_start = cursor;
                cursor += 1;
                while cursor < chars.len() && chars[cursor] != ')' {
                    cursor += 1;
                }
                if cursor == chars.len() {
                    return Err("unterminated chord in sequence".to_string());
                }
                cursor += 1;
                cursor = consume_event_suffix(&chars, cursor)?;
                tokens.push(Token::Event(chars[event_start..cursor].iter().collect()));
            }
            '{' | '}' => return Err("tuplet braces must follow 'tuplet N:M'".to_string()),
            _ => {
                if chars[cursor..].starts_with(&['t', 'r', 'e', 'm', 'o', 'l', 'o'])
                    && cursor + 7 < chars.len()
                    && chars[cursor + 7].is_whitespace()
                {
                    tokens.push(parse_tremolo_token(&chars, &mut cursor, nesting_depth)?);
                    continue;
                }
                let mut parsed_grace = false;
                for (keyword, style) in [
                    ("acciaccatura", "acciaccatura"),
                    ("appoggiatura", "appoggiatura"),
                    ("grace", "grace"),
                ] {
                    let keyword_chars: Vec<char> = keyword.chars().collect();
                    if chars[cursor..].starts_with(&keyword_chars)
                        && cursor + keyword_chars.len() < chars.len()
                        && chars[cursor + keyword_chars.len()].is_whitespace()
                    {
                        tokens.push(parse_grace_token(
                            &chars,
                            &mut cursor,
                            keyword,
                            style,
                            nesting_depth,
                        )?);
                        parsed_grace = true;
                        break;
                    }
                }
                if parsed_grace {
                    continue;
                }
                if chars[cursor..].starts_with(&['t', 'u', 'p', 'l', 'e', 't'])
                    && cursor + 6 < chars.len()
                    && chars[cursor + 6].is_whitespace()
                {
                    tokens.push(parse_tuplet_token(&chars, &mut cursor, nesting_depth)?);
                    continue;
                }
                let event_start = cursor;
                while cursor < chars.len()
                    && !chars[cursor].is_whitespace()
                    && chars[cursor] != '|'
                    && chars[cursor] != '~'
                    && chars[cursor] != '/'
                    && chars[cursor] != '{'
                    && chars[cursor] != '}'
                {
                    if chars[cursor] == '[' {
                        while cursor < chars.len() && chars[cursor] != ']' {
                            cursor += 1;
                        }
                        if cursor == chars.len() {
                            return Err("unterminated annotation block in sequence".to_string());
                        }
                    }
                    cursor += 1;
                }
                tokens.push(Token::Event(chars[event_start..cursor].iter().collect()));
            }
        }
    }

    Ok(tokens)
}

fn parse_tremolo_token(
    chars: &[char],
    cursor: &mut usize,
    nesting_depth: usize,
) -> Result<Token, String> {
    *cursor += "tremolo".len();
    while *cursor < chars.len() && chars[*cursor].is_whitespace() {
        *cursor += 1;
    }
    let subdivision_start = *cursor;
    while *cursor < chars.len() && chars[*cursor].is_ascii_digit() {
        *cursor += 1;
    }
    if subdivision_start == *cursor {
        return Err("tremolo must specify subdivision 8, 16, 32, or 64".to_string());
    }
    let subdivision: String = chars[subdivision_start..*cursor].iter().collect();
    let subdivision = subdivision
        .parse::<u32>()
        .map_err(|_| "invalid tremolo subdivision".to_string())?;
    if !matches!(subdivision, 8 | 16 | 32 | 64) {
        return Err("tremolo subdivision must be 8, 16, 32, or 64".to_string());
    }
    while *cursor < chars.len() && chars[*cursor].is_whitespace() {
        *cursor += 1;
    }
    if *cursor == chars.len() || chars[*cursor] != '{' {
        return Err("tremolo subdivision must be followed by '{ ... }'".to_string());
    }
    *cursor += 1;
    let contents_start = *cursor;
    let mut depth = 1_usize;
    while *cursor < chars.len() && depth > 0 {
        match chars[*cursor] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *cursor += 1;
    }
    if depth != 0 {
        return Err("unterminated alternating tremolo".to_string());
    }
    let contents: String = chars[contents_start..*cursor - 1].iter().collect();
    Ok(Token::Tremolo {
        subdivision,
        contents: tokenize_sequence_with_depth(&contents, nesting_depth + 1)?,
    })
}

fn parse_grace_token(
    chars: &[char],
    cursor: &mut usize,
    keyword: &str,
    style: &str,
    nesting_depth: usize,
) -> Result<Token, String> {
    *cursor += keyword.chars().count();
    while *cursor < chars.len() && chars[*cursor].is_whitespace() {
        *cursor += 1;
    }
    if *cursor == chars.len() || chars[*cursor] != '{' {
        return Err(format!("{keyword} must be followed by '{{ ... }}'"));
    }
    *cursor += 1;
    let contents_start = *cursor;
    let mut depth = 1_usize;
    while *cursor < chars.len() && depth > 0 {
        match chars[*cursor] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *cursor += 1;
    }
    if depth != 0 {
        return Err(format!("unterminated {keyword} group"));
    }
    let contents: String = chars[contents_start..*cursor - 1].iter().collect();
    Ok(Token::Grace {
        style: style.to_string(),
        contents: tokenize_sequence_with_depth(&contents, nesting_depth + 1)?,
    })
}

fn parse_tuplet_token(
    chars: &[char],
    cursor: &mut usize,
    nesting_depth: usize,
) -> Result<Token, String> {
    *cursor += "tuplet".len();
    while *cursor < chars.len() && chars[*cursor].is_whitespace() {
        *cursor += 1;
    }
    let numerator_start = *cursor;
    while *cursor < chars.len() && chars[*cursor].is_ascii_digit() {
        *cursor += 1;
    }
    if numerator_start == *cursor || *cursor == chars.len() || chars[*cursor] != ':' {
        return Err("tuplet ratio must look like 3:2".to_string());
    }
    let numerator: String = chars[numerator_start..*cursor].iter().collect();
    *cursor += 1;
    let denominator_start = *cursor;
    while *cursor < chars.len() && chars[*cursor].is_ascii_digit() {
        *cursor += 1;
    }
    if denominator_start == *cursor {
        return Err("tuplet ratio must look like 3:2".to_string());
    }
    let denominator: String = chars[denominator_start..*cursor].iter().collect();
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
    let mut has_bracket_option = false;
    let mut has_side_option = false;
    if *cursor < chars.len() && chars[*cursor] == '[' {
        *cursor += 1;
        let options_start = *cursor;
        while *cursor < chars.len() && chars[*cursor] != ']' {
            *cursor += 1;
        }
        if *cursor == chars.len() {
            return Err("unterminated tuplet options".to_string());
        }
        let options: String = chars[options_start..*cursor].iter().collect();
        *cursor += 1;
        for option in options.replace(',', " ").split_whitespace() {
            if let Some(value) = option.strip_prefix("bracket=") {
                if has_bracket_option {
                    return Err(
                        "tuplet bracket option is repeated; specify bracket only once".to_string(),
                    );
                }
                if !matches!(value, "auto" | "always" | "never") {
                    return Err("tuplet bracket must be auto, always, or never".to_string());
                }
                has_bracket_option = true;
                bracket = value.to_string();
            } else if let Some(value) = option.strip_prefix("side=") {
                if has_side_option {
                    return Err(
                        "tuplet side option is repeated; specify side only once".to_string()
                    );
                }
                if !matches!(value, "auto" | "above" | "below") {
                    return Err("tuplet side must be auto, above, or below".to_string());
                }
                has_side_option = true;
                side = value.to_string();
            } else {
                return Err(format!("unknown tuplet option {option:?}"));
            }
        }
    }
    while *cursor < chars.len() && chars[*cursor].is_whitespace() {
        *cursor += 1;
    }
    if *cursor == chars.len() || chars[*cursor] != '{' {
        return Err("tuplet ratio must be followed by '{ ... }'".to_string());
    }
    *cursor += 1;
    let contents_start = *cursor;
    let mut depth = 1_usize;
    while *cursor < chars.len() && depth > 0 {
        match chars[*cursor] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *cursor += 1;
    }
    if depth != 0 {
        return Err("unterminated tuplet".to_string());
    }
    let contents: String = chars[contents_start..*cursor - 1].iter().collect();
    Ok(Token::Tuplet {
        numerator,
        denominator,
        bracket,
        side,
        contents: tokenize_sequence_with_depth(&contents, nesting_depth + 1)?,
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
    Ok(
        parse_sequence_with_auto_rests_relative(input, time, Clef::Treble, None, None)?
            .events
            .into_iter()
            .map(|item| (item.event, item.beam_directive))
            .collect(),
    )
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
            Token::Event(event_text) => {
                let parsed_event = parse_sequence_event_relative(
                    &event_text,
                    &mut pitch_anchor,
                    &mut duration_anchor,
                    clef,
                )?;
                known_total =
                    known_total.checked_add(duration_to_rational(parsed_event.duration()))?;
                slots.push(Some(SequencedEvent {
                    event: parsed_event,
                    beam_directive: pending_beam,
                    duration_scale: Rational::new(1, 1),
                    grace: None,
                }));
                pending_beam = BeamDirective::Auto;
            }
            Token::Grace { style, contents } => {
                if pending_beam != BeamDirective::Auto {
                    return Err(
                        "beam marker '/' or '-' cannot appear before a grace group".to_string()
                    );
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
            Token::Tremolo {
                subdivision,
                contents,
            } => {
                if pending_beam != BeamDirective::Auto {
                    return Err(
                        "beam marker '/' or '-' cannot appear before an alternating tremolo"
                            .to_string(),
                    );
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
                    known_total =
                        known_total.checked_add(duration_to_rational(item.event.duration()))?;
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
                    let scaled_duration = duration_to_rational(item.event.duration())
                        .checked_mul(item.duration_scale)?;
                    known_total = known_total.checked_add(scaled_duration)?;
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
                set_pending_beam_directive(&mut pending_beam, BeamDirective::Break)?;
            }
            Token::BeamJoin => {
                set_pending_beam_directive(&mut pending_beam, BeamDirective::Join)?;
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
    if slots
        .iter()
        .rev()
        .flatten()
        .next()
        .is_some_and(|item| item.grace.is_some())
    {
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
    let mut generated_rest_durations = rest_durations.into_iter();
    let mut expanded_events = Vec::new();
    for slot in slots {
        match slot {
            Some(event) => expanded_events.push(event),
            None => {
                let duration = generated_rest_durations.next().ok_or_else(|| {
                    "internal auto-rest expansion produced too few durations".to_string()
                })?;
                expanded_events.push(SequencedEvent {
                    event: ParsedEvent::Rest(Rest {
                        duration,
                        annotations: Vec::new(),
                    }),
                    beam_directive: BeamDirective::Auto,
                    duration_scale: Rational::new(1, 1),
                    grace: None,
                });
            }
        }
    }
    Ok(ParsedSequenceState {
        events: expanded_events,
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
        Rational::new(u64::from(self.numerator), u64::from(self.denominator))
    }

    /// The beat used to group beams: a dotted unit in compound meters
    /// (6/8, 9/8, 12/8, ...), one denominator unit otherwise.
    fn beat_unit(self) -> Rational {
        if self.numerator > 3 && self.numerator.is_multiple_of(3) && self.denominator >= 8 {
            Rational::new(3, u64::from(self.denominator))
        } else {
            Rational::new(1, u64::from(self.denominator))
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

fn consume_event_suffix(chars: &[char], mut cursor: usize) -> Result<usize, String> {
    while cursor < chars.len()
        && !chars[cursor].is_whitespace()
        && chars[cursor] != '|'
        && chars[cursor] != '~'
        && chars[cursor] != '/'
    {
        if chars[cursor] == '[' {
            while cursor < chars.len() && chars[cursor] != ']' {
                cursor += 1;
            }
            if cursor == chars.len() {
                return Err("unterminated annotation block in chord suffix".to_string());
            }
        }
        cursor += 1;
    }
    Ok(cursor)
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
    let duration_value = duration_to_rational(duration).checked_mul(duration_scale)?;
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
    if kind == "rest" {
        let invalid_annotation = annotations.iter().find(|mark| {
            matches!(
                mark.as_str(),
                "stacc"
                    | "staccatissimo"
                    | "tenuto"
                    | "legato"
                    | "accent"
                    | "marcato"
                    | "strong"
                    | "turn"
                    | "chromatic-turn"
                    | "inverted-turn"
                    | "trill"
                    | "mordent"
                    | "inverted-mordent"
                    | "arpeggio"
            ) || mark.starts_with("f=")
                || mark.starts_with("turn-f=")
                || mark.starts_with("arpeggio=")
                || mark.starts_with("tremolo=")
                || (mark.starts_with('s') && (mark.ends_with('(') || mark.ends_with(')')))
        });
        if let Some(annotation) = invalid_annotation {
            return Err(format!(
                "annotation {annotation:?} requires a note or chord and cannot be attached to a rest; move it to a pitched event"
            ));
        }
    }
    if annotations.iter().any(|mark| mark.starts_with("turn-f="))
        && !annotations.iter().any(|mark| is_turn_ornament(mark))
    {
        return Err(
            "turn-f=... requires turn, chromatic-turn, or inverted-turn on the same event; add the ornament or remove its fingering"
                .to_string(),
        );
    }
    if annotations.iter().filter(|mark| is_ornament(mark)).count() > 1 {
        return Err(
            "an event may carry only one ornament (turn, inverted-turn, chromatic-turn, trill, mordent, or inverted-mordent); choose one"
                .to_string(),
        );
    }

    let (is_grace, grace_style, grace_group, grace_index, grace_count) = match grace {
        Some(meta) => (
            true,
            Some(meta.style),
            Some(meta.group),
            meta.index,
            meta.count,
        ),
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

fn layout_events(events: Vec<SequencedEvent>, clef: Clef) -> Result<Vec<NoteLayout>, String> {
    let mut onset = Rational::new(0, 1);
    let mut event_layouts = Vec::with_capacity(events.len());
    for sequenced_event in events {
        let duration_value = duration_to_rational(sequenced_event.event.duration())
            .checked_mul(sequenced_event.duration_scale)?;
        event_layouts.push(layout_event(
            sequenced_event.event,
            clef,
            sequenced_event.beam_directive,
            onset,
            sequenced_event.duration_scale,
            sequenced_event.grace,
        )?);
        onset = onset.checked_add(duration_value)?;
    }
    let mut pending_graces = 0_usize;
    for layout in &mut event_layouts {
        if layout.grace {
            pending_graces += 1;
        } else {
            layout.grace_before = pending_graces;
            pending_graces = 0;
        }
    }
    Ok(event_layouts)
}

fn attach_tuplets(layouts: &mut [NoteLayout], tuplets: Vec<ParsedTuplet>) -> Result<(), String> {
    for tuplet in &tuplets {
        if tuplet.start >= layouts.len() || tuplet.end == 0 || tuplet.end > layouts.len() {
            return Err("internal tuplet range is outside the parsed event sequence".to_string());
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
    Ok(())
}

fn attach_tremolos(layouts: &mut [NoteLayout], tremolos: Vec<ParsedTremolo>) -> Result<(), String> {
    for tremolo in tremolos {
        if tremolo.start >= layouts.len() || tremolo.end >= layouts.len() {
            return Err("internal tremolo range is outside the parsed event sequence".to_string());
        }
        layouts[tremolo.start].tremolo_starts.push(TremoloLayout {
            subdivision: tremolo.subdivision,
            end_index: tremolo.end,
        });
        layouts[tremolo.start].alternating_tremolo = true;
        layouts[tremolo.end].alternating_tremolo = true;
        layouts[tremolo.start].beam_group = None;
        layouts[tremolo.end].beam_group = None;
    }
    Ok(())
}

/// Group consecutive flagged notes into beams. With a beat unit, groups also
/// break at beat boundaries; rests and explicit '/' breaks always split.
fn assign_beam_groups(layouts: &mut [NoteLayout], beat: Option<Rational>) -> Result<(), String> {
    let mut next_group = 0_usize;
    let mut current_beam_group: Option<(usize, u64)> = None;
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
            current_beam_group = None;
            grace_group = None;
            previous_was_grace = layout.grace;
            continue;
        }
        if layout.grace {
            current_beam_group = None;
            let source_group = layout.grace_group.ok_or_else(|| {
                "internal grace event is missing its group identifier".to_string()
            })?;
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
            current_beam_group = None;
        }
        previous_was_grace = false;
        let beat_index = beat
            .map(|beat| {
                u64::from(layout.onset.numerator) * u64::from(beat.denominator)
                    / (u64::from(layout.onset.denominator) * u64::from(beat.numerator))
            })
            .unwrap_or(0);
        if layout.beam_join_before && current_beam_group.is_none() {
            return Err(format!(
                "beam join marker '-' before event {} requires an adjacent flagged note or chord before it",
                index + 1
            ));
        }
        let continues_current_beam_group = layout.beam_join_before
            || (matches!(current_beam_group, Some((_, index)) if index == beat_index)
                && !layout.beam_break_before);
        if !continues_current_beam_group {
            current_beam_group = Some((next_group, beat_index));
            next_group += 1;
        } else if layout.beam_join_before {
            current_beam_group = current_beam_group.map(|(group, _)| (group, beat_index));
        }
        let group =
            current_beam_group.ok_or_else(|| "internal beam group was not assigned".to_string())?;
        layout.beam_group = Some(group.0);
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
    attach_tuplets(&mut layouts, parsed.tuplets)?;
    attach_tremolos(&mut layouts, parsed.tremolos)?;
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
    attach_tuplets(&mut layouts, parsed.tuplets)?;
    attach_tremolos(&mut layouts, parsed.tremolos)?;
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

    #[derive(Serialize)]
    struct PluginResponse<T> {
        ok: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<T>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    }

    fn encode_plugin_response<T: Serialize>(result: Result<T, String>) -> Result<Vec<u8>, String> {
        let response = match result {
            Ok(data) => PluginResponse {
                ok: true,
                data: Some(data),
                error: None,
            },
            Err(error) => PluginResponse {
                ok: false,
                data: None,
                error: Some(error),
            },
        };
        serde_json::to_vec(&response)
            .map_err(|error| format!("typed-scores internal JSON serialization error: {error}"))
    }

    #[wasm_func]
    pub fn layout_note(input: &[u8]) -> Result<Vec<u8>, String> {
        encode_plugin_response((|| {
            let request_text = core::str::from_utf8(input)
                .map_err(|error| format!("input is not valid UTF-8: {error}"))?;
            let (clef_text, note_text) = request_text
                .split_once('\n')
                .unwrap_or(("treble", request_text));
            let clef = parse_clef(clef_text)?;
            layout_note_native(note_text, clef)
        })())
    }

    #[wasm_func]
    pub fn parse_events(input: &[u8]) -> Result<Vec<u8>, String> {
        encode_plugin_response((|| {
            let request_text = core::str::from_utf8(input)
                .map_err(|error| format!("input is not valid UTF-8: {error}"))?;
            parse_sequence(request_text)
        })())
    }

    #[wasm_func]
    pub fn layout_sequence(input: &[u8]) -> Result<Vec<u8>, String> {
        encode_plugin_response((|| {
            let request_text = core::str::from_utf8(input)
                .map_err(|error| format!("input is not valid UTF-8: {error}"))?;
            let (clef_text, sequence_text) = request_text
                .split_once('\n')
                .unwrap_or(("treble", request_text));
            let clef = parse_clef(clef_text)?;
            layout_sequence_native(sequence_text, clef)
        })())
    }

    #[wasm_func]
    pub fn layout_sequence_timed(input: &[u8]) -> Result<Vec<u8>, String> {
        encode_plugin_response((|| {
            let request_text = core::str::from_utf8(input)
                .map_err(|error| format!("input is not valid UTF-8: {error}"))?;
            let mut request_lines = request_text.splitn(3, '\n');
            let clef_text = request_lines.next().unwrap_or("treble");
            let time_signature = request_lines.next().unwrap_or("4/4");
            let sequence_text = request_lines.next().unwrap_or("");
            let clef = parse_clef(clef_text)?;
            layout_sequence_with_time_native(sequence_text, clef, time_signature)
        })())
    }

    #[wasm_func]
    pub fn layout_sequence_relative(input: &[u8]) -> Result<Vec<u8>, String> {
        encode_plugin_response((|| {
            let request_text = core::str::from_utf8(input)
                .map_err(|error| format!("input is not valid UTF-8: {error}"))?;
            let mut request_lines = request_text.splitn(4, '\n');
            let clef_text = request_lines.next().unwrap_or("treble");
            let pitch_anchor = request_lines.next().filter(|value| !value.is_empty());
            let duration_anchor = request_lines.next().filter(|value| !value.is_empty());
            let sequence_text = request_lines.next().unwrap_or("");
            let clef = parse_clef(clef_text)?;
            layout_sequence_relative_with_state_native(
                sequence_text,
                clef,
                pitch_anchor,
                duration_anchor,
            )
        })())
    }

    #[wasm_func]
    pub fn layout_sequence_timed_relative(input: &[u8]) -> Result<Vec<u8>, String> {
        encode_plugin_response((|| {
            let request_text = core::str::from_utf8(input)
                .map_err(|error| format!("input is not valid UTF-8: {error}"))?;
            let mut request_lines = request_text.splitn(5, '\n');
            let clef_text = request_lines.next().unwrap_or("treble");
            let time_signature = request_lines.next().unwrap_or("4/4");
            let pitch_anchor = request_lines.next().filter(|value| !value.is_empty());
            let duration_anchor = request_lines.next().filter(|value| !value.is_empty());
            let sequence_text = request_lines.next().unwrap_or("");
            let clef = parse_clef(clef_text)?;
            layout_sequence_with_time_relative_state_native(
                sequence_text,
                clef,
                time_signature,
                pitch_anchor,
                duration_anchor,
            )
        })())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn staff_position(pitch: &str, clef: Clef) -> i32 {
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
            assert_eq!(staff_position(pitch, Clef::Treble), expected, "{pitch}");
        }
    }

    #[test]
    fn clef_anchor_positions_are_correct() {
        assert_eq!(staff_position("C4", Clef::Treble), 0);
        assert_eq!(staff_position("E4", Clef::Treble), 2);
        assert_eq!(staff_position("F5", Clef::Treble), 10);
        assert_eq!(staff_position("A5", Clef::Treble), 12);

        assert_eq!(staff_position("E2", Clef::Bass), 0);
        assert_eq!(staff_position("G2", Clef::Bass), 2);
        assert_eq!(staff_position("A3", Clef::Bass), 10);
        assert_eq!(staff_position("C4", Clef::Bass), 12);

        assert_eq!(staff_position("D3", Clef::Alto), 0);
        assert_eq!(staff_position("F3", Clef::Alto), 2);
        assert_eq!(staff_position("C4", Clef::Alto), 6);
        assert_eq!(staff_position("G4", Clef::Alto), 10);
        assert_eq!(staff_position("B4", Clef::Alto), 12);

        assert_eq!(staff_position("B2", Clef::Tenor), 0);
        assert_eq!(staff_position("D3", Clef::Tenor), 2);
        assert_eq!(staff_position("C4", Clef::Tenor), 8);
        assert_eq!(staff_position("E4", Clef::Tenor), 10);
        assert_eq!(staff_position("G4", Clef::Tenor), 12);
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
            assert_eq!(staff_position(pitch, Clef::Treble), expected, "{pitch}");
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
    fn rhythmic_overflow_returns_an_error_instead_of_panicking() {
        let maximum = Rational::new(u64::MAX, 1);
        assert!(maximum.checked_mul(maximum).is_err());
        assert!(maximum.checked_add(maximum).is_err());
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
        let layout_response = layout_sequence_with_time_relative_state_native(
            "C4:e _",
            Clef::Treble,
            "4/4",
            None,
            None,
        )
        .unwrap();
        assert_eq!(layout_response.duration_anchor.as_deref(), Some("e"));
        assert_eq!(
            layout_response.layouts[1].duration,
            parse_duration("h..").unwrap()
        );
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
        let layout_response =
            layout_sequence_relative_native("(a4 c E):q (f A c) d (C4 g4 c5)", Clef::Treble, None)
                .unwrap();
        let chord_pitches = |index: usize| {
            layout_response.layouts[index]
                .pitches
                .iter()
                .map(|pitch| pitch_anchor_string(&pitch.pitch))
                .collect::<Vec<_>>()
        };
        assert_eq!(chord_pitches(0), vec!["A4", "C5", "E5"]);
        assert_eq!(chord_pitches(1), vec!["F4", "A4", "C5"]);
        assert_eq!(
            pitch_anchor_string(&layout_response.layouts[2].pitches[0].pitch),
            "D4"
        );
        assert_eq!(chord_pitches(3), vec!["C4", "G4", "C5"]);
    }

    #[test]
    fn chords_anchor_following_events_from_their_first_written_pitch() {
        let layout_response =
            layout_sequence_relative_native("(C4 E G):q A", Clef::Treble, None).unwrap();
        assert_eq!(
            pitch_anchor_string(&layout_response.layouts[1].pitches[0].pitch),
            "A3"
        );
        assert_eq!(layout_response.anchor.as_deref(), Some("A3"));
    }

    #[test]
    fn an_unanchored_chord_uses_the_clef_default_for_its_first_pitch() {
        let layout_response =
            layout_sequence_relative_native("(C E G):q", Clef::Bass, None).unwrap();
        let pitches: Vec<_> = layout_response.layouts[0]
            .pitches
            .iter()
            .map(|pitch| pitch_anchor_string(&pitch.pitch))
            .collect();
        assert_eq!(pitches, vec!["C3", "E3", "G3"]);
        assert_eq!(layout_response.anchor.as_deref(), Some("C3"));
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
        assert!(parse_note("C4:q[trill]").is_ok());
        assert!(parse_note("C4:q[mordent]").is_ok());
        assert!(parse_note("C4:q[inverted-mordent]").is_ok());
        assert!(parse_note("C4:q[inverted-turn]").is_ok());
        assert!(parse_note("C4:q[inverted-turn turn-f=2]").is_ok());
        assert!(parse_note("C4:q[text=dolce p1( h2<]").is_ok());
        assert!(parse_note("C4:q[unknown]").is_err());
        assert!(parse_note("C4:q[dyn=quiet]").is_err());
        assert!(parse_note("C4:q[s(]").is_err());
    }

    #[test]
    fn rejects_annotations_that_would_be_ignored_or_ambiguous() {
        assert!(parse_note("C4:q[]")
            .unwrap_err()
            .contains("empty annotation"));
        assert!(parse_note("C4:q[dyn=p dyn=f]")
            .unwrap_err()
            .contains("more than one dynamic"));
        assert!(parse_note("C4:q[stacc stacc]")
            .unwrap_err()
            .contains("repeated"));
        assert!(layout_sequence_native("r:q[stacc]", Clef::Treble)
            .unwrap_err()
            .contains("cannot be attached to a rest"));
        assert!(layout_sequence_native("C4:q[turn-f=3]", Clef::Treble)
            .unwrap_err()
            .contains("requires turn"));
        assert!(
            layout_sequence_native("C4:q[turn chromatic-turn]", Clef::Treble)
                .unwrap_err()
                .contains("only one ornament")
        );
        assert!(
            layout_sequence_native("C4:q[trill mordent]", Clef::Treble)
                .unwrap_err()
                .contains("only one ornament")
        );
        assert!(
            layout_sequence_native("C4:q[trill turn]", Clef::Treble)
                .unwrap_err()
                .contains("only one ornament")
        );
        assert!(layout_sequence_native("r:q[trill]", Clef::Treble)
            .unwrap_err()
            .contains("cannot be attached to a rest"));
    }

    #[test]
    fn rejects_unrenderable_octaves_and_excessive_tuplet_options() {
        assert!(parse_pitch("C10")
            .unwrap_err()
            .contains("supported range -1 through 9"));
        assert!(layout_sequence_native(
            "tuplet 3:2[side=above side=below] { C4:e D E }",
            Clef::Treble,
        )
        .unwrap_err()
        .contains("side option is repeated"));
    }

    #[test]
    fn rejects_chords_that_cannot_produce_distinct_noteheads() {
        assert!(parse_sequence("(C4):q")
            .unwrap_err()
            .contains("at least two pitches"));
        assert!(parse_sequence("(C4 E4 C4):q")
            .unwrap_err()
            .contains("repeats the same written pitch"));
    }

    #[test]
    fn sequence_errors_include_the_invalid_event_text() {
        let error = layout_sequence_native("C4:q H4:q", Clef::Treble).unwrap_err();
        assert!(error.contains("invalid event \"H4:q\""), "{error}");
    }

    #[test]
    fn lays_out_single_note_for_typst() {
        let note_layout = layout_note_native("C4:q", Clef::Treble).unwrap();
        assert_eq!(note_layout.kind, "note");
        assert_eq!(note_layout.pitches[0].staff_position, 0);
        assert_eq!(note_layout.notehead, "black");
        assert!(note_layout.stem);
        assert_eq!(note_layout.flags, 0);
    }

    #[test]
    fn lays_out_sequence_for_typst() {
        let event_layouts = layout_sequence_native("C4:q D4:e E4:e r:q", Clef::Treble).unwrap();
        assert_eq!(event_layouts.len(), 4);
        assert_eq!(event_layouts[0].pitches[0].staff_position, 0);
        assert_eq!(event_layouts[1].flags, 1);
        assert!(event_layouts[3].rest);
    }

    #[test]
    fn slash_marks_next_note_as_beam_break() {
        let event_layouts = layout_sequence_native("E4:e / F4:e G4:e", Clef::Treble).unwrap();
        assert!(!event_layouts[0].beam_break_before);
        assert!(event_layouts[1].beam_break_before);
        assert!(!event_layouts[2].beam_break_before);
    }

    #[test]
    fn onsets_accumulate_durations() {
        let event_layouts = layout_sequence_native("C4:q D4:e E4:e F4:h", Clef::Treble).unwrap();
        assert_eq!(event_layouts[0].onset, Rational::new(0, 1));
        assert_eq!(event_layouts[1].onset, Rational::new(1, 4));
        assert_eq!(event_layouts[2].onset, Rational::new(3, 8));
        assert_eq!(event_layouts[3].onset, Rational::new(1, 2));
    }

    #[test]
    fn untimed_sequences_beam_consecutive_flagged_notes() {
        let event_layouts =
            layout_sequence_native("C4:e D4:e E4:q F4:e G4:e", Clef::Treble).unwrap();
        assert_eq!(event_layouts[0].beam_group, Some(0));
        assert_eq!(event_layouts[1].beam_group, Some(0));
        assert_eq!(event_layouts[2].beam_group, None);
        assert_eq!(event_layouts[3].beam_group, Some(1));
        assert_eq!(event_layouts[4].beam_group, Some(1));
    }

    #[test]
    fn simple_meter_beams_break_at_quarter_beats() {
        let event_layouts = layout_sequence_with_time_native(
            "C4:e D4:e E4:e F4:e G4:e A4:e B4:e C5:e",
            Clef::Treble,
            "4/4",
        )
        .unwrap();
        let beam_groups: Vec<_> = event_layouts
            .iter()
            .map(|layout| layout.beam_group)
            .collect();
        assert_eq!(
            beam_groups,
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
        let event_layouts =
            layout_sequence_with_time_native("C4:e D4:e E4:e F4:e G4:e A4:e", Clef::Treble, "6/8")
                .unwrap();
        let beam_groups: Vec<_> = event_layouts
            .iter()
            .map(|layout| layout.beam_group)
            .collect();
        assert_eq!(
            beam_groups,
            vec![Some(0), Some(0), Some(0), Some(1), Some(1), Some(1)]
        );
    }

    #[test]
    fn hyphens_force_local_beam_joins_across_metric_boundaries() {
        let event_layouts =
            layout_sequence_with_time_native("C5:s D:s - E:s / F:s - G:s A:s", Clef::Treble, "3/8")
                .unwrap();
        let beam_groups: Vec<_> = event_layouts
            .iter()
            .map(|layout| layout.beam_group)
            .collect();
        assert_eq!(
            beam_groups,
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
        let event_layouts = layout_sequence_with_time_native(
            "C4:e D4:e r:e E4:e / F4:e G4:e r:q",
            Clef::Treble,
            "4/4",
        )
        .unwrap();
        let beam_groups: Vec<_> = event_layouts
            .iter()
            .map(|layout| layout.beam_group)
            .collect();
        assert_eq!(
            beam_groups,
            vec![Some(0), Some(0), None, Some(1), Some(2), Some(2), None]
        );
    }

    #[test]
    fn tuplets_scale_time_and_preserve_their_written_durations() {
        let event_layouts =
            layout_sequence_with_time_native("tuplet 3:2 { C4:e D E } F:q", Clef::Treble, "2/4")
                .unwrap();
        assert_eq!(event_layouts.len(), 4);
        assert_eq!(event_layouts[0].duration_value, Rational::new(1, 12));
        assert_eq!(event_layouts[1].onset, Rational::new(1, 12));
        assert_eq!(event_layouts[2].onset, Rational::new(1, 6));
        assert_eq!(event_layouts[3].onset, Rational::new(1, 4));
        assert_eq!(event_layouts[0].tuplet_starts.len(), 1);
        assert_eq!(event_layouts[0].tuplet_starts[0].numerator, 3);
        assert_eq!(event_layouts[0].tuplet_starts[0].denominator, 2);
        assert_eq!(event_layouts[0].tuplet_starts[0].end_index, 2);
        assert_eq!(event_layouts[0].beam_group, Some(0));
        assert_eq!(event_layouts[2].beam_group, Some(0));
    }

    #[test]
    fn tuplets_accept_options_and_nesting() {
        let event_layouts = layout_sequence_native(
            "tuplet 5:4[bracket=always side=above] { C4:s D tuplet 3:2 { E F G } A B }",
            Clef::Treble,
        )
        .unwrap();
        assert_eq!(event_layouts.len(), 7);
        assert_eq!(event_layouts[0].tuplet_starts[0].bracket, "always");
        assert_eq!(event_layouts[0].tuplet_starts[0].side, "above");
        assert_eq!(event_layouts[2].tuplet_starts[0].numerator, 3);
        assert_eq!(event_layouts[0].tuplet_starts[0].depth, 1);
        assert_eq!(event_layouts[2].tuplet_starts[0].depth, 0);
    }

    #[test]
    fn tuplets_report_malformed_groups() {
        assert!(layout_sequence_native("tuplet 3:2 C4:e D E", Clef::Treble)
            .unwrap_err()
            .contains("must be followed"));
        assert!(layout_sequence_native("tuplet 3:2 { }", Clef::Treble)
            .unwrap_err()
            .contains("must contain"));
        assert!(
            layout_sequence_native("tuplet 3:2[bracket=sideways] { C4:e D E }", Clef::Treble)
                .unwrap_err()
                .contains("bracket must be")
        );
    }

    #[test]
    fn grace_groups_take_no_bar_time_and_attach_to_the_following_event() {
        let event_layouts = layout_sequence_with_time_native(
            "C5:q acciaccatura { D5:s E } F5:q G5:h",
            Clef::Treble,
            "4/4",
        )
        .unwrap();

        assert_eq!(event_layouts.len(), 5);
        assert!(event_layouts[1].grace);
        assert!(event_layouts[2].grace);
        assert_eq!(
            event_layouts[1].grace_style.as_deref(),
            Some("acciaccatura")
        );
        assert_eq!(event_layouts[1].grace_group, event_layouts[2].grace_group);
        assert_eq!(event_layouts[1].grace_index, 0);
        assert_eq!(event_layouts[2].grace_index, 1);
        assert_eq!(event_layouts[1].grace_count, 2);
        assert_eq!(event_layouts[1].duration_value, Rational::new(0, 1));
        assert_eq!(event_layouts[1].onset, Rational::new(1, 4));
        assert_eq!(event_layouts[2].onset, Rational::new(1, 4));
        assert_eq!(event_layouts[3].onset, Rational::new(1, 4));
        assert_eq!(event_layouts[3].grace_before, 2);
        assert_eq!(event_layouts[1].beam_group, event_layouts[2].beam_group);
        assert!(event_layouts[1].beam_group.is_some());
    }

    #[test]
    fn grace_groups_reject_ambiguous_contents_and_missing_principal_notes() {
        let rest_error = layout_sequence_native("grace { r:e } C5:q", Clef::Treble).unwrap_err();
        assert!(
            rest_error.contains("rests are not supported"),
            "{rest_error}"
        );
        assert!(layout_sequence_native("C5:q grace { D5:e }", Clef::Treble)
            .unwrap_err()
            .contains("followed by a main"));
        let nested_error =
            layout_sequence_native("grace { grace { C5:e } } D5:q", Clef::Treble).unwrap_err();
        assert!(nested_error.contains("cannot nest"), "{nested_error}");
    }

    #[test]
    fn alternating_tremolos_attach_to_two_equal_events() {
        let event_layouts =
            layout_sequence_with_time_native("tremolo 16 { C5:h G5:h }", Clef::Treble, "4/4")
                .unwrap();

        assert_eq!(event_layouts.len(), 2);
        assert_eq!(event_layouts[0].tremolo_starts.len(), 1);
        assert_eq!(event_layouts[0].tremolo_starts[0].subdivision, 16);
        assert_eq!(event_layouts[0].tremolo_starts[0].end_index, 1);
        assert!(event_layouts[0].alternating_tremolo);
        assert!(event_layouts[1].alternating_tremolo);
        assert_eq!(event_layouts[0].beam_group, None);
        assert_eq!(event_layouts[1].beam_group, None);

        assert!(layout_sequence_native("tremolo 16 { C5:h }", Clef::Treble)
            .unwrap_err()
            .contains("exactly two"));
        let unequal_error =
            layout_sequence_native("tremolo 16 { C5:h G5:q }", Clef::Treble).unwrap_err();
        assert!(
            unequal_error.contains("same written duration"),
            "{unequal_error}"
        );
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
        let parsed_events = parse_sequence_with_auto_rests("_ Eb4:h _", "4/4").unwrap();
        assert_eq!(parsed_events.len(), 3);
        assert!(matches!(parsed_events[0].0, ParsedEvent::Rest(_)));
        assert!(matches!(parsed_events[1].0, ParsedEvent::Note(_)));
        assert!(matches!(parsed_events[2].0, ParsedEvent::Rest(_)));
        assert_eq!(parsed_events[0].0.duration().base, DurationBase::Quarter);
        assert_eq!(parsed_events[2].0.duration().base, DurationBase::Quarter);
    }

    #[test]
    fn expands_ambiguous_auto_rests_with_representable_durations() {
        let parsed_events = parse_sequence_with_auto_rests("_ E4:q _", "4/4").unwrap();
        assert_eq!(parsed_events.len(), 3);
        assert_eq!(
            duration_to_rational(parsed_events[0].0.duration()),
            Rational::new(3, 8)
        );
        assert_eq!(
            duration_to_rational(parsed_events[2].0.duration()),
            Rational::new(3, 8)
        );
    }
}
