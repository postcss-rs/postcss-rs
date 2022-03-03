import init, { ast } from "postcss-rs-wasm";
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
      // console.log();
      interactiveAst.innerHTML = `<pre>${ast(editor.state.doc.text.join("\n"))}</pre>`;
    },
  });

  interactiveAst.innerHTML = `<pre>${ast(editor.state.doc.text.join("\n"))}</pre>`;
})();
