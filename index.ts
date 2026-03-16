import { mapStyles } from "./src/restyle"
import { appendFile } from "fs/promises"
import { pascalCase } from "change-case"
import { parseArgs } from "util"

const [_, __, ...args] = parseArgs({
  args: Bun.argv,
  allowPositionals: true,
}).positionals

;(async () => {
  await Bun.write(
    "termblock.rs",
    "use crate::ux::TermBox;\nuse leptos::prelude::*;\n"
  )

  for (let termFile of args) {
    let hltFile = termFile.replace(/\.term$/, ".hlt")
    let term = await Bun.file(termFile).text()
    let hlt = mapStyles(term)
    let compName = pascalCase(
      termFile.replace(/^termblocks\//, "").replace(/\.term$/, "")
    )
    let component = `
#[component]
pub fn ${compName}(#[prop(optional)] tiny: bool) -> impl IntoView {
  view! { <TermBox tiny=tiny hlt=include_str!("${hltFile}") /> }
}

`
    if (hlt) {
      await Bun.write(hltFile, hlt)
      appendFile("termblock.rs", component, "utf-8")
    }
  }
})()
