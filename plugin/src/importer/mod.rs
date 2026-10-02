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
            let Some((name, value)) = line.split_once('=') else { continue };
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
    if bytes.starts_with(b"PK") || bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) || first == Some(&b'<') {
        "musicxml"
    } else {
        "abc"
    }
}

/// Import a score and return the JSON payload for the Typst side.
pub fn import(bytes: &[u8], options: &str) -> Result<String, String> {
    let request = Request::parse(options);
    let (emitted, numbers) = convert(bytes, &request)?;
    let source = emitted.source(request.package, request.source_name, request.scale, &numbers);
    Ok(emitted.json(&source))
}

/// Read, normalize, and emit; also return the source measure numbers.
pub fn convert(bytes: &[u8], request: &Request<'_>) -> Result<(Emitted, Vec<String>), String> {
    let format = match request.format {
        "auto" => detect_format(bytes),
        "musicxml" | "mxl" | "xml" => "musicxml",
        "abc" => "abc",
        other => return Err(format!("unsupported import format {other:?}; expected auto, musicxml, or abc")),
    };
    let mut score = if format == "abc" { abc::read(bytes, request.tune)? } else { musicxml::read(bytes)? };
    normalize::normalize(&mut score)?;
    let numbers = score.measures.iter().map(|measure| measure.number.clone()).collect();
    let emitted = emit::emit(&mut score)?;
    Ok((emitted, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_of(bytes: &[u8], name: &str) -> Result<String, String> {
        let request = Request { format: "auto", tune: None, package: "../../src/lib.typ", source_name: name, scale: "0.7" };
        let (emitted, numbers) = convert(bytes, &request)?;
        Ok(emitted.source(request.package, request.source_name, request.scale, &numbers))
    }

    fn without_first_line(text: &str) -> &str {
        text.split_once('\n').map(|(_, rest)| rest).unwrap_or(text)
    }

    #[test]
    fn fixtures_match_their_committed_typst() {
        for (name, source, expected) in [
            ("song-with-piano.musicxml", &include_bytes!("../../../tests/import/song-with-piano.musicxml")[..], include_str!("../../../tests/import/song-with-piano.typ")),
            ("export-quirks.musicxml", &include_bytes!("../../../tests/import/export-quirks.musicxml")[..], include_str!("../../../tests/import/export-quirks.typ")),
            ("folk-duet.abc", &include_bytes!("../../../tests/import/folk-duet.abc")[..], include_str!("../../../tests/import/folk-duet.typ")),
        ] {
            assert_eq!(source_of(source, name).unwrap(), expected, "{name} no longer imports to its committed .typ");
        }
    }

    #[test]
    fn compressed_musicxml_imports_like_the_plain_score() {
        let compressed = source_of(include_bytes!("../../../tests/import/song-with-piano.mxl"), "x").unwrap();
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
        assert!(overfull.contains("bar 1 holds 4/4 of music, more than its 3/4 meter allows"), "{overfull}");
        let breve = r#"<score-partwise><part-list><score-part id="P1"/></part-list><part id="P1"><measure number="7">
            <attributes><divisions>1</divisions></attributes>
            <note><pitch><step>C</step><octave>5</octave></pitch><duration>8</duration><type>breve</type></note></measure></part></score-partwise>"#;
        assert_eq!(source_of(breve.as_bytes(), "x").unwrap_err(), "bar 7: breve notes cannot be written in typed-scores");
        assert!(source_of(b"<score-partwise><part>", "x").unwrap_err().contains("not well-formed XML"));
        assert!(source_of(b"X:1\nT:Only a title\n", "x").unwrap_err().contains("no music"));
    }

    #[test]
    fn selects_abc_tunes_by_number() {
        let book = b"X:1\nK:C\nC4|]\n\nX:7\nT:Seventh\nK:G\nG4|]\n";
        let request = Request { format: "abc", tune: Some("7"), package: "p", source_name: "", scale: "0.7" };
        let (emitted, _) = convert(book, &request).unwrap();
        assert_eq!(emitted.title.as_deref(), Some("Seventh"));
        let missing = Request { tune: Some("3"), ..request };
        assert_eq!(convert(book, &missing).err().as_deref(), Some("no tune with X:3"));
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
        assert_eq!(root.elements().map(|node| node.name()).collect::<Vec<_>>(), ["child"]);
        assert!(xml::parse("<a><b></a>").is_err());
        assert!(xml::parse("<a></a><b/>").is_err());
    }
}
