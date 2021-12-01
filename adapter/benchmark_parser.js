const fs = require('fs');
const parse = require("postcss/lib/parse");

const css = fs.readFileSync(`assets/${process.argv[2]}`).toString();
const start = process.hrtime();
parse(css, { map: false });
console.log(process.hrtime(start)[1]);
