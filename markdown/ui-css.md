# ui.css
This is a very minimal semantic UI library for my projects.

You start by writing or generating an HTML file with
the following structure:
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="color-scheme" content="light dark">
    <link rel="stylesheet" href="/ui.css">
    <title>app name</title>
  </head>
  <body class="desktop">
    <main>
    </main>
    <aside>
    </aside>
  </body>
</html>
```

The main section is where your main UI, like article text,
image/video preview, description, etc. goes. Things here are
*mostly* non-interactive.

The aside can only contain `card`s and `button-group`s and
is used for elements that control the app. For instance,
the wiki will have a search bar and incoming & outgoing
links for the current file.

## Primitives
*Typography elements* are up to 80 characters wide, then they
wrap. These are everything that you can write in markdown,
i.e., headings, blockquotes, paragraphs, lists, code blocks,
emphasized text, and tables.

*Cards* are small blocks that have typography elements at the top
and two inputs below them. The inputs are wrapped in a div
or a form. That div is assigned a class, the either `.golden` ratio
to split 1:1.618, `.golden-inverted` to flip that, or `.even` to split
1:1. There are three kinds of elements that can be inputs:
- `input`s
- `a`nchors
- `button`s

```html
<div class="card">
  <h2>Card title</h2>
  <p>Card text</p>

  <div class="golden">
    <a href="#">inverted</a>
    <button>primary</button>
  </div>
</div>
```

The button style can be overwritten using the `.primary` and `inverted`
utility classes.

*Input groups* are essentially just cards without the typograhy part.

## Adaptive UI
Add the `.desktop` class to the `main` for the big devices,

or do the following for small ones:
```html
<body>
  <main>
    (main content)
  </main>

  <aside class="hidden">
    (...)
  </aside>

  <div class="fixed-to-bottom input-group golden">
    <button>secondary</button>
    <button>menu</button>
  </div>
</body>
```

