use std::{fs, path::PathBuf};

use walkdir::WalkDir;

struct Document {
    name: String,
    tags: Vec<String>,
    outgoing_links: Vec<String>,
    incoming_links: Vec<String>,

    blocks: Vec<Node>,
}

enum Node {
    BlockRegular {
        kind: BlockRegular,
        body: Box<Node>,
    },
    BlockNestable {
        kind: BlockNestable,
        body: Box<Node>,
    },
    Inline {
        kind: Inline,
        body: String,
    },
}

enum BlockRegular {
    Heading1,
    Heading2,
    Paragraph,
    Blockquote,
}

enum BlockNestable {
    UnorderedList,
    OrderedList,
    TaskList,
}

enum Inline {
    Text,
    Emphasis,
    Strike,
    Code,
    Tag,
    Link,
    Image,
}

fn main() {
    let files: Vec<Document> = list_files("markdown".into())
        .map(parse_file)
        .flatten()
        .collect();

    todo!();
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
    let blocks: Vec<Node> = parse_nodes(&content);

    Some(Document {
        name,
        blocks,
        tags: Vec::new(),
        outgoing_links: Vec::new(),
        incoming_links: Vec::new(),
    })
}

fn parse_nodes(text: &str) -> Vec<Node> {
    let mut stack = Vec::new();
    let mut blocks = Vec::new();

    struct Start {
        node: Block,
        children: Node,
        idx: usize,
    }

    enum Block {
        Regular(BlockRegular),
        Nestable(BlockNestable),
    }

    for (idx, line) in text.lines().enumerate() {
        let start = if line.starts_with("# ") {
            Some(Start {
                node: Block::Regular(BlockRegular::Heading1),
                idx,
            })
        } else if line.starts_with("## ") {
            Some(Start {
                node: Block::Regular(BlockRegular::Heading2),
                idx,
            })
        } else if line.starts_with("> ") {
            Some(Start {
                node: Block::Regular(BlockRegular::Blockquote),
                idx,
            })
        } else {
            None
        };

        if let Some(start) = start {
            stack.push(start);
            continue;
        }

        if line.is_empty() {
            let start = match stack.pop() {
                Some(data) => data,
                None => continue,
            };

            let node = match start.node {
                Block::Regular(kind) => Node::BlockRegular {
                    kind,
                    body: text[start.idx..idx],
                },
                _ => todo!(),
            };

            blocks.push(node);
        }
    }

    todo!();
}
