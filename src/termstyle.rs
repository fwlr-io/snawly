// replacements
//
// [
//  '<div style="display: inline;',
//  '<span style="'
// ]
//
// [
//  /<\/div>(?!$)/g,
//  "</span>"
// ]
//
// [
//   /href="zed:\/\/file\/\/Users\/scottfowler([^"]*)"/g,
//   "onclick='alert(\"opens `~$1` in your editor\");'",
// ]
//
// [
//   'style="font-family: monospace; white-space: pre;background-color: #1c1b19;color: #fce8c3;"',
//   'class="whitespace-pre"',
// ]

pub fn restyle() {
    eprintln!("not yet implemented")
}
