#[macro_use]
extern crate macro_rules_attribute;

use futures_concurrency::prelude::*;
use smol::{
    fs::{self, DirEntry},
    io,
    prelude::*,
    stream,
};
use smol_macros::{Executor, main};
use std::{path::PathBuf, sync::LazyLock};

mod util;
use util::prelude::*;

mod highlight;
use highlight::highlight;

mod termstyle;
use termstyle::termstyle;

use clap::Parser;
use convert_case::{Case, Casing};

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

const HIGHLIGHTED_EXT: &str = "hlt";
const SRC_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from("/Users/scottfowler/dev/website/src/"));

#[apply(main!)]
async fn main(ex: &Executor) -> io::Result<()> {
    let cli = Cli::parse();
    let prefix = LazyLock::new(|| {
        cli.prefix
            .expect("`--prefix` is required")
            .to_case(Case::Snake)
    });

    let codeblocks = (
        fs::read_dir(SRC_DIR.join("codeblocks"))
            .await?
            .filter_map(|r| match r {
                Ok(p) if p.file_name().to_str()?.ends_with("DS_Store") => None,
                Ok(p) if p.file_name().to_str()?.ends_with(HIGHLIGHTED_EXT) => None,
                Ok(p) => Some(Ok(p.path())),
                Err(_) => None,
            }),
        stream::iter(cli.code).then(async |from| {
            let to = SRC_DIR.join("codeblocks").join(format!(
                "{prefix}_{file_name}",
                prefix = &*prefix,
                file_name = &from.file_name().ok()?.to_string_lossy()
            ));
            fs::copy(from, &to).await.map(|_| to)
        }),
    )
        .merge()
        .then(async |path| {
            let path = path?;
            let source = fs::read(&path).await?;
            let html = highlight(&path, source).map_err(io::Error::other)?;
            fs::write(&path.with_extension(HIGHLIGHTED_EXT), html)
                .await
                .map(|_| path)
        })
        .filter_map(|path| {
            let path = path.unwrap();
            code_component(path)
        })
        .fold(
            "use leptos::prelude::*;\nuse crate::ux::CodeBox;\n".into(),
            |mut acc: Vec<u8>, comp| {
                acc.append(&mut comp.into_bytes());
                acc
            },
        );

    let termblocks = (
        fs::read_dir(SRC_DIR.join("termblocks"))
            .await?
            .filter_map(|r| match r {
                Ok(p) if p.file_name().to_str()?.ends_with("DS_Store") => None,
                Ok(p) if p.file_name().to_str()?.ends_with(HIGHLIGHTED_EXT) => None,
                Ok(p) => Some(Ok(p.path())),
                Err(_) => None,
            }),
        stream::iter(cli.term).then(async |from| {
            let to = SRC_DIR.join("termblocks").join(format!(
                "{prefix}_{file_name}",
                prefix = &*prefix,
                file_name = &from.file_name().ok()?.to_string_lossy()
            ));
            fs::copy(from, &to).await.map(|_| to)
        }),
    )
        .merge()
        .then(async |path| {
            let path = path?;
            let term = termstyle(fs::read_to_string(&path).await?);
            fs::write(&path.with_extension(HIGHLIGHTED_EXT), term)
                .await
                .map(|_| path)
        })
        .filter_map(|path| {
            let path = path.unwrap();
            term_component(path)
        })
        .fold(
            "use leptos::prelude::*;\nuse crate::ux::TermBox;\n".into(),
            |mut acc: Vec<u8>, comp| {
                acc.append(&mut comp.into_bytes());
                acc
            },
        );

    (
        async {
            let mut modfile = fs::File::create(SRC_DIR.join("codeblock.rs")).await?;
            modfile.write_all(&codeblocks.await).await?;
            modfile.sync_all().await
        },
        async {
            let mut modfile = fs::File::create(SRC_DIR.join("termblock.rs")).await?;
            modfile.write_all(&termblocks.await).await?;
            modfile.sync_all().await
        },
    )
        .try_join()
        .await?;

    Ok(())
}

fn _only_non_hlt_paths(r: io::Result<DirEntry>) -> Option<io::Result<PathBuf>> {
    match r {
        Ok(p) if p.file_name().to_str()?.ends_with("DS_Store") => None,
        Ok(p) if p.file_name().to_str()?.ends_with(HIGHLIGHTED_EXT) => None,
        Ok(p) => Some(Ok(p.path())),
        Err(_) => None,
    }
}

fn code_component(path: PathBuf) -> Option<String> {
    let rel = path.strip_prefix(&SRC_DIR.as_path()).ok()?;
    format!(
        "
#[component]
pub fn {name}() -> impl IntoView {{
    view! {{
        <CodeBox
            raw=include_str!(\"{file}\")
            code=include_str!(\"{html}\")
        />
    }}
}}
",
        name = rel.file_stem()?.to_str()?.to_case(Case::Pascal),
        file = rel.to_str()?,
        html = rel.with_extension(HIGHLIGHTED_EXT).to_str()?
    )
    .into()
}

fn term_component(path: PathBuf) -> Option<String> {
    let rel = path.strip_prefix(&SRC_DIR.as_path()).ok()?;
    format!(
        "
#[component]
pub fn {name}(#[prop(optional)] tiny: bool) -> impl IntoView {{
    view! {{
        <TermBox
            tiny=tiny
            hlt=include_str!(\"{html}\")
        />
    }}
}}
",
        name = rel.file_stem()?.to_str()?.to_case(Case::Pascal),
        html = rel.with_extension(HIGHLIGHTED_EXT).to_str()?
    )
    .into()
}
