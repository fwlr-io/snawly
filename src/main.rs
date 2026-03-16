use convert_case::{Case, Casing};
use std::{env, fs, io::Write, path::Path};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub mod highlight;
use highlight::{apply_highlight, config_for};

const HIGHLIGHTED_EXT: &str = "hlt";

fn modfile_component(file: &Path, hlt: &Path) -> Option<String> {
    let component_name = file.file_stem()?.to_str()?.to_case(Case::Pascal);

    let dir_path = Path::new("codeblocks/");
    let file_path = dir_path.join(file.file_name()?);
    let file_path = file_path.to_str()?;
    let hlt_path = dir_path.join(hlt.file_name()?);
    let hlt_path = hlt_path.to_str()?;

    Some(format!(
        "
#[component]
pub fn {component_name}() -> impl IntoView {{
    view! {{
        <CodeBox
            raw=include_str!(\"{file_path}\")
            code=include_str!(\"{hlt_path}\")
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
    modfile.write_all("use crate::ux::CodeBox;\nuse leptos::prelude::*;\n".as_bytes())?;

    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    for entry in fs::read_dir(dir)? {
        let file = entry?.path();
        let file_ext = file.extension().unwrap().to_str().unwrap();
        let hlt = match file_ext {
            HIGHLIGHTED_EXT => continue,
            _ => file.with_extension(HIGHLIGHTED_EXT),
        };

        let config = config_for(file_ext).unwrap();
        let source = fs::read_to_string(&file)?.into_bytes();

        let highlights = highlighter.highlight(config, &source, None, |lang| config_for(lang))?;

        renderer.render(highlights, &source, &apply_highlight)?;
        let html = String::from_utf8(std::mem::take(&mut renderer.html))?;
        renderer.reset();

        let component = modfile_component(&file, &hlt).unwrap();
        fs::write(&hlt, html)?;
        modfile.write_all(component.as_bytes())?;
    }

    Ok(())
}
