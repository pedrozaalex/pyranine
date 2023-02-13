<div align="center">

  <h1><code>pyranine</code></h1>
<sub>Built with 🦀 + 🕸

</div>

## About

This is a simple library publishing a WASM module that can be used to highlight code in JavaScript, through the use of [syntect](https://github.com/trishume/syntect).

The name pyranine comes from the [dye](https://en.wikipedia.org/wiki/Pyranine) commonly used in commercial yellow (paper) highlighters (and also in soup lol).

## Usage

```ts
import initHighlighter, * as wasm from "pyranine";

const code = "const foo = 'bar';";

/* The following themes are available:
"base16-ocean.dark",
"base16-eighties.dark",
"base16-mocha.dark",
"base16-ocean.light",
"InspiredGitHub",
"Solarized (dark)",
"Solarized (light)",
*/

function tryHighlightStringAsHTML(
  code: string,
  language: string,
  theme: string
) {
  return new Promise()<string>((resolve) => {
    initHighlighter().then(() =>
      resolve(wasm.highlight(code, language, theme))
    );
  });
}

tryHighlightStringAsHTML(code, "javascript", "base16-ocean.dark").then((c) =>
  console.log(c)
);
```

## Note

You may get an warning saying "pyranine doesn't appear to be written in CJS...". This is because the library is written in Rust and compiled to WASM, and the tooling around that is still a bit immature. You can safely ignore this warning.
