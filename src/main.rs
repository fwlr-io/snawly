use convert_case::{Case, Casing};
use std::{env, fs, io::Write, path::Path};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub mod highlight;
use highlight::{apply_highlight, config_for};

const HIGHLIGHTED_EXT: &str = "hhlt";

fn modfile(dir: &Path, filename: &str) -> Result<fs::File, std::io::Error> {
    let path = dir.to_path_buf().parent().unwrap().join(filename);
    fs::remove_file(&path)?;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&path)?;
    file.write_all("use crate::ux;\nuse leptos::prelude::*;\n".as_bytes())?;
    Ok(file)
}

fn modfile_components(file: &Path, hhlt: &Path) -> Option<(String, String)> {
    let dir_path = Path::new("codeblocks/");
    let file_path = dir_path.join(file.file_name()?);
    let file_path = file_path.to_str()?;
    let hhlt_path = dir_path.join(hhlt.file_name()?);
    let hhlt_path = hhlt_path.to_str()?;
    let component_name = file.file_stem()?.to_str()?.to_case(Case::Pascal);

    let props = "
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
";
    let base_component = format!(
        "#[component]
pub fn {component_name}({props}) -> impl IntoView {{
    let raw = include_str!(\"{file_path}\");
    let code = include_str!(\"{hhlt_path}\");
",
    );

    let spread_props = "raw=raw code=code class=class container_class=container_class";
    Some((
        format!("\n{base_component}\n    view! {{ <ux::PlainCode {spread_props} /> }}\n}}"),
        format!("\n{base_component}\n    view! {{ <ux::FancyCode {spread_props} /> }}\n}}"),
    ))
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new("/Users/scottfowler/dev/fwlr-io/src/codeblocks");
    for from_file in env::args().skip(1) {
        let to_file = dir.join(Path::new(&from_file).file_name().unwrap());
        fs::copy(&from_file, &to_file)?;
    }

    let mut plain_modfile = modfile(&dir, "codeblock.rs")?;
    let mut fancy_modfile = modfile(&dir, "fancy_codeblock.rs")?;

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

        let (plain_component, fancy_component) = modfile_components(&file, &hhlt).unwrap();

        fs::write(&hhlt, html)?;
        plain_modfile.write_all(plain_component.as_bytes())?;
        fancy_modfile.write_all(fancy_component.as_bytes())?;
    }

    Ok(())
}
