const fs = require('fs');
const Input = require("postcss/lib/input");
const tokenize = require("postcss/lib/tokenize");

const css = fs.readFileSync(`assets/${process.argv[2]}`).toString();
const start = process.hrtime();
const input = new Input(css);
const processor = tokenize(input, { ignoreErrors: false });
while (!processor.endOfFile()) {
    processor.nextToken();
}
console.log(process.hrtime(start)[1]);
