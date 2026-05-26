use convert_case::{Case, Casing};
use std::{fs::DirEntry, path::Path};

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
    pub fn new(file: &Path) -> Self {
        let hlt = file.with_extension(HIGHLIGHTED_EXT);
        let sub_path = Path::new(file.parent().unwrap().file_name().unwrap());
        Self {
            file: file.to_str().unwrap().to_owned(),
            file_ext: file.extension().unwrap().to_str().unwrap().to_owned(),
            file_relative: sub_path
                .join(file.file_name().unwrap())
                .to_str()
                .unwrap()
                .to_owned(),
            hlt_file: hlt.to_str().unwrap().to_owned(),
            hlt_relative: sub_path
                .join(hlt.file_name().unwrap())
                .to_str()
                .unwrap()
                .to_owned(),
            component_name: file
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_case(Case::Pascal),
        }
    }

    pub fn maybe_from(p: DirEntry) -> Option<Self> {
        let file = p.path();
        match file.extension()?.to_str()? {
            HIGHLIGHTED_EXT => None,
            _ => Some(Hlt::new(file.as_path())),
        }
    }

    pub fn as_code_component(&self) -> String {
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
",
            component_name = self.component_name,
            file_relative = self.file_relative,
            hlt_relative = self.hlt_relative
        )
    }

    pub fn as_term_component(&self) -> String {
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
",
            component_name = self.component_name,
            hlt_relative = self.hlt_relative
        )
    }
}
