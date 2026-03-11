use convert_case::{Case, Casing};
use std::{env, fs, io::Write, path::Path};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub mod highlight;
use highlight::{apply_highlight, config_for};

const HIGHLIGHTED_EXT: &str = "hhlt";

fn modfile_component(file: &Path, hhlt: &Path) -> Option<String> {
    let component_name = file.file_stem()?.to_str()?.to_case(Case::Pascal);

    let dir_path = Path::new("codeblocks/");
    let file_path = dir_path.join(file.file_name()?);
    let file_path = file_path.to_str()?;
    let hhlt_path = dir_path.join(hhlt.file_name()?);
    let hhlt_path = hhlt_path.to_str()?;

    Some(format!(
        "
#[component]
pub fn {component_name}(
    #[prop(optional)] plain: bool,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {{
    view! {{
        <ux::Code
            plain=plain
            attr:class=class
            raw=include_str!(\"{file_path}\")
            code=include_str!(\"{hhlt_path}\")
        />
    }}
}}
"
    ))
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new("/Users/scottfowler/dev/website/src/codeblocks");
    for from_file in env::args().skip(1) {
        let to_file = dir.join(Path::new(&from_file).file_name().unwrap());
        fs::copy(&from_file, &to_file)?;
    }

    let path = &dir.to_path_buf().parent().unwrap().join("codeblock.rs");
    fs::remove_file(&path)?;
    let mut modfile = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&path)?;
    modfile.write_all("use crate::ux;\nuse leptos::prelude::*;\n".as_bytes())?;

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

        renderer.render(highlights, &source, &apply_highlight)?;
        let html = String::from_utf8(std::mem::take(&mut renderer.html))?;
        renderer.reset();

        let component = modfile_component(&file, &hhlt).unwrap();
        fs::write(&hhlt, html)?;
        modfile.write_all(component.as_bytes())?;
    }

    Ok(())
}
