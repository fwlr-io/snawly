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
  const modfileAppend = async (component: string) =>
    await appendFile("termblock.rs", component, "utf-8")

  for (let fname of args) {
    let term = await Bun.file(fname).text()
    let hlt = mapStyles(term)
    if (hlt) {
      let hltFile = fname.replace(/\.term$/, ".hlt")
      let compName = pascalCase(
        fname.replace(/^termblocks\//, "").replace(/\.term$/, "")
      )
      await Bun.write(hltFile, hlt)
      await modfileAppend(
        `
#[component]
pub fn ${compName}(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! { <TermBox tiny=tiny hlt=include_str!("${hltFile}") /> }
}

`
      )
    }
  }
})()
