# snawly
## Makes codeblocks on [https://fwlr.io]. 

Uses [tree-sitter](https://tree-sitter.github.io/tree-sitter) to markup code blocks with highlighting, output as html, and automates the drudgery and boilerplate of making them available to leptos.

Want it to be build-time: most syntax-highlighting-on-your-website options out there are _heavy_, this only requires shipping the end-result html.

Achieved: go from a file at `src/interesting_component.rs` to writing `<codeblock::InterestingComponent />` and have it all Just Work.

Next: using `tree-sitter-cli` to select relevant chunks of the file, e.g. something like `snawly ./src/test.rs @Foo` to find and pull out the `pub fn Foo() ->  { ... }` block.

etymology: "scarf" + "gnaw", diminutive -ly
