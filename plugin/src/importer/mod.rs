//! Convert MusicXML and ABC scores into `score()` arguments.

mod abc;
mod collections;
mod emit;
mod frac;
mod model;
mod musicxml;
mod normalize;
mod xml;
mod zip;

pub use emit::Emitted;

/// Options sent by `read-score`, one `name=value` per line.
pub struct Request<'a> {
    pub format: &'a str,
    pub tune: Option<&'a str>,
    pub package: &'a str,
    pub source_name: &'a str,
    pub scale: &'a str,
}

impl<'a> Request<'a> {
    pub fn parse(options: &'a str) -> Request<'a> {
        let mut request = Request {
            format: "auto",
            tune: None,
            package: "@preview/typed-scores",
            source_name: "",
            scale: "0.7",
        };
        for line in options.lines() {
            let Some((name, value)) = line.split_once('=') else {
                continue;
            };
            match name {
                "format" => request.format = value,
                "tune" if !value.is_empty() => request.tune = Some(value),
                "package" => request.package = value,
                "source" => request.source_name = value,
                "scale" => request.scale = value,
                _ => {}
            }
        }
        request
    }
}

fn detect_format(bytes: &[u8]) -> &'static str {
    let text = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
    let first = text.iter().find(|byte| !byte.is_ascii_whitespace());
    if bytes.starts_with(b"PK")
        || bytes.starts_with(&[0xFF, 0xFE])
        || bytes.starts_with(&[0xFE, 0xFF])
        || first == Some(&b'<')
    {
        "musicxml"
    } else {
        "abc"
    }
}

/// Import a score and return the JSON payload for the Typst side.
pub fn import(bytes: &[u8], options: &str) -> Result<String, String> {
    let request = Request::parse(options);
    let (emitted, numbers) = convert(bytes, &request)?;
    let source = emitted.source(
        request.package,
        request.source_name,
        request.scale,
        &numbers,
    );
    Ok(emitted.json(&source))
}

/// Read, normalize, and emit; also return the source measure numbers.
pub fn convert(bytes: &[u8], request: &Request<'_>) -> Result<(Emitted, Vec<String>), String> {
    if bytes.iter().all(u8::is_ascii_whitespace) {
        return Err("the import source is empty".into());
    }
    let format = match request.format {
        "auto" => detect_format(bytes),
        "musicxml" | "mxl" | "xml" => "musicxml",
        "abc" => "abc",
        other => {
            return Err(format!(
                "unsupported import format {other:?}; expected auto, musicxml, or abc"
            ))
        }
    };
    if format == "musicxml" && request.tune.is_some() {
        return Err(
            "tune selection applies only to ABC; remove the tune option for MusicXML".into(),
        );
    }
    let mut score = if format == "abc" {
        abc::read(bytes, request.tune)?
    } else {
        musicxml::read(bytes)?
    };
    normalize::normalize(&mut score)?;
    let numbers = score
        .measures
        .iter()
        .map(|measure| measure.number.clone())
        .collect();
    let emitted = emit::emit(&mut score)?;
    Ok((emitted, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_of(bytes: &[u8], name: &str) -> Result<String, String> {
        let request = Request {
            format: "auto",
            tune: None,
            package: "../../src/lib.typ",
            source_name: name,
            scale: "0.7",
        };
        let (emitted, numbers) = convert(bytes, &request)?;
        Ok(emitted.source(
            request.package,
            request.source_name,
            request.scale,
            &numbers,
        ))
    }

    fn without_first_line(text: &str) -> &str {
        text.split_once('\n').map(|(_, rest)| rest).unwrap_or(text)
    }

    #[test]
    fn hidden_musicxml_tuplet_note_stays_inside_its_tuplet() {
        let xml = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="1"><attributes><divisions>3</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type><time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification><notations><tuplet type="start"/></notations></note><note print-object="no"><pitch><step>D</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type><time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification></note><note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><type>eighth</type><time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification><notations><tuplet type="stop"/></notations></note><note><pitch><step>D</step><octave>5</octave></pitch><duration>3</duration><type>quarter</type></note><note><pitch><step>F</step><octave>5</octave></pitch><duration>3</duration><type>quarter</type></note></measure></part></score-partwise>"#;
        let source = source_of(xml.as_bytes(), "hidden-tuplet").unwrap();
        assert!(
            source.contains("tuplet 3:2 { C5:e s:e E5:e }"),
            "{source}"
        );
        assert!(!source.contains("tuplet 3:2[bracket=never"), "{source}");
    }

    #[test]
    fn fixtures_match_their_committed_typst() {
        for (name, source, expected) in [
            (
                "song-with-piano.musicxml",
                &include_bytes!("../../../tests/import/song-with-piano.musicxml")[..],
                include_str!("../../../tests/import/song-with-piano.typ"),
            ),
            (
                "export-quirks.musicxml",
                &include_bytes!("../../../tests/import/export-quirks.musicxml")[..],
                include_str!("../../../tests/import/export-quirks.typ"),
            ),
            (
                "folk-duet.abc",
                &include_bytes!("../../../tests/import/folk-duet.abc")[..],
                include_str!("../../../tests/import/folk-duet.typ"),
            ),
        ] {
            assert_eq!(
                source_of(source, name).unwrap(),
                expected,
                "{name} no longer imports to its committed .typ"
            );
        }
    }

    #[test]
    fn compressed_musicxml_imports_like_the_plain_score() {
        let compressed = source_of(
            include_bytes!("../../../tests/import/song-with-piano.mxl"),
            "x",
        )
        .unwrap();
        let plain = include_str!("../../../tests/import/song-with-piano.typ");
        assert_eq!(without_first_line(&compressed), without_first_line(plain));
    }

    #[test]
    fn timewise_musicxml_imports_like_partwise() {
        let partwise = r#"<score-partwise><part-list><score-part id="P1"><part-name>Flute</part-name></score-part></part-list>
            <part id="P1"><measure number="1"><attributes><divisions>1</divisions><time><beats>2</beats><beat-type>4</beat-type></time></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>2</duration><type>half</type></note></measure></part></score-partwise>"#;
        let timewise = r#"<score-timewise><part-list><score-part id="P1"><part-name>Flute</part-name></score-part></part-list>
            <measure number="1"><part id="P1"><attributes><divisions>1</divisions><time><beats>2</beats><beat-type>4</beat-type></time></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>2</duration><type>half</type></note></part></measure></score-timewise>"#;
        let expected = source_of(partwise.as_bytes(), "x").unwrap();
        assert_eq!(source_of(timewise.as_bytes(), "x").unwrap(), expected);
        assert!(expected.contains("flute: \"C5:h\""), "{expected}");
    }

    #[test]
    fn reports_unrepresentable_music() {
        let overfull = source_of(b"X:1\nM:3/4\nL:1/4\nK:C\nC D E F | G3 |]\n", "x").unwrap_err();
        assert!(
            overfull.contains("bar 1 holds 4/4 of music, more than its 3/4 meter allows"),
            "{overfull}"
        );
        let breve = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="7">
            <attributes><divisions>1</divisions></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>8</duration><type>breve</type></note></measure></part></score-partwise>"#;
        assert_eq!(
            source_of(breve.as_bytes(), "x").unwrap_err(),
            "bar 7: breve notes cannot be written in typed-scores"
        );
        assert!(source_of(b"<score-partwise><part>", "x")
            .unwrap_err()
            .contains("not well-formed XML"));
        assert!(source_of(b"X:1\nT:Only a title\n", "x")
            .unwrap_err()
            .contains("no music"));
    }

    #[test]
    fn selects_abc_tunes_by_number() {
        let book = b"X:1\nK:C\nC4|]\n\nX:7\nT:Seventh\nK:G\nG4|]\n";
        let request = Request {
            format: "abc",
            tune: Some("7"),
            package: "p",
            source_name: "",
            scale: "0.7",
        };
        let (emitted, _) = convert(book, &request).unwrap();
        assert_eq!(emitted.title.as_deref(), Some("Seventh"));
        let missing = Request {
            tune: Some("3"),
            ..request
        };
        assert_eq!(
            convert(book, &missing).err().as_deref(),
            Some("no tune with X:3")
        );
    }

    #[test]
    fn xml_reader_handles_prolog_entities_and_namespaces() {
        let text = "<?xml version=\"1.0\"?>\n<!DOCTYPE score-partwise PUBLIC \"-//x\" \"y.dtd\" [ <!ENTITY a \"b\"> ]>\n\
            <!-- comment --><m:root xmlns:m=\"u\" m:kind='x &amp; y'>A &lt;&#65;&#x42;&gt;<![CDATA[<raw>]]><child/></m:root>";
        let document = xml::parse(text).unwrap();
        let root = document.root();
        assert_eq!(root.name(), "root");
        assert_eq!(root.attribute("kind"), Some("x & y"));
        assert_eq!(root.text(), "A <AB><raw>");
        assert_eq!(
            root.elements().map(|node| node.name()).collect::<Vec<_>>(),
            ["child"]
        );
        assert!(xml::parse("<a><b></a>").is_err());
        assert!(xml::parse("<a></a><b/>").is_err());
    }
    #[test]
    fn abc_rejects_invalid_durations_and_meter_values() {
        for (field, music) in [
            ("L:1/0", "C D E F"),
            ("L:0/4", "C D E F"),
            ("L:oops", "C D E F"),
            ("M:4/0", "C D E F"),
            ("M:0/4", "C D E F"),
            ("M:4/4garbage", "C D E F"),
            ("L:1/4", "C/0 D E F"),
            ("L:1/4", "C0 D E F"),
            ("L:1/4", "(0 C D E F"),
            ("L:1/4", "(3:0:3 C D E F"),
            ("L:1/4", "(3:2:0 C D E F"),
            ("L:1/4", "C999999999999999999 D E F"),
        ] {
            let abc = format!("X:1\nM:4/4\nL:1/4\n{field}\nK:C\n{music}|]\n");
            assert!(source_of(abc.as_bytes(), "x").is_err(), "{abc}");
        }
        assert!(source_of(b"X:1\nK:C\n\xff", "x")
            .unwrap_err()
            .contains("UTF-8"));
    }

    #[test]
    fn musicxml_rejects_malformed_numeric_and_pitch_values() {
        for fragment in [
            "<attributes><divisions>0</divisions></attributes>",
            "<attributes><divisions>1.5</divisions></attributes>",
            "<attributes><time><beats>4</beats><beat-type>0</beat-type></time></attributes>",
            "<attributes><time><beats>4294967295+1</beats><beat-type>4</beat-type></time></attributes>",
            "<note><pitch><step>H</step><octave>5</octave></pitch><duration>1</duration></note>",
            "<note><pitch><step>C</step></pitch><duration>1</duration></note>",
            "<note><pitch><step>C</step><octave>5</octave></pitch><duration>-1</duration></note>",
            "<note><pitch><step>C</step><octave>5</octave></pitch><duration>bad</duration></note>",
            "<note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><dot/><dot/><dot/></note>",
        ] {
            let xml = format!("<score-partwise><part-list><score-part id=\"P1\"/></part-list><part id=\"P1\"><measure number=\"1\">{fragment}</measure></part></score-partwise>");
            assert!(source_of(xml.as_bytes(), "x").is_err(), "{fragment}");
        }
        assert!(source_of(&[0xff, 0xfe, 0x43], "x")
            .unwrap_err()
            .contains("UTF-16"));
    }

    #[test]
    fn imported_staff_names_avoid_all_score_metadata_fields() {
        assert_eq!(model::slug("Figures"), "figures-staff");
        assert_eq!(model::slug("Set"), "set-staff");
        assert_eq!(model::slug("If"), "if-staff");
    }

    #[test]
    fn abc_accidentals_carry_across_octaves_and_tied_notes_take_syllables() {
        let accidentals = source_of(b"X:1\nM:4/4\nL:1/4\nK:C\n^C c C c |]\n", "x").unwrap();
        assert!(accidentals.contains("C#4"), "{accidentals}");
        assert!(accidentals.contains("C#5"), "{accidentals}");

        let lyrics =
            source_of(b"X:1\nM:4/4\nL:1/4\nK:C\nc-c d e |\nw: la- la do re\n", "x").unwrap();
        assert!(lyrics.contains("lyrics: \"la -- la do re\""), "{lyrics}");
    }

    #[test]
    fn abc_grace_decorations_do_not_become_notes_and_unclosed_groups_error() {
        let source = source_of(b"X:1\nM:4/4\nL:1/4\nK:C\n{!accent!c}d e f g|]\n", "x").unwrap();
        assert!(source.contains("grace { C5:e } D5:q"), "{source}");
        assert!(source.contains("grace group was dropped"), "{source}");
        let chord = source_of(b"X:1\nM:4/4\nL:1/4\nK:C\n{[ce]}d e f g|]\n", "x").unwrap();
        assert!(chord.contains("grace { (C5 E5):e } D5:q"), "{chord}");
        assert!(source_of(b"X:1\nM:4/4\nL:1/4\nK:C\n[c d e\n", "x")
            .unwrap_err()
            .contains("missing its closing bracket"));
    }

    #[test]
    fn abc_chords_reject_swallowed_barlines_and_drop_decorations_with_warning() {
        assert!(source_of(b"K:C\n[CEG c d e |]\n", "x")
            .unwrap_err()
            .contains("barline before its closing bracket"));
        let decorated = source_of(b"K:C\n[!accent!CEG] |]", "x").unwrap();
        assert!(decorated.contains("(C4 E4 G4)"), "{decorated}");
        assert!(
            decorated.contains("chord decoration \"accent\" was dropped"),
            "{decorated}"
        );
    }

    #[test]
    fn abc_midmeasure_endings_warn_instead_of_emitting_invalid_voltas() {
        let source = source_of(b"K:C\nM:4/4\nL:1/4\nC D [1 E F :| [2 G A |]\n", "x").unwrap();
        assert!(
            source.contains("mid-measure ABC endings are not supported"),
            "{source}"
        );
        assert!(!source.contains("label: \"1.\""), "{source}");
    }

    #[test]
    fn unequal_musicxml_part_lengths_fill_missing_staff_slots() {
        let xml = r#"<score-partwise><part-list>
            <score-part id="P1"><part-name>Flute</part-name></score-part>
            <score-part id="P2"><part-name>Cello</part-name></score-part>
            </part-list><part id="P1">
            <measure number="1"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type></note></measure>
            <measure number="2"><note><pitch><step>D</step><octave>5</octave></pitch><duration>4</duration><type>whole</type></note></measure>
            </part><part id="P2"><measure number="1"><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>4</duration><type>whole</type></note></measure></part></score-partwise>"#;
        let source = source_of(xml.as_bytes(), "x").unwrap();
        assert!(source.contains("cello: \"C3:w\""), "{source}");
        assert!(source.contains("cello: \"_\""), "{source}");
    }

    #[test]
    fn rest_fermatas_are_preserved_by_both_importers() {
        let abc = source_of(b"X:1\nM:4/4\nL:1/4\nK:C\n!fermata!z C D E|]\n", "x").unwrap();
        assert!(abc.contains("r:q[fermata]"), "{abc}");
        let xml = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="1"><attributes><divisions>1</divisions></attributes><note><rest/><duration>4</duration><type>whole</type><notations><fermata/></notations></note></measure></part></score-partwise>"#;
        let musicxml = source_of(xml.as_bytes(), "x").unwrap();
        assert!(musicxml.contains("r:w[fermata]"), "{musicxml}");
    }

    #[test]
    fn abc_bass_key_token_is_a_clef_and_body_fields_are_not_music() {
        let source = source_of(b"X:1\nM:4/4\nL:1/4\nK:bass\nC D E F|]\n", "x").unwrap();
        assert!(source.contains("clef: \"bass\""), "{source}");
        assert!(source.contains("key: \"C\""), "{source}");

        let source = source_of(
            b"X:1\nM:4/4\nL:1/4\nK:C\nC D E F|\nH:History\nG A B c|]\n",
            "x",
        )
        .unwrap();
        assert!(!source.contains("segno"), "{source}");
        assert!(!source.contains("fermata"), "{source}");
        assert!(source.contains("G4:q A4 B4 C5"), "{source}");

        let blank_line =
            source_of(b"X:1\nM:4/4\nL:1/4\nK:C\nC D E F|\n\nG A B c|]\n", "x").unwrap();
        assert!(blank_line.contains("C4:q D4 E4 F4"), "{blank_line}");
        assert!(!blank_line.contains("G4:q A4 B4 C5"), "{blank_line}");

        let comment_line = source_of(
            b"X:1\nM:4/4\nL:1/4\nK:C\nC D E F|\n% comment\nG A B c|]\n",
            "x",
        )
        .unwrap();
        assert!(comment_line.contains("G4:q A4 B4 C5"), "{comment_line}");
    }

    #[test]
    fn abc_dropped_tempo_and_octave_up_clef_are_reported() {
        let tempo = source_of(b"X:1\nM:4/4\nL:1/4\nQ:3/8=60\nK:C\nC D E F|]\n", "x").unwrap();
        assert!(
            tempo.contains("tempo \"3/8=60\" is not supported"),
            "{tempo}"
        );
        let clef = source_of(b"X:1\nK:C clef=treble+8\nC D E F|]\n", "x").unwrap();
        assert!(
            clef.contains("clef \"clef=treble+8\" is not supported"),
            "{clef}"
        );
        let additive = source_of(b"X:1\nM:2+3/8\nL:1/8\nK:C\nC D E F G|]\n", "x").unwrap();
        assert!(additive.contains("time: \"5/8\""), "{additive}");
        assert!(
            additive.contains("additive meter grouping is not retained"),
            "{additive}"
        );
    }

    #[test]
    fn abc_tuplet_ratio_continues_across_a_barline() {
        let source = source_of(b"X:1\nM:4/4\nL:1/4\nK:C\n(3C D | E F G |]\n", "x").unwrap();
        let measures: Vec<_> = source.split("// m.").collect();
        assert!(measures[1].contains("tuplet 3:2"), "{source}");
        assert!(measures[2].contains("tuplet 3:2"), "{source}");
    }

    #[test]
    fn abc_lyric_extenders_end_at_their_explicit_target_note() {
        let source = source_of(
            b"X:1\nM:4/4\nL:1/4\nK:C\nc c c c | d d d d |]\nw: Ho _ * ly * *\n",
            "x",
        )
        .unwrap();
        assert!(source.contains("lyrics: \"Ho __ _ ly\""), "{source}");
        assert_eq!(source.matches("lyrics:").count(), 1, "{source}");
    }

    #[test]
    fn abc_multibar_rests_keep_dynamics_and_chord_positions() {
        let source = source_of(
            b"X:1\nM:4/4\nL:1/4\nK:C\n!f!Z2 | \"Am\"Z2 | c d e f|]\n",
            "x",
        )
        .unwrap();
        assert!(source.contains("notes: \"r:w[dyn=f]\""), "{source}");
        let bar_three = source.split("// m. 3").nth(1).unwrap();
        assert!(bar_three.contains("harmony: \"Am:w\""), "{source}");
    }

    #[test]
    fn importer_rejects_undeclared_xml_entities_and_zero_tune_numbers() {
        let xml = r#"<score-partwise><part-list><score-part id="P1"><part-name>A &custom; B</part-name></score-part></part-list><part id="P1"><measure number="1"/></part></score-partwise>"#;
        assert!(source_of(xml.as_bytes(), "x")
            .unwrap_err()
            .contains("entity &custom; is not supported"));
        let request = Request {
            format: "abc",
            tune: Some("0"),
            package: "p",
            source_name: "",
            scale: "0.7",
        };
        assert_eq!(
            convert(b"X:1\nK:C\nC4|]\n", &request).err().unwrap(),
            "ABC tune number must be a positive integer"
        );
    }

    #[test]
    fn musicxml_reports_multiple_rest_and_transposition_metadata() {
        let xml = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="1">
            <attributes><divisions>1</divisions><measure-style><multiple-rest>3</multiple-rest></measure-style><transpose><diatonic>-1</diatonic><chromatic>-2</chromatic></transpose></attributes>
            <note><rest/><duration>4</duration><type>whole</type></note></measure></part></score-partwise>"#;
        let source = source_of(xml.as_bytes(), "x").unwrap();
        assert!(
            source.contains("multiple-measure rests are not expanded"),
            "{source}"
        );
        assert!(
            source.contains("instrument transposition metadata is not represented"),
            "{source}"
        );
    }

    #[test]
    fn musicxml_orders_numeric_lyric_verses_and_warns_for_unknown_marks() {
        let xml = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="1"><attributes><divisions>1</divisions></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type>
            <lyric number="2"><text>second</text></lyric><lyric number="1"><text>first</text></lyric>
            <notations><articulations><stress/></articulations><ornaments><shake/></ornaments><technical><up-bow/><fingering>one</fingering></technical></notations>
            </note></measure></part></score-partwise>"#;
        let source = source_of(xml.as_bytes(), "x").unwrap();
        assert!(
            source.contains("lyrics: (\"first\", \"second\")"),
            "{source}"
        );
        assert!(
            source.contains("articulation stress was dropped"),
            "{source}"
        );
        assert!(source.contains("ornament shake was dropped"), "{source}");
        assert!(source.contains("up-bow marks were dropped"), "{source}");
        assert!(
            source.contains("non-numeric MusicXML fingering was dropped"),
            "{source}"
        );
    }

    #[test]
    fn musicxml_warns_for_unsupported_single_note_tremolo_strokes() {
        let xml = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="1"><attributes><divisions>1</divisions></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><type>whole</type><notations><ornaments><tremolo type="single">5</tremolo></ornaments></notations></note>
            </measure></part></score-partwise>"#;
        let source = source_of(xml.as_bytes(), "x").unwrap();
        assert!(
            source.contains("single-note tremolo with 5 strokes was dropped"),
            "{source}"
        );
    }

    #[test]
    fn musicxml_warns_for_above_text_hidden_chord_heads_and_midbar_keys() {
        let xml = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="1">
            <attributes><divisions>1</divisions><time><beats>2+3</beats><beat-type>8</beat-type></time></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note>
            <attributes><key><fifths>1</fifths></key></attributes>
            <direction placement="above"><direction-type><words>rit.</words></direction-type></direction>
            <note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note>
            <note print-object="no"><chord/><pitch><step>G</step><octave>5</octave></pitch><duration>1</duration><type>quarter</type></note>
            </measure></part></score-partwise>"#;
        let source = source_of(xml.as_bytes(), "x").unwrap();
        assert!(
            source.contains("mid-measure MusicXML key changes"),
            "{source}"
        );
        assert!(
            source.contains("above-staff MusicXML direction text"),
            "{source}"
        );
        assert!(
            source.contains("invisible MusicXML chord members"),
            "{source}"
        );
        assert!(
            source.contains("additive meter grouping is not retained"),
            "{source}"
        );
        assert!(!source.contains("G5"), "{source}");
    }

    #[test]
    fn imported_staff_names_avoid_every_reserved_bar_field() {
        for name in [
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
        ] {
            assert_eq!(model::slug(name), format!("{name}-staff"), "{name}");
        }
    }
}
