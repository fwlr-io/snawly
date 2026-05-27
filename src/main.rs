use clap::Parser;
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub mod highlight;
use highlight::{apply_highlight, config_for};

pub mod termstyle;
use termstyle::restyle;

pub mod hlt;
use hlt::Hlt;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILES", num_args(1..))]
    code: Vec<PathBuf>,
    #[arg(short, long, value_name = "FILES", num_args(1..))]
    term: Vec<PathBuf>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let src_dir: &Path = Path::new("/Users/scottfowler/dev/website/src/");

    // Codeblocks

    let codeblocks_dir = src_dir.join("codeblocks");
    let codeblock_modfile_path = src_dir.join("codeblock.rs");

    for from_file in cli.code {
        let file_name = from_file.file_name().unwrap();
        fs::copy(&from_file, &codeblocks_dir.join(file_name))?;
    }

    fs::remove_file(&codeblock_modfile_path)?;
    let mut codeblock_modfile = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&codeblock_modfile_path)?;
    codeblock_modfile.write_all("use crate::ux::CodeBox;\nuse leptos::prelude::*;\n".as_bytes())?;

    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    for entry in
        fs::read_dir(&codeblocks_dir)?.filter_map(|p| Hlt::try_from(p.ok()?.path().as_path()))
    {
        let config = config_for(&entry.file_ext).unwrap();
        let source = fs::read_to_string(&entry.file)?.into_bytes();
        let highlights = highlighter.highlight(config, &source, None, |lang| config_for(lang))?;

        renderer.render(highlights, &source, &apply_highlight)?;
        fs::write(&entry.hlt_file, &mut renderer.html)?;
        renderer.reset();
        codeblock_modfile.write_all(entry.as_code_component().as_bytes())?;
    }

    // Termblocks

    let termblocks_dir = src_dir.join("termblocks");
    let termblock_modfile_path = src_dir.join("termblock.rs");

    for from_file in cli.term {
        let file_name = from_file.file_name().unwrap();
        fs::copy(&from_file, &termblocks_dir.join(file_name))?;
    }

    fs::remove_file(&termblock_modfile_path)?;
    let mut termblock_modfile = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&termblock_modfile_path)?;
    termblock_modfile.write_all("use crate::ux::TermBox;\nuse leptos::prelude::*;\n".as_bytes())?;

    for entry in
        fs::read_dir(&termblocks_dir)?.filter_map(|p| Hlt::try_from(p.ok()?.path().as_path()))
    {
        let source = fs::read_to_string(&entry.file)?;
        fs::write(&entry.hlt_file, restyle(source))?;
        termblock_modfile.write_all(entry.as_term_component().as_bytes())?;
    }

    Ok(())
}
