const STYLEMAP = [
  [
    'style="display: inline;color: rgb(224, 44, 109);text-decoration-line: underline;text-decoration-style: solid;"',
    'class="inline text-dim-magenta"',
  ],
  [
    'style="display: inline;color: rgb(81, 159, 80);text-decoration-line: underline;text-decoration-style: solid;"',
    'class="inline text-dim-green"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(51, 49, 42);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(95, 95, 95);"',
    'class="inline text-dim-grey"',
  ],
  [
    'style="display: inline;color: rgb(145, 129, 117);"',
    'class="inline text-mid-grey"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);font-weight: bold;"',
    'class="inline font-bold text-white"',
  ],
  [
    'style="display: inline;color: rgb(145, 129, 117);font-style: italic;"',
    'class="inline font-italic text-mid-grey"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);"',
    'class="inline text-white"',
  ],
  [
    'style="display: inline;color: rgb(48, 48, 48);"',
    'class="inline text-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);background-color: rgb(63, 0, 1);"',
    'class="inline text-white bg-deep-red"',
  ],
  [
    'style="display: inline;color: rgb(239, 47, 39);background-color: rgb(63, 0, 1);"',
    'class="inline bg-deep-red text-dim-red"',
  ],
  [
    'style="display: inline;background-color: rgb(63, 0, 1);"',
    'class="inline bg-deep-red"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);background-color: rgb(0, 40, 0);"',
    'class="inline text-white bg-deep-green"',
  ],
  [
    'style="display: inline;color: rgb(239, 47, 39);background-color: rgb(0, 40, 0);"',
    'class="inline text-dim-red bg-deep-green"',
  ],
  [
    'style="display: inline;color: rgb(224, 44, 109);font-weight: bold;"',
    'class="inline font-bold text-dim-magenta"',
  ],
  [
    'style="display: inline;background-color: rgb(0, 40, 0);"',
    'class="inline bg-deep-green"',
  ],
  [
    'style="display: inline;color: rgb(215, 0, 95);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="inline font-bold text-magenta bg-mid-black"',
  ],
  [
    'style="display: inline;background-color: rgb(48, 48, 48);"',
    'class="inline bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(228, 228, 228);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="inline font-bold bg-mid-black text-white"',
  ],
  [
    'style="display: inline;color: rgb(239, 47, 39);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="inline font-bold text-dim-red bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(28, 27, 25);font-weight: bold;"',
    'class="inline font-bold text-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(186, 166, 127);background-color: rgb(224, 44, 109);"',
    'class="inline bg-dim-magenta text-dim-white"',
  ],
  [
    'style="display: inline;color: rgb(95, 215, 255);"',
    'class="inline text-cyan"',
  ],
  [
    'style="display: inline;color: rgb(0, 255, 135);"',
    'class="inline text-green"',
  ],
  [
    'style="display: inline;background-color: rgb(239, 47, 39);"',
    'class="inline bg-dim-red"',
  ],
  [
    'style="display: inline;color: rgb(224, 44, 109);text-decoration-line: underline;text-decoration-style: solid;font-weight: bold;"',
    'class="inline font-bold text-dim-magenta"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);font-weight: bold;font-style: italic;"',
    'class="inline font-bold font-italic text-white"',
  ],
  [
    'style="display: inline;color: rgb(239, 47, 39);font-weight: bold;font-style: italic;"',
    'class="inline font-bold font-italic text-dim-red"',
  ],
  [
    'style="display: inline;color: rgb(186, 166, 127);font-style: italic;"',
    'class="inline font-italic text-dim-white"',
  ],
  [
    'style="display: inline;color: rgb(158, 186, 194);"',
    'class="inline text-mid-grey"',
  ],
  [
    'style="display: inline;color: rgb(44, 120, 191);background-color: rgb(63, 0, 1);"',
    'class="inline bg-deep-red text-dim-blue"',
  ],
  [
    'style="display: inline;color: rgb(44, 120, 191);background-color: rgb(0, 40, 0);"',
    'class="inline bg-deep-green text-dim-blue"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);background-color: rgb(81, 159, 80);font-weight: bold;"',
    'class="inline font-bold bg-dim-green text-white"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);background-color: rgb(239, 47, 39);font-weight: bold;"',
    'class="inline font-bold text-white bg-dim-red"',
  ],
  [
    'style="display: inline;color: rgb(239, 47, 39);background-color: rgb(239, 47, 39);font-weight: bold;"',
    'class="inline font-bold text-dim-red bg-dim-red"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);font-weight: bold;"',
    'class="inline font-bold text-cyan"',
  ],
  [
    'style="display: inline;color: rgb(224, 44, 109);text-decoration-line: underline;text-decoration-style: solid;font-style: italic;"',
    'class="inline font-italic text-dim-magenta"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(145, 129, 117);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-grey"',
  ],
  [
    'style="display: inline;color: rgb(10, 174, 179);background-color: rgb(28, 27, 25);font-weight: bold;"',
    'class="inline font-bold text-dim-cyan bg-black"',
  ],
  [
    'style="display: inline;color: rgb(252, 232, 195);background-color: rgb(145, 129, 117);font-weight: bold;"',
    'class="inline font-bold text-white bg-mid-grey"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(78, 69, 63);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-dim-grey"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(44, 42, 38);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(51, 51, 50);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(51, 49, 45);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(51, 49, 41);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(51, 49, 49);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(43, 228, 208);background-color: rgb(44, 38, 38);font-weight: bold;"',
    'class="inline font-bold text-cyan bg-mid-black"',
  ],
  [
    'style="display: inline;color: rgb(135, 175, 215);font-weight: bold;"',
    'class="inline font-bold text-blue"',
  ],
  [
    'style="display: inline;color: rgb(135, 175, 215);"',
    'class="inline text-blue"',
  ],
  [
    'style="display: inline;color: rgb(135, 175, 135);"',
    'class="inline text-dim-green"',
  ],
  [
    'style="display: inline;color: rgb(175, 215, 175);background-color: rgb(48, 48, 48);font-weight: bold;"',
    'class="inline font-bold bg-mid-black text-green"',
  ],
  [
    'style="display: inline;color: rgb(117, 113, 94);"',
    'class="inline text-grey"',
  ],
  [
    'style="font-family: monospace; white-space: pre;background-color: #1c1b19;color: #fce8c3;"',
    'class="font-mono whitespace-pre bg-black text-white"',
  ],
  ['style="display: inline;opacity: 0.5;"', 'class="inline opacity-50"'],
  [
    'style="display: inline;color: rgb(239, 47, 39);"',
    'class="inline text-dim-red"',
  ],
  [
    'style="display: inline;color: rgb(81, 159, 80);"',
    'class="inline text-dim-green"',
  ],
  [
    'style="display: inline;color: rgb(10, 174, 179);"',
    'class="inline text-dim-cyan"',
  ],
  [
    'style="display: inline;color: rgb(44, 120, 191);"',
    'class="inline text-dim-blue"',
  ],
  [
    'style="display: inline;color: rgb(251, 184, 41);"',
    'class="inline text-dim-yellow"',
  ],
  ['style="display: inline;font-weight: bold;"', 'class="inline font-bold"'],
  [
    'style="display: inline;font-weight: bold;opacity: 0.5;"',
    'class="inline font-bold opacity-50"',
  ],
  [
    'style="display: inline;text-decoration-line: underline;text-decoration-style: solid;"',
    'class="inline"',
  ],
  [
    'style="display: inline;color: rgb(239, 47, 39);font-weight: bold;"',
    'class="inline text-dim-red font-bold"',
  ],
  [
    'style="display: inline;color: rgb(81, 159, 80);font-weight: bold;"',
    'class="inline text-dim-green font-bold"',
  ],
  [
    'style="display: inline;color: rgb(44, 120, 191);font-weight: bold;"',
    'class="inline text-dim-blue font-bold"',
  ],
  [
    'style="display: inline;color: rgb(251, 184, 41);font-weight: bold;"',
    'class="inline text-dim-yellow font-bold"',
  ],
  [
    'style="display: inline;color: rgb(145, 129, 117);font-weight: bold;"',
    'class="inline text-mid-grey font-bold"',
  ],
  [
    'style="display: inline;color: rgb(251, 184, 41);text-decoration-line: underline;text-decoration-style: solid;font-weight: bold;"',
    'class="inline text-dim-yellow font-bold"',
  ],
] as const

export const mapStyles = (term: string): { ok: boolean; res: string } => {
  let ok = true
  let res = STYLEMAP.reduce(
    (acc, [style, twclass]) => acc.replaceAll(style, twclass),
    term
  )
  let remains = new Set(res.matchAll(/style="[^"]+"/g).map(m => m[0]))
  if (remains.size) {
    ok = false
    console.error(`Unhandled style declarations!`)
    for (let rem of remains) {
      console.log("  " + rem)
    }
  }
  return { ok, res }
}
