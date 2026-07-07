use fancy_regex::{Captures, Regex};
use std::sync::LazyLock;

pub fn restyle(term: String) -> String {
    let term = term.trim_prefix(PARENT_OPEN).replace(DIV_OPEN, SPAN_OPEN);
    let term = term.trim_suffix(DIV_CLOSE).replace(DIV_CLOSE, SPAN_CLOSE);
    let term = ZED_LINK.replace_all(&term, WINDOW_ALERT);
    let term = STYLE_REMAPPER.replace_all(&term, style_to_class);
    let term = EMPTY_CLASS.replace_all(&term, "");
    term.into()
}

const PARENT_OPEN: &str = "<div style=\"font-family: monospace; white-space: pre;background-color: #1c1b19;color: #fce8c3;\">";
const DIV_OPEN: &str = "<div style=\"display: inline;";
const SPAN_OPEN: &str = "<span style=\"";

const DIV_CLOSE: &str = "</div>";
const SPAN_CLOSE: &str = "</span>";

static ZED_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"href="zed:\/\/file\/\/Users\/scottfowler([^"]*)""#).unwrap());
const WINDOW_ALERT: &str = "onclick='alert(\"opens `~$1` in your editor\");'";

static STYLE_REMAPPER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"style="([^"]*)""#).unwrap());

fn style_to_class(caps: &Captures) -> String {
    format!(
        "class=\"{}\"",
        caps[1]
            .split(";")
            .filter_map(to_tailwind)
            .collect::<Vec<String>>()
            .join(" ")
    )
}

fn to_tailwind(p: &str) -> Option<String> {
    let (a, v) = p.split_once(":")?;
    match (a.trim(), v.trim().replace(" ", "")) {
        ("color", val) => Some(format!("text-[{val}]")),
        (atr, val) if atr.ends_with("color") => Some(format!("bg-[{val}]")),
        (atr, val) if atr.starts_with("font-") => Some(format!("font-{val}")),
        (atr, _) if atr.starts_with("text-decoration") => None,
        (atr, val) => {
            eprintln!("unrecognised: atr = '{atr}', val = '{val}'");
            None
        }
    }
}

static EMPTY_CLASS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\s*class="\s*""#).unwrap());
