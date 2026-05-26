use clap::Parser;
use std::{
    fs::{self},
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

fn make_modfile(path: PathBuf, preamble: &str) -> Result<fs::File, Box<dyn std::error::Error>> {
    fs::remove_file(&path)?;
    let mut modfile = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&path)?;
    modfile.write_all(preamble.as_bytes())?;

    Ok(modfile)
}

fn highlight_codeblocks(src_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut highlighter = Highlighter::new();
    let mut renderer = HtmlRenderer::new();

    let mut codeblock_modfile = make_modfile(
        src_dir.join("codeblock.rs"),
        "use crate::ux::CodeBox;\nuse leptos::prelude::*;\n",
    )?;

    for entry in fs::read_dir(&src_dir.join("codeblocks"))?.filter_map(|p| Hlt::maybe_from(p.ok()?))
    {
        let config = config_for(&entry.file_ext).unwrap();
        let source = fs::read_to_string(&entry.file)?.into_bytes();
        let highlights = highlighter.highlight(config, &source, None, |lang| config_for(lang))?;

        renderer.render(highlights, &source, &apply_highlight)?;
        fs::write(&entry.hlt_file, &mut renderer.html)?;
        renderer.reset();

        codeblock_modfile.write_all(entry.as_code_component().as_bytes())?;
    }
    Ok(())
}

fn restyle_termblocks(src_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut termblock_modfile = make_modfile(
        src_dir.join("termblock.rs"),
        "use crate::ux::TermBox;\nuse leptos::prelude::*;\n",
    )?;

    for entry in fs::read_dir(&src_dir.join("termblocks"))?.filter_map(|p| Hlt::maybe_from(p.ok()?))
    {
        let source = fs::read_to_string(&entry.file)?;
        fs::write(&entry.hlt_file, restyle(source))?;
        termblock_modfile.write_all(entry.as_term_component().as_bytes())?;
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILES", num_args(1..))]
    code: Vec<String>,
    #[arg(short, long, value_name = "FILES", num_args(1..))]
    term: Vec<String>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let src_dir: &Path = Path::new("/Users/scottfowler/dev/website/src/");

    for from_file in cli.code {
        let file_name = Path::new(&from_file).file_name().unwrap();
        fs::copy(&from_file, &src_dir.join("codeblocks").join(file_name))?;
    }

    for from_file in cli.term {
        let file_name = Path::new(&from_file).file_name().unwrap();
        fs::copy(&from_file, &src_dir.join("termblocks").join(file_name))?;
    }

    highlight_codeblocks(src_dir)?;

    restyle_termblocks(src_dir)?;

    Ok(())
}
