const REPLACEMENTS = [
  ['<div style="display: inline;', '<span style="'],
  [/<\/div>(?!$)/g, "</span>"],
  [
    /href="zed:\/\/file\/\/Users\/scottfowler([^"]*)"/g,
    "onclick='alert(\"opens `~$1` in your editor\");'",
  ],
  [
    'style="font-family: monospace; white-space: pre;background-color: #1c1b19;color: #fce8c3;"',
    'class="whitespace-pre"',
  ],
] as const

const styleToClass = (atr: string, val: string): string | null => {
  if (atr.endsWith("color")) {
    return `${atr === "color" ? "text" : "bg"}-[${val.replaceAll(" ", "")}]`
  } else if (atr.startsWith("font-")) {
    return `font-${val}`
  } else if (atr.startsWith("text-decoration-")) {
    return null
  } else {
    console.error(`${atr}: ${val}`)
    return null
  }
}

export const mapStyles = (term: string): string | null =>
  REPLACEMENTS.reduce(
    (acc, [raw, rep]) => acc.replaceAll(raw, rep),
    term
  ).replaceAll(
    /style="([^"]*)"/g,
    (_, m: string) =>
      `class="${m
        .split(";")
        .filter(s => s.length)
        .map(p => p.split(":").map(s => s.trim()))
        .flatMap(([atr, val]) => styleToClass(atr!, val!) ?? [])
        .join(" ")}"`
  )
