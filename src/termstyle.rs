use fancy_regex::{Captures, Regex};
use std::sync::LazyLock;

pub fn termstyle(term: String) -> String {
    let term = term.replace(PARENT_STYLE, PARENT_CLASS);
    let term = term.replace(DIV_OPEN, SPAN_OPEN);
    let term = DIV_CLOSE.replace_all(&term, SPAN_CLOSE);
    let term = ZED_LINK.replace_all(&term, WINDOW_ALERT);

    STYLE_REMAPPER
        .replace_all(&term, |caps: &Captures| {
            format!(
                "class=\"{}\"",
                caps[1]
                    .split(";")
                    .filter(|s| !s.is_empty())
                    .filter_map(style_to_class)
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        })
        .replace(" class=\" \"", "")
}

fn style_to_class(p: &str) -> Option<String> {
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

const DIV_OPEN: &str = "<div style=\"display: inline;";
const SPAN_OPEN: &str = "<span style=\"";

static DIV_CLOSE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<\/div>(?!$)"#).unwrap());
const SPAN_CLOSE: &str = "</span>";

static ZED_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"href="zed:\/\/file\/\/Users\/scottfowler([^"]*)""#).unwrap());
const WINDOW_ALERT: &str = "onclick='alert(\"opens `~$1` in your editor\");'";

const PARENT_STYLE: &str =
    "style=\"font-family: monospace; white-space: pre;background-color: #1c1b19;color: #fce8c3;\"";
const PARENT_CLASS: &str = "class=\"whitespace-pre\"";

static STYLE_REMAPPER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"style="([^"]*)""#).unwrap());
