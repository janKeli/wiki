use std::{fs, path::PathBuf};

use walkdir::WalkDir;

#[derive(Debug)]
struct Document {
    name: String,
    tags: Vec<String>,
    outgoing_links: Vec<String>,
    incoming_links: Vec<String>,

    blocks: Vec<Block>,
}

#[derive(Debug)]
enum Block {
    Heading1(Vec<Inline>),
    Heading2(Vec<Inline>),
    Heading3(Vec<Inline>),
    Paragraph(Vec<Inline>),

    Blockquote(Vec<Inline>),
    UnorderedList(Vec<Block>),
    OrderedList(Vec<Block>),
    TaskList(Vec<Task>),

    Code(String),

    HorizontalRule,

    Table {
        head: Vec<Inline>,
        lines: Vec<Vec<Inline>>,
    },
}

#[derive(Debug)]
struct Task {
    text: Block,
    done: bool,
}

#[derive(Debug)]
enum Inline {
    Text(String),
    Italic(String),
    Bold(String),
    Strikethrough(String),
    Code(String),
    Tag(String),

    RawHTML(String),

    Link { url: String, text: Vec<Inline> },
    Image { url: String, text: Vec<Inline> },
}

fn main() {
    let files: Vec<Document> = list_files("markdown".into())
        .map(parse_file)
        .flatten()
        .collect();

    dbg!(files);
}

fn list_files(directory: PathBuf) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(directory)
        .into_iter()
        .flatten()
        .map(|entry| entry.path().into())
}

fn parse_file(path: PathBuf) -> Option<Document> {
    let name = path.file_name()?.to_str()?.to_string();
    if name.split('.').last() != Some("md") {
        return None;
    }

    let content = fs::read_to_string(&path).ok()?;

    let blocks: Vec<Block> = content.split("\n\n").into_iter().map(parse_block).collect();

    Some(Document {
        name,
        blocks,
        tags: Vec::new(),
        outgoing_links: Vec::new(),
        incoming_links: Vec::new(),
    })
}

fn parse_block(text: &str) -> Block {
    if text.starts_with("# ") {
        Block::Heading1(parse_inline(&text[2..]))
    } else if text.starts_with("## ") {
        Block::Heading2(parse_inline(&text[3..]))
    } else if text.starts_with("### ") {
        Block::Heading3(parse_inline(&text[4..]))
    } else if text.starts_with("> ") {
        Block::Blockquote(parse_inline(&text[2..]))
    } else if text.starts_with("- [ ] ") || text.starts_with("- [x] ") {
        Block::TaskList(
            text.lines()
                .into_iter()
                .map(|line| Task {
                    text: parse_block(&line[6..]),
                    done: line.as_bytes()[3] == b'x',
                })
                .collect(),
        )
    } else if text.starts_with("- ") {
        Block::UnorderedList(
            text.lines()
                .into_iter()
                .map(|line| &line[2..])
                .map(parse_block)
                .collect(),
        )
    } else if text.starts_with("1. ") || text.starts_with("1) ") {
        Block::OrderedList(
            text.lines()
                .into_iter()
                .map(|line| &line[2..])
                .map(parse_block)
                .collect(),
        )
    } else if text.starts_with("```") {
        Block::Code(
            text.lines()
                .filter(|line| !line.starts_with("```"))
                .map(|line| format!("{}\n", line))
                .collect(),
        )
    } else {
        Block::Paragraph(parse_inline(text))
    }
}

fn parse_inline(text: &str) -> Vec<Inline> {
    return vec![Inline::Text(String::from(text))];
}
