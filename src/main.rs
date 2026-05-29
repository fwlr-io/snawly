use std::{
    error::Error,
    io::Write,
    path::{Path, PathBuf},
};
// use tokio::fs;
use tokio::io::AsyncWriteExt;
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio_stream::{self as stream, StreamExt};
// use futures_concurrency::prelude::*;

use clap::Parser;
use convert_case::{Case, Casing};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub mod highlight;
use highlight::{apply_highlight, config_for};

pub mod termstyle;
use termstyle::restyle;

pub mod hlt;
use hlt::Hlt;

#[derive(Parser)]
/// A tool for putting source code and terminal captures on the fwlr.io website.
///
/// For blocks of code, it uses tree-sitter to syntax highlight most common languages.
/// For terminal output, it translates ghostty's html styling into tailwind classes.
/// With or without arguments, it will always rebuild the contents of `codeblocks` & `termblocks`.
struct Cli {
    /// A prefix for the files to be copied, to avoid name collisions.
    ///
    /// For example, `snawly --prefix foo --code main.rs mod.rs`
    /// will produce `codeblocks/foo_main.hlt` & `codeblocks/foo_mod.hlt` files,
    /// and `codeblocks::FooMain` & `codeblocks::FooMod` leptos components.
    #[arg(short, long, value_name = "STRING")]
    prefix: Option<String>,
    /// Files containing code to be copied and syntax-highlighted.
    #[arg(short, long, num_args(1..), value_name = "FILES", requires = "prefix")]
    code: Vec<PathBuf>,
    /// Files containing html captures of terminal output, to be copied and restyled.
    #[arg(short, long, num_args(1..), value_name = "FILES", requires = "prefix")]
    term: Vec<PathBuf>,
}

// The synchronous ordering of the original main function
// 'accidentally' enforces some ordering constraints.
// e.g. we can simply process the entire `codeblocks` directory,
// but only because we previously copied all new codeblocks in.
pub fn main() -> Result<(), Box<dyn Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // let cli = Cli::parse();
            let src_dir: &Path = Path::new("/Users/scottfowler/dev/website/src/");
            // let codeblocks_dir = src_dir.join("codeblocks");
            let codeblock_modfile = async {
                let path = src_dir.join("codeblock.rs");

                tokio::fs::remove_file(&path).await?;
                let mut modfile = tokio::fs::OpenOptions::new()
                    .append(true)
                    .create_new(true)
                    .open(&path)
                    .await?;
                modfile
                    .write_all("use crate::ux::CodeBox;\nuse leptos::prelude::*;\n".as_bytes())
                    .await?;

                Ok::<tokio::fs::File, Box<dyn Error>>(modfile)
            };

            // let termblocks_dir = src_dir.join("termblocks");
            let termblock_modfile = async {
                let path = src_dir.join("termblock.rs");

                tokio::fs::remove_file(&path).await?;
                let mut modfile = tokio::fs::OpenOptions::new()
                    .append(true)
                    .create_new(true)
                    .open(&path)
                    .await?;
                modfile
                    .write_all("use crate::ux::TermBox;\nuse leptos::prelude::*;\n".as_bytes())
                    .await?;
                Ok::<tokio::fs::File, Box<dyn Error>>(modfile)
            };

            let _ = inner_main(
                &mut codeblock_modfile.await.unwrap().into_std().await,
                &mut termblock_modfile.await.unwrap().into_std().await,
            );
        });
    Ok(())
}

pub fn inner_main(
    codeblock_modfile: &mut std::fs::File,
    termblock_modfile: &mut std::fs::File,
) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let src_dir: &Path = Path::new("/Users/scottfowler/dev/website/src/");

    // Codeblocks

    let codeblocks_dir = src_dir.join("codeblocks");

    // Parallelizable. On completing a file copy,
    // should trigger a highlight/format for the new file.
    for from_file in cli.code {
        std::fs::copy(
            &from_file,
            &codeblocks_dir.join(format!(
                "{prefix}_{file_name}",
                prefix = cli
                    .prefix
                    .clone()
                    .expect("--prefix is required")
                    .to_case(Case::Snake),
                file_name = from_file.file_name().unwrap().to_str().unwrap()
            )),
        )?;
    }

    // Here be state
    let mut highlighter = Highlighter::new();
    // Here be state
    let mut renderer = HtmlRenderer::new();

    // Parallelizable. New targets may be added to the directory
    // after this has already been started.
    for entry in
        std::fs::read_dir(&codeblocks_dir)?.filter_map(|p| Hlt::try_from(p.ok()?.path().as_path()))
    {
        // functional
        let config = config_for(&entry.file_ext).unwrap();
        // async-able
        let source = std::fs::read_to_string(&entry.file)?.into_bytes();
        // stateful
        let highlights = highlighter.highlight(config, &source, None, |lang| config_for(lang))?;
        // stateful
        renderer.render(highlights, &source, &apply_highlight)?;
        // async-able
        std::fs::write(&entry.hlt_file, &mut renderer.html)?;
        // stateful
        renderer.reset();
        // async-able
        codeblock_modfile.write_all(entry.as_code_component().as_bytes())?;
    }

    // Termblocks

    let termblocks_dir = src_dir.join("termblocks");

    // Parallelizable. On completing a file copy,
    // should trigger a highlight/format for the new file.
    for from_file in cli.term {
        std::fs::copy(
            &from_file,
            &termblocks_dir.join(format!(
                "{prefix}_{file_name}",
                prefix = cli
                    .prefix
                    .clone()
                    .expect("--prefix is required")
                    .to_case(Case::Snake),
                file_name = from_file.file_name().unwrap().to_str().unwrap()
            )),
        )?;
    }

    // Parallelizable. New targets may be added to the directory
    // after this has already been started.
    for entry in
        std::fs::read_dir(&termblocks_dir)?.filter_map(|p| Hlt::try_from(p.ok()?.path().as_path()))
    {
        // async-able
        let source = std::fs::read_to_string(&entry.file)?;
        // async-able
        let restyled = restyle(source);
        // async-able
        std::fs::write(&entry.hlt_file, restyled)?;
        // async-able
        termblock_modfile.write_all(entry.as_term_component().as_bytes())?;
    }

    Ok(())
}
