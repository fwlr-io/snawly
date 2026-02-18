use crate::names::NAMES;
use constcat::concat;
use std::sync::LazyLock;
use tree_sitter_highlight::HighlightConfiguration;

const NO_INJECTIONS_QUERY: &str = "";
const NO_LOCALS_QUERY: &str = "";

static CSS: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_css::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "css",
        HIGHLIGHTS_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});

static HTML: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_html::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "html",
        HIGHLIGHTS_QUERY,
        INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static JSON: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_json::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "json",
        HIGHLIGHTS_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static JAVASCRIPT: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_javascript::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "js",
        HIGHLIGHT_QUERY,
        INJECTIONS_QUERY,
        LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static JSX: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_javascript::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "jsx",
        concat!(JSX_HIGHLIGHT_QUERY, HIGHLIGHT_QUERY),
        INJECTIONS_QUERY,
        LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static MARKDOWN_BLOCK: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_md::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "md",
        HIGHLIGHT_QUERY_BLOCK,
        INJECTION_QUERY_BLOCK,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static RUST_WITH_RSTML: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_rstml::*;
    let mut config = HighlightConfiguration::new(
        language_rust_with_rstml(),
        "rust",
        HIGHLIGHTS_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static _RSTML: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_rstml::*;
    let mut config = HighlightConfiguration::new(
        language_rstml(),
        "rstml",
        HIGHLIGHTS_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static _RUST: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_rust::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "rust",
        HIGHLIGHTS_QUERY,
        INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static TOML: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_toml::*;
    let mut config = HighlightConfiguration::new(
        language(),
        "toml",
        HIGHLIGHT_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static TYPESCRIPT: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_javascript as js;
    use tree_sitter_typescript::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE_TYPESCRIPT.into(),
        "ts",
        concat!(HIGHLIGHTS_QUERY, js::HIGHLIGHT_QUERY),
        js::INJECTIONS_QUERY,
        concat!(LOCALS_QUERY, js::LOCALS_QUERY),
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static TSX: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_javascript as js;
    use tree_sitter_typescript::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE_TSX.into(),
        "tsx",
        concat!(
            js::JSX_HIGHLIGHT_QUERY,
            HIGHLIGHTS_QUERY,
            js::HIGHLIGHT_QUERY
        ),
        js::INJECTIONS_QUERY,
        concat!(LOCALS_QUERY, js::LOCALS_QUERY),
    )
    .unwrap();
    config.configure(NAMES);
    config
});
static YAML: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_yaml::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "yaml",
        HIGHLIGHTS_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});

pub fn config_for(name: &str) -> Option<&'static HighlightConfiguration> {
    Some(match name {
        "css" => &CSS,
        "html" => &HTML,
        "json" => &JSON,
        "js" | "javascript" => &JAVASCRIPT,
        "jsx" => &JSX,
        "md" => &MARKDOWN_BLOCK,
        "toml" => &TOML,
        "ts" | "typescript" => &TYPESCRIPT,
        "tsx" => &TSX,
        "yml" | "yaml" => &YAML,
        "rs" | "rust" => &RUST_WITH_RSTML, // todo: if contains `view!` macro; else just `rust`
        x => {
            eprintln!("not a recognised extension: {x}");
            return None;
        }
    })
}
