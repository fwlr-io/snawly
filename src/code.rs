use constcat::concat;
use phf::phf_map;
use std::{path::PathBuf, sync::LazyLock};
use tree_sitter_highlight::{Error, Highlight, HighlightConfiguration, Highlighter, HtmlRenderer};

pub fn highlight(path: &PathBuf, source: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut renderer: HtmlRenderer = HtmlRenderer::new();
    let mut highlighter: Highlighter = Highlighter::new();
    let config = config_for_file(path).unwrap();
    let html = {
        renderer.render(
            highlighter.highlight(config, &source, None, config_for)?,
            &source,
            &|hl, acc| {
                acc.extend(b"class=\"text-");
                acc.extend(get_highlight(&hl).as_bytes());
                acc.extend(b"\"");
            },
        )?;
        Ok(std::mem::take(&mut renderer.html))
    };
    renderer.reset();
    html
}

fn get_highlight(hl: &Highlight) -> &str {
    NAME_COLOR_MAP.get(NAMES[hl.0]).unwrap_or_else(|| {
        eprintln!("unrecognised: {}", NAMES[hl.0]);
        &"yellow"
    })
}

const NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "definition.class",
    "definition.constant",
    "definition.function",
    "definition.interface",
    "definition.macro",
    "definition.method",
    "definition.module",
    "doc",
    "embedded",
    "error",
    "escape",
    "function",
    "function.builtin",
    "function.macro",
    "function.method",
    "keyword",
    "label",
    "local.definition",
    "local.reference",
    "local.scope",
    "module",
    "name",
    "namespace",
    "none",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "reference.call",
    "reference.class",
    "reference.implementation",
    "reference.type",
    "string",
    "string.escape",
    "string.special",
    "string.special.key",
    "tag",
    "tag.attribute",
    "tag.delimiter",
    "tag.error",
    "text.literal",
    "text.reference",
    "text.title",
    "text.uri",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

const NAME_COLOR_MAP: phf::Map<&str, &str> = phf_map! {
    "module" => "dim-magenta",
    "keyword" => "dim-magenta",
    "constructor" => "dim-magenta",
    "error" => "red",
    "tag.error" => "red",
    "function" => "dim-red",
    "function.method" => "dim-red",
    "function.builtin" => "dim-red",
    "tag" => "dim-yellow",
    "string" => "green",
    "string.special" => "green",
    "attribute" => "dim-green",
    "function.macro" => "dim-green",
    "escape" => "cyan",
    "string.escape" => "cyan",
    "constant" => "cyan",
    "constant.builtin" => "cyan",
    "embedded" => "dim-cyan",
    "namespace" => "dim-cyan",
    "tag.attribute" => "dim-cyan",
    "number" => "blue",
    "boolean" => "blue",
    "variable" => "blue",
    "variable.builtin" => "blue",
    "variable.parameter" => "blue",
    "type" => "dim-blue",
    "type.builtin" => "dim-blue",
    "comment" => "grey",
    "comment.documentation" => "grey",
    "tag.delimiter" => "grey",
    "punctuation.delimiter" => "grey",
    "label" => "dim-white",
    "operator" => "dim-white",
    "property" => "dim-white",
    "punctuation" => "dim-white",
    "punctuation.bracket" => "dim-white"
};

pub fn config_for<'a>(ext_or_name: &str) -> Option<&'a HighlightConfiguration> {
    Some(match ext_or_name {
        "css" => &CSS,
        "csv" => &CSV,
        "dot" | "gv" => &DOT,
        "html" => &HTML,
        "json" => &JSON,
        "js" | "javascript" => &JAVASCRIPT,
        "jsx" => &JSX,
        "md" => &MARKDOWN_BLOCK,
        "toml" => &TOML,
        "ts" | "typescript" => &TYPESCRIPT,
        "tsx" => &TSX,
        "yml" | "yaml" => &YAML,
        "sh" | "shell" => &SHELL,
        "rs" | "rust" => &RUST_WITH_RSTML, // todo: if contains `view!` macro; else just `rust`
        x => {
            eprintln!("not a recognised extension: {x}");
            return None;
        }
    })
}

pub fn config_for_file(path: &PathBuf) -> Option<&'static HighlightConfiguration> {
    config_for(path.extension()?.to_str()?)
}

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

static CSV: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_csv::*;
    let mut config = HighlightConfiguration::new(
        language().into(),
        "csv",
        NO_HIGHLIGHTS_QUERY,
        NO_INJECTIONS_QUERY,
        NO_LOCALS_QUERY,
    )
    .unwrap();
    config.configure(NAMES);
    config
});

static DOT: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_dot::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "dot",
        HIGHLIGHTS_QUERY,
        INJECTIONS_QUERY,
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
static SHELL: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    use tree_sitter_bash::*;
    let mut config = HighlightConfiguration::new(
        LANGUAGE.into(),
        "shell",
        HIGHLIGHT_QUERY,
        NO_INJECTIONS_QUERY,
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

const NO_HIGHLIGHTS_QUERY: &str = "";
const NO_INJECTIONS_QUERY: &str = "";
const NO_LOCALS_QUERY: &str = "";
