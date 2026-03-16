const STYLEMAP = [
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
  [
    'style="color: rgb(224, 44, 109);text-decoration-line: underline;text-decoration-style: solid;"',
    'class="text-dim-magenta"',
  ],
  [
    'style="color: rgb(81, 159, 80);text-decoration-line: underline;text-decoration-style: solid;"',
    'class="text-dim-green"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(51, 49, 42);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  ['style="color: rgb(95, 95, 95);"', 'class="text-dim-grey"'],
  ['style="color: rgb(145, 129, 117);"', 'class="text-mid-grey"'],
  [
    'style="color: rgb(252, 232, 195);font-weight: bold;"',
    'class="font-bold text-white"',
  ],
  [
    'style="color: rgb(145, 129, 117);font-style: italic;"',
    'class="font-italic text-mid-grey"',
  ],
  ['style="color: rgb(252, 232, 195);"', 'class="text-white"'],
  ['style="color: rgb(48, 48, 48);"', 'class="text-mid-black"'],
  [
    'style="color: rgb(252, 232, 195);background-color: rgb(63, 0, 1);"',
    'class="text-white bg-deep-red"',
  ],
  [
    'style="color: rgb(239, 47, 39);background-color: rgb(63, 0, 1);"',
    'class="bg-deep-red text-dim-red"',
  ],
  ['style="background-color: rgb(63, 0, 1);"', 'class="bg-deep-red"'],
  [
    'style="color: rgb(252, 232, 195);background-color: rgb(0, 40, 0);"',
    'class="text-white bg-deep-green"',
  ],
  [
    'style="color: rgb(239, 47, 39);background-color: rgb(0, 40, 0);"',
    'class="text-dim-red bg-deep-green"',
  ],
  [
    'style="color: rgb(224, 44, 109);font-weight: bold;"',
    'class="font-bold text-dim-magenta"',
  ],
  ['style="background-color: rgb(0, 40, 0);"', 'class="bg-deep-green"'],
  [
    'style="color: rgb(215, 0, 95);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="font-bold text-magenta bg-mid-black"',
  ],
  ['style="background-color: rgb(48, 48, 48);"', 'class="bg-mid-black"'],
  [
    'style="color: rgb(228, 228, 228);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="font-bold bg-mid-black text-white"',
  ],
  [
    'style="color: rgb(239, 47, 39);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="font-bold text-dim-red bg-mid-black"',
  ],
  [
    'style="color: rgb(28, 27, 25);font-weight: bold;"',
    'class="font-bold text-mid-black"',
  ],
  [
    'style="color: rgb(186, 166, 127);background-color: rgb(224, 44, 109);"',
    'class="bg-dim-magenta text-dim-white"',
  ],
  ['style="color: rgb(95, 215, 255);"', 'class="text-cyan"'],
  ['style="color: rgb(0, 255, 135);"', 'class="text-green"'],
  ['style="background-color: rgb(239, 47, 39);"', 'class="bg-dim-red"'],
  [
    'style="color: rgb(224, 44, 109);text-decoration-line: underline;text-decoration-style: solid;font-weight: bold;"',
    'class="font-bold text-dim-magenta"',
  ],
  [
    'style="color: rgb(252, 232, 195);font-weight: bold;font-style: italic;"',
    'class="font-bold font-italic text-white"',
  ],
  [
    'style="color: rgb(239, 47, 39);font-weight: bold;font-style: italic;"',
    'class="font-bold font-italic text-dim-red"',
  ],
  [
    'style="color: rgb(186, 166, 127);font-style: italic;"',
    'class="font-italic text-dim-white"',
  ],
  ['style="color: rgb(158, 186, 194);"', 'class="text-mid-grey"'],
  [
    'style="color: rgb(44, 120, 191);background-color: rgb(63, 0, 1);"',
    'class="bg-deep-red text-dim-blue"',
  ],
  [
    'style="color: rgb(44, 120, 191);background-color: rgb(0, 40, 0);"',
    'class="bg-deep-green text-dim-blue"',
  ],
  [
    'style="color: rgb(252, 232, 195);background-color: rgb(81, 159, 80);font-weight: bold;"',
    'class="font-bold bg-dim-green text-white"',
  ],
  [
    'style="color: rgb(252, 232, 195);background-color: rgb(239, 47, 39);font-weight: bold;"',
    'class="font-bold text-white bg-dim-red"',
  ],
  [
    'style="color: rgb(239, 47, 39);background-color: rgb(239, 47, 39);font-weight: bold;"',
    'class="font-bold text-dim-red bg-dim-red"',
  ],
  [
    'style="color: rgb(43, 228, 208);font-weight: bold;"',
    'class="font-bold text-cyan"',
  ],
  [
    'style="color: rgb(224, 44, 109);text-decoration-line: underline;text-decoration-style: solid;font-style: italic;"',
    'class="font-italic text-dim-magenta"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(145, 129, 117);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-grey"',
  ],
  [
    'style="color: rgb(10, 174, 179);background-color: rgb(28, 27, 25);font-weight: bold;"',
    'class="font-bold text-dim-cyan bg-black"',
  ],
  [
    'style="color: rgb(252, 232, 195);background-color: rgb(145, 129, 117);font-weight: bold;"',
    'class="font-bold text-white bg-mid-grey"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(78, 69, 63);font-weight: bold;"',
    'class="font-bold text-cyan bg-dim-grey"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(44, 42, 38);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(51, 51, 50);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(51, 49, 45);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(51, 49, 41);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(51, 49, 49);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="color: rgb(43, 228, 208);background-color: rgb(44, 38, 38);font-weight: bold;"',
    'class="font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="color: rgb(135, 175, 215);font-weight: bold;"',
    'class="font-bold text-blue"',
  ],
  ['style="color: rgb(135, 175, 215);"', 'class="text-blue"'],
  ['style="color: rgb(135, 175, 135);"', 'class="text-dim-green"'],
  [
    'style="color: rgb(175, 215, 175);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="font-bold bg-mid-black text-green"',
  ],
  ['style="color: rgb(117, 113, 94);"', 'class="text-grey"'],
  ['style="opacity: 0.5;"', 'class="opacity-50"'],
  ['style="color: rgb(239, 47, 39);"', 'class="text-dim-red"'],
  ['style="color: rgb(81, 159, 80);"', 'class="text-dim-green"'],
  ['style="color: rgb(10, 174, 179);"', 'class="text-dim-cyan"'],
  ['style="color: rgb(44, 120, 191);"', 'class="text-dim-blue"'],
  ['style="color: rgb(251, 184, 41);"', 'class="text-dim-yellow"'],
  ['style="color: rgb(224, 44, 109);"', 'class="text-dim-magenta"'],
  ['style="font-weight: bold;"', 'class="font-bold"'],
  ['style="font-weight: bold;opacity: 0.5;"', 'class="font-bold opacity-50"'],
  ['style="text-decoration-line: underline;text-decoration-style: solid;"', ""],
  [
    'style="color: rgb(239, 47, 39);font-weight: bold;"',
    'class="text-dim-red font-bold"',
  ],
  [
    'style="color: rgb(81, 159, 80);font-weight: bold;"',
    'class="text-dim-green font-bold"',
  ],
  [
    'style="color: rgb(44, 120, 191);font-weight: bold;"',
    'class="text-dim-blue font-bold"',
  ],
  [
    'style="color: rgb(251, 184, 41);font-weight: bold;"',
    'class="text-dim-yellow font-bold"',
  ],
  [
    'style="color: rgb(145, 129, 117);font-weight: bold;"',
    'class="text-mid-grey font-bold"',
  ],
  [
    'style="color: rgb(251, 184, 41);text-decoration-line: underline;text-decoration-style: solid;font-weight: bold;"',
    'class="text-dim-yellow font-bold"',
  ],
] as const

export const mapStyle = (term: string) => {
  const res = STYLEMAP.reduce(
    (acc, [raw, rep]) => acc.replaceAll(raw, rep),
    term
  )
  const remains = new Set(res.matchAll(/style="[^"]+"/g).map(m => m[0]))

  if (remains.size) {
    console.error(`Unhandled style declarations!`)
    for (let rem of remains) {
      console.log("  " + rem)
    }
    return null
  }

  return res
}
