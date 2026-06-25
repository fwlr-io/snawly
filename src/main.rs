#[macro_use]
extern crate macro_rules_attribute;

use clap::{Parser, builder::ArgPredicate::IsPresent};
use convert_case::{Case, Casing};
use futures_concurrency::prelude::*;
use smol::{fs, io, prelude::*, stream};
use smol_macros::{Executor, main};
use std::{collections::HashMap, path::PathBuf};

use snawly::code;
use snawly::ok_into_result::*;
use snawly::term;

#[derive(Parser, Debug)]
/// A tool for putting source code and terminal captures on the fwlr.io website.
/// Use `-c/--code` for source code, `-t/--term` for terminal captures.
/// With or without flags, it will always rebuild the contents of `blocks`.
///
///
/// For source code, it uses tree-sitter to syntax highlight
/// some common languages. For example,
///     `snawly foo --code main.rs ok_into_result.rs`
/// will copy two files:
///     `blocks/foo_main.rs`        `blocks/foo_ok-into-result.rs`
/// produce two more files:
///     `blocks/foo_main.hlhtml`    `blocks/foo_ok-into-result.hlhtml`
/// and `pub mod` two Leptos components:
///     `blocks::foo::Main`         `blocks::foo::OkIntoResult`
///
/// For terminal captures, it translates ghostty's html styling into
/// tailwind classes and extracts a plain text version. For example,
///     `snawly bar --term fastGrep /tmp/jhlkjl/selection.txt`
/// will copy the tempfile and rename it:
///     `blocks/bar_fast-grep.term`
/// produce two files:
///     `blocks/bar_fast-grep.hlhtml`   `blocks/bar_fast-grep.txterm`
/// and will `pub mod` one Leptos component:
///     `blocks::bar::FastGrep`
///
/// For terminal capture, a name is mandatory, so the prefix is optional:
///     `snawly --term slowGrep /tmp/akgkf/selection.txt`
/// will copy the tempfile and rename it:
///     `blocks/term_slow-grep.term`
/// produce two files:
///     `blocks/term_slow-grep.hlhtml`   `blocks/term_slow-grep.txterm`
/// and will `pub mod` one Leptos component:
///     `blocks::term::SlowGrep`
///
struct Cli {
    /// A prefix to use for the file(s) to be copied, to avoid name collisions.
    #[arg(num_args = 0..=1, default_value_if("term", IsPresent, Some("term")))]
    prefix: Option<String>,
    /// Files containing code to be copied and syntax-highlighted.
    #[arg(short, long, num_args = 1.., value_name = "FILES", requires = "prefix")]
    code: Vec<PathBuf>,
    // A name for the terminal output file, and the tempfile containing
    // html captures of terminal output.
    #[arg(short, long, num_args = 2, value_names = ["NAME", "TEMPFILE"], value_parser = parse_term_args)]
    term: Vec<(String, PathBuf)>,
}
fn parse_term_args(term: &str) -> io::Result<(String, PathBuf)> {
    term.split_once(" ")
        .and_then(|(name, path)| Some((String::from(name), PathBuf::from(path))))
        .ok_or(io::Error::other(
            "Failed parsing arguments for --term flag. Name first, then tempfile.",
        ))
}

const TERM_EXT: &str = "term";
const HLHTML_EXT: &str = "hlhtml";
const ROOT: &str = "/Users/scottfowler/dev/website/src/";

#[apply(main!)]
async fn main(ex: &Executor) -> io::Result<()> {
    let cli = Cli::parse();
    let prefix = cli.prefix.unwrap_or("".into()).to_case(Case::Snake);
    let src = PathBuf::from(ROOT).join("blocks");
    let to_blocks = |name: &str| src.join(format!("{prefix}_{}", name.to_case(Case::Kebab)));

    let move_new_blocks = (
        stream::iter(cli.term)
            .map(|(name, from)| (to_blocks(&name).with_extension(TERM_EXT), from)),
        stream::iter(cli.code)
            .map(|from| (to_blocks(from.file_name().unwrap().to_str().unwrap()), from)),
    )
        .merge()
        .co()
        .try_for_each(async |(to, from)| fs::copy(from, to).await.and(Ok(())));

    move_new_blocks.await?;

    let convert_all_blocks = fs::read_dir(&src)
        .await?
        .filter_map(|d| {
            d.ok().and_then(|p| match p {
                _ if p.file_name().to_str()?.ends_with(HLHTML_EXT) => None,
                _ => Some(p.path()),
            })
        })
        .then(async |path| {
            let source = fs::read(&path).await?;

            let converted: Vec<u8> = if path.extension().ok()?.to_str().ok()? == "term" {
                term::restyle(String::from_utf8(source).map_err(io::Error::other)?).into()
            } else {
                code::highlight(&path, source).map_err(io::Error::other)?
            };

            fs::write(&path.with_extension(HLHTML_EXT), converted)
                .await
                .and(Ok(path))
        });

    let modstore = convert_all_blocks
        .fold(HashMap::<String, String>::new(), |mut acc, el| {
            let (prefix, comp) = prefixed_component_for(el.unwrap()).unwrap();
            acc.entry(prefix)
                .or_insert_with_key(|prefix| {
                    format!("\npub mod {prefix} {{\n    use leptos::prelude::*;\n")
                })
                .push_str(&comp);
            acc
        })
        .await;

    let mut modfile = fs::File::create(PathBuf::from(ROOT).join("block.rs")).await?;
    modfile.write_all(&gather(modstore)).await?;
    modfile.sync_all().await?;

    Ok(())
}

fn prefixed_component_for(path: PathBuf) -> Option<(String, String)> {
    let (prefix, name) = path.file_stem()?.to_str()?.rsplit_once("_")?;
    let name = name.to_case(Case::Pascal);
    let rel = path.strip_prefix(&ROOT).ok()?.with_extension(HLHTML_EXT);
    let hlt = rel.to_str()?;
    Some((
        prefix.to_case(Case::Snake),
        format!(
            "
#[component]
pub fn {name}() -> impl IntoView {{
    view! {{
        <crate::ux::SourceBox hlt=include_str!(\"{hlt}\") />
    }}
}}
"
        ),
    ))
}

fn gather(modstore: HashMap<String, String>) -> Vec<u8> {
    modstore
        .into_values()
        .map(|mut s| {
            s.push_str("]\n");
            s
        })
        .collect::<String>()
        .into_bytes()
}
