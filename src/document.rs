use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::path::PathBuf;

pub struct Document {
    pub path: PathBuf,
    pub title: Option<String>,
    pub headings: Vec<String>,
    pub body: String,
}

pub fn parse_markdown(path: PathBuf, content: &str) -> Document {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let mut title = None;
    let mut headings = Vec::new();
    let mut body = String::new();
    let mut heading: Option<String> = None;
    let mut metadata: Option<String> = None;

    for event in Parser::new_ext(content, options) {
        match event {
            Event::Start(Tag::MetadataBlock(_)) => metadata = Some(String::new()),
            Event::End(TagEnd::MetadataBlock(_)) => {
                title = metadata.take().and_then(|m| parse_title(&m));
            }
            Event::Start(Tag::Heading { .. }) => heading = Some(String::new()),
            Event::End(TagEnd::Heading(..)) => {
                if let Some(h) = heading.take() {
                    headings.push(h.trim().to_string());
                }
            }
            Event::Text(t) | Event::Code(t) => {
                if let Some(m) = metadata.as_mut() {
                    m.push_str(&t);
                } else {
                    heading.as_mut().unwrap_or(&mut body).push_str(&t);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                heading.as_mut().unwrap_or(&mut body).push(' ');
            }
            Event::End(
                TagEnd::Paragraph | TagEnd::Item | TagEnd::CodeBlock | TagEnd::TableCell,
            ) => body.push(' '),
            _ => {}
        }
    }

    Document {
        path,
        title,
        headings,
        body,
    }
}

fn parse_title(metadata: &str) -> Option<String> {
    metadata.lines().find_map(|line| {
        let t = line.strip_prefix("title:")?.trim();
        Some(t.trim_matches(|c| c == '"' || c == '\'').to_string())
    })
}
