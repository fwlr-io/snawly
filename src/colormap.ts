const COLORS: Record<string, string> = {
  "rgb(0, 255, 135)": "green",
  "rgb(0, 40, 0)": "deep-green",
  "rgb(10, 174, 179)": "dim-cyan",
  "rgb(117, 113, 94)": "grey",
  "rgb(135, 175, 135)": "dim-green",
  "rgb(135, 175, 215)": "blue",
  "rgb(145, 129, 117)": "grey",
  "rgb(158, 186, 194)": "mid-grey",
  "rgb(175, 215, 175)": "green",
  "rgb(186, 166, 127)": "mid-grey",
  "rgb(215, 0, 95)": "magenta",
  "rgb(224, 44, 109)": "dim-magenta",
  "rgb(228, 228, 228)": "dim-white",
  "rgb(239, 47, 39)": "dim-red",
  "rgb(251, 184, 41)": "dim-yellow",
  "rgb(252, 232, 195)": "dim-white",
  "rgb(28, 27, 25)": "dim-grey",
  "rgb(43, 228, 208)": "cyan",
  "rgb(44, 120, 191)": "dim-blue",
  "rgb(44, 38, 38)": "mid-black",
  "rgb(48, 48, 48)": "mid-black",
  "rgb(44, 42, 38)": "mid-black",
  "rgb(51, 49, 41)": "mid-black",
  "rgb(51, 49, 42)": "mid-black",
  "rgb(51, 49, 45)": "mid-black",
  "rgb(51, 49, 49)": "mid-black",
  "rgb(51, 51, 50)": "mid-black",
  "rgb(63, 0, 1)": "deep-red",
  "rgb(78, 69, 63)": "dim-grey",
  "rgb(81, 159, 80)": "dim-green",
  "rgb(95, 215, 255)": "cyan",
  "rgb(95, 95, 95)": "dim-grey",
}

const colorFrom = (rgb: string): string => {
  let res = COLORS[rgb]
  if (res === undefined) {
    console.error(rgb)
    return "[rgb(240,240,255)]"
  }
  return res
}

export const styleToClass = (atr: string, val: string): string[] =>
  atr === "background-color" ? [`bg-${colorFrom(val)}`]
  : atr === "color" ? [`text-${colorFrom(val)}`]
  : atr.startsWith("font-") ? [`font-${val}`]
  : []
