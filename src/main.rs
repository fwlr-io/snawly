use snawly::config::config_for;
use snawly::highlight::apply_highlight;

use convert_case::{Case, Casing};
use std::{env, fs, io::Write, path::Path};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

const HIGHLIGHTED_EXT: &str = "hhlt";

fn make_modfile(dir: &Path, name: &str) -> Result<fs::File, std::io::Error> {
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(dir.to_path_buf().parent().unwrap().join(name))
}

fn make_components(file: &Path, hhlt: &Path) -> Option<(String, String)> {
    let dir_path = Path::new("codeblocks/");
    let file_path = dir_path.join(file.file_name()?);
    let file_path = file_path.to_str()?;
    let hhlt_path = dir_path.join(hhlt.file_name()?);
    let hhlt_path = hhlt_path.to_str()?;
    let component_name = file.file_stem()?.to_str()?.to_case(Case::Pascal);

    let base_component = format!(
        "#[component]
pub fn {component_name}() -> IntoView {{
    let raw = include_str!(\"{file_path}\");
    let code = include_str!(\"{hhlt_path}\");
",
    );

    Some((
        format!("\n{base_component}\n    view! {{ <ux::PlainCode raw=raw code=code /> }}\n}}"),
        format!("\n{base_component}\n    view! {{ <ux::FancyCode raw=raw code=code /> }}\n}}"),
    ))
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new("../fwlr-io/src/codeblocks");
    for from_file in env::args().skip(1) {
        let to_file = dir.join(Path::new(&from_file).file_name().unwrap());
        fs::copy(&from_file, &to_file)?;
    }

    let mut modfile = make_modfile(dir, "codeblock.rs")?;
    modfile.write_all("use crate::ux;\nuse leptos::prelude::*;\n".as_bytes())?;

    let mut fancy_modfile = make_modfile(dir, "fancy_codeblock.rs")?;
    fancy_modfile.write_all("use crate::ux;\nuse leptos::prelude::*;\n".as_bytes())?;

    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    for entry in fs::read_dir(dir)? {
        let file = entry?.path();
        let file_ext = file.extension().unwrap().to_str().unwrap();
        let hhlt = match file_ext {
            HIGHLIGHTED_EXT => continue,
            _ => file.with_extension(HIGHLIGHTED_EXT),
        };

        let config = config_for(file_ext).unwrap();
        let source = fs::read_to_string(&file)?.into_bytes();

        let highlights = highlighter.highlight(config, &source, None, |lang| config_for(lang))?;

        renderer.reset();
        renderer.render(highlights, &source, &apply_highlight)?;

        let html = String::from_utf8(std::mem::take(&mut renderer.html))?;
        let (component, fancy_component) = make_components(&file, &hhlt).unwrap();

        fs::write(&hhlt, html)?;
        modfile.write_all(component.as_bytes())?;
        fancy_modfile.write_all(fancy_component.as_bytes())?;
    }

    Ok(())
}
