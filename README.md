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

## Example usage

Consider the following OpenFOAM dictionary:

```cpp
// this is ?{\color{orange} \LaTeX code}?
key val;
dict {
    key1 "val";
    key2 {
        /*
            More ?\LaTeX code if you want: $\phi$?
        */
    }
}
```

Running the highlighter with `foam-highlighter html dictName` returns:
```html
<pre><code class="language-foam hljs">
<span class="hljs-comment">// this is ?{\color{orange} \LaTeX code}?</span>
<span class="hljs-title">key</span> val<span class="hljs-punctuation">;</span>
<span class="hljs-type">dict</span> <span class="hljs-punctuation">{</span>
    <span class="hljs-title">key1</span> <span class="hljs-string">"val"</span><span class="hljs-punctuation">;</span>
    <span class="hljs-type">key2</span> <span class="hljs-punctuation">{</span>
        <span class="hljs-comment">/*
            More ?\LaTeX code if you want: $\phi$?
        */</span>
    <span class="hljs-punctuation">}</span>
<span class="hljs-punctuation">}</span>
</code></pre>
```

While running it with `foam-highlighter pygtex dictName '?'`  returns:
```tex
\PYG{c+c1}{// this is {\color{orange} \LaTeX code}}
\PYG{vc}{key} val\PYG{o}{;}
\PYG{kp}{dict} \PYG{o}{\PYGZob{}}
    \PYG{vc}{key1} \PYG{l+s}{\PYGZdq{}val\PYGZdq{}}\PYG{o}{;}
    \PYG{kp}{key2} \PYG{o}{\PYGZob{}}
        \PYG{c+c1}{/*}
\PYG{c+c1}{            More \LaTeX code if you want: $\phi$}
\PYG{c+c1}{        */}
    \PYG{o}{\PYGZcb{}}
\PYG{o}{\PYGZcb{}}
```
Which you can use in a Latex document by copying it into the `Verbatim` environment of the corresponding
`*.pygtex` cache file for an empty `minted` environment:
```tex

\documentclass[9pt]{article}

%% A minimal example of working OpenFOAM hilighting using minted
%% Compiles with all major engines (pdflatex, lualatex and xelatex)

\usepackage{amsmath, amsfonts}
\usepackage[cachedir=_minted-cache]{minted}
\makeatletter
\def\minted@jobname{openfoam-highlight}
\makeatother

\begin{document}
    %% Source: dictName
    %% Cache file: D41D8CD98F00B204E9800998ECF8427EBC2AC3F26D5FFE2ED2ACF73E1678E792.pygtex
    \begin{minted}[escapeinside=??, linenos]{cpp}
    \end{minted}
\end{document}
```
By doing this, you get nicely highlighted OpenFOAM dictionaries in Latex documents with the ability
to run arbitrary Tex code in the OpenFOAM comments

> Note that the `escapeinside=??` minted option needs to what's used in the OpenFOAM dictionary
> and the last (optional) parameter to the highlighter

Sounds painful? Well this is the best we can do - The only other option is to write a Pygments Lexer
for OpenFOAM case files; and nobody will do that!
