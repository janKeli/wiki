# The wiki engine

The engine
- gets the list of all markdown files in a directory `fn listFiles(dir)`
- reads them
- splits on empty lines, treats each item as a block `split`
- parses them into an AST `fn parseBlock(text: &str) -> BlockNode` should eventually be `const fn`
  - determines the block type
  - finds the content
  - creates a vector of inline nodes from the content `fn parseInlines(text: &str) -> Vec<InlineNode>`
  - wraps the inline nodes and the type into a block
- saves the result in a vector
- generates the links between files
- generates the tag structure
- wraps the result of all this in `Document`s
- writes them to the dist directory
- gets the full list of tags plus the file names and creates an `index.json`
- generates feeds for `#blog`

## Supported markdown syntax

- [Commonmark](https://commonmark.org/help)
- GFM's [tables](https://github.github.com/gfm/#tables-extension-)
- GFM's [tasklists](https://github.github.com/gfm/#task-list-items-extension-)
- GFM's [strikethrough](https://github.github.com/gfm/#strikethrough-extension-)
- GFM-style [disallowed tags](https://github.github.com/gfm/#disallowed-raw-html-extension-)
- Obsidian-style wikilinks

## Structures

```rust
struct Document {
    name: String,
    tags: Vec<String>,
    outgoingLinks: Vec<String>,
    incomingLinks: Vec<String>,

    blocks: Vec<Node>,
}

enum BlockNode {
    Heading1(Vec<InlineNode>),
    Heading2(Vec<InlineNode>),
    Heading3(Vec<InlineNode>),
    Heading4(Vec<InlineNode>),
    Heading5(Vec<InlineNode>),
    Heading6(Vec<InlineNode>),
    Paragraph(Vec<InlineNode>),

    Blockquote(Vec<BlockNode>),
    UnorderedList(Vec<BlockNode>),
    OrderedList(Vec<BlockNode>),
    TaskList(Vec<TaskListItem>),

    Code(String),

    HorizontalRule,

    Table {
        head: Vec<InlineNode>,
        lines: Vec<Vec<InlineNode>>,
    },
}

struct TaskListItem {
    text: Vec<InlineNode>,
    done: bool,
}

enum InlineNode {
    Text(String),
    Italic(String),
    Bold(String),
    Strikethrough(String),
    Code(String),

    RawHTML(String),

    Link {
        url: String,
        text: Vec<InlineNode>,
    },
    Image {
        url: String,
        text: Vec<InlineNode>,
    },
}
```
