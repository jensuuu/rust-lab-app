use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

const BOOK_JSON: &str = include_str!("../data/book.json");
const SNIPPET_RADIUS: usize = 70;
const MIN_QUERY_LENGTH: usize = 2;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Paragraph { text: String },
    Heading { level: u8, text: String },
    Item { text: String },
    Caption { text: String },
    Code { code: String },
    Figure,
}

#[derive(Deserialize, Serialize)]
pub struct Section {
    pub id: usize,
    pub title: String,
    pub blocks: Vec<Block>,
}

#[derive(Deserialize)]
struct Chapter {
    id: usize,
    number: Option<u32>,
    title: String,
    sections: Vec<Section>,
}

#[derive(Serialize)]
pub struct SectionSummary {
    id: usize,
    title: String,
}

#[derive(Serialize)]
pub struct ChapterSummary {
    id: usize,
    number: Option<u32>,
    title: String,
    sections: Vec<SectionSummary>,
}

#[derive(Serialize)]
pub struct SearchHit {
    chapter_id: usize,
    section_id: usize,
    chapter_title: String,
    section_title: String,
    snippet: String,
}

pub fn outline() -> Vec<ChapterSummary> {
    chapters()
        .iter()
        .map(|chapter| ChapterSummary {
            id: chapter.id,
            number: chapter.number,
            title: chapter.title.clone(),
            sections: chapter
                .sections
                .iter()
                .map(|section| SectionSummary {
                    id: section.id,
                    title: section.title.clone(),
                })
                .collect(),
        })
        .collect()
}

pub fn section(chapter_id: usize, section_id: usize) -> Option<&'static Section> {
    chapters()
        .get(chapter_id)?
        .sections
        .get(section_id)
}

pub fn search(query: &str, limit: usize) -> Vec<SearchHit> {
    let lowered = query.trim().to_ascii_lowercase();
    let needle = lowered.as_str();

    if needle.len() < MIN_QUERY_LENGTH {
        return Vec::new();
    }

    chapters()
        .iter()
        .flat_map(|chapter| {
            chapter
                .sections
                .iter()
                .filter_map(move |section| find_in_section(chapter, section, needle))
        })
        .take(limit)
        .collect()
}

fn chapters() -> &'static [Chapter] {
    static BOOK: OnceLock<Vec<Chapter>> = OnceLock::new();

    BOOK.get_or_init(|| {
        serde_json::from_str(BOOK_JSON).expect("the embedded book data is valid JSON")
    })
}

fn find_in_section(chapter: &Chapter, section: &Section, needle: &str) -> Option<SearchHit> {
    let snippet = title_match(section, needle).or_else(|| block_match(section, needle))?;

    Some(SearchHit {
        chapter_id: chapter.id,
        section_id: section.id,
        chapter_title: chapter.title.clone(),
        section_title: section.title.clone(),
        snippet,
    })
}

fn title_match(section: &Section, needle: &str) -> Option<String> {
    section
        .title
        .to_ascii_lowercase()
        .contains(needle)
        .then(|| section.title.clone())
}

fn block_match(section: &Section, needle: &str) -> Option<String> {
    section
        .blocks
        .iter()
        .filter_map(searchable_text)
        .find_map(|text| snippet_around(text, needle))
}

fn searchable_text(block: &Block) -> Option<&str> {
    match block {
        Block::Paragraph { text }
        | Block::Heading { text, .. }
        | Block::Item { text }
        | Block::Caption { text } => Some(text),
        Block::Code { .. } | Block::Figure => None,
    }
}

fn snippet_around(text: &str, needle: &str) -> Option<String> {
    let position = text.to_ascii_lowercase().find(needle)?;
    let start = floor_boundary(text, position.saturating_sub(SNIPPET_RADIUS));
    let end = ceil_boundary(text, position + needle.len() + SNIPPET_RADIUS);

    let prefix = if start > 0 { "…" } else { "" };
    let suffix = if end < text.len() { "…" } else { "" };
    Some(format!("{prefix}{}{suffix}", &text[start..end]))
}

fn floor_boundary(text: &str, mut index: usize) -> usize {
    while !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_boundary(text: &str, index: usize) -> usize {
    let mut index = index.min(text.len());
    while !text.is_char_boundary(index) {
        index += 1;
    }
    index
}
