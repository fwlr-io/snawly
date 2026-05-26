use fancy_regex::{Captures, Regex};

fn style_to_class(atr: &str, val: &str) -> String {
    if atr.ends_with("color") {
        format!(
            "{pfx}-[{sfx}]",
            pfx = if atr == "color" { "text" } else { "bg" },
            sfx = val.replace(" ", "")
        )
    } else if atr.starts_with("font-") {
        format!("font-{val}")
    } else {
        // intentionally ignoring underlines
        // warn if something else is ignored
        if !atr.starts_with("text-decoration-") {
            eprintln!("unrecognised: atr = '{atr}', val = '{val}'")
        }
        "".to_string()
    }
}

pub fn restyle(term: &str) -> String {
    let all_but_last_div_close = Regex::new(r#"<\/div>(?!$)"#).unwrap();
    let zed_links = Regex::new(r#"href="zed:\/\/file\/\/Users\/scottfowler([^"]*)"#).unwrap();
    let style_remapper = Regex::new(r#"style="([^"]*)""#).unwrap();

    let term = term.replace("<div style=\"display: inline;", "<span style=\"");
    let term = all_but_last_div_close.replace_all(&term, "</span>");
    let term = zed_links.replace_all(&term, "onclick='alert(\"opens `~$1` in your editor\");'");
    let term = term.replace("style=\"font-family: monospace; white-space: pre;background-color: #1c1b19;color: #fce8c3;\"", "class=\"whitespace-pre\"");

    style_remapper
        .replace(&term, |caps: &Captures| {
            caps[1]
                .split(";")
                .filter(|s| s.len() > 0)
                .filter_map(|p| p.split_once(":").map(|(a, b)| (a.trim(), b.trim())))
                .map(|(atr, val)| style_to_class(atr, val))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .to_string()
}
