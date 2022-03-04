import init, { ast, gen } from "postcss-rs-wasm";
import { EditorState } from "@codemirror/next/state";
import { css } from "@codemirror/next/lang-css";
import { EditorView, basicSetup } from "@codemirror/next/basic-setup";
import { tagExtension, EditorSelection } from "@codemirror/next/state";
// import { astStringToAst, generateHtmlFromAstNode } from "./ast";
const code = `
.a {
  width: 100px;
  height: 200px;
  background: #ccc;
}
`.trim();
(async () => {
  const interactiveAst = document.getElementById("interactive-ast");
  const output = document.getElementById("output");
  await init();
  const languageTag = Symbol("language");

  const editor = new EditorView({
    state: EditorState.create({
      doc: code,
      extensions: [
        basicSetup,
        tagExtension(languageTag, css()),
        // autoLanguage,
      ],
      // selection: EditorSelection.create([EditorSelection.range(58, 101), EditorSelection.cursor(101)]),
    }),
    parent: document.querySelector("#editor"),
    dispatch: t => {
      editor.update([t]);
      let sourceCode = editor.state.doc.text.join("\n");
      output.innerHTML = gen(sourceCode);
      interactiveAst.innerHTML = `<pre>${ast(sourceCode)}</pre>`;
    },
  });

  let sourceCode = editor.state.doc.text.join("\n");
  interactiveAst.innerHTML = `<pre>${ast(sourceCode)}</pre>`;
  output.innerHTML = gen(sourceCode);
})();