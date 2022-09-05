# Standalone Syntax highlighter for OpenFOAM dictionaries

> DISCLAIMER: This offering is not approved or endorsed by OpenCFD Limited,
> producer and distributor of the OpenFOAM software and owner of the
> OPENFOAM® and OpenCFD® trade marks.

## Installation & Usage

As this is a Rust (mini) project, you'll have to have `cargo` installed:

```bash
cargo install foam-highlighter
```

Then, provide both the output format and the input filename to parse:
```bash
foam-highlighter html /path/to/foam/Dict
```
If parsing was successful, a styled HTML snippet will be sent to your `stdout`.

## Configuration

Options for the output format are `html` and `pygtex`:
- **html** output gives you a populated `<code>` tag and uses [highlight.js](https://highlightjs.org/)
    classes for styling. Currently no way of changing the class names is provided
    (Aside from changing the hard-coded names in `src/main.rs`). This output format is ideal obviously if
    you want to highlight OpenFOAM code on webpages and HTML-based presentations.
- **pygtex** output gives a (fake-but-valid) Pygments output in the `pygtex` format. This is the format
    used by `minted` package to highlight code in Latex. Starting with an empty minted environment, copying
    the output into the empty Verbatim environment of the corresponding `*pytex` file and recompiling will
    result in correct highlighting of OpenFOAM snippets in academic publications.

From an efficiency point view, both the parse and the highlighter are quite fast. A Pygments lexer for OpenFOAM
is not available unfortunately so no performance comparisons are possible.

We're also depending on [OpenFOAM grammar for Tree-Sitter](https://github.com/FoamScience/tree-sitter-foam),
which can parse all tutorial files from OpenFOAM 8 and Foam-Extend 4 repositories.
If you suspect there is an issue with the parser, please open an issue there.
