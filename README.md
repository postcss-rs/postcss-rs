# postcss-rs

🚀 Fast and 100% API compatible postcss replacer, built in Rust

> ⚠️ DO NOT USE. STILL WORK IN PROGRESS.

## Performance Improvement 

Tokenize [bootstrap.css](./assets/bootstrap.css) (Compare with Node.js v14.18.1):

```bash
rust: 0.00s user 0.00s system  71% cpu 0.006 total
js:   0.11s user 0.02s system 126% cpu 0.107 total

# tokenize bootstrap-reboot.css ~34x
js:   tokenizer/small(7K)       : 2.248ms
rust: tokenizer/small(7K)       : 0.061ms

# tokenize bootstrap.css ~26x
js:   tokenizer/small(201K)     : 29.577ms
rust: tokenizer/small(201K)     :  1.127ms
```

🎉 Welcome contribute, here is a guide:

```bash
git checkout main
cargo bench -- --save-baseline main
```

Create a baseline, Then do some optimization work here.

```bash
git checkout feature  ## `feature` is your branch 
```

Compare against the `main` baseline.

```bash
cargo bench -- --baseline main
```

## License

[postcss-rs](https://github.com/justjavac/postcss-rs) is released under the
MIT License. See the bundled [LICENSE](./LICENSE) file for details.

**Most implementation of postcss-rs are modified from [postcss](https://github.com/postcss/postcss), under MIT License(See the origin [POSTCSS.LICENSE](./POSTCSS.LICENSE) file), thanks to Andrey Sitnik([@ai](https://github.com/ai)).**
