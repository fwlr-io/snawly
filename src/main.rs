#[macro_use]
extern crate macro_rules_attribute;

use futures_concurrency::prelude::*;
use smol::prelude::*;
use smol::{
    fs, io,
    stream::{self, StreamExt},
};
use smol_macros::{Executor, main};
use std::os::fd::{AsRawFd, FromRawFd};
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub mod highlight;
use highlight::{apply_highlight, config_for};
pub mod termstyle;
use termstyle::restyle;
pub mod hlt;
use hlt::Hlt;

use clap::Parser;
use convert_case::{Case, Casing};
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

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

#[apply(main!)]
async fn main(_ex: &Executor) -> io::Result<()> {
    let cli = Cli::parse();
    let prefix = LazyLock::new(|| {
        cli.prefix
            .expect("`--prefix` is required")
            .to_case(Case::Snake)
    });
    let src_dir: &Path = Path::new("/Users/scottfowler/dev/website/src/");

    let modfile = async |path: &str, preamble: &str| {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .truncate(true)
            .open(src_dir.join(path))
            .await?;
        file.write_all(b"use leptos::prelude::*;\n").await?;
        file.write_all(preamble.as_bytes()).await.map(|_| file)
    };
    let (mut codeblock_modfile, mut termblock_modfile) = (
        modfile("codeblock.rs", "use crate::ux::CodeBox;\n"),
        modfile("termblock.rs", "use crate::ux::TermBox;\n"),
    )
        .try_join()
        .await?;

    let to_from = |from: &PathBuf| {
        format!(
            "{prefix}_{file_name}",
            prefix = &*prefix,
            file_name = from.file_name().unwrap().to_string_lossy()
        )
    };

    let codeblocks = stream::iter(cli.code)
        .then(async |from| {
            let to = src_dir.join("codeblocks").join(to_from(&from));
            fs::copy(from, &to).await.map(|_| to)
        })
        .merge(
            fs::read_dir(src_dir.join("codeblocks"))
                .await?
                .map(|d| d.map(|f| f.path())),
        );

    let termblocks = stream::iter(cli.term)
        .then(async |from: PathBuf| {
            let to = src_dir.join("termblocks").join(to_from(&from));
            fs::copy(from, &to).await.map(|_| to)
        })
        .merge(
            fs::read_dir(src_dir.join("codeblocks"))
                .await?
                .map(|d| d.map(|f| f.path())),
        );

    codeblock_modfile.sync_all().await?;
    let mut codeblock_modfile =
        unsafe { std::fs::File::from_raw_fd(codeblock_modfile.as_raw_fd()) };
    termblock_modfile.sync_all().await?;
    let mut termblock_modfile =
        unsafe { std::fs::File::from_raw_fd(termblock_modfile.as_raw_fd()) };

    inner_main(
        &mut codeblock_modfile,
        &mut termblock_modfile,
        codeblocks
            .filter_map(|f| f.ok().and_then(Hlt::try_from))
            .collect::<Vec<_>>()
            .await,
        termblocks
            .filter_map(|f| f.ok().and_then(Hlt::try_from))
            .collect::<Vec<_>>()
            .await,
    )
    .unwrap();
    Ok(())
}

pub fn inner_main(
    codeblock_modfile: &mut std::fs::File,
    termblock_modfile: &mut std::fs::File,
    codeblocks: Vec<Hlt>,
    termblocks: Vec<Hlt>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Codeblocks

    // Here be state
    let mut highlighter = Highlighter::new();
    // Here be state
    let mut renderer = HtmlRenderer::new();

    for entry in codeblocks.into_iter() {
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

    for entry in termblocks.into_iter() {
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
