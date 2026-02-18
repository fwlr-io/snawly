mod config;
mod names;

use config::config_for;
use names::NAMES;

use convert_case::{Case, Casing};
use std::fs;
use std::io::Write;
use std::path;
use tree_sitter_highlight::{Highlight, Highlighter, HtmlRenderer};

const HIGHLIGHTED_EXT: &str = "hhlt";

fn apply_highlight(hl: Highlight, acc: &mut Vec<u8>) {
    let tag_index = hl.0;
    let Some(tag) = NAMES.get(tag_index) else {
        eprintln!("unrecognised tag index: {tag_index}");
        return;
    };
    acc.extend(
        format!(
            "class=\"text-{}\"",
            match *tag {
                "tag" => "dim-yellow",
                "function" => "dim-red",
                "function.macro" => "green",
                "tag.attribute" => "dim-cyan",
                "type" | "type.builtin" => "dim-blue",
                "number" | "variable" | "variable.parameter" => "blue",
                "escape" | "constant" => "cyan",
                "attribute" | "string" => "dim-green",
                "constructor" | "keyword" | "module" => "dim-magenta",
                "comment" | "punctuation.delimiter" | "tag.delimiter" => "grey",
                "label" | "operator" | "property" | "punctuation.bracket" => "dim-white",
                _ => {
                    eprintln!("unrecognised: {tag}");
                    "yellow"
                }
            }
        )
        .as_bytes(),
    )
}

fn make_components(file: &path::Path) -> Option<String> {
    let dir_path = path::Path::new("codeblocks/").to_path_buf();
    let file_name = path::Path::new(file.file_name()?);

    let hhlt_path = dir_path.join(file_name.with_extension(HIGHLIGHTED_EXT));
    let hhlt_path = hhlt_path.to_str()?;

    let component_path = dir_path.join(file_name);
    let component_path = component_path.to_str()?;

    let component_name = file_name.file_stem()?.to_str()?.to_case(Case::Pascal);

    format!(
        "\n#[component]
pub fn {component_name}() -> impl IntoView {{
    let code = include_str!(\"{hhlt_path}\");
    view! {{ <ux::Code code=code /> }}
}}\n
#[component]
pub fn Fancy{component_name}() -> impl IntoView {{
    let code = include_str!(\"{hhlt_path}\");
    let raw = include_str!(\"{component_path}\");
    view! {{ <ux::FancyCode code=code raw=raw /> }}
}}\n"
    )
    .into()
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = path::Path::new("../fwlr-io/src/codeblocks");

    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    let modfile_path = dir.to_path_buf().with_file_name("codeblock.rs");
    fs::write(&modfile_path, "use crate::ux;\nuse leptos::prelude::*;\n")?;
    let mut modfile = fs::OpenOptions::new().append(true).open(modfile_path)?;

    for entry in fs::read_dir(dir)? {
        let file = entry?.path();
        let file_ext = file
            .extension()
            .ok_or("no ext")?
            .to_str()
            .ok_or("not utf8")?;
        match file.extension() {
            Some(s) => {
                if s == HIGHLIGHTED_EXT {
                    continue;
                }
            }
            None => continue,
        };

        let config = config_for(file_ext).ok_or("no config")?;
        let source = fs::read_to_string(&file)?.into_bytes();

        let highlights = highlighter.highlight(config, &source, None, |lang| config_for(lang))?;

        renderer.reset();
        renderer.render(highlights, &source, &apply_highlight)?;

        let html = String::from_utf8(renderer.html.clone())?;
        fs::write(file.with_extension(HIGHLIGHTED_EXT), html)?;
        modfile.write_all(make_components(&file).ok_or("couldnt write")?.as_bytes())?;
    }

    Ok(())
}

// #[derive(Debug)]
// pub struct Err {}

// impl<E> From<E> for Err
// where
//     E: std::fmt::Display,
// {
//     fn from(e: E) -> Self {
//         eprintln!("{e}");
//         Err {}
//     }
// }

// impl Err {
//     pub fn new(e: impl std::fmt::Display) -> Self {
//         eprintln!("{e}");
//         Err {}
//     }
// }
// impl From<std::string::FromUtf8Error> for Err {
//     fn from(e: std::string::FromUtf8Error) -> Self {
//         Err::new(e)
//     }
// }
// impl From<std::io::Error> for Err {
//     fn from(e: std::io::Error) -> Self {
//         Err::new(e)
//     }
// }
// impl From<tree_sitter_highlight::Error> for Err {
//     fn from(e: tree_sitter_highlight::Error) -> Self {
//         Err::new(e)
//     }
// }
