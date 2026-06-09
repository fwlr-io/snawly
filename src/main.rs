#[macro_use]
extern crate macro_rules_attribute;

use futures_concurrency::prelude::*;
use smol::{fs, io, pin, prelude::*, stream};
use smol_macros::{Executor, main};
use std::{path::PathBuf, sync::LazyLock};

mod highlight;
use highlight::{config_for, config_for_file, get_highlight};

mod termstyle;
use termstyle::restyle;

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

    let (mut codeblock_modfile, mut termblock_modfile) = (
        async {
            let mut file = fs::File::create(SRC_DIR.join("codeblock.rs")).await?;
            file.write_all(b"use leptos::prelude::*;\nuse crate::ux::CodeBox;\n")
                .await
                .and(Ok(file))
        },
        async {
            let mut file = fs::File::create(SRC_DIR.join("termblock.rs")).await?;
            file.write_all(b"use leptos::prelude::*;\nuse crate::ux::TermBox;\n")
                .await
                .and(Ok(file))
        },
    )
        .try_join()
        .await?;

    let codeblocks = (
        stream::iter(cli.code).then(async |from| {
            let to = SRC_DIR.join("codeblocks").join(format!(
                "{prefix}_{file_name}",
                prefix = &*prefix,
                file_name = &from.file_name().unwrap().to_string_lossy()
            ));
            fs::copy(from, &to).await.map(|_| to).unwrap()
        }),
        fs::read_dir(SRC_DIR.join("codeblocks"))
            .await
            .unwrap()
            .map(|d| d.unwrap().path()),
    )
        .merge()
        .then(async |path| {
            let p = path.clone();
            let config = config_for_file(&p);
            let cfg = config.unwrap();
            let source = fs::read(&path).await;
            let highlights = Highlighter::new()
                .highlight(config.unwrap(), &source.unwrap(), None, config_for)
                .expect("");
            (source.unwrap(), path, highlights)
        })
        .map(|(source, path, highlights)| {
            let mut renderer = HtmlRenderer::new();
            renderer
                .render(highlights, &source, &|hl, acc| {
                    acc.extend(b"class=\"text-");
                    acc.extend(get_highlight(&hl).as_bytes());
                    acc.extend(b"\"");
                })
                .unwrap();
            let html = std::mem::take(&mut renderer.html);
            renderer.reset();
            (path, html)
        })
        .then(async |(path, html)| {
            let relative = path.strip_prefix(SRC_DIR.as_path()).unwrap().to_owned();
            let comp = code_component(&relative);
            comp
        })
        .collect::<Vec<String>>()
        .await
        .join("\n");

    //     let highlights = highlighter
    //         .highlight(config_for_file(&path).unwrap(), &source, None, config_for)
    //         .unwrap();

    //     renderer
    //         .render(highlights, &source, &|hl, acc| {
    //             acc.extend(b"class=\"text-");
    //             acc.extend(get_highlight(&hl).as_bytes());
    //             acc.extend(b"\"");
    //         })
    //         .unwrap();
    //     let html = std::mem::take(&mut renderer.html);
    //     renderer.reset();
    //     (file, html)
    // });

    // pin!(codeblock_mods);

    // codeblock_mods
    //     .then(async |(file, html)| {
    //         let dest = file
    //             .with_extension(HIGHLIGHTED_EXT)
    //             .to_string_lossy()
    //             .into_owned();
    //         let comp = code_component(&file).ok()?;
    //         fs::write(dest, html).await.map(|_| comp)
    //     })
    //     .fold(Vec::<String>::new(), |mut acc, comp| {
    //         acc.extend(comp);
    //         acc
    //     })
    //     .await;

    // pin!(codeblock_mods);

    // while let Some(comp) = codeblock_mods.try_next().await? {

    // }

    // codeblock_mods.fold(async |s| codeblock_modfile.write_all(&s?.as_bytes()).await);

    let termblocks = stream::iter(cli.term)
        .then(async |from: PathBuf| {
            let to = SRC_DIR.join("termblocks").join(format!(
                "{prefix}_{file_name}",
                prefix = &*prefix,
                file_name = &from.file_name().unwrap().to_string_lossy()
            ));
            fs::copy(from, &to).await.map(|_| to)
        })
        .merge(
            fs::read_dir(SRC_DIR.join("termblocks"))
                .await?
                .map(|d| d.map(|f| f.path())),
        );

    // pin!(codeblock_mods);
    // while let Some(r) = codeblock_mods.try_next().await? {
    //     codeblock_modfile.write_all(&r.into_bytes()).await?;
    // }

    // stream::once(codeblock_mods.fold(
    //     String::from("use leptos::prelude::*;\nuse crate::ux::CodeBox;\n"),
    //     |mut acc: String, el| {
    //         acc.extend(el.ok().unwrap().into());
    //         acc
    //     },
    // ))
    // .then(async |s| {
    //     let ss = s.await;
    // });

    Ok(())
}

fn code_component(rel: &PathBuf) -> String {
    let hlt = rel.with_extension(HIGHLIGHTED_EXT);
    let stem = rel.file_stem().unwrap();
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
        name = stem.to_string_lossy().to_case(Case::Pascal),
        file = rel.to_string_lossy(),
        html = hlt.to_string_lossy(),
    )
    .into()
}

fn term_component(rel: &PathBuf) -> String {
    let hlt = rel.with_extension(HIGHLIGHTED_EXT);
    let stem = rel.file_stem().unwrap();
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
        name = stem.to_string_lossy().to_case(Case::Pascal),
        html = hlt.to_string_lossy()
    )
    .into()
}

// pub fn inner_main(termblocks: Vec<Hlt>) -> Result<(), Box<dyn std::error::Error>> {
//     // Termblocks

//     for entry in termblocks.into_iter() {
//         // async-able
//         let source = std::fs::read_to_string(&entry.file)?;
//         // async-able
//         let restyled = restyle(source);
//         // async-able
//         std::fs::write(&entry.hlt_file, restyled)?;
//         // async-able
//         termblock_modfile.write_all(entry.as_term_component().as_bytes())?;
//     }

//     Ok(())
// }
