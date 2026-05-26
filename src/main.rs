use convert_case::{Case, Casing};
use std::{env, fs, io::Write, path::Path};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub mod highlight;
use highlight::{apply_highlight, config_for};

pub mod termstyle;
use termstyle::restyle;

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

fn copy_blocks(from_files: Vec<String>, to_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for from_file in from_files {
        let to_file = to_dir.join(Path::new(&from_file).file_name().unwrap());
        fs::copy(&from_file, &to_file)?;
    }

    Ok(())
}

fn make_modfile(modfile_path: &Path) -> Result<fs::File, Box<dyn std::error::Error>> {
    fs::remove_file(&modfile_path)?;

    let modfile = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&modfile_path)?;

    Ok(modfile)
}

fn highlight_codeblocks(
    codeblocks: &Path,
    modfile: &mut fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    for entry in fs::read_dir(codeblocks)? {
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

fn restyle_termblocks(
    termblocks: &Path,
    modfile: &mut fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sources = env::args().skip(1).collect::<Vec<_>>();
    let codeblock_sources = sources;
    let termblock_sources = vec![String::from("")];

    let src_dir: &Path = Path::new("/Users/scottfowler/dev/website/src/");

    copy_blocks(codeblock_sources, &src_dir.join("/codeblocks/"))?;
    let mut codeblock_modfile = make_modfile(&src_dir.join("codeblock.rs"))?;
    codeblock_modfile.write_all("use crate::ux::CodeBox;\nuse leptos::prelude::*;\n".as_bytes())?;
    highlight_codeblocks(&src_dir.join("/codeblocks/"), &mut codeblock_modfile)?;

    copy_blocks(termblock_sources, &src_dir.join("/termblocks/"))?;
    let mut termblock_modfile = make_modfile(&src_dir.join("termblock.rs"))?;
    termblock_modfile.write_all("use crate::ux::TermBox;\nuse leptos::prelude::*;\n".as_bytes())?;
    restyle_termblocks(&src_dir.join("/termblocks/"), &mut termblock_modfile)?;

    Ok(())
}
