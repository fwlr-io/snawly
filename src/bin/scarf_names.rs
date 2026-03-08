use fancy_regex::Regex;
use std::collections::HashSet;

const QUERIES: &[&str] = &[
    tree_sitter_bash::HIGHLIGHT_QUERY,
    tree_sitter_css::HIGHLIGHTS_QUERY,
    tree_sitter_dot::HIGHLIGHTS_QUERY,
    tree_sitter_html::HIGHLIGHTS_QUERY,
    tree_sitter_javascript::HIGHLIGHT_QUERY,
    tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
    tree_sitter_javascript::LOCALS_QUERY,
    tree_sitter_javascript::TAGS_QUERY,
    tree_sitter_json::HIGHLIGHTS_QUERY,
    tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
    tree_sitter_rstml::HIGHLIGHTS_QUERY,
    tree_sitter_rust::HIGHLIGHTS_QUERY,
    tree_sitter_rust::TAGS_QUERY,
    tree_sitter_toml::HIGHLIGHT_QUERY,
    tree_sitter_typescript::HIGHLIGHTS_QUERY,
    tree_sitter_typescript::LOCALS_QUERY,
    tree_sitter_typescript::TAGS_QUERY,
    tree_sitter_yaml::HIGHLIGHTS_QUERY,
    "@punctuation",
    "@module",
];

const NEW_QUERIES: &[&str] = &[];

fn names_from(queries: &[&str]) -> HashSet<String> {
    let mut name_set = HashSet::new();
    let find_names = Regex::new(r#"(?<!")@[\w\.]+(?!")"#).unwrap();

    for query in queries {
        for m in find_names.find_iter(query) {
            let s = m.unwrap().as_str().strip_prefix("@").unwrap();
            name_set.insert(s.to_owned());
        }
    }

    name_set
}

fn format_names(name_set: HashSet<String>) -> Vec<String> {
    let mut names = name_set
        .iter()
        .map(|s| format!("    \"{s}\","))
        .collect::<Vec<_>>();
    names.sort();
    names
}

pub fn main() {
    let names = names_from(QUERIES);
    let new_names = names_from(NEW_QUERIES);
    for name in new_names.difference(&names) {
        eprintln!("new name: {name}");
    }

    let namestrings = format_names(names);
    let res = namestrings.join("\n");
    println!("const NAMES: &[&str] = &[\n{res}\n];");
}
