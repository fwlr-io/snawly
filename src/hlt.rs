use convert_case::{Case, Casing};
use std::path::Path;

const HIGHLIGHTED_EXT: &str = "hlt";

pub struct Hlt {
    pub file: String,
    pub file_ext: String,
    pub file_relative: String,
    pub hlt_file: String,
    pub hlt_relative: String,
    pub component_name: String,
}

impl Hlt {
    pub fn try_from(file: &Path) -> Option<Self> {
        if file.extension()?.to_str()? == HIGHLIGHTED_EXT {
            return None;
        }
        let hlt = file.with_extension(HIGHLIGHTED_EXT);
        let sub_path = Path::new(file.parent()?.file_name()?);
        Some(Self {
            file: file.to_str()?.into(),
            file_ext: file.extension()?.to_str()?.into(),
            file_relative: sub_path.join(file.file_name()?).to_str()?.into(),
            hlt_file: hlt.to_str()?.into(),
            hlt_relative: sub_path.join(hlt.file_name()?).to_str()?.into(),
            component_name: file.file_stem()?.to_str()?.to_case(Case::Pascal),
        })
    }

    pub fn as_code_component(&self) -> String {
        let Self {
            component_name,
            file_relative,
            hlt_relative,
            ..
        } = self;
        format!(
            "
#[component]
pub fn {component_name}() -> impl IntoView {{
    view! {{
        <CodeBox
            raw=include_str!(\"{file_relative}\")
            code=include_str!(\"{hlt_relative}\")
        />
    }}
}}
"
        )
    }

    pub fn as_term_component(&self) -> String {
        let Self {
            component_name,
            hlt_relative,
            ..
        } = self;
        format!(
            "
#[component]
pub fn {component_name}(#[prop(optional)] tiny: bool) -> impl IntoView {{
    view! {{
        <TermBox
            tiny=tiny
            hlt=include_str!(\"{hlt_relative}\")
        />
    }}
}}
"
        )
    }
}
