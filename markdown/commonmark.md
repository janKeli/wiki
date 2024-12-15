# Commonmark

Commonmark is the most widely supported markdown spec.

An empty line is a break between paragraphs (or blocks).

_For now, I will consider blocks to have an empty line
between them always, even though titles and lists
don't have to have them._

## Level 2 heading

1. this
2. is
3. an
4. ordered
5. list

- and
- this
- is
- an
* unordered
+ list

This is _italic type A_ and *italic type B*,
**bold type A** and __bold type B__,
`inline code` and [a link](https://commonmark.org/help).

This is an image ![https://upload.wikimedia.org/wikipedia/commons/4/48/Markdown-mark.svg]
from [wikimedia commons](https://commons.wikimedia.org/wiki/File:Markdown-mark.svg).

> Some quote
  by someone

> Another quote
> by someone

---

that was the first horizontal rule

***

and that the second one

```rust
fn main() {
  println!("here is a proper code block");
}
```

And there should be a tab-style codeblock
but I don't like it and will never use it.


