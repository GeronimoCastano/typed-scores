//! A small XML tree reader, sufficient for MusicXML.
//!
//! It keeps elements, attributes, and text; decodes the predefined and
//! numeric entities and CDATA; and skips the prolog, comments, processing
//! instructions, and the DOCTYPE. Namespace prefixes are dropped from names.

use super::model::ImportResult;

struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Content>,
}

enum Content {
    Element(usize),
    Text(String),
}

pub struct Document {
    elements: Vec<Element>,
}

#[derive(Clone, Copy)]
pub struct Node<'a> {
    document: &'a Document,
    index: usize,
}

fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

fn decode(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }
    let mut decoded = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        decoded.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find(';').filter(|end| *end <= 10) else {
            decoded.push('&');
            rest = after;
            continue;
        };
        let entity = &after[..end];
        let character = match entity {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .or_else(|| {
                    entity
                        .strip_prefix('#')
                        .and_then(|digits| digits.parse().ok())
                })
                .and_then(char::from_u32),
        };
        match character {
            Some(character) => {
                decoded.push(character);
                rest = &after[end + 1..];
            }
            None => {
                decoded.push('&');
                rest = after;
            }
        }
    }
    decoded.push_str(rest);
    decoded
}

fn error(message: &str) -> String {
    format!("the MusicXML file is not well-formed XML: {message}")
}

pub fn parse(text: &str) -> ImportResult<Document> {
    let mut document = Document {
        elements: Vec::new(),
    };
    let mut stack: Vec<usize> = Vec::new();
    let mut root = None;
    let mut rest = text;
    while !rest.is_empty() {
        let Some(open) = rest.find('<') else {
            if !stack.is_empty() {
                return Err(error("the document ends inside an element"));
            }
            break;
        };
        if open > 0 {
            if let Some(parent) = stack.last() {
                document.elements[*parent]
                    .children
                    .push(Content::Text(decode(&rest[..open])));
            }
        }
        rest = &rest[open..];
        if let Some(after) = rest.strip_prefix("<!--") {
            let end = after
                .find("-->")
                .ok_or_else(|| error("a comment is not closed"))?;
            rest = &after[end + 3..];
        } else if let Some(after) = rest.strip_prefix("<![CDATA[") {
            let end = after
                .find("]]>")
                .ok_or_else(|| error("a CDATA section is not closed"))?;
            if let Some(parent) = stack.last() {
                document.elements[*parent]
                    .children
                    .push(Content::Text(after[..end].to_string()));
            }
            rest = &after[end + 3..];
        } else if rest.starts_with("<?") {
            let end = rest
                .find("?>")
                .ok_or_else(|| error("a processing instruction is not closed"))?;
            rest = &rest[end + 2..];
        } else if rest.starts_with("<!") {
            // DOCTYPE, possibly with an internal subset in brackets.
            let bracket = rest.find('[');
            let close = rest
                .find('>')
                .ok_or_else(|| error("a declaration is not closed"))?;
            let end = match bracket {
                Some(bracket) if bracket < close => {
                    let subset_end = rest[bracket..]
                        .find("]")
                        .map(|offset| bracket + offset)
                        .ok_or_else(|| error("a DOCTYPE subset is not closed"))?;
                    rest[subset_end..]
                        .find('>')
                        .map(|offset| subset_end + offset)
                        .ok_or_else(|| error("a declaration is not closed"))?
                }
                _ => close,
            };
            rest = &rest[end + 1..];
        } else if let Some(after) = rest.strip_prefix("</") {
            let end = after
                .find('>')
                .ok_or_else(|| error("a closing tag is not finished"))?;
            let name = local_name(after[..end].trim());
            let open = stack
                .pop()
                .ok_or_else(|| error(&format!("</{name}> closes nothing")))?;
            if document.elements[open].name != name {
                return Err(error(&format!(
                    "<{}> is closed by </{name}>",
                    document.elements[open].name
                )));
            }
            rest = &after[end + 1..];
        } else {
            let after = &rest[1..];
            let name_end = after
                .find(|c: char| c.is_ascii_whitespace() || c == '/' || c == '>')
                .ok_or_else(|| error("a tag is not finished"))?;
            let name = local_name(&after[..name_end]).to_string();
            if name.is_empty() {
                return Err(error("a tag has no name"));
            }
            let mut attributes = Vec::new();
            let mut position = &after[name_end..];
            let self_closing;
            loop {
                position = position.trim_start_matches(|c: char| c.is_ascii_whitespace());
                if let Some(next) = position.strip_prefix("/>") {
                    self_closing = true;
                    position = next;
                    break;
                }
                if let Some(next) = position.strip_prefix('>') {
                    self_closing = false;
                    position = next;
                    break;
                }
                let equals = position
                    .find('=')
                    .ok_or_else(|| error(&format!("an attribute of <{name}> has no value")))?;
                let key = local_name(position[..equals].trim()).to_string();
                let value_start =
                    position[equals + 1..].trim_start_matches(|c: char| c.is_ascii_whitespace());
                let quote = value_start
                    .chars()
                    .next()
                    .filter(|c| *c == '"' || *c == '\'')
                    .ok_or_else(|| error(&format!("attribute {key} is not quoted")))?;
                let value_end = value_start[1..]
                    .find(quote)
                    .ok_or_else(|| error(&format!("attribute {key} is not closed")))?;
                attributes.push((key, decode(&value_start[1..1 + value_end])));
                position = &value_start[value_end + 2..];
            }
            document.elements.push(Element {
                name,
                attributes,
                children: Vec::new(),
            });
            let index = document.elements.len() - 1;
            match stack.last() {
                Some(parent) => document.elements[*parent]
                    .children
                    .push(Content::Element(index)),
                None if root.is_none() => root = Some(index),
                None => return Err(error("the document has more than one root element")),
            }
            if !self_closing {
                stack.push(index);
            }
            rest = position;
        }
    }
    if !stack.is_empty() {
        return Err(error("the document ends inside an element"));
    }
    match root {
        Some(0) => Ok(document),
        _ => Err(error("the document has no root element")),
    }
}

impl Document {
    pub fn root(&self) -> Node<'_> {
        Node {
            document: self,
            index: 0,
        }
    }
}

impl<'a> Node<'a> {
    fn element(&self) -> &'a Element {
        &self.document.elements[self.index]
    }

    pub fn name(&self) -> &'a str {
        &self.element().name
    }

    pub fn attribute(&self, name: &str) -> Option<&'a str> {
        self.element()
            .attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    /// Child elements, in document order.
    pub fn elements(self) -> impl Iterator<Item = Node<'a>> {
        let document = self.document;
        self.element()
            .children
            .iter()
            .filter_map(move |child| match child {
                Content::Element(index) => Some(Node {
                    document,
                    index: *index,
                }),
                Content::Text(_) => None,
            })
    }

    /// The element's own text, excluding text inside child elements.
    pub fn text(&self) -> String {
        self.element()
            .children
            .iter()
            .filter_map(|child| match child {
                Content::Text(text) => Some(text.as_str()),
                Content::Element(_) => None,
            })
            .collect()
    }

    /// This element and every element inside it, in document order.
    pub fn descendants(self) -> Vec<Node<'a>> {
        let mut found = vec![self];
        let mut position = 0;
        while position < found.len() {
            let children: Vec<Node<'a>> = found[position].elements().collect();
            found.splice(position + 1..position + 1, children);
            position += 1;
        }
        found
    }
}
