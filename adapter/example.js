const postcss = require("postcss");
const fs = require('fs');

const file = fs.readFileSync('../assets/bootstrap.css').toString()
const code = `
/**
 * Paste or drop some CSS here and explore
 * the syntax tree created by chosen parser.
 * Enjoy!
 */

@media screen and (min-width: 480px) {
    body, resulkt, .result {
        background-color: lightgreen;
    }
}

#main {
    border: 1px solid black;
}

ul li {
	padding: 5px;
}

`;

function normalize(node) {
  if (node?.source?.inputId != undefined) {
    delete node.source.inputId;
  }

  if (node.inputs) {
    delete node.inputs;
  }
  if (node.nodes) {
    node.nodes.forEach(n => {
      normalize(n);
    });
  }
  return node
}

console.time('label')
const root = postcss.parse(file).root()
root.toString()
console.timeEnd('label')

// const normalizeRoot = normalize(root.toJSON())
// console.log(JSON.stringify(normalizeRoot))

// console.log(normalizeRoot.toJSON())

