use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use std::convert::TryInto;
use std::{env,io,fs,process};
use regex::Regex;

fn main() -> io::Result<()> {

    // CMD args setup
    let args: Vec<String> = env::args().collect();
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const DESC: &str = env!("CARGO_PKG_DESCRIPTION");
    const REPO: &str = env!("CARGO_PKG_REPOSITORY");

    if args.len() != 3 && args.len() != 4 {
        println!("{} - v{}", NAME, VERSION);
        println!("Written by: {}", AUTHORS);
        println!("To: {}\n", DESC);

        println!("Usage:");
        println!("{} <format> <filename> [sepchar]", NAME);
        println!("    format is either \'html\' or \'pygtex\'.");
        println!("    filename is the input OpenFOAM dictionary.");
        println!("    sepchar is an optional character (default: '?') to denote sections where Tex code will not be parsed.\n");

        println!("Please refer to: {} for more info.", REPO);
        
        process::exit(1);
    }

    let format = &args[1];
    let filename = &args[2];
    let mut skipchar = "?";
    if args.len() == 4 {
        skipchar = &args[3];
    }

    // Read content of file
    let buffer = fs::read_to_string(&filename)?;
    let source = buffer.as_bytes();

    let highlight_names : &[_] = &[
        "attribute",
        "comment",
        "conditional",
        "escape",
        "float",
        "function",
        "namespace",
        "punctuation",
        "string",
        "type",
    ];

    // This has to reflect the order of highlights
    let css_classes : &[_] = &[
        "hljs-keyword",
        "hljs-comment",
        "hljs-built_in",
        "hljs-built_in",
        "hljs-number",
        "hljs-title",
        "hljs-meta",
        "hljs-punctuation",
        "hljs-string",
        "hljs-type",
    ];

    // This has to reflect the order of highlights
    let pygtex_classes : &[_] = &[
        "k",
        "c+c1",
        "kt",
        "kt",
        "l+m+mf",
        "vc",
        "gp",
        "o",
        "l+s",
        "kp",
    ];



    let mut highlighter = Highlighter::new();
    let mut foam_config = HighlightConfiguration::new(
        tree_sitter_foam::language(),
        tree_sitter_foam::HIGHLIGHTS_QUERY,
        tree_sitter_foam::INJECTIONS_QUERY,
        tree_sitter_foam::LOCALS_QUERY,
    ).unwrap();

    
    //let source = b"// comment\ndiv(1|U,d) dsq;\nFoamFile { version 2.0;\nclass volScalarField; }";
    foam_config.configure(&highlight_names);

    let highlights = highlighter.highlight(
        &foam_config,
        source,
        None,
        |_| None
    ).unwrap();
    //let r = renderer.render(
    //    highlights,
    //    source,
    //    &simple_callback
    //).unwrap();

    if format == "html" {
        println!("<pre><code class=\"language-foam hljs\">");
        for event in highlights {
            match event.unwrap() {
                HighlightEvent::Source {start, end} => {
                    let slice = &source[start..end];
                    let s = match std::str::from_utf8(slice) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                    print!("{}", s);
                },
                HighlightEvent::HighlightStart(s) => {
                    //eprintln!("highlight style started: {:?}", s);
                    let hl : usize = s.0.try_into().unwrap();
                    print!("<span class={:?}>", css_classes[hl]);
                },
                HighlightEvent::HighlightEnd => {
                    //eprintln!("highlight style ended");
                    print!("</span>");
                },
            }
        }
        println!("</code></pre>");
    } else if format == "pygtex" {
        for event in highlights {
            match event.unwrap() {
                HighlightEvent::Source {start, end} => {
                    let slice = &source[start..end];
                    let s = match std::str::from_utf8(slice) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };

                    if s.contains("//") || s.contains("/*") {
                        // It's a comment, do two things:
                        // 1. Tokenize the string to see if we should skip processing tex code
                        // 2. Tokenize the tokens in seach for new lines in multi-line comments
                        let seperator = Regex::new(format!("[{}]", skipchar).as_str()).expect("Invalid comment tokenization regex");
                        let splits: Vec<_> = seperator.split(s).into_iter().collect();
                        for id in 0..splits.len() {
                            let newline = Regex::new("[\n]").expect("Invalid newline separation regex");
                            let lines: Vec<_> = newline.split(splits[id]).into_iter().collect();
                            if id % 2 == 0 {
                                for ln in 0..lines.len() {
                                    let len =  if ln != lines.len()-1 {"}\n\\PYG{c+c1}{"} else {""};
                                    print!(
                                        "{}{}",
                                        lines[ln].replace("\\", "\\PYGZbs")
                                                 .replace("}", "\\PYGZcb")
                                                 .replace("{", "\\PYGZob")
                                                 .replace("\\PYGZob", "\\PYGZob{}")
                                                 .replace("\\PYGZcb", "\\PYGZcb{}")
                                                 .replace("\\PYGZbs", "\\PYGZbs{}")
                                                 .replace("_", "\\PYGZus{}")
                                                 .replace("^", "\\PYGZca{}")
                                                 .replace("&", "\\PYGZam{}")
                                                 .replace("<", "\\PYGZlt{}")
                                                 .replace(">", "\\PYGZgt{}")
                                                 .replace("#", "\\PYGZsh{}")
                                                 .replace("%", "\\PYGZpc{}")
                                                 .replace("$", "\\PYGZdl{}")
                                                 .replace("-", "\\PYGZhy{}")
                                                 .replace("\'", "\\PYGZsq{}")
                                                 .replace("\"", "\\PYGZdq{}")
                                                 .replace("~", "\\PYGZti{}"),
                                        len
                                        //if ln % 2 == 0 { "}}\n\\PYG{{c+c1}}{{" }  else { "" }
                                    );
                                }
                            } else {
                                print!("{}", splits[id]);
                            }
                        }
                    } else {
                        print!(
                            "{}",
                            s.replace("\\", "\\PYGZbs")
                             .replace("}", "\\PYGZcb")
                             .replace("{", "\\PYGZob")
                             .replace("\\PYGZob", "\\PYGZob{}")
                             .replace("\\PYGZcb", "\\PYGZcb{}")
                             .replace("\\PYGZbs", "\\PYGZbs{}")
                             .replace("_", "\\PYGZus{}")
                             .replace("^", "\\PYGZca{}")
                             .replace("&", "\\PYGZam{}")
                             .replace("<", "\\PYGZlt{}")
                             .replace(">", "\\PYGZgt{}")
                             .replace("#", "\\PYGZsh{}")
                             .replace("%", "\\PYGZpc{}")
                             .replace("$", "\\PYGZdl{}")
                             .replace("-", "\\PYGZhy{}")
                             .replace("\'", "\\PYGZsq{}")
                             .replace("\"", "\\PYGZdq{}")
                             .replace("~", "\\PYGZti{}")
                        );
                    }
                },

                HighlightEvent::HighlightStart(s) => {
                    //eprintln!("highlight style started: {:?}", s);
                    let hl : usize = s.0.try_into().unwrap();
                    print!("\\PYG{{{}}}{{", pygtex_classes[hl]);
                },
                HighlightEvent::HighlightEnd => {
                    //eprintln!("highlight style ended");
                    print!("}}");
                },
            }
        }
    } else {
        eprintln!("Parsed input but no valid output format is provided. Available options: html or pygtex");
    }
    Ok(())
}
