use convert_case::{Case, Casing};
use std::{fs, path::Path};
use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter, HtmlRenderer};

pub const NAMES: &[&str] = &[
    "function",
    "attribute",
    "string",
    "comment",
    "escape",
    "constant",
    "constructor",
    "keyword",
    "label",
    "module",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "tag",
    "tag.attribute",
    "tag.delimiter",
    "number",
    "variable",
    "type",
];

fn rstml_config() -> HighlightConfiguration {
    let mut config = HighlightConfiguration::new(
        tree_sitter_rstml::language_rust_with_rstml(),
        "rstml",
        tree_sitter_rstml::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .unwrap();
    config.configure(NAMES);

    config
}

fn typescript_config() -> HighlightConfiguration {
    let mut highlights = tree_sitter_typescript::HIGHLIGHTS_QUERY.to_owned();
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);
    let mut config = HighlightConfiguration::new(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "typescript",
        &highlights,
        "",
        "",
    )
    .unwrap();
    config.configure(NAMES);

    config
}

fn css_config() -> HighlightConfiguration {
    let mut config = HighlightConfiguration::new(
        tree_sitter_css::LANGUAGE.into(),
        "css",
        &tree_sitter_css::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .unwrap();
    config.configure(NAMES);

    config
}

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
                "type" => "dim-blue",
                "tag" => "dim-yellow",
                "function" => "dim-red",
                "tag.attribute" => "dim-cyan",
                "number" | "variable" => "blue",
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

fn make_components(file: &Path) -> String {
    let dir_path = Path::new("codeblocks/").to_path_buf();
    let file_name = Path::new(file.file_name().unwrap());

    let html_path = dir_path.join(file_name.with_extension("html"));
    let html_path = html_path.to_str().unwrap();
    let component_path = dir_path.join(file_name);
    let component_path = component_path.to_str().unwrap();
    let component_name = file_name
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_case(Case::Pascal);

    format!(
        "\n#[component]
pub fn {component_name}() -> impl IntoView {{
    let code = include_str!(\"{html_path}\");
    view! {{ <ux::Code code=code /> }}
}}\n
#[component]
pub fn Fancy{component_name}() -> impl IntoView {{
    let code = include_str!(\"{html_path}\");
    let raw = include_str!(\"{component_path}\");
    view! {{ <ux::FancyCode code=code raw=raw /> }}
}}\n"
    )
}

pub fn main() {
    let dir = Path::new("../fwlr-io/src/codeblocks");

    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    let modfile_path = dir.to_path_buf().with_file_name("codeblock.rs");
    let mut modfile = String::from("use crate::ux;\nuse leptos::prelude::*;\n");

    for file in fs::read_dir(dir).unwrap().map(|f| f.unwrap()) {
        let file_ext = file.path();
        let file_ext = file_ext.extension().unwrap().to_str().unwrap();
        if file_ext == "html" {
            continue;
        }

        let source = fs::read_to_string(file.path()).unwrap();
        let out_file = file.path().with_extension("html");

        let config = match file_ext {
            "rs" => rstml_config(),
            "css" => css_config(),
            "ts" => typescript_config(),
            _ => {
                eprintln!("filetype not recognised: {file_ext}");
                break;
            }
        };
        let highlights = highlighter
            .highlight(&config, source.as_bytes(), None, |_| None)
            .unwrap();

        renderer.reset();
        renderer
            .render(highlights, source.as_bytes(), &apply_highlight)
            .unwrap();

        let html = String::from_utf8(renderer.html.clone()).unwrap();
        let _ = fs::write(out_file, html);

        modfile.push_str(&make_components(&file.path()));
    }

    let _ = fs::write(modfile_path, modfile);
}
