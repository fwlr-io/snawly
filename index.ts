import { mapStyles } from "./src/stylemap"
import { appendFile } from "fs/promises"
import { pascalCase } from "change-case"
import { parseArgs } from "util"

const [_, __, ...targs] = parseArgs({
  args: Bun.argv,
  allowPositionals: true,
}).positionals

let nameFrom = (out: string): string =>
  pascalCase(out.replace("termblocks/", "").replace(".thlt", ""))

let component = (out: string): string => `
#[component]
pub fn ${nameFrom(out)}() -> impl IntoView {
  view! { <Term term=include_str!("${out}") /> }
}

`

;(async () => {
  await Bun.write(
    "termblock.rs",
    "use crate::ux::Term;\nuse leptos::prelude::*;\n"
  )

  for (let fname of targs) {
    let term = await Bun.file(fname).text()
    let out = fname.replace(".term", ".thlt")
    let { ok, res } = mapStyles(term)
    if (ok) {
      await Bun.write(out, res)
      await appendFile("termblock.rs", component(out), "utf-8")
    }
  }
})()
