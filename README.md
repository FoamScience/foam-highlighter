# (HTML-based) Standalone Syntax highlighter for OpenFOAM dictionaries

## Installation & Usage

As this is a Rust (mini) project, you'll have to have `cargo` installed:

```bash
cargo install foam-highlighter
```

Then, all you have to do is to point to the file you want highlighted:
```bash
foam-highlighter /path/to/foam/Dict
```

If parsing was successful, a styled HTML snippet will be sent to your `stdout`.

## Configuration

By default, I'm using CSS classes from [highlight.js](https://highlightjs.org/)
to highlight the code in the least intrusive manner. I currently provide no way
of changing this behaviour (Aside from changing the hard-coded names in `src/main.rs`).

We're also depending on
[OpenFOAM grammar for Tree-Sitter](https://github.com/FoamScience/tree-sitter-foam),
which can parse all tutorial files from OpenFOAM 8 and Foam-Extend 4 repositories.
If you suspect there is an issue with the parser, please open an issue there.
