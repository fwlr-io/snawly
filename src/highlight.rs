use std::{collections::HashMap, sync::LazyLock};

use crate::names::NAMES;
use tree_sitter_highlight::Highlight;

const COLORS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("module", "dim-magenta"),
        ("keyword", "dim-magenta"),
        ("constructor", "dim-magenta"),
        ("tag.error", "red"),
        ("function", "dim-red"),
        ("function.method", "dim-red"),
        ("function.builtin", "dim-red"),
        ("tag", "dim-yellow"),
        ("string", "green"),
        ("string.special", "green"),
        ("attribute", "dim-green"),
        ("function.macro", "dim-green"),
        ("escape", "cyan"),
        ("string.escape", "cyan"),
        ("constant", "cyan"),
        ("constant.builtin", "cyan"),
        ("tag.attribute", "dim-cyan"),
        ("number", "blue"),
        ("boolean", "blue"),
        ("variable", "blue"),
        ("variable.parameter", "blue"),
        ("type", "dim-blue"),
        ("type.builtin", "dim-blue"),
        ("comment", "grey"),
        ("comment.documentation", "grey"),
        ("tag.delimiter", "grey"),
        ("punctuation.delimiter", "grey"),
        ("label", "dim-white"),
        ("operator", "dim-white"),
        ("property", "dim-white"),
        ("punctuation", "dim-white"),
        ("punctuation.bracket", "dim-white"),
    ])
});

fn color_for(tag: &str) -> Vec<u8> {
    format!(
        "class=\"text-{}\"",
        COLORS.get(tag).unwrap_or_else(|| {
            eprintln!("unrecognised: {tag}");
            &"yellow"
        })
    )
    .into_bytes()
}

pub fn apply_highlight(hl: Highlight, acc: &mut Vec<u8>) {
    let tag = *NAMES.get(hl.0).unwrap();
    acc.extend(color_for(tag))
}
